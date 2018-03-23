use super::*;

use serde_json::{Error, Value};

pub struct Queue<'a> {
    pub client: &'a mut Client,
    pub name: String,
}

impl<'a> Queue<'a> {
    pub fn info(&mut self) -> String {
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

        String::from_utf8(res.to_vec()).unwrap()
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

        let response: Value = serde_json::from_slice(&res).unwrap();
        response["ids"][0].to_string()
    }
}