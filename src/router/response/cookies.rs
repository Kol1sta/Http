use std::collections::HashMap;

#[derive(Debug)]
pub struct Cookie {
    pub key: String,
    pub value: String
}

impl Cookie {
    pub fn new(key: &str, value: &str) -> Self {
        Cookie {
            key: key.to_string(),
            value: value.to_string()
        }
    }

    pub fn create(key: &str, value: &str) -> Self {
        Self::new(key, value)
    }
}

#[derive(Debug)]
pub struct Cookies {
    cookies: Vec<Cookie>,
    pub params: HashMap<String, CookieParam>
}

#[derive(Debug, Clone)]
pub enum CookieParam {
    Expires(String),
    Path(String),
    Secure(bool),
    HttpOnly(bool),
    SameSite(SameSiteValue),
    Domain(String)
}

#[derive(Debug, Clone)]
pub enum SameSiteValue {
    Strict,
    Lax,
    None
}

impl Cookies {
    pub fn new() -> Self {
        Cookies { cookies: vec![], params: HashMap::new() }
    }

    pub fn create() -> Self {
        Self::new()
    }

    pub fn add(&mut self, cookie: Cookie) {
        self.cookies.push(cookie);
    }

    pub fn build_cookie(&self) -> String {
        if self.cookies.is_empty() {
            return String::new();
        }

        let mut parts: Vec<String> = vec![];
        let mut cookies_string = String::new();

        for part in &self.cookies {
            cookies_string.push_str(format!("{}={}; ", part.key, part.value).as_str());
        }

        cookies_string.truncate(cookies_string.len() - 2);
        parts.push(cookies_string);
        
        for (key, param) in &self.params {
            match param {
                CookieParam::Expires(date) => parts.push(format!("Expires={}", date)),
                CookieParam::Path(path) => parts.push(format!("Path={}", path)),
                CookieParam::Secure(true) => parts.push("Secure".to_string()),
                CookieParam::HttpOnly(true) => parts.push("HttpOnly".to_string()),
                CookieParam::SameSite(value) => parts.push(format!("SameSite={:?}", value)),
                CookieParam::Domain(domain) => parts.push(format!("Domain={}", domain)),
                _ => {}
            }
        }
        
        parts.join("; ")
    }
}