use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, BufRead, BufReader};
use std::thread;
use std::time::{Duration, Instant};
#[allow(unused_imports)]


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let args: Vec<String> = std::env::args().collect();
    let mut directory = "/tmp/".to_string();

    for i in 0..args.len() {
        if args[i] == "--directory" && i + 1 < args.len() {
            directory = args[i + 1].clone();
            break;
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let dir_clone = directory.clone();
                thread::spawn(move || {
                    handle_client(_stream, dir_clone);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_client(mut stream: TcpStream, directory: String) {
    stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    loop {
        let mut buf_reader = BufReader::new(&stream);
        let mut request_line = String::new();

        match buf_reader.read_line(&mut request_line) {
            Ok(0) => break, 
            Ok(_) => {},
            Err(_) => break,
        }
        
        request_line = request_line.trim_end().to_string();
        if request_line.is_empty() {
            break;
        }


        let request_parts: Vec<&str> = request_line.split_whitespace().collect();
        if request_parts.len() < 2 {
            break;
        }
        let method = request_parts[0];
        let uri = request_parts[1];
        println!("method: {}", method);
        println!("uri: {}", uri);

        let resp_200 = "HTTP/1.1 200 OK";
        let resp_404 = "HTTP/1.1 404 Not Found";
        let mut content_type = "text/plain".to_string();
        let mut content_length = 0;
        let mut status_line = if uri == "/" {
            resp_200
        }else if uri.starts_with("/echo") {
            resp_200
        }else if uri.starts_with("/files") && method == "GET" {
            content_type = "application/octet-stream".to_string();
            resp_200
        }else if uri.starts_with("/user-agent") {
            resp_200
        } else if method == "POST" && uri.starts_with("/files") {
            "HTTP/1.1 201 Created"
        } else {
            resp_404
        };

        let mut user_agent = String::new();
        let mut headers = Vec::new();
        let mut keep_alive = false;

        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line).unwrap();
            let l = line.trim_end().to_string();
            if l.is_empty() {
                break;
            }
            headers.push(l.clone());
            println!("line: {}", l);

            if l.to_lowercase().starts_with("connection") {
                let connection_value = l.split_once(": ").unwrap_or(("", "")).1.to_string();
                keep_alive = connection_value.contains("keep-alive");

                if connection_value.contains("close") {
                    keep_alive = false;
                }
            }
            if !headers.iter().any(|h| h.to_lowercase().starts_with("connection")) {
                if request_parts.len() > 2  && request_parts[2].contains("keep-alive") {
                    keep_alive = true;
                }
            }
            if l.starts_with("User-Agent") {
                user_agent = l.split_once(": ").unwrap_or(("", "")).1.to_string();
            }
            if l.starts_with("Content-Length") {
                content_length = l.split_once(": ").unwrap_or(("", "")).1.to_string().parse::<usize>().unwrap_or(0);
            }
        }

        // Read body of the POST request
        let mut body = String::new();
        if method == "POST" && content_length > 0 {
            // Read exactly content_length bytes
            let mut buffer = vec![0; content_length];
            buf_reader.read_exact(&mut buffer).unwrap_or_else(|e| {
                println!("Error reading body: {}", e);
                String::new();
            });
            body = String::from_utf8_lossy(&buffer).to_string();
            println!("body: {}", body);
        }
        let request_uri: &str = request_line.split_whitespace().nth(1).unwrap();
        let body = if method == "GET"{
            if request_uri.starts_with("/echo") {
                request_uri.split("/").nth(2).unwrap_or("").to_string()
            }else if request_uri.starts_with("/user-agent") {
                user_agent.clone()
            }else if request_uri.starts_with("/files") {
                let file_path = request_uri.split("/").nth(2).unwrap_or("");
                if let Ok(content) = std::fs::read_to_string(format!("{}{}", directory, file_path)) {
                    status_line = resp_200;
                    content
                } else {
                    status_line = resp_404;
                    "File not found".to_string()
                }
            }else {
                String::new()
            }
        } else if method == "POST" && request_uri.starts_with("/files") {
            let file_path = request_uri.split("/").nth(2).unwrap_or("");
            if !file_path.is_empty() {
                if let Ok(_content) = std::fs::write(format!("{}{}", directory, file_path), body) {
                    println!("File created: {}", file_path);
                    println!("status_line: {}", status_line);
                    String::new()
                } else {
                    println!("Failed to create file: {}", file_path);
                    status_line = "HTTP/1.1 501 Internal Server Error";
                    "Failed to create file".to_string()
                }
            } else {
                status_line = "HTTP/1.1 400 Bad Request";
                "Invalid file path".to_string()
            }
        } else {
            String::new()
        };

        let connection_header = if keep_alive {
            "Connection: keep-alive\r\n"
        } else {
            "Connection: close\r\n"
        };
        
        let response = format!(
            "{status_line}\r\nContent-Type: {content_type}\r\n{connection}Content-Length: {len}\r\n\r\n{user_agent}",
            status_line=status_line,
            content_type=content_type,
            connection=connection_header,
            len=body.len(),
            user_agent=body.clone()
        );
        if let Err(_) = stream.write_all(response.as_bytes()) {
            break;
        }

        stream.flush().unwrap_or(());

        if !keep_alive {
            break;
        }
    }
}