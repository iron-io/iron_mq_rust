mod http_client;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::{Method, Request};
use hyper::header::{ContentType, Authorization};
use http_client::*;

pub struct IronMQ {
    pub base_path: String,
    http_client: HttpClient,
    token: String
}

impl IronMQ {
    pub fn new(host: &str, project_id: &str, token: &str) -> IronMQ {
        let base_path = format!("https://{}/3/projects/{}/", host,project_id);
        let http_client = HttpClient::new();

        IronMQ {
            base_path: base_path,
            http_client: http_client,
            token: token.to_string()
        }
    }
}

