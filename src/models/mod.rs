use std::sync::{Arc, Mutex};
use std::time;

mod request;
mod serialization;

use models::serialization::DeserializedPhotoset;
use models::request::Request;
use threadpool::Threadpool;

use Config;
use HasStatus;
use Status;
use Status::*;

pub type Images = Vec<Arc<Mutex<Image>>>;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Images,
}

impl Photoset {
    pub fn from_json(json: &str) -> Photoset {
        DeserializedPhotoset::from_json(json)
    }

    pub fn download_and_save(&self) {
        Threadpool::batch(4, &self.images, |image: &mut Image| image.spawn_request());
    }
}

#[derive(Debug)]
pub struct Image {
    pub index: i32,
    pub url: String,
    pub request: Option<Result<Request, Request>>,
}

impl Image {
    fn filename(&self) -> String {
        let t = time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        format!("{}/{}_{}", Config::DATA_DIR, self.index, t)
    }

    pub fn spawn_request(&mut self) {
        let request = Request::build(&self.url, &self.filename());
        self.request = Some(request.perform_and_save());
    }
}

impl HasStatus for Image {
    fn status(&self) -> Status {
        match self.request {
            None => Pending,
            Some(Ok(_)) => Success,
            Some(Err(_)) => Failure,
        }
    }
}
