#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

fn main() {
    let image: Photoset = serde_json::from_str(&get_json())
        .unwrap();
    println!("deserialized = {:?}", image);
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
