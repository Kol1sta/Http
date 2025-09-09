# CookiesOptions
```rs
pub struct CookiesOptions {
    params: HashMap<String, CookieParam>
}
```

Create new object
```rs
pub fn new() -> Self
```

Set cookies expires
```rs
pub fn expires(mut self, date: String) -> Self
```

Set cookies path
```rs
pub fn path(mut self, path: String) -> Self
```

Set cookies secure (true for https)
```rs
pub fn secure(mut self, secure: bool) -> Self
```

Set http_only
```rs
pub fn http_only(mut self, http_only: bool) -> Self
```

Set same_site
```rs
pub fn same_site(mut self, same_site: SameSiteValue) -> Self
```

Set cookies domain
```rs
pub fn domain(mut self, domain: String) -> Self
```

Use cookies in middleware
```rs
pub fn cookies(self) -> Box<dyn Fn(&HttpRequest, &mut HttpResponse) + Send + Sync>
```

### Usage
```rs
server.use_middleware(CookiesOptions::new()
    .same_site(SameSiteValue::Lax)
    .path("/".to_string())
    .cookies()
);
```