use std::io::Result;
use http::Http;
use http::router::request::HttpRequest;
use http::router::response::HttpResponse;

fn main() -> Result<()> {
    let mut server = Http::new();
    server.listen(8080, "127.0.0.1", || println!("Сервер запущен"));
    
    Ok(())
}