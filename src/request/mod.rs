use std::str::{Lines, SplitN, SplitWhitespace};
use http_body_params::HttpBodyParams;

pub mod http_body_params;

pub struct HttpRequest {
    pub route: String,
    pub method: String,
    pub headers: Vec<String>,
    pub body: Vec<HttpBodyParams>
}

impl HttpRequest {
    pub fn new(req: &str) -> Self {
        let mut lines: Lines<'_> = req.lines();
        let request_line: &str = lines.next().unwrap_or("");
        let mut parts: SplitWhitespace<'_> = request_line.split_whitespace();
        
        let method: String = parts.next().unwrap_or("GET").to_string();
        let route: String = parts.next().unwrap_or("/").to_string();
        
        let mut headers: Vec<String> = Vec::new();
        let mut content_length: Option<usize> = None;

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let header_line: String = line.to_string();
            headers.push(header_line.clone());

            if header_line.to_lowercase().starts_with("content-length:") {
                if let Some(length_str) = header_line.split(':').nth(1) {
                    content_length = length_str.trim().parse().ok();
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

        let mut body: Vec<HttpBodyParams> = Vec::new();

        for param_pair in body_string.split('&') {
            let mut parts: SplitN<'_, char> = param_pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                body.push(HttpBodyParams::new(key.to_owned().into_boxed_str(), value.to_owned().into_boxed_str()));
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