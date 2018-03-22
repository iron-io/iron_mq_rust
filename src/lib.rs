mod http_client;
mod queue;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;

use futures::{Future, Stream};
use hyper::{Method, Request};
use hyper::header::{Authorization, ContentType};
use http_client::*;
use queue::*;

use std::env;

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
    
    pub fn queue(&mut self,name: String) -> Queue {
        Queue {
            client: self,
            name
        }
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
