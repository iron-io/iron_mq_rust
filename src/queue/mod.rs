pub mod queue_info;
pub mod message;

use serde_json::Value;

use super::*;
use message::{Message, ReservationConfig};

pub struct Queue<'a> {
    pub client: &'a mut Client,
    pub name: String,
}

impl<'a> Queue<'a> {
    pub fn info(&mut self) -> QueueInfo {
        let path = format!("{}queues/{}", self.client.base_path, self.name);

        let res = self.client
            .http_client
            .request(Method::Get, path, String::new());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let queue_info: QueueInfo = serde_json::from_value(v["queue"].clone()).unwrap();

        queue_info
    }

    pub fn push_message(&mut self, message: Message) -> Result<String, String> {
        let messages: Vec<Message> = vec![message];
        let mut ids = match self.push_messages(messages) {
            Ok(ids) => ids,
            Err(e) => return Err(e),
        };

        Ok(ids.pop().unwrap())
    }

    pub fn push_messages(&mut self, messages: Vec<Message>) -> Result<Vec<String>, String> {
        let path = format!("{}queues/{}/messages", self.client.base_path, self.name);

        let message = json!({ "messages": messages });

        let res = self.client
            .http_client
            .request(Method::Post, path, message.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let ids: Vec<String> = match serde_json::from_value(v["ids"].clone()) {
            Ok(ids) => ids,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(ids)
    }

    pub fn push_string(&mut self, body: &str) -> Result<String, String> {
        let message = Message::with_body(body);
        self.push_message(message)
    }

    pub fn push_strings(&mut self, bodies: Vec<&str>) -> Result<Vec<String>, String> {
        let messages = bodies.into_iter()
            .map(|b| {
                Message::with_body(b)
            }).collect();
        self.push_messages(messages)
    }

    pub fn get_message(&mut self, id: &String) -> Result<Message, String> {
        let path = format!(
            "{}queues/{}/messages/{}",
            self.client.base_path, self.name, id
        );

        let res = self.client
            .http_client
            .request(Method::Get, path, String::new());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let message: Message = match serde_json::from_value(v["message"].clone()) {
            Ok(message) => message,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(message)
    }

    pub fn long_poll(&mut self, count: u8, timeout: u32, wait: u32, delete: bool) -> Result<Vec<Message>, String> {
        let path = format!("{}queues/{}/reservations", self.client.base_path, self.name);

        let reservation_config = json!(ReservationConfig::new(count, timeout, wait, delete));

        let res =
            self.client
                .http_client
                .request(Method::Post, path, reservation_config.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let messages: Vec<Message> = match serde_json::from_value(v["messages"].clone()) {
            Ok(messages) => messages,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(messages)
    }

    pub fn reserve_messages_with_timeout(
        &mut self,
        count: u8,
        timeout: u32,
    ) -> Result<Vec<Message>, String> {
        let default_wait = 0;
        let delete = false;

        self.long_poll(count, timeout, default_wait, delete)
    }

    pub fn reserve_message_with_timeout(&mut self, timeout: u32) -> Result<Message, String> {
        let default_count = 1;

        let mut messages = match self.reserve_messages_with_timeout(default_count, timeout) {
            Ok(messages) => messages,
            Err(e) => return Err(e),
        };

        Ok(messages.pop().unwrap())
    }

    pub fn reserve_messages(&mut self, count: u8) -> Result<Vec<Message>, String> {
        let default_timeout = 60;
        self.reserve_messages_with_timeout(count, default_timeout)
    }

    pub fn reserve_message(&mut self) -> Result<Message, String> {
        let default_timeout = 60;
        self.reserve_message_with_timeout(default_timeout)
    }

    pub fn pop_message(&mut self) -> Result<Message, String> {
        let default_count = 1;

        let mut messages = match self.pop_messages(default_count) {
            Ok(messages) => messages,
            Err(e) => return Err(e)
        };

        Ok(messages.pop().unwrap())
    }

    pub fn pop_messages(&mut self, count: u8) -> Result<Vec<Message>, String> {
        let delete = true;
        self.long_poll(count, 0, 0, delete) 
    }

    pub fn release_message(&mut self, message: Message, delay: u32) -> String {
        let path = format!(
            "{}queues/{}/messages/{}/release",
            self.client.base_path,
            self.name,
            message.id.unwrap()
        );

        let reservation_id = message.reservation_id.expect("Missed reservation id");
        let body = json!({
            "reservation_id": reservation_id,
            "delay": delay
        });

        let res = self.client
            .http_client
            .request(Method::Post, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let msg = v["msg"].to_string();

        msg
    }

    pub fn delete_message(&mut self, message: Message) -> String {
        let message_id = message.id.expect("Missed message id");
        let path = format!(
            "{}queues/{}/messages/{}",
            self.client.base_path, self.name, message_id
        );

        let reservation_id = message.reservation_id.expect("Missed reservation id");
        let body = json!({ "reservation_id": reservation_id });

        let res = self.client
            .http_client
            .request(Method::Delete, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let msg = v["msg"].to_string();

        msg
    }

    pub fn delete_messages(&mut self, messages: Vec<Message>) -> String {
        let path = format!("{}queues/{}/messages", self.client.base_path, self.name)
            .parse()
            .unwrap();

        let ids: Vec<Value> = messages
            .into_iter()
            .map(|m| {
                json!({
                    "id": m.id,
                    "reservation_id": m.reservation_id
                })
            })
            .collect();

        let body = json!({ "ids": ids });

        let res = self.client
            .http_client
            .request(Method::Delete, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let msg = v["msg"].to_string();

        msg
    }

    pub fn touch_message_with_timeout(&mut self, message: Message, timeout: u32) -> Result<String, String> {
        let message_id = message.id.expect("Missed message id");
        let path = format!(
            "{}queues/{}/messages/{}/touch",
            self.client.base_path, self.name, message_id
        );

        let reservation_id = message.reservation_id.expect("Missed reservation id");
        let body = json!({
            "reservation_id": reservation_id,
            "timeout": timeout
        });

        let res = self.client
            .http_client
            .request(Method::Post, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let new_reservation_id: String = match serde_json::from_value(v["reservation_id"].clone()) {
            Ok(reservation_id) => reservation_id,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(new_reservation_id)
    }

    pub fn touch_message(&mut self, message: Message) -> Result<String, String> {
        let default_timeout = 60;
        self.touch_message_with_timeout(message, default_timeout)
    }

    pub fn peek_messages(&mut self, count: u8) -> Result<Vec<Message>, String> {
        let path = format!(
            "{}queues/{}/messages?n={}",
            self.client.base_path, self.name, count
        );

        let res = self.client
            .http_client
            .request(Method::Get, path, String::new());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let messages: Vec<Message> = match serde_json::from_value(v["messages"].clone()) {
            Ok(messages) => messages,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(messages)
    }
    
    pub fn add_subscribers(&mut self, subscribers: Vec<QueueSubscriber>) -> String {
        let path = format!("{}queues/{}/subscribers", self.client.base_path, self.name);
        
        let body = json!({
            "subscribers": subscribers
        });

        let res = self.client
            .http_client
            .request(Method::Post, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let msg = v["msg"].to_string();

        msg
    }

    pub fn replace_subscribers(&mut self, subscribers: Vec<QueueSubscriber>) -> String {
        let path = format!("{}queues/{}/subscribers", self.client.base_path, self.name);
        
        let body = json!({
            "subscribers": subscribers
        });

        let res = self.client
            .http_client
            .request(Method::Put, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let msg = v["msg"].to_string();

        msg
    }

    pub fn remove_subscribers(&mut self, subscribers: Vec<QueueSubscriber>) -> String {
        let path = format!("{}queues/{}/subscribers", self.client.base_path, self.name);

        let body = json!({
            "subscribers": subscribers
        });

        let res = self.client
            .http_client
            .request(Method::Delete, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let msg = v["msg"].to_string();

        msg
    }


    pub fn update(&mut self, config: &QueueInfo) -> Result<QueueInfo, String> {
        let path = format!("{}queues/{}", self.client.base_path, self.name)
            .parse()
            .expect("Incorrect path");

        let body = json!({ "queue": config });

        let res = self.client
            .http_client
            .request(Method::Patch, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();

        let queue_info: QueueInfo = match serde_json::from_value(v["queue"].clone()) {
            Ok(queue_info) => queue_info,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(queue_info)
    }

    pub fn clear(&mut self) -> String {
        let path = format!("{}queues/{}/messages", self.client.base_path, self.name);

        let body = json!({});

        let res = self.client
            .http_client
            .request(Method::Delete, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let msg = v["msg"].to_string();

        msg
    }

    pub fn delete(&mut self) {
        let path = format!("{}queues/{}", self.client.base_path, self.name);

        let _res = self.client
            .http_client
            .request(Method::Delete, path, String::new());
    }
}
