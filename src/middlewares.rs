use std::sync::Arc;
use crate::{request::HttpRequest, response::HttpResponse};

type Handler = Arc<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

pub struct Middlewares {
    handlers: Vec<Handler>
}

impl Middlewares {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn add<F: Fn(&HttpRequest, &mut HttpResponse) + 'static + Send + Sync>(&mut self, middleware: F) {
        self.handlers.push(Arc::new(middleware));
    }

    pub fn execute(&self, request: &HttpRequest, response: &mut HttpResponse) {
        for handler in &self.handlers {
            handler(request, response);
        }
    }
}