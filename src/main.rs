#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate curl;

use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
    let photoset = deserialize_set(&get_json());
    photoset.store();
    println!("Done");
}

impl Photoset {
    fn store(&self) -> Vec<Easy> {
        let mut requests: Vec<Easy> = Vec::new();
        for image in &self.images {
            let mut request = build_request(&image.url);
            request.perform().unwrap();
            println!("{}", request.response_code().unwrap());
            requests.push(request);
        }
        requests
    }
}

fn build_request(uri: &str) -> Easy {
    let mut request = Easy::new();
    request.url(uri).unwrap();
    // request.write_function(|data| {
    //     Ok(stdout().write(data).unwrap())
    // }).unwrap();
    request
}

fn deserialize_set(json: &str) -> Photoset {
    serde_json::from_str(json).unwrap()
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
struct Photoset {
    name: String,
    images: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    index: i32,
    url: String,
}
