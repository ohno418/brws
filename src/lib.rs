mod url;

use crate::url::URL;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn request(url: &str) -> Result<(), String> {
    let url = URL::new(url)?;

    // Create a socket.
    let ip_addr = format!("{}:80", url.host);
    let mut stream = match TcpStream::connect(&ip_addr) {
        Ok(stream) => stream,
        Err(_) => return Err(format!("Cannot connect to {}", ip_addr)),
    };

    // Send a request.
    let request = format!(
        "GET {} HTTP/1.0\r\n\
         HOST: {}\r\n\
         \r\n",
        url.path, url.host
    );
    if stream.write_all(request.as_bytes()).is_err() {
        return Err("Failed to send a request".into());
    }

    // Receive a response.
    let mut response = String::new();
    if stream.read_to_string(&mut response).is_err() {
        return Err("Failed to receiver a response".into());
    };

    // Parse the response headers.
    let mut res_lines = response.split("\r\n");
    // status line:
    let status_line = res_lines
        .next()
        .expect("Failed to get HTTP response status line");
    let (version, rest) = status_line
        .split_once(" ")
        .expect("Expected a space in a status line");
    let (status, explanation) = rest
        .split_once(" ")
        .expect("Expected a space in a status line");
    // other headers:
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in res_lines {
        if line.is_empty() {
            break;
        }

        let (key, value) = line
            .split_once(" ")
            .expect("Expected a space in a resonse headers");
        headers.insert(key.into(), value.into());
    }

    dbg!(&response);
    dbg!(&(version, status, explanation));
    dbg!(&headers);

    Ok(())
}
