use serde_json;

use models::{Image, Photoset};
use models::request::Request;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedPhotoset {
    name: String,
    images: Vec<DeserializedImage>,
}

impl DeserializedPhotoset {
    fn photoset(self) -> Photoset {
        let mut images: Vec<Image> = Vec::new();
        for image in self.images {
            images.push(image.image())
        }

        Photoset {
            images,
            name: self.name,
        }
    }

    pub fn from_json(json: &str) -> Photoset {
        let deserialized: DeserializedPhotoset = serde_json::from_str(json).unwrap();
        deserialized.photoset()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeserializedImage {
    index: i32,
    url: String,
}

impl DeserializedImage {
    fn image(self) -> Image {
        let request = Request::build(&self.url);
        Image {
            request,
            url: self.url,
            index: self.index,
        }
    }
}
