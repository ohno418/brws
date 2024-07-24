mod html;
mod response;
mod url;

use crate::html::parse_html;
use crate::response::parse_response;
use crate::url::parse_url;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn load(url: &str) -> Result<(), String> {
    let url = parse_url(url)?;

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
    let mut res_buf = String::new();
    if stream.read_to_string(&mut res_buf).is_err() {
        return Err("Failed to receiver a response".into());
    };
    let response = parse_response(&res_buf)?;

    // Parse a HTML body and print it.
    print!("{}", parse_html(&response.body));

    Ok(())
}
