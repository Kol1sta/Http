use std::sync::Arc;
use crate::router::{middleware::Middleware, request::HttpRequest, response::HttpResponse};

pub mod middleware;
pub mod request;
pub mod response;

#[derive(Clone)]
pub struct Middlewares {
    handlers: Vec<Middleware>
}

impl Middlewares {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn add<F: Fn(&HttpRequest, &mut HttpResponse) + 'static + Send + Sync>(&mut self, middleware: F) {
        self.handlers.push(Middleware::new(None, None, Arc::new(middleware)));
    }

    pub fn add_route<F: Fn(&HttpRequest, &mut HttpResponse) + 'static + Send + Sync>(&mut self, route: &str, method: &str, middleware: F) {
        self.handlers.push(Middleware::new(Some(route.to_string()), Some(method.to_string()), Arc::new(middleware)));
    }

    pub fn execute(&self, request: &HttpRequest, response: &mut HttpResponse) {
        for middleware in &self.handlers {
            if middleware.route.is_none() || (middleware.route.as_ref() == Some(&request.route) && middleware.method.as_ref() == Some(&request.method)) {
                (middleware.handler)(request, response);
            }
        }
    }
}
