
use serde_json;
mod request;
use models::request::Request;


#[derive(Debug)]
pub struct Photoset {
    name: String,
    pub images: Vec<Image>,
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

    pub fn download_and_save(&mut self) {
        for image in &mut self.images {
            image.download_and_save();
        }
    }
}

#[derive(Debug)]
pub struct Image {
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

    pub fn download_and_save(&mut self) {
        self.perform_request();
        self.save_file()
    }

    fn perform_request(&mut self) {
        // self.request
        //     .raw
        //     .write_function(|data| Ok(stdout().write(data).unwrap()))
        //     .unwrap();
        self.request.raw.perform().unwrap();
        println!("{}", self.request.raw.response_code().unwrap());
    }

    fn save_file(&self) {}
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
