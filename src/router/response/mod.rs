use std::net::TcpStream;
use std::io::{Write, prelude::*};
use std::fs::File;
use get_content_type::get_content_type;

use crate::router::response::cookies::Cookie;
use crate::router::response::cookies::Cookies;
use crate::router::response::headers::Headers;

pub mod get_content_type;
pub mod headers;
pub mod cookies;

#[derive(Debug)]
pub struct HttpResponse<'a> {
    stream: &'a mut TcpStream,
    pub headers: Headers,
    pub cookies: Cookies
}

impl<'a> HttpResponse<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        HttpResponse { stream, headers: Headers::new(), cookies: Cookies::new() }
    }

    pub fn create(stream: &'a mut TcpStream) -> Self {
        Self::new(stream)
    }

    pub fn send(&mut self, res: &str) {
        self.headers.set_headers("Content-Type", "text/plain");
        self.headers.set_headers("Content-Length", res.len().to_string().as_str());
        
        let response: String = format!("{}{}", self.headers.build_headers_string(), res);
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send_html(&mut self, content: &str) {
        self.headers.set_headers("Content-Type", "text/html");
        self.headers.set_headers("Content-Length", content.len().to_string().as_str());

        let response: String = format!("{}{}", self.headers.build_headers_string(), content);
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send_json(&mut self, content: &str) {
        self.headers.set_headers("Content-Type", "application/json");
        self.headers.set_headers("Content-Length", content.len().to_string().as_str());

        let response: String = format!("{}{}", self.headers.build_headers_string(), content);
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send_file(&mut self, path: &str) {
        let mut file: File = File::open(path).expect("Не удалось открыть файл");
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Не удалось записать данные из файла");

        self.headers.set_headers("Content-Type", get_content_type(path));
        self.headers.set_headers("Content-Length", buffer.len().to_string().as_str());

        let response: String = format!("{}", self.headers.build_headers_string());
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.write_all(&buffer).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn redirect(&mut self, location: &str) {
        self.headers.delete_headers("Content-Type");

        self.headers.status_code = 302;
        self.headers.set_headers("Location", location);
        self.headers.set_headers("Content-Length", "0");

        let response: String = format!("{}", self.headers.build_headers_string());
        
        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn set_cookie(&mut self, cookie: Cookie) {
        self.cookies.add(cookie);
        self.headers.set_headers("Set-Cookie", &self.cookies.build_cookie());
    }
}