
mod request;
mod serialization;

use models::serialization::DeserializedPhotoset;
use models::request::Request;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Vec<Image>,
}

impl Photoset {
    pub fn from_json(json: &str) -> Photoset {
        DeserializedPhotoset::from_json(json)
    }

    pub fn download_and_save(&mut self) {
        for image in &mut self.images {
            image.download_and_save();
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub index: i32,
    pub url: String,
    pub request: Option<Request>,
}

impl Image {
    pub fn download_and_save(&mut self) {
        self.perform_request();
        self.save_file()
    }

    fn perform_request(&mut self) {
        let mut request = Request::build(&self.url);
        request.perform();
        println!("{:?}", request.response_code);
        self.request = Some(request);
    }

    fn save_file(&self) {}
}
