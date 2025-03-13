use std::{collections::HashMap, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, env};
use serde_json;

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or("8000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

       handle_connection(stream);
    }
}


fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let request_line = &http_request[0];

    if request_line == "GET /ping HTTP/1.1" {
        println!("Request ping: {}", request_line);
        get_ping(http_request, stream);
    } 
    else {
        println!("Request other: {}", request_line);
        get_404_page(stream);
    }    
}


fn get_ping(http_request: Vec<String>, mut stream: TcpStream) {
    let mut headers = HashMap::new();

    for line in http_request {
        let line = line;
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    let json = serde_json::to_string(&headers).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let content_length = json.len();
    let response = format!(
        "{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_line, content_length, json
    );

    stream.write_all(response.as_bytes()).unwrap();
}


fn get_404_page(mut _stream: TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let content_length = 0;

    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n");
    _stream.write_all(response.as_bytes()).unwrap();
}