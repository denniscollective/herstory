#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate curl;

use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
    let mut photoset = deserialize_set(&get_json());
    photoset.perform_requests();
    println!("{:?}", &photoset)
}

fn build_request(uri: &str) -> Easy {
    let mut request = Easy::new();
    request.url(uri).unwrap();
    request
}

#[derive(Debug)]
struct Photoset {
 name: String,
 images: Vec<Image>,
}

#[derive(Debug)]
struct Image {
    index: i32,
    url: String,
    request: Easy,
}

impl Image {
    fn from(image: DeserializedImage) -> Image {
        let request = build_request(&image.url);

        Image {
            request,
            url: image.url,
            index: image.index
        }
    }

    fn perform_request(&mut self) {
        self.request.write_function(|data| {
            Ok(stdout().write(data).unwrap())
        }).unwrap();
        self.request.perform().unwrap();
        println!("{}", self.request.response_code().unwrap());
    }

}

impl Photoset {
    fn from(photoset: DeserializedPhotoset) -> Photoset {
        let mut images: Vec<Image> = Vec::new();
        for image in photoset.images {
            images.push(Image::from(image))
        }

        Photoset {
            images,
            name: photoset.name
        }
    }

    fn perform_requests(&mut self) {
        for image in &mut self.images {
            image.perform_request()
        }
    }
}

fn deserialize_set(json: &str) -> Photoset {
    let photoset: DeserializedPhotoset = serde_json::from_str(json).unwrap();
    Photoset::from(photoset)
}

fn get_json() -> String {
    let json = "{
        \"name\": \"wat\",
        \"images\": [
            {
                \"index\": 0,
                \"url\": \"http://cowboyparty.com\"
            },
            {
                \"index\": 1,
                \"url\": \"http://www.owow.org\"
             }
        ]
    }";

    String::from(json)
}

#[derive(Serialize, Deserialize, Debug)]
struct DeserializedPhotoset {
    name: String,
    images: Vec<DeserializedImage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeserializedImage {
    index: i32,
    url: String
}
