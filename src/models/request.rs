use curl::easy::Easy;

#[derive(Debug)]
pub struct Request {
    pub raw: Easy,
    pub response_code: Option<u32>,
}

impl Request {
    pub fn build(uri: &str) -> Request {
        let mut request = Easy::new();
        request.url(uri).unwrap();

        Request {
            raw: request,
            response_code: None,
        }
    }

    pub fn perform(&mut self) {
        self.raw.perform().unwrap();
        self.response_code = Some(self.raw.response_code().unwrap());
    }
}
