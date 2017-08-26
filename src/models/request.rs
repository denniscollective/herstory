use curl::easy::Easy;

#[derive(Debug)]
pub struct Request {
    pub raw: Easy,
}

impl Request {
    pub fn build(uri: &str) -> Request {
        let mut request = Easy::new();
        request.url(uri).unwrap();

        Request { raw: request }
    }
}
