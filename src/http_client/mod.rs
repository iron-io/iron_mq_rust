extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate num_cpus;

use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

pub struct HttpClient {
    pub core: tokio_core::reactor::Core,
    pub handle: tokio_core::reactor::Handle,
    pub client: Client<HttpsConnector<HttpConnector>>,
}

impl HttpClient {
     pub fn new() -> HttpClient {
        let num_cpus = num_cpus::get();
        let core = tokio_core::reactor::Core::new().expect("Tokio core initialization error");
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(num_cpus, &handle).expect("Https connector initialization error"))
            .build(&handle);
    
        HttpClient {
            core: core,
            handle: handle,
            client: client,
        }
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn https_request() {
        let mut http_client = HttpClient::new();
        let req = http_client.client.get("https://hyper.rs".parse().unwrap());
        let res = http_client.core.run(req).unwrap();
        assert!(res.status().is_success());
    }
}