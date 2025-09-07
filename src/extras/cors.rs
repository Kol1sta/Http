use crate::{router::{request::HttpRequest, response::HttpResponse}, Http};

pub struct CorsOptions {
    origin: Vec<&'static str>,
    methods: Vec<&'static str>,
    allowed_headers: Vec<&'static str>,
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
            origin: vec!["*"],
            methods: vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"],
            allowed_headers: vec!["Content-Type", "Authorization"],
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

    pub fn with_origin(mut self, origin: &'static str) -> Self {
        self.origin = vec![origin];
        self
    }
    
    pub fn with_methods(mut self, methods: Vec<&'static str>) -> Self {
        self.methods = methods;
        self
    }
    
    pub fn with_credentials(mut self, credentials: bool) -> Self {
        self.credentials = credentials;
        self
    }
    
    pub fn with_max_age(mut self, max_age: Option<u32>) -> Self {
        self.max_age = max_age;
        self
    }
}