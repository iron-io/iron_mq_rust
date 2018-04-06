pub mod http_client;
pub mod queue;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use hyper::{ Method };
use serde_json::{Value};
use std::env;

use queue::*;
use queue::queue_info::*;
use http_client::*;

const PER_PAGE: u8 = 30;

pub struct Client {
    pub base_path: String,
    http_client: HttpClient
}

impl Client {
    pub fn new(host: String, project_id: String, token: String) -> Client {
        let base_path = format!("https://{}/3/projects/{}/", host, project_id);
        let http_client = HttpClient::new(token);

        Client {
            base_path,
            http_client,
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
        let config = QueueInfo::new(name.to_string());
        Client::create_queue_with_config(self, name, &config)
    }

    pub fn queue_list(&mut self, prefix: &str, prev: &str, per_page: u8) -> Vec<QueueInfo> {
        let path = format!("{}queues?prefix={}&prev={}&per_page={}", self.base_path, prefix, prev, per_page);

        let res = self.http_client.request(Method::Get, path, "".to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
        let queues_info: Vec<QueueInfo> = serde_json::from_value(v["queues"].clone()).unwrap();

        queues_info
    }

    pub fn list_page(&mut self, prev: &str, per_page: u8) -> Vec<QueueInfo> {
        self.queue_list("", prev, per_page)
    }

    pub fn filter(&mut self, prefix: &str) -> Vec<QueueInfo> {
        self.queue_list(prefix, "", PER_PAGE)
    }

    pub fn list(&mut self) -> Vec<QueueInfo> {
        self.queue_list("", "", PER_PAGE)
    }

    pub fn create_queue_with_config(&mut self, name: &String, config: &QueueInfo) -> QueueInfo {
        let path = format!("{}queues/{}", self.base_path, name);
        
        let body = json!({
            "queue": config
        });

        let res = self.http_client.request(Method::Put, path, body.to_string());

        let v: Value = serde_json::from_slice(&res).unwrap();
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
