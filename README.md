# gato-http-server

This package provider the ability host a local HTTP/1.1 server.

- **Do not use in production without a proxy** (nginx for example).
- This package is in BETA, use carefully.

## Usage

```
use gato_http_server::HttpServerServiceProvider;

service_provider.register_provider(HttpServerServiceProvider::new());
```