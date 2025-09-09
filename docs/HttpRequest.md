# HttpRequest
```rs
pub struct HttpRequest {
    pub req_string: String,
    pub route: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: HashMap<String, String>
}
```

### Methods
Create new object
```rs
pub fn new(req: &str) -> Self
```

Create new object
```rs
pub fn create(req: &str) -> Self
```