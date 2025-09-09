# HttpResponse
```rs
pub struct HttpResponse<'a> {
    stream: &'a mut TcpStream,
    pub headers: Headers,
    pub cookies: Cookies
}
```

### Methods
Create new object
```rs
pub fn new(stream: &'a mut TcpStream) -> Self
```

Create new object
```rs
pub fn create(stream: &'a mut TcpStream) -> Self
```

Send text message
```rs
pub fn send(&mut self, res: &str)
```

Send html
```rs
pub fn send_html(&mut self, content: &str)
```

Send json data
```rs
pub fn send_json(&mut self, content: &str)
```

Send file from path
```rs
pub fn send_file(&mut self, path: &str)
```

Redirect to location
```rs
pub fn redirect(&mut self, location: &str)
```

Set new cookie
```rs
pub fn set_cookie(&mut self, cookie: Cookie)
```