#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    let photoset = deserialize_set(&get_json());
    store_photoset(photoset);
    println!("Done");
}

fn store_photoset (photoset: Photoset) -> () {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    for image in &photoset.images {
        let uri = image.url.parse().unwrap();

        let work = client.get(uri).map(|res| {
            println!("Response: {}", res.status());
            // res.body().for_each(|chunk| {
                // io::stdout()
                    // .write_all(&chunk)
                    // .map_err(From::from)
            // })
        });
        core.run(work).unwrap()
    }
}

// fn store_photoset(photoset: Photoset, client: &Client<hyper::client::HttpConnector>, core: &tokio_core::reactor::Core ){
//     let image_futures: &mut Vec<hyper::client::FutureResponse> = &mut Vec::new();
//     for image in &photoset.images {
//         let work = client.get(image.url.parse().unwrap());
//
//         println!("{:?} - {:?}", image.index, &work);
//         image_futures.push(work)
//     }
//
//     for image in image_futures {
//         image.wait();
//
//         image.and_then(|res| {
//             println!("Response: {}", res.status());
//         });
//
//
//     }
//     println!("deserialized = {:?}", photoset);
// }

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
