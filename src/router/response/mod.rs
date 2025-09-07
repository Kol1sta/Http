use std::net::TcpStream;
use std::io::{Write, prelude::*};
use std::fs::File;
use get_content_type::get_content_type;

use crate::router::response::headers::Headers;

pub mod get_content_type;
pub mod headers;

#[derive(Debug)]
pub struct HttpResponse<'a> {
    stream: &'a mut TcpStream,
    pub headers: Headers
}

impl<'a> HttpResponse<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        HttpResponse { stream, headers: Headers::new() }
    }

    pub fn create(stream: &'a mut TcpStream) -> Self {
        Self::new(stream)
    }

    pub fn send(&mut self, res: &str) {
        self.headers.set_headers("Content-Type", "text/plain");
        self.headers.set_headers("Content-Length", res.len().to_string().as_str());
        
        let response: String = format!("{}{}", self.headers.build_headers_string(), res);
        println!("{}", response);
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send_status(&mut self, status_code: u16, message: &str) {
        let status_text: &str = match status_code {
            200 => "OK",
            201 => "Created",
            400 => "Bad Request",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        };
        
        let response: String = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            status_text,
            message.len(),
            message
        );
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send_html(&mut self, content: &str) {
        let response: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send_json(&mut self, content: &str) {
        let response: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send_file(&mut self, path: &str) {
        let mut file: File = File::open(path).expect("Не удалось открыть файл");
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Не удалось записать данные из файла");

        let response: String = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            get_content_type(path),
            buffer.len()
        );
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.write_all(&buffer).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn redirect(&mut self, location: &str) {
        let response: String = format!(
            "HTTP/1.1 302 Found\r\nLocation: {}\r\nContent-Length: 0\r\n\r\n",
            location
        );
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}