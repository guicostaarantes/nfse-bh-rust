use std::collections::HashMap;

struct Collector(Vec<u8>);

impl curl::easy::Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, curl::easy::WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub enum RequestMethod {
    GET,
    POST(String),
}

pub struct Request {
    method: RequestMethod,
    url: String,
    headers: HashMap<String, String>,
    certificate_path: Option<String>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: RequestMethod::GET,
            url: String::new(),
            headers: HashMap::new(),
            certificate_path: None,
        }
    }
}

impl Request {
    pub fn set_method(mut self, method: RequestMethod) -> Self {
        self.method = method;
        self
    }
}

impl Request {
    pub fn set_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }
}

impl Request {
    pub fn set_header(mut self, key: String, value: Option<String>) -> Self {
        match value {
            Some(val) => {
                self.headers.insert(key, val);
            }
            None => {
                self.headers.remove(&key);
            }
        }
        self
    }
}

impl Request {
    pub fn set_certificate_path(mut self, certificate_path: Option<String>) -> Self {
        self.certificate_path = certificate_path;
        self
    }
}

impl Request {
    pub fn run(self) -> Result<(u32, Vec<u8>), curl::Error> {
        let mut handle = curl::easy::Easy2::new(Collector(Vec::new()));
        let mut list = curl::easy::List::new();

        handle.url(&self.url)?;

        handle.useragent(&format!("curl/{}", curl::Version::get().version()))?;

        if let Some(cert_path) = &self.certificate_path {
            handle.ssl_cert(cert_path)?;
        }

        for (k, v) in &self.headers {
            list.append(&format!("{}: {}", k, v))?;
        }

        handle.http_headers(list)?;

        match self.method {
            RequestMethod::GET => {
                handle.get(true)?;
            }
            RequestMethod::POST(data) => {
                handle.post(true)?;
                handle.post_fields_copy(data.as_bytes())?;
            }
        }

        handle.perform()?;

        let status_code = handle.response_code()?;
        let data = handle.get_ref().0.clone();

        Ok((status_code, data))
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, RequestMethod};
    use httptest::{matchers, responders, Expectation, Server};

    #[test]
    fn request() {
        let server = Server::run();
        server.expect(
            Expectation::matching(matchers::all_of![
                matchers::request::method("POST"),
                matchers::request::path("/foo"),
                matchers::request::headers(matchers::contains(("foo", "bar"))),
                matchers::request::body("example post data"),
            ])
            .respond_with(
                responders::status_code(200)
                    // .insert_header("foo2", "bar2")
                    .body("example response"),
            ),
        );
        let req = Request::new()
            .set_url(server.url("/foo").to_string())
            .set_header(String::from("foo"), Some(String::from("bar")))
            .set_method(RequestMethod::POST(String::from("example post data")));
        let response = req.run().unwrap();
        let (status_code, data) = (response.0, String::from_utf8(response.1).unwrap());
        assert_eq!(status_code, 200);
        assert_eq!(data, String::from("example response"));
    }
}
