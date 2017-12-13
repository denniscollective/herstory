use serde_json;
use std::sync::{Arc, Mutex};
use std::time;
use std::fs;

use errors::*;

mod request;

use models::request::Request;
use threadpool::Threadpool;
use photoset_dir;

use HasStatus;
use Status;


pub struct Factory {}

impl Factory {
    fn image(&self, artist: &str, deserialized: DeserializedImage) -> Image {
        let url = deserialized.scaled_url;
        let index = deserialized.index;
        let photoset_id = deserialized.photoset_id;
        let filename = Image::filename_for(artist, &photoset_id, &index);
        let request = Request::build(&url, &filename);

        Image {
            request,
            url,
            index,
            photoset_id,
        }
    }

    pub fn photoset_from_json(&self, artist: &str, json: &str) -> Vec<Photoset> {
        let sets: Vec<DeserializedPhotoset> = serde_json::from_str(json).unwrap();

        sets.into_iter()
            .map(|json| {
                self.photoset_from_deserialized(artist, json).unwrap()
            })
            .collect()
    }

    fn photoset_from_deserialized(
        &self,
        artist: &str,
        deserialized: DeserializedPhotoset,
    ) -> Result<Photoset> {
        let mut images: Vec<Arc<Mutex<Image>>> = Vec::new();

        fs::create_dir_all(photoset_dir(artist, &deserialized.id))
            .chain_err(|| "Couldn't create Directory")?;
        for image in deserialized.images {
            images.push(Arc::new(Mutex::new(self.image(artist, image))))
        }

        Ok(Photoset {
            id: deserialized.id,
            name: deserialized.name,
            images: images,
        })
    }
}

pub type Images = Vec<Arc<Mutex<Image>>>;

#[derive(Debug)]
pub struct Photoset {
    pub id: u32,
    pub name: String,
    pub images: Images,
}

impl Photoset {
    pub fn download_and_save(&self) -> Result<()> {
        Threadpool::batch(4, &self.images, |image: &mut Image| image.spawn_request());
        Ok(())
    }
}

#[derive(Debug)]
pub struct Image {
    pub photoset_id: u32,
    pub index: u32,
    pub url: String,
    pub request: Request,
}

impl Image {
    fn filename_for(artist: &str, photoset_id: &u32, index: &u32) -> String {
        let t = time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        format!("{}/{}_{}.jpg", photoset_dir(artist, photoset_id), index, t)
    }

    pub fn spawn_request(&mut self) {
        self.request.perform_and_save().ok();
    }
}

impl HasStatus for Photoset {
    fn status(&self) -> Status {
        Status::Success
    }
}

impl HasStatus for Image {
    fn status(&self) -> Status {
        self.request.status()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedPhotoset {
    id: u32,
    name: String,
    images: Vec<DeserializedImage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedImage {
    photoset_id: u32,
    index: u32,
    scaled_url: String,
}
