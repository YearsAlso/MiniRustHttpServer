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
    let request_lines = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_lines == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response = format!(
        "{status_line}\r\n \
        Content-Length:{length} \r\n\r\n\
        {content}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
