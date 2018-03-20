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
    
}

