use curl::easy::{Easy2, Handler, WriteError};
use std::fs::File;
use std::io::prelude::*;
use std::fmt;

pub struct FileDownload(File);
impl Handler for FileDownload {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        Ok(self.0.write(data).unwrap())
    }
}

pub struct Request {
    pub raw: Easy2<FileDownload>,
    pub response_code: Option<u32>,
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request {{ response_code: {} }}", self.response_code())
    }
}

impl Request {
    pub fn build(uri: &str, filename: &str) -> Request {
        let file = File::create(filename).unwrap();
        let mut request = Easy2::new(FileDownload(file));
        request.url(uri).unwrap();

        Request {
            raw: request,
            response_code: None,
        }
    }

    pub fn perform_and_save(mut self) -> Request {
        self.raw.perform().unwrap();
        self.response_code = Some(self.raw.response_code().unwrap());
        self
    }

    fn response_code(&self) -> u32 {
        match self.response_code {
            Some(ref code) => *code,
            None => 0,
        }
    }
}
