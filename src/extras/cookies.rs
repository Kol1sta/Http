// REWRITE IT LATER PLS 🙏🙏🙏

use std::collections::HashMap;
use crate::router::{request::HttpRequest, response::{cookies::{self, CookieParam, SameSiteValue}, HttpResponse}};

pub struct CookiesOptions {
    params: HashMap<String, CookieParam>
}

impl CookiesOptions {
    pub fn new() -> Self {
        Self { params: HashMap::new() }
    }

    pub fn expires(mut self, date: String) -> Self {
        self.params.insert("Expires".to_string(), CookieParam::Expires(date));
        self
    }

    pub fn path(mut self, path: String) -> Self {
        self.params.insert("Path".to_string(), CookieParam::Path(path));
        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.params.insert("Secure".to_string(), CookieParam::Secure(secure));
        self
    }

    pub fn http_only(mut self, http_only: bool) -> Self {
        self.params.insert("HttpOnly".to_string(), CookieParam::HttpOnly(http_only));
        self
    }

    pub fn same_site(mut self, same_site: SameSiteValue) -> Self {
        self.params.insert("SameSite".to_string(), CookieParam::SameSite(same_site));
        self
    }

    pub fn domain(mut self, domain: String) -> Self {
        self.params.insert("Domain".to_string(), CookieParam::Domain(domain));
        self
    }
    
    pub fn cookies(self) -> Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync> {
        Box::new(move |req: &HttpRequest, res: &mut HttpResponse| {
            for (key, param) in self.params.clone() {
                match param {
                    CookieParam::Expires(date) => {
                        res.cookies.params.insert("Expires".to_string(), CookieParam::Expires(date));
                    }
                    CookieParam::Path(path) => {
                        res.cookies.params.insert("Path".to_string(), CookieParam::Path(path));
                    }
                    CookieParam::Secure(secure) => {
                        if secure {
                            res.cookies.params.insert("Secure".to_string(), CookieParam::Secure(true));
                        }
                    }
                    CookieParam::HttpOnly(http_only) => {
                        if http_only {
                            res.cookies.params.insert("HttpOnly".to_string(), CookieParam::HttpOnly(true));
                        }
                    }
                    CookieParam::SameSite(value) => {
                        res.cookies.params.insert("SameSite".to_string(), CookieParam::SameSite(value));
                    }
                    CookieParam::Domain(domain) => {
                        res.cookies.params.insert("Domain".to_string(), CookieParam::Domain(domain));
                    }
                }
            }

            res.headers.set_headers("Set-Cookie", &res.cookies.build_cookie());
        })
    }
}