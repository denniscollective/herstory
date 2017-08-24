#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    index: i32,
    url: String,
}

fn main() {
    let json = "{
           \"index\": 0,
           \"url\": \"cowboyparty.com\"
         }";
    let image: Image = serde_json::from_str(&json).expect("failboat");
    println!("deserialized = {:?}", image);
}
