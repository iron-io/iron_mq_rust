extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate num_cpus;
extern crate tokio_core;

use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use hyper::{Method, Request};
use hyper::header::{Authorization, ContentType};
use futures::{Future, Stream};

pub struct HttpClient {
    core: tokio_core::reactor::Core,
    client: Client<HttpsConnector<HttpConnector>>,
    token: String
}

impl HttpClient {
    pub fn new(token: String) -> HttpClient {
        let num_cpus = num_cpus::get();
        let core = tokio_core::reactor::Core::new().expect("Tokio core initialization error");
        let handle = core.handle();
        let connector = HttpsConnector::new(num_cpus, &handle).expect("Https connector initialization error");

        let client = Client::configure()
            .connector(connector)
            .build(&handle);

        HttpClient {
            core,
            client,
            token
        }
    }

    pub fn request(&mut self, method: Method, path: String, body: String) -> hyper::Chunk {
        let mut req = Request::new(method, path.parse().expect("Request error"));

        let authorization_header = format!("OAuth {}", self.token);

        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(Authorization(authorization_header));

        req.set_body(body);

        let work = self
            .client
            .request(req)
            .and_then(|res| res.body().concat2());

        self.core.run(work).expect("Request error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn https_request() {
        let uri = "https://hyper.rs".parse().unwrap();
        let mut http_client = HttpClient::new("some-token".to_string());
        let req = http_client.client.get(uri);
        let res = http_client.core.run(req).unwrap();
        assert!(res.status().is_success());
    }
}
