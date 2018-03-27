pub mod queue_info;
pub mod message;

use serde_json::{Value};

use super::*;
use message::Message;
use message::ReservationConfig;

pub struct Queue<'a> {
    pub client: &'a mut Client,
    pub name: String,
}

impl<'a> Queue<'a> {
    pub fn info(&mut self) -> QueueInfo {
        let path = format!("{}queues/{}", self.client.base_path, self.name);
        let mut req = Request::new(Method::Get, path.parse().unwrap());
        req.headers_mut().set(ContentType::json());

        let authorization_header = format!("OAuth {}", self.client.token);
        req.headers_mut().set(Authorization(authorization_header));

        let get = self.client
            .http_client
            .client
            .request(req)
            .and_then(|res| res.body().concat2());

        let res = self.client
            .http_client
            .core
            .run(get)
            .unwrap();

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
        let mut req = Request::new(Method::Post, path.parse().unwrap());
        req.headers_mut().set(ContentType::json());

        let authorization_header = format!("OAuth {}", self.client.token);
        req.headers_mut().set(Authorization(authorization_header));

        let message = json!({
            "messages": messages
        });

        req.set_body(message.to_string());

        let post = self.client
            .http_client
            .client
            .request(req)
            .and_then(|res| res.body().concat2());

        let res = self.client
            .http_client
            .core
            .run(post)
            .unwrap();

        let v: Value = serde_json::from_slice(&res).unwrap();
        let ids: Vec<String> = match serde_json::from_value(v["ids"].clone()) {
            Ok(ids) => ids,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(ids)
    }

    pub fn get_message(&mut self, id: &String) -> Result<Message, String> {
        let path = format!("{}queues/{}/messages/{}", self.client.base_path, self.name, id);
        let mut req = Request::new(Method::Get, path.parse().unwrap());
        req.headers_mut().set(ContentType::json());

        let authorization_header = format!("OAuth {}", self.client.token);
        req.headers_mut().set(Authorization(authorization_header));

        let get = self.client
            .http_client
            .client
            .request(req)
            .and_then(|res| res.body().concat2());

        let res = self.client
            .http_client
            .core
            .run(get)
            .unwrap();

        let v: Value = serde_json::from_slice(&res).unwrap();
        let message: Message = match serde_json::from_value(v["message"].clone()) {
            Ok(message) => message,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(message)
    }

    pub fn long_poll(&mut self, count: u8, timeout: u32, wait: u32, delete: bool) -> Result<Vec<Message>, String> {
        let path = format!("{}queues/{}/reservations", self.client.base_path, self.name);
        let mut req = Request::new(Method::Post, path.parse().unwrap());
        req.headers_mut().set(ContentType::json());

        let authorization_header = format!("OAuth {}", self.client.token);
        req.headers_mut().set(Authorization(authorization_header));

        let reservation_config = json!(
            ReservationConfig::new(count, timeout, wait, delete)
        );

        req.set_body(reservation_config.to_string());

        let get = self.client
            .http_client
            .client
            .request(req)
            .and_then(|res| res.body().concat2());

        let res = self.client
            .http_client
            .core
            .run(get)
            .unwrap();

        let v: Value = serde_json::from_slice(&res).unwrap();
        let messages: Vec<Message> = match serde_json::from_value(v["messages"].clone()) {
            Ok(messages) => messages,
            Err(_) => return Err(v["msg"].to_string()),
        };

        Ok(messages)
    }

    pub fn reserve_messages_with_timeout(&mut self, count: u8, timeout: u32) -> Result<Vec<Message>, String> {
        let default_wait = 0;
        let delete = false;

        self.long_poll(count, timeout, default_wait, delete)
    }

    pub fn reserve_message_with_timeout(&mut self, timeout: u32) -> Result<Message, String> {
        let default_count = 1;
        
        let mut messages = match self.reserve_messages_with_timeout(default_count, timeout) {
            Ok(messages) => messages,
            Err(e) => return Err(e)
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

    pub fn update(&mut self, config: &QueueInfo) -> Result<QueueInfo, String> {
        let path = format!("{}queues/{}", self.client.base_path, self.name).parse().expect("Incorrect path");
        let mut req = Request::new(Method::Patch, path);
        req.headers_mut().set(ContentType::json());

        let authorization_header = format!("OAuth {}", self.client.token);
        req.headers_mut().set(Authorization(authorization_header));

        let body = json!({
            "queue": config
        });

        req.set_body(body.to_string());
        let patch = self.client
            .http_client
            .client
            .request(req)
            .and_then(|res| res.body().concat2());

        let res = self.client
            .http_client
            .core
            .run(patch)
            .unwrap();

        let v: Value = serde_json::from_slice(&res).unwrap();
        
        let queue_info: QueueInfo = match serde_json::from_value(v["queue"].clone()) {
            Ok(queue_info) => queue_info,
            Err(_) => return Err(v["msg"].to_string()),
        };
        
        Ok(queue_info)
    }

    pub fn delete(&mut self) {
        let path = format!("{}queues/{}", self.client.base_path, self.name);
        let mut req = Request::new(Method::Delete, path.parse().unwrap());
        req.headers_mut().set(ContentType::json());

        let authorization_header = format!("OAuth {}", self.client.token);
        req.headers_mut().set(Authorization(authorization_header));

        let delete = self.client
            .http_client
            .client
            .request(req)
            .and_then(|res| res.body().concat2());

        self.client
            .http_client
            .core
            .run(delete)
            .unwrap();
    }

}
