use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::collections::HashMap;
use gato_core::kernel::{Logger, RequestBuilder, RouterHandler};

fn code_to_text(code: i32) -> String {
    match code {
        100 => "Continue",
        101 => "Switching Protocols",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        305 => "Use Proxy",
        307 => "Temporary Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Time-out",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Request Entity Too Large",
        414 => "Request-URI Too Large",
        415 => "Unsupported Media Type",
        416 => "Requested range not satisfiable",
        417 => "Expectation Failed",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Time-out",
        505 => "HTTP Version not supported",
        _ => "EITA"
    }.to_string()
}

fn parse_header(header: &str) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();
    let lines: Vec<&str> = header.split("\r\n").collect();
    for line in lines {
        let res: Vec<&str> = line.split(":").collect();
        if res.len() > 0 {
            let key = res[0].to_string();
            if res.len() == 2 {
                headers.insert(key, res[1].to_string());
            } else {
                headers.insert(key, "".to_owned());
            }
        }
    }
    return headers;
}

/**
* Receive from the TcpStream all data Written by the client
**/
fn read_stream(stream: &mut TcpStream) -> String {
    let buffer_size = 512;
    let mut request_buffer: Vec<u8> = vec![];
    // Is used to crop the request_buffer to prevent null termination
    let mut request_size: usize = 0;
    loop {
        let mut buffer = vec![0; buffer_size];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    request_size += n;
                    request_buffer.append(buffer.as_mut());
                    /*
                    * IF n is LESS then buffer_size that mean that
                    * this is the last part of the message.
                    **/
                    if n < buffer_size {
                        break;
                    }
                }
            }
            Err(e) => {
                Logger::error(format!("Received error {:?}", e).as_str());
                break;
            }
        }
    }
    return std::str::from_utf8(&request_buffer[0..request_size]).unwrap().to_string();
}

fn handle_client(mut stream: TcpStream) {
    let request = read_stream(&mut stream);

    //println!("REQUEST:\n{}", request);

    let header_index = request.find("\r\n\r\n").unwrap();
    let (header, mut body) = request.split_at(header_index);

    if body.len() >= 4 {
        body = &body[4..];
    }

    let (protocol, header) = header.split_at(header.find("\r\n").unwrap());

    let protocol: Vec<&str> = protocol.split(" ").collect();

    let headers = parse_header(header);

    // Get RouterHandler Driver
    let router_handler = RouterHandler::get_driver();

    let mut request = RequestBuilder::new();
    request.add_headers(headers);
    request.add_body(body.to_string());
    request.add_method(protocol[0].to_string());
    request.add_uri(protocol[1].to_string());

    let response = router_handler.handle(&mut request);

    Logger::info(format!("{} [{}]: {}", stream.peer_addr().unwrap(), response.get_code(), protocol[1]).as_str());

    let mut headers_response = format!("HTTP/1.1 {} {}\r\nServer: Gato Framework\r\n", response.get_code(), code_to_text(response.get_code()));


    for (key, val) in response.get_headers() {
        headers_response += format!("{}: {}\r\n", key, val).as_str();
    }

    headers_response += format!("\r\n{}", response.get_body()).as_str();

    //println!("response is:\n{}", headers_response);

    stream.write(headers_response.as_bytes()).unwrap();
    stream.flush().unwrap();

    stream.shutdown(Shutdown::Both).unwrap();
}

pub fn start_server() {
    let port = std::env::var("PORT").unwrap_or("3333".to_owned());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    // accept connections and process them, spawning a new thread for each one
    Logger::info("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                Logger::info(format!("New connection: {}", stream.peer_addr().unwrap()).as_str());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                Logger::info(format!("Error: {}", e).as_str());
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}