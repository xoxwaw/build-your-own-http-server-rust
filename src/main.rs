use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, BufReader};
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
    let buf_reader = BufReader::new(&stream);
    let lines = &mut buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();

    let resp_200 = "HTTP/1.1 200 OK";
    let resp_404 = "HTTP/1.1 404 Not Found";
    let mut content_type = "text/plain".to_string();

    let uri = request_line.split_whitespace().nth(1).unwrap_or("/");
    let mut status_line = if uri == "/" {
        resp_200
    }else if uri.starts_with("/echo") {
        resp_200
    }else if uri.starts_with("/files") {
        content_type = "application/octet-stream".to_string();
        resp_200
    }else if uri.starts_with("/user-agent") {
        resp_200
    } else {
        resp_404
    };

    let mut user_agent = String::new();
    
    for line in lines {
        let line_ = line.unwrap();
        let l = &line_;
        if l.is_empty() {
            break;
        }
        if l.starts_with("User-Agent") {
            user_agent = l.split_once(": ").unwrap_or(("", "")).1.to_string();
        }
    }

    let request_uri: &str = request_line.split_whitespace().nth(1).unwrap();
    let body = if request_uri.starts_with("/echo") {
        request_uri.split("/").nth(2).unwrap_or("").to_string()
    }else if request_uri.starts_with("/user-agent") {
        user_agent.clone()
    }else if request_uri.starts_with("/files"){
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