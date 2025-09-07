#[derive(Debug)]
pub struct HttpBodyParams {
    pub key: Box<str>,
    pub value: Box<str>
}

impl HttpBodyParams {
    pub fn new(key: Box<str>, value: Box<str>) -> Self {
        HttpBodyParams { key, value }
    }
}