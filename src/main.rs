use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

use crate::http::{parse_start_line, parser::StartLine};

mod http;

#[tokio::main]
async fn main() -> () {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("HTTP server listening on 127.0.0.1:8080");
    
    loop {
        let (socket, address) = listener.accept().await.unwrap();
        println!("Connection from: {}", address);

        tokio::spawn(async move {
            process_socket(socket).await;
        });
    }
}

async fn process_socket(mut socket: TcpStream) {
    let mut buffer = [0u8; 8192];
    match socket.read(&mut buffer).await {
        Ok(n) => {
            let request_str = String::from_utf8_lossy(&buffer[..n]);
            let lines: Vec<&str> = request_str.lines().collect();

            if lines.is_empty() {
                let error_response = "HTTP/1.1 400 Bad Request\r\n\r\nEmpty request";
                socket.write_all(error_response.as_bytes()).await.unwrap();
                return;
            }

            let start_line = parse_start_line(lines[0]);
            match start_line {
                Ok(first_line) => {
                    match first_line {
                        StartLine::RequestLine(req) => {
                            println!("Method {:?}, Path: {}", req.method, req.request_target);
                            match req.method {
                                crate::http::parser::Method::GET => {
                                    let body = "Hello World!";
                                    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
                                    socket.write_all(response.as_bytes()).await.unwrap();
                                },
                                crate::http::parser::Method::HEAD => {
                                    // HEAD is like GET but without body
                                    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\n";
                                    socket.write_all(response.as_bytes()).await.unwrap();
                                },
                            }
                        },
                        StartLine::StatusLine(_) => {
                            let error_response = "HTTP/1.1 400 Bad Request\r\n\r\nUnexpected status line";
                            socket.write_all(error_response.as_bytes()).await.unwrap();
                        }
                    };
                },
                Err(status_code) => {
                    let response = format!("HTTP/1.1 {} Error\r\n\r\nRequest parsing failed", status_code as u16);
                    socket.write_all(response.as_bytes()).await.unwrap();
                },
            };
        }
        Err(e) => {
            println!("Failed to read from socket: {}", e);
        }
    }
}