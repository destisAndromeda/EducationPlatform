use std::collections::HashMap;

pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: &'static str,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>, 
}

impl HttpResponse {
    pub fn new(
        status_code: u16,
        status_text: &'static str,
        content_type: &'static str,
        cookie: Option<String>,
        body: Vec<u8>,
    ) -> Self {
        let mut headers = HashMap::new();
    
        headers.insert(
            "Content-Type".to_string(),
            content_type.to_string(),
        );
        
        headers.insert(
            "Content-Length".to_string(),
            body.len().to_string(),
        );

        headers.insert(
            "Connection".to_string(),
            "close".to_string(),
        );

        match cookie {
            Some(cookie) => {
                headers.insert(
                    "Set-Cookie".to_string(),
                    cookie,
                );
            },
            None => eprintln!("Formed new response without cookie"),
        };

        Self {
            status_code,
            status_text,
            headers,
            body,
        }
    }

    pub fn json_with_cookie(body: String, cookie: String) -> Self {
        Self::new(200, "OK", "application/json; charset=utf-8", Some(cookie), body.into_bytes())
    }

    pub fn json(body: String) -> Self {
        Self::new(200, "OK", "application/json; charset=utf-8", None, body.into_bytes())
    }

    fn build_response(&self) -> Vec<u8> {
        let mut out = Vec::new();

        let headers = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n",
            self.status_code,
            self.status_text,
            self.headers.iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\r\n"),
        );

        out.extend_from_slice(headers.as_bytes());
        out.extend_from_slice(&self.body);
        out
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.build_response()
    }
}