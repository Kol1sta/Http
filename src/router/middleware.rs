use std::sync::Arc;
use crate::{router::request::HttpRequest, router::response::HttpResponse};

pub type Handler = Arc<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>;

#[derive(Clone)]
pub struct Middleware {
    pub route: Option<String>,
    pub method: Option<String>,
    pub handler: Handler
}

impl Middleware {
    pub fn new(route: Option<String>, method: Option<String>, handler: Handler) -> Self {
        Middleware { route, method, handler }
    }

    pub fn create(route: Option<String>, method: Option<String>, handler: Handler) -> Self {
        Self::new(route, method, handler)
    }
}
