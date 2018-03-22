mod http_client;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::{Method, Request};
use hyper::header::{Authorization, ContentType};
use http_client::*;

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

    pub fn get_queue_info(&mut self, queue_id: &str) -> String {
        let path = format!("{}queues/{}", self.base_path, queue_id);
        let mut req = Request::new(Method::Get, path.parse().unwrap());
        req.headers_mut().set(ContentType::json());
        let authorization_header = format!("OAuth {}", self.token);
        req.headers_mut().set(Authorization(authorization_header));

        let get = self.http_client
            .client
            .request(req)
            .and_then(|res| res.body().concat2());
        let res = self.http_client.core.run(get).unwrap();

        String::from_utf8(res.to_vec()).unwrap()
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
