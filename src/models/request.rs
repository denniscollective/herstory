use std::fs;
use std::io::prelude::*;
use std::fmt;

use curl::easy::{Easy2, Handler, WriteError};

use HasStatus;
use Status;

use std::result::Result as RawResult;
use errors::*;

pub struct Request {
    filename: String,
    status: Status,
    pub error: Option<Error>,
    pub raw: CurlRequest,
    pub response_code: Option<u32>,
}

impl Request {
    pub fn build(uri: &str, filename: &str) -> Request {
        let f = fs::File::create(filename).unwrap();
        let mut request = Easy2::new(FileDownload(f));
        request.url(uri).unwrap();
        Self::build_with_request(uri, filename, CurlRequest(request))
    }

    fn build_with_request(_uri: &str, filename: &str, request: CurlRequest) -> Self {
        Request {
            error: None,
            filename: filename.to_string(),
            raw: request,
            response_code: None,
            status: Status::Pending,
        }
    }

    pub fn perform_and_save(&mut self) -> Result<()> {
        match self.raw.perform() {
            Ok(_) => {
                self.status = Status::Success;
                self.response_code = self.raw.response_code();                
                Ok(())
            }

            Err(err) => {
                self.status = Status::Failure;
                self.error = Some(err);
                self.response_code = self.raw.response_code();
                fs::remove_file(&self.filename).ok();                        
                bail!("Request Failed")
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

pub struct FileDownload(fs::File);
impl Handler for FileDownload {
    fn write(&mut self, data: &[u8]) -> RawResult<usize, WriteError> {
        Ok(self.0.write(data).unwrap())
    }
}

pub trait Requestable {
    fn response_code(&mut self) -> Option<u32>;
    fn perform(&mut self) -> Result<()>;
}

pub struct CurlRequest(Easy2<FileDownload>);

impl Requestable for CurlRequest {
    fn perform(&mut self) -> Result<()> {
        self.0.perform().map_err(|err| err.into())
    }

    fn response_code(&mut self) -> Option<u32> {
        self.0.response_code().ok()
    }
}
