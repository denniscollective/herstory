use std::fs;
use std::io::prelude::*;
use std::fmt;

use curl;
use curl::easy::{Easy2, Handler, WriteError};

use HasStatus;
use Status;

pub struct FileDownload(fs::File);
impl Handler for FileDownload {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        Ok(self.0.write(data).unwrap())
    }
}

pub struct Request {
    filename: String,
    status: Status,
    pub error: Option<curl::Error>,
    pub raw: Easy2<FileDownload>,
    pub response_code: Option<u32>,
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Request {{ response_code: {}, error: {:?} }}",
            self.response_code(),
            &self.error
        )
    }
}

impl Request {
    pub fn build(uri: &str, filename: &str) -> Request {
        let f = fs::File::create(filename).unwrap();
        let mut request = Easy2::new(FileDownload(f));
        request.url(uri).unwrap();
        Request {
            error: None,
            filename: filename.to_string(),
            raw: request,
            response_code: None,
            status: Status::Pending,
        }
    }


    pub fn perform_and_save(&mut self) -> Result<(), ()> {
        match self.raw.perform() {
            Ok(_) => {
                self.response_code = self.raw.response_code().ok();
                self.status = Status::Success;
                Ok(())
            }
            Err(err) => {
                fs::remove_file(&self.filename).ok();
                self.response_code = self.raw.response_code().ok();
                self.status = Status::Failure;
                self.error = Some(err);
                Err(())
            }
        }
    }

    fn response_code(&self) -> u32 {
        match self.response_code {
            Some(ref code) => *code,
            None => 0,
        }
    }
}

impl HasStatus for Request {
    fn status(&self) -> Status {
        self.status
    }
}
