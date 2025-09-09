use std::{collections::HashMap, str::{Lines, Split, SplitN, SplitWhitespace}};

pub struct HttpRequest {
    pub route: String,
    pub method: String,
    pub headers: HashMap<String, String>,
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
        let mut content_length: Option<usize> = None;

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let mut header: Split<'_, &'static str> = line.split(":");

            if let (Some(key), Some(value)) = (header.next(), header.next()) {
                if key.to_lowercase().starts_with("content-length:") {
                    content_length = value.trim().parse().ok()
                }

                headers.insert(key.to_string(), value.to_string());
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
            route,
            method,
            headers,
            body
        }
    }

    pub fn create(req: &str) -> Self {
        Self::new(req)
    }
}