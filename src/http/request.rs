use std::collections::HashMap;
use std::net::{ TcpStream };
use std::io::{ BufRead, BufReader };

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn parse_request(stream: &TcpStream) -> Option<Self> {
        let mut reader = BufReader::new(stream);

        let mut request_line = String::new();
        reader.read_line(&mut request_line).ok()?;
        let request_line = request_line.trim();

        let mut parts = request_line.splitn(3, ' ');
        let method    = parts.next()?.to_string();
        let path      = parts.next()?.to_string();
        let version   = parts.next()?.to_string();

        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).ok()?;

            if line == "\r\n" || line == "\n" {
                break;
            }
        
            if let Some((key, value)) = line.trim().split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        };

        let body = if let Some(len) = headers.get("Content-Length") {
            let len: usize = len.trim().parse().ok()?;
            let mut body = vec![0u8; len];
            std::io::Read::read_exact(&mut reader, &mut body).ok()?;
            body
        } else {
            Vec::new()
        };

        Some( Self {
            method,
            path,
            version,
            headers,
            body
        } )
    }
}