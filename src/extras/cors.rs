use crate::{router::{request::HttpRequest, response::HttpResponse}, Http};

pub struct CorsOptions {
    origin: Vec<String>,
    methods: Vec<String>,
    allowed_headers: Vec<String>,
    credentials: bool,
    max_age: Option<u32>
}

impl CorsOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create() -> Self {
        Self::default()
    }
    
    pub fn default() -> Self {
        CorsOptions {
            origin: vec!["*".to_string()],
            methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string(), "OPTIONS".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            credentials: false,
            max_age: Some(86400)
        }
    }

    pub fn cors(self) -> Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync> {
        Box::new(move |req: &HttpRequest, res: &mut HttpResponse| {
            res.headers.set_headers("Access-Control-Allow-Origin", &self.origin.join(", "));
            res.headers.set_headers("Access-Control-Allow-Methods", &self.methods.join(", "));
            res.headers.set_headers("Access-Control-Allow-Headers", &self.allowed_headers.join(", "));

            if self.credentials {
                res.headers.set_headers("Access-Control-Allow-Credentials", "true");
            }

            if let Some(max_age) = self.max_age {
                res.headers.set_headers("Access-Control-Max-Age", &max_age.to_string());
            }
        })
    }

    pub fn origin(mut self, origin: Vec<String>) -> Self {
        self.origin = origin;
        self
    }
    
    pub fn methods(mut self, methods: Vec<String>) -> Self {
        self.methods = methods;
        self
    }
    
    pub fn credentials(mut self, credentials: bool) -> Self {
        self.credentials = credentials;
        self
    }
    
    pub fn max_age(mut self, max_age: Option<u32>) -> Self {
        self.max_age = max_age;
        self
    }
}