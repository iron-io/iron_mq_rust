mod http_client;

#[macro_use]
extern crate dotenv_codegen;
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
    pub fn new(host: &str, project_id: &str, token: &str) -> Client {
        let base_path = format!("https://{}/3/projects/{}/", host, project_id);
        let http_client = HttpClient::new();

        Client {
            base_path: base_path,
            http_client: http_client,
            token: token.to_string(),
        }
    }

    pub fn from_env() -> Client {
        let host = dotenv!("IRON_HOST");
        let project_id = dotenv!("IRON_PROJECT_ID");
        let token = dotenv!("IRON_TOKEN");

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
