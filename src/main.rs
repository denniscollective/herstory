#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate curl;

mod models;
mod stub;

fn main() {
    let mut photoset = stub::photoset();
    photoset.perform_requests();
    println!("{:?}", &photoset)
}
