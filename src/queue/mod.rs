use super::*;

use serde_json::{Error, Value};

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
            .http_client.core
            .run(get)
            .unwrap();
        
        let response_body = String::from_utf8(res.to_vec()).unwrap();
        let v: Value = serde_json::from_str(&response_body).unwrap();

        let queue_info: QueueInfo = serde_json::from_value(v["queue"].clone()).unwrap();

        queue_info
    }

    pub fn push_message(&mut self, message: &str) -> String {
        let path = format!("{}queues/{}/messages", self.client.base_path, self.name);
        let mut req = Request::new(Method::Post, path.parse().unwrap());
        req.headers_mut().set(ContentType::json());

        let authorization_header = format!("OAuth {}", self.client.token);
        req.headers_mut().set(Authorization(authorization_header));

        let message = json!(
            {
                "messages": [
                    {
                        "body": message
                    }
                ]
            }
        );

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

        let response_body = String::from_utf8(res.to_vec()).unwrap();
        let response: Value = serde_json::from_str(&response_body).unwrap();
        response["ids"][0].to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueInfo {
    pub name: String,
    pub project_id: String,
    pub message_timeout: u32,
    pub message_expiration: u32,
    #[serde(rename = "type")] 
    pub queue_type: String,
    pub size: Option<usize>,
    pub total_messages: Option<usize>,
}
