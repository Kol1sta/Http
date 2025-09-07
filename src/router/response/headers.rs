use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers {
    headers: HashMap<String, String>,
    pub status_code: u16
}

impl Headers {
    pub fn new() -> Self {
        Headers { headers: HashMap::new(), status_code: 200 }
    }

    pub fn create() -> Self {
        Self::new()
    }

    pub fn set_headers(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn delete_headers(&mut self, key: &str) {
        self.headers.remove(key);
    }

    pub fn get_status_text(&self) -> &'static str {
        match self.status_code {
            200 => "OK",
            201 => "Created",
            302 => "Found",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown"
        }
    }

    pub fn build_headers_string(&self) -> String {
        let mut result: String = String::from(format!("HTTP/1.1 {} {}\r\n", self.status_code, self.get_status_text()));
        for (key, value) in &self.headers {
            result.push_str(format!("{}: {}\r\n", key, value).as_str());
        }

        result.push_str("\r\n");
        result
    }
}