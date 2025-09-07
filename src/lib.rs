use std::{
    io::{
        self,
        Read
    }, 
    net::{
        TcpListener, 
        TcpStream
    }
};
use crate::{
    middlewares::Middlewares, 
    request::HttpRequest, 
    response::HttpResponse
};

pub mod request;
pub mod response;
pub mod middlewares;

pub struct Http {
    middlewares: Middlewares
}

impl Http  {
    pub fn new() -> Self {
        Http {
            middlewares: Middlewares::new()
        }
    }

    pub fn create() -> Self {
        Self::new()
    }

    pub fn listen(&mut self, port: u16, host: &str, on_start: fn()) -> io::Result<()> {
        let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port)).expect("Не удалось создать сервер");
        on_start();

        for stream in listener.incoming() {
            Self::requests_handler(&self, &mut stream?);
        }

        Ok(())
    }

    pub fn use_middleware<F: Fn(&HttpRequest, &mut HttpResponse) + 'static + Send + Sync>(&mut self, middleware: F) {
        self.middlewares.add(middleware);
    }

    fn requests_handler(&self, stream: &mut TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];

        stream.read(&mut buffer).expect("Не удалость записать поток в буффер");
        let request: String = String::from_utf8_lossy(&buffer[..]).to_string();
        let mut response: HttpResponse = HttpResponse::new(stream);
        let request: HttpRequest = HttpRequest::new(&request);
        
        self.middlewares.execute(&request, &mut response);
    }
}