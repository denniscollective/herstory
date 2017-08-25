#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

fn main() {
    let photoset = deserialize_set(&get_json());
    store_photoset(photoset);
}

fn store_photoset(photoset: Photoset){
    for image in &photoset.images {
        println!("{:?} - {:?}", image.index, image.url);
    }
    println!("deserialized = {:?}", photoset);
}

fn deserialize_set(json: &String) -> Photoset {
    serde_json::from_str(json).unwrap()
}

fn get_json() -> String {
    let json = "{
        \"name\": \"wat\",
        \"images\": [
            {
                \"index\": 0,
                \"url\": \"cowboyparty.com\"
            },
            {
                \"index\": 1,
                \"url\": \"owow.org\"
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
