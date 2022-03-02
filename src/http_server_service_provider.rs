use gato_core::kernel::HttpCoreHandler;
use gato_core::kernel::Provider;
use crate::HttpServerHttpCore;

pub struct HttpServerServiceProvider { }

impl HttpServerServiceProvider {
    pub fn new() -> Box<Self> {
        return Box::new(Self {  });
    }
}

impl Provider for HttpServerServiceProvider {
    fn boot(&self) {
        let http_server_http_core = HttpServerHttpCore::new();
        HttpCoreHandler::set_driver(Box::new(http_server_http_core));
    }
}

