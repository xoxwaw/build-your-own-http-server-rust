#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{Write, BufRead, BufReader};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                
                // Read the request line
                let reader = BufReader::new(&stream);
                let request_line = reader.lines().next().unwrap().unwrap();
                
                // Parse the path from the request line
                let path = request_line.split_whitespace().nth(1).unwrap_or("/");
                let endpoint = path.split("/").nth(1).unwrap_or("");
                let echo_content = path.split("/").nth(2).unwrap_or("");
                // Create response based on path
                let (status_line, body) = if endpoint == "echo" {
                    ("HTTP/1.1 200 OK", echo_content)
                } else if path == "/" {
                    ("HTTP/1.1 200 OK", "\r\n\r\n")
                } else {
                    ("HTTP/1.1 404 Not Found", "Not Found")
                };
                
                // Write the response
                let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, body.len(), body);
                
                // Write the response to the stream
                
                let response = format!(
                    "{}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    status_line,
                    body.len(),
                    body
                );
                
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    println!("Error writing to stream: {}", e);
                }
                if let Err(e) = stream.flush() {
                    println!("Error flushing stream: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
