#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate curl;

use std::fs;
use std::io;


mod models;

struct Config;

impl Config {
    const DATA_DIR: &'static str = "photoset";
}

fn get_json() -> &'static str {
    "{
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
    }"
}

pub fn photoset() -> models::Photoset {
    models::Photoset::from_json(get_json())
}

pub fn run() -> Result<models::Photoset, io::Error> {
    fs::create_dir_all(Config::DATA_DIR)?;
    let mut photoset = photoset();
    photoset.download_and_save()?;
    Ok(photoset)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let path = Config::DATA_DIR;
        fs::remove_dir_all(path).unwrap();
        let photoset = run().unwrap();
        let paths = fs::read_dir(path).unwrap();
        let count = &paths.count();
        assert_eq!(photoset.images.iter().count(), 2);
        assert_eq!(*count, 2);
    }

}
