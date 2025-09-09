use std::{collections::HashMap, str::{Lines, Split, SplitN, SplitWhitespace}};

pub struct HttpRequest {
    pub req_string: String,
    pub route: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: HashMap<String, String>
}

impl HttpRequest {
    pub fn new(req: &str) -> Self {
        let mut lines: Lines<'_> = req.lines();
        let request_line: &str = lines.next().unwrap_or("");
        let mut parts: SplitWhitespace<'_> = request_line.split_whitespace();
        
        let method: String = parts.next().unwrap_or("GET").to_string();
        let route: String = parts.next().unwrap_or("/").to_string();
        
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut cookies: HashMap<String, String> = HashMap::new();
        let mut content_length: Option<usize> = None;

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let mut header: Split<'_, &'static str> = line.split(":");

            if let (Some(key), Some(value)) = (header.next(), header.next()) {
                let key = key.trim().to_lowercase();
                let value = value.trim().to_string();
                
                headers.insert(key.clone(), value.clone());
                
                if key == "content-length" {
                    content_length = value.parse().ok();
                }
                
                if key == "cookie" {
                    for cookie_pair in value.split(';') {
                        let mut parts = cookie_pair.splitn(2, '=');
                        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                            cookies.insert(key.trim().to_string(), value.trim().to_string());
                        }
                    }
                }
            }
        }

        let remaining_lines: Vec<&str> = lines.collect();
        let remaining_content: String = remaining_lines.join("\n");

        let body_string: String = if let Some(length) = content_length {
            remaining_content.chars().take(length).collect()
        } else {
            remaining_content
        };

        let mut body: HashMap<String, String> = HashMap::new();

        for param_pair in body_string.split('&') {
            let mut parts: SplitN<'_, char> = param_pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                body.insert(key.to_owned(), value.to_owned());
            }
        }

        Self {
            req_string: req.to_string(),
            route,
            method,
            headers,
            cookies,
            body
        }
    }

    pub fn create(req: &str) -> Self {
        Self::new(req)
    }
}