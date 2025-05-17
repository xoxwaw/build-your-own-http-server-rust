# Rust HTTP Server Implementation

This project is a from-scratch implementation of an HTTP/1.1 server in Rust. It demonstrates how to build low-level networking applications while learning Rust's powerful features for systems programming.

## What I've Built

A fully functional HTTP/1.1 server that:
- Handles concurrent client connections using Rust's threading model
- Supports persistent connections (keep-alive)
- Implements proper HTTP response generation
- Serves static files from a directory
- Processes different HTTP methods (GET, POST)
- Handles request routing and URI parsing

## Why Rust for Low-Level Projects

Rust is an ideal language for building systems-level applications:
- Memory safety without garbage collection
- Thread safety through ownership and borrowing
- Zero-cost abstractions for high performance
- Expressive type system that prevents entire classes of bugs

This HTTP server showcases these strengths by handling concurrent connections with memory safety guarantees that would be difficult to ensure in languages like C or C++.

## Running the Server

```sh
# Clone the repository
git clone <repository-url>
cd rust-http-server

# Run the server (serves files from /tmp by default)
cargo run

# Specify a custom directory
cargo run -- --directory /path/to/files
```

## Testing the Server

You can interact with the server using standard HTTP tools:

```sh
# Make a basic request
curl -v http://localhost:4221/echo/hello

# Make multiple requests over a single connection
curl --http1.1 -v http://localhost:4221/echo/one http://localhost:4221/echo/two

# Upload a file
curl -X POST -d "content" http://localhost:4221/files/example.txt
```

## What I Learned

Building this project deepened my understanding of:
- TCP socket programming
- HTTP protocol details and implementation
- Rust's concurrency model
- Ownership and borrowing in thread contexts
- Error handling in network applications
- Proper resource management for server applications



