use serde_json;
use std::sync::{Arc, Mutex};

use models::{DeserializedImage, DeserializedPhotoset, Image, Photoset, Request};

use models::request::{ Requestable, Kind as RequestKind, CurlRequest, FakeRequest};

#[derive(Default)]
pub struct Factory {
    request_type: RequestKind        
}

impl Factory {    
    fn image(&self, deserialized: DeserializedImage) -> Image {
        let url = deserialized.url;
        let index = deserialized.index;
        let filename = Image::filename_for(&index);
        
        Image {
            request: self.request(&url, &filename),
            url: url,
            index: index,
        }
    }

    fn request <T>(&self, url: &str, filename: &str) -> Request<T>    
    where T: Requestable{
        let raw: Box<Requestable>  = match self.request_type {
            RequestKind::Curl => Box::new(CurlRequest::raw_request(url, filename)), 
            RequestKind::Fake => Box::new(FakeRequest::raw_request(url, filename))            
        };
        

        Request::build(url, filename, *raw)
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

