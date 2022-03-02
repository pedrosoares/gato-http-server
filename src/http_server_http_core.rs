use std::collections::HashMap;
use gato_core::kernel::{HttpCore};
use crate::http_driver::start_server;


pub struct HttpServerHttpCore { }

impl HttpCore for HttpServerHttpCore {

    fn handle(&self) {
        start_server();
    }

    fn get_request_headers(&self) -> HashMap<String, String> {
        return HashMap::new();
    }

    fn get_post_data(&self) -> String {
        return "".to_owned();
    }
}

impl HttpServerHttpCore {
    pub fn new() -> Self {
        return Self { };
    }
}