extern crate curl;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fmt;

mod models;
mod stub;
mod threadpool;

mod errors {
    use curl;

    error_chain! {
        foreign_links {
            Curl(curl::Error);
        }
    }
}

use errors::*;
use std::os::raw::c_char;
use std::ffi::{CStr};

struct Config;

impl Config {
    const DATA_DIR_BASE: &'static str = "data";
}

pub fn photosets(json: &str) -> Vec<models::Photoset> {
    let factory = models::Factory {};
    factory.photoset_from_json(json)
}

pub fn photoset_dir(photoset_id: &u32) -> String {
    format!("{}/photoset_{}", Config::DATA_DIR_BASE, photoset_id)
}

#[no_mangle]
pub fn run_rb(ptr: *const c_char) {
    let json = unsafe { CStr::from_ptr(ptr) };
    println!("hai");
    println!("{:?}", &json);
    println!("printed");
    run(json.to_str().unwrap()).unwrap();
}

pub fn run(json: &str) -> Result<Vec<models::Photoset>> {
    let sets = photosets(json).into_iter().map(|photoset| {
        photoset.download_and_save().ok();
        photoset
    }).collect();
    Ok(sets)
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
    use std::fs;

    use super::*;
    use Status::*;
    use models::Photoset;
    use photoset_dir;

    #[test]
    fn it_works() {
        fs::remove_dir_all(Config::DATA_DIR_BASE).ok();
        let photosets = run(&stub::get_json().unwrap()).unwrap();
        let paths = fs::read_dir(photoset_dir(&1)).unwrap();
        let photoset = &photosets[0];

        let file_count = &paths.count();
        let success_count = images_with_status(&photoset, Success);
        let failure_count = images_with_status(&photoset, Failure);
        let all_images_count = photoset.images.iter().count();

        assert_eq!(*file_count, 3);
        assert_eq!(all_images_count, 4);
        assert_eq!(success_count, 3);
        assert_eq!(failure_count, 1);

        assert_eq!(fs::read_dir(photoset_dir(&7)).unwrap().count(), 4)
    }

    fn images_with_status(photoset: &Photoset, status: Status) -> usize {
        photoset
            .images
            .iter()
            .filter(|image| image.lock().unwrap().status() == status)
            .count()
    }
}
