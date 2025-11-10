use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // Listen on port 7878 (same as before)
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running at http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Turn request into a string for inspection
    let request = String::from_utf8_lossy(&buffer[..]);

    // --- Route matching ---
    let (status_line, filename) = if request.starts_with("GET / ") {
        ("HTTP/1.1 200 OK", "index.html")
    } else if request.starts_with("GET /about") {
        ("HTTP/1.1 200 OK", "about.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // --- Load file and build response ---
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        String::from("<h1>404 Not Found</h1><p>The page does not exist.</p>")
    });

    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
