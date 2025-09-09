![Static Badge](https://img.shields.io/badge/MIT-e9c7bc?label=license) ![Static Badge](https://img.shields.io/badge/Rust-bfd3ed?label=lang)
# Http
Simple framework for creating web-servers written in Rust

### Features
- Middlewares
- Cors
- Routing
- Cookies

### Example
```rust
use std::io::Result;
use http::{extras::cors::CorsOptions, Http, router::{request::HttpRequest, response::HttpResponse}};

fn main() -> Result<()> {
    let mut server: Http = Http::new();

    server.use_middleware(CorsOptions::new().cors());

    server.get("/", |req: &HttpRequest, res: &mut HttpResponse| {
        println!("{:?}", req.headers);
        res.send_file("./static/index.html");
    });

    server.get("/about", |req: &HttpRequest, res: &mut HttpResponse| {
        res.send_file("./static/about.html");
    });

    server.listen(8080, "127.0.0.1", || println!("Сервер запущен"));
    
    Ok(())
}
```