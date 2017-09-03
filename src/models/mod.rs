// use std::io;
use std::time;
use std::thread;

mod request;
mod serialization;

use models::serialization::DeserializedPhotoset;
use models::request::Request;

use Config;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Vec<Image>,
}

impl Photoset {
    pub fn from_json(json: &str) -> Photoset {
        DeserializedPhotoset::from_json(json)
    }

    pub fn download_and_save(mut self) -> Photoset {
        self.images = {
            let handles: Vec<thread::JoinHandle<Image>> = self.images
                .into_iter()
                .map(|mut image: Image| -> thread::JoinHandle<Image> {
                    thread::spawn(move || -> Image {
                        image.request = Some(image.spawn_request());
                        image
                    })
                })
                .collect(); //collect here to spawn all threads

            handles
                .into_iter()
                .map(|handle| -> Image { handle.join().unwrap() })
                .collect() //cast thread handles back to images
        };

        self
    }
}

#[derive(Debug)]
pub struct Image {
    pub index: i32,
    pub url: String,
    pub request: Option<Request>,
}

impl Image {
    fn filename(&self) -> String {
        let t = time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        format!("{}/{}_{}", Config::DATA_DIR, self.index, t)
    }

    pub fn spawn_request(&mut self) -> Request {
        let request = Request::build(&self.url, &self.filename());
        request.perform_and_save()
    }
}
