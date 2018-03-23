mod http_client;
mod queue;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use futures::{Future, Stream};
use hyper::{Method, Request};
use hyper::header::{Authorization, ContentType};
use http_client::*;
use serde_json::{Error, Value};
use std::env;
use std::collections::HashMap;

use queue::*;

pub struct Client {
    pub base_path: String,
    http_client: HttpClient,
    token: String,
}

impl Client {
    pub fn new(host: String, project_id: String, token: String) -> Client {
        let base_path = format!("https://{}/3/projects/{}/", host, project_id);
        let http_client = HttpClient::new();

        Client {
            base_path,
            http_client,
            token,
        }
    }

    pub fn from_env() -> Client {
        let host = get_from_env("IRON_HOST");
        let project_id = get_from_env("IRON_PROJECT_ID");
        let token = get_from_env("IRON_TOKEN");

        Client::new(host, project_id, token)
    }
    
    pub fn queue(&mut self, name: String) -> Queue {
        Queue {
            client: self,
            name
        }
    }

    pub fn create_queue(&mut self, name: &String) -> QueueInfo {
        let config = HashMap::new();
        Client::create_queue_with_config(self, name, &config)
    }

    pub fn create_queue_with_config(&mut self, name: &String, config: &HashMap<String, String>) -> QueueInfo {
        let path = format!("{}queues/{}", self.base_path, name).parse().expect("Incorrect path");
        let mut req = Request::new(Method::Put, path);
        req.headers_mut().set(ContentType::json());

        let authorization_header = format!("OAuth {}", self.token);
        req.headers_mut().set(Authorization(authorization_header));

        let body = json!({
            "queue": config
        });

        req.set_body(body.to_string());
        let put = self.http_client
            .client
            .request(req)
            .and_then(|res| res.body().concat2());

        let res = self
            .http_client
            .core
            .run(put)
            .unwrap();

        let response_body = String::from_utf8(res.to_vec()).unwrap();
        let v: Value = serde_json::from_str(&response_body).unwrap();
        let queue_info: QueueInfo = serde_json::from_value(v["queue"].clone()).unwrap();

        queue_info
    }
}

fn get_from_env(variable: &str) -> String {
    let error_message = format!("Missed {} environment variable!", variable);

    let var = env::var(variable)
        .ok()
        .and_then(|p| p.parse().ok())
        .expect(&error_message);

    var
}
