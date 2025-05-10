#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::Write;
fn main() {

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let body = "Hello, World!";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                std::thread::sleep(std::time::Duration::from_secs(2));
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
