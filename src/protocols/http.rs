use std::net::TcpStream;
use std::io::{Read, Write};

pub fn load(url: &str) -> String {
    // separate url into host and path
    let (host, path) = url.split_once("/").unwrap();
    
    // connect the tcpstream
    let mut stream = TcpStream::connect(format!("{}:80", host)).unwrap();
    
    // GET request
    stream.write(format!("GET /{} HTTP/1.0\r\n", path).as_bytes()).unwrap();

    // Host header (required)
    stream.write(format!("Host: {}\r\n", host).as_bytes()).unwrap();

    // end request
    stream.write(b"\r\n").unwrap();

    // return full http response
    let mut response = String::new();
    let _ = stream.read_to_string(&mut response);
    return response;
}
