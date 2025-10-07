use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    println!("Connection established!")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap();
    let length = content.len();

    let response = format!(
        "{status_line}\r\n \
        Content-Length:{length} \r\n\r\n\
        {content}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
