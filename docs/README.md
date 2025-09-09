# Get Started
Http is simple framework for creating web-servers written in Rust. For start you need make server and start listening

```rs
use http::Http;

fn main() -> Result<()> {
    let mut server = Http::new();
    server.listen(8080, "127.0.0.1", || println!("Server was started"));
}
```

### Middlewares
You can create your middlewares or use their from http/extras (Cookies, Cors). Every http request will go from first to last middleware

Example
```rs
use http::Http;

use http::extras::{cookies::CookiesOptions, cors::CorsOptions};

fn main() -> Result<()> {
    let mut server = Http::new();

    server.use_middleware(CorsOptions::new().cors());
    server.use_middleware(CookiesOptions::new().secure(true).cookies());

    server.listen(8080, "127.0.0.1", || println!("Server was started"));
}
```

### Routing
You can use built-in routing or get method from HttpRequest object

```rs
use http::Http;

use http::extras::{cookies::CookiesOptions, cors::CorsOptions};

fn main() -> Result<()> {
    let mut server = Http::new();

    server.get("/", |req: &HttpRequest, res: &mut HttpResponse| {
        res.send("Hello, world!");
    });

    // or
    server.use_middleware(|req: &HttpRequest, res: &mut HttpResponse| {
        if req.method == "get" && req.route == "/" {
            res.send("hello, world")
        }
    });

    server.listen(8080, "127.0.0.1", || println!("Server was started"));
}
```