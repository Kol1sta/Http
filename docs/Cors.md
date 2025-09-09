# Cors
```rs
pub struct CorsOptions {
    origin: Vec<String>,
    methods: Vec<String>,
    allowed_headers: Vec<String>,
    credentials: bool,
    max_age: Option<u32>
}
```

### Methods
Create new object
```rs
pub fn new() -> Self
```

Create new object
```rs
pub fn create() -> Self
```

Use cors in middleware
```rs
pub fn cors(self) -> Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>
```

Set cors origins
```rs
pub fn origin(mut self, origin: Vec<String>) -> Self
```

Set cors methods
```rs
pub fn methods(mut self, methods: Vec<String>) -> Self
```

Set cors credentials
```rs
pub fn credentials(mut self, credentials: bool) -> Self
```

Set cors max age
```rs
pub fn max_age(mut self, max_age: Option<u32>) -> Self
```

### Usage
```rs
server.use_middleware(CorsOptions::new()
    .credentials(true)
    .origin(vec!["*".to_string())
    .cors()
);
```