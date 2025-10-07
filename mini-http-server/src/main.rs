use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").unwrap();

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

    println!("Request: {:?}", http_request)
}
