use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufRead, BufReader};
#[allow(unused_imports)]


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let lines = &mut buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();

    let resp_200 = "HTTP/1.1 200 OK\r\n\r\n";
    let resp_404 = "HTTP/1.1 404 Not Found\r\n\r\n";

    let uri = request_line.split_whitespace().nth(1).unwrap_or("/");
    let status_line = if uri == "/" {
        resp_200
    }else if uri.starts_with("/echo") {
        resp_200
    }else if uri.starts_with("/user-agent") {
        resp_200
    }else {
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
    }else {
        String::new()
    };
    
    let response = format!(
        "{status_line}\r\nContent-Type: text/plain\r\nContent-Length: {len}\r\n\r\n{user_agent}",
        len=body.len(),
        user_agent=body.clone()
    );
    stream.write_all(response.as_bytes()).unwrap();
        
}