// use std::io::prelude::*;
use serde_json;
mod request;
use models::request::Request;


#[derive(Debug)]
pub struct Photoset {
    name: String,
    images: Vec<Image>,
}

impl Photoset {
    fn from(photoset: DeserializedPhotoset) -> Photoset {
        let mut images: Vec<Image> = Vec::new();
        for image in photoset.images {
            images.push(Image::from(image))
        }

        Photoset {
            images,
            name: photoset.name,
        }
    }

    pub fn from_json(json: &str) -> Photoset {
        let photoset: DeserializedPhotoset = serde_json::from_str(json).unwrap();
        Photoset::from(photoset)
    }


    pub fn perform_requests(&mut self) {
        for image in &mut self.images {
            image.perform_request()
        }
    }
}

#[derive(Debug)]
struct Image {
    index: i32,
    url: String,
    request: Request,
}

impl Image {
    fn from(image: DeserializedImage) -> Image {
        let request = Request::build(&image.url);

        Image {
            request,
            url: image.url,
            index: image.index,
        }
    }

    fn perform_request(&mut self) {
        // self.request
        //     .raw
        //     .write_function(|data| Ok(stdout().write(data).unwrap()))
        //     .unwrap();
        self.request.raw.perform().unwrap();
        println!("{}", self.request.raw.response_code().unwrap());
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DeserializedPhotoset {
    name: String,
    images: Vec<DeserializedImage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeserializedImage {
    index: i32,
    url: String,
}
