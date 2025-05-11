use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, BufRead, BufReader};
use std::thread;
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
    let mut buf_reader = BufReader::new(&stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();
    request_line = request_line.trim_end().to_string();

    let request_parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = request_parts[0];
    let uri = request_parts[1];

    let resp_200 = "HTTP/1.1 200 OK";
    let resp_404 = "HTTP/1.1 404 Not Found";
    let mut content_type = "text/plain".to_string();
    let mut content_length = 0;
    let mut status_line = if uri == "/" {
        resp_200
    }else if uri.starts_with("/echo") {
        resp_200
    }else if uri.starts_with("/files") {
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

    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        let l = line.trim_end().to_string();
        if l.is_empty() {
            break;
        }
        headers.push(l.clone());
        println!("line: {}", l);
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
    
    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {len}\r\n\r\n{user_agent}",
        status_line=status_line,
        content_type=content_type,
        len=body.len(),
        user_agent=body.clone()
    );
    stream.write_all(response.as_bytes()).unwrap();
        
}