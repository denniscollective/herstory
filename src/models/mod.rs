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
        {
            for mut image in &mut self.images {
                image.spawn_request();
            }
        }


        self.images = self.images
            .into_iter()
            .map(|image: Image| -> Image { image.wait() })
            .collect();

        self
    }
}

#[derive(Debug)]
pub struct Image {
    pub index: i32,
    pub url: String,
    pub request: Option<Request>,
    thread_handle: Option<thread::JoinHandle<Request>>,
}

impl Image {
    fn filename(&self) -> String {
        let t = time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        format!("{}/{}_{}", Config::DATA_DIR, self.index, t)
    }

    pub fn spawn_request(&mut self) {
        let request = Request::build(&self.url, &self.filename());
        self.thread_handle = Some(thread::spawn(move || request.perform_and_save()));
    }

    pub fn wait(self) -> Image {
        let hanldle = self.thread_handle.unwrap();
        let request = hanldle.join().ok();

        Image {
            thread_handle: None,
            request,
            ..self
        }
    }
}
