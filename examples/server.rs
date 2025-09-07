use std::io::Result;
use http::Http;
use http::request::HttpRequest;
use http::response::HttpResponse;

fn main() -> Result<()> {
    let mut server = Http::new();

    server.use_middleware(|req: &HttpRequest, res: &mut HttpResponse| {
        match req.method.as_str() {
            "GET" => match req.route.as_str() {
                "/" => {
                    res.send_file("./static/index.html")
                },
                _ => res.send("./static/404.html")
            },
            "POST" => match req.route.as_str() {
                "/api/getProfile" => {
                    println!("{:?}", req.body[0].key);
                    if req.body[0].key.to_string() == "aboba" {
                        res.send("chill bro");
                    }
                },
                _ => res.send("./static/kitachan.png")
            }
            _ => {}
        }
    });

    server.listen(8080, "127.0.0.1", || println!("Сервер запущен"));
    
    Ok(())
}