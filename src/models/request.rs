use std::fs;
use std::io::prelude::*;
use std::fmt;

use curl::easy::{Easy2, Handler, WriteError};

use HasStatus;
use Status;

use std::result::Result as RawResult;
use errors::*;

pub struct Request<T>
where
    T: Requestable,
{
    filename: String,
    status: Status,
    pub error: Option<Error>,
    pub raw: T,
    pub response_code: Option<u32>,
}

impl<T> Request<T>
where
    T: Requestable,
{
    fn build_with_request(_uri: &str, filename: &str, request: T) -> Request<T> {
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

impl<T: Write> Request<CurlRequest<T>> {
    pub fn build_with_writer(uri: &str, filename: &str, writer: T) -> Self {
        let mut request = Easy2::new(WriterDownload(writer));
        request.url(uri).unwrap();
        Self::build_with_request(uri, filename, CurlRequest(request))
    }
}

impl Request<CurlRequest<fs::File>> {
    pub fn build(uri: &str, filename: &str) -> Self {
        let f = fs::File::create(filename).unwrap();
        Self::build_with_writer(uri, filename, f)
    }
}

impl Request<CurlRequest<Vec<u8>>> {
    pub fn build_vec(uri: &str, filename: &str) -> Self {
        let vec: Vec<u8> = Vec::new();
        Self::build_with_writer(uri, filename, vec)
    }
}

impl<T> HasStatus for Request<T>
where
    T: Requestable,
{
    fn status(&self) -> Status {
        self.status
    }
}

impl<T> fmt::Debug for Request<T>
where
    T: Requestable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Request {{ response_code: {}, error: {:?} }}",
            self.response_code(),
            &self.error
        )
    }
}

pub struct WriterDownload<T: Write>(T);
impl<T: Write> Handler for WriterDownload<T> {
    fn write(&mut self, data: &[u8]) -> RawResult<usize, WriteError> {
        Ok(self.0.write(data).unwrap())
    }
}

pub trait Requestable {
    fn response_code(&mut self) -> Option<u32>;
    fn perform(&mut self) -> Result<()>;
}

pub struct CurlRequest<T: Write>(Easy2<WriterDownload<T>>);

impl<T: Write> Requestable for CurlRequest<T> {
    fn perform(&mut self) -> Result<()> {
        self.0.perform().map_err(|err| err.into())
    }

    fn response_code(&mut self) -> Option<u32> {
        self.0.response_code().ok()
    }
}
