extern crate curl;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fmt;
use std::fs;

mod models;
mod stub;
mod threadpool;

mod errors {
    use curl;

    error_chain!{
        foreign_links {
            Curl(curl::Error);
        }
    }
}

use errors::*;

struct Config;
impl Config {
    const DATA_DIR: &'static str = "data/photoset";
}

pub fn photoset() -> models::Photoset {
    let factory = models::Factory{};
    factory.photoset_from_json(stub::get_json())    
}

pub fn run() -> Result<models::Photoset> {
    fs::create_dir_all(Config::DATA_DIR).chain_err(
        || "Couldn't create Directory",
    )?;
    let photoset = photoset();
    photoset.download_and_save();
    Ok(photoset)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Success,
    Failure,
    Pending,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub trait HasStatus: fmt::Debug {
    fn status(&self) -> Status;
}

#[cfg(test)]
mod tests {
    use super::*;
    use Status::*;
    use models::Photoset;

    #[test]
    fn it_works() {
        let path = Config::DATA_DIR;
        fs::remove_dir_all(path).ok();
        let photoset = run().unwrap();
        let paths = fs::read_dir(path).unwrap();

        let file_count = &paths.count();
        let success_count = images_with_status(&photoset, Success);
        let failure_count = images_with_status(&photoset, Failure);
        let all_images_count = photoset.images.iter().count();

        assert_eq!(*file_count, 2);
        assert_eq!(all_images_count, 3);
        assert_eq!(success_count, 2);
        assert_eq!(failure_count, 1);
    }

    fn images_with_status(photoset: &Photoset, status: Status) -> usize {
        photoset
            .images
            .iter()
            .filter(|image| image.lock().unwrap().status() == status)
            .count()
    }
}
