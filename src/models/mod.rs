use serde_json;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::time;

mod request;

use models::request::{Request, CurlRequest};
use threadpool::Threadpool;

use Config;
use HasStatus;
use Status;


pub struct Factory {}
impl Factory {
    fn image(&self, deserialized: DeserializedImage) -> Image {
        let url = deserialized.url;
        let index = deserialized.index;
        let filename = Image::filename_for(&index);

        Image {
            request: Request::build(&url, &filename),
            url: url,
            index: index,
        }
    }

    pub fn photoset_from_json(&self, json: &str) -> Photoset {
        self.photoset_from_deserialized(serde_json::from_str(json).unwrap())
    }

    fn photoset_from_deserialized(&self, deserialized: DeserializedPhotoset) -> Photoset {
        let mut images: Vec<Arc<Mutex<Image>>> = Vec::new();
        for image in deserialized.images {
            images.push(Arc::new(Mutex::new(self.image(image))))
        }

        Photoset {
            images: images,
            name: deserialized.name,
        }
    }
}

pub type Images = Vec<Arc<Mutex<Image>>>;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Images,
}

impl Photoset {
    pub fn download_and_save(&self) {
        Threadpool::batch(4, &self.images, |image: &mut Image| image.spawn_request());
    }
}

#[derive(Debug)]
pub struct Image {
    pub index: i32,
    pub url: String,
    pub request: Request<CurlRequest<File>>,
}

impl Image {
    fn filename_for(index: &i32) -> String {
        let t = time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        format!("{}/{}_{}", Config::DATA_DIR, index, t)
    }

    pub fn spawn_request(&mut self) {
        self.request.perform_and_save().ok();
    }
}

impl HasStatus for Image {
    fn status(&self) -> Status {
        self.request.status()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedPhotoset {
    name: String,
    images: Vec<DeserializedImage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedImage {
    index: i32,
    url: String,
}
