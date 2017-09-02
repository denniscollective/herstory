use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::time;

mod request;
mod serialization;

use models::serialization::DeserializedPhotoset;
use models::request::Request;

use Config;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Vec<Image>,
}

impl Photoset {
    pub fn from_json(json: &str) -> Photoset {
        DeserializedPhotoset::from_json(json)
    }

    pub fn download_and_save(&mut self) -> Result<(), io::Error> {
        for image in &mut self.images {
            image.download_and_save()?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Image {
    pub index: i32,
    pub url: String,
    pub request: Option<Request>,
}

impl Image {
    fn filename(&self) -> String {
        let t = time::UNIX_EPOCH.elapsed().unwrap().as_secs();
        format!("{}/{}_{}", Config::DATA_DIR, self.index, t)
    }

    pub fn download_and_save(&mut self) -> Result<(), io::Error> {
        let mut file = File::create(self.filename())?;

        self.perform_request(move |data| -> Result<usize, io::Error> {
            let res = file.write(data)?;
            file.sync_all()?;
            Ok(res)
        }).unwrap();

        Ok(())
    }

    fn perform_request<T>(&mut self, mut func: T) -> Result<(), io::Error>
    where
        T: FnMut(&[u8]) -> Result<usize, io::Error> + Send + 'static,
    {
        let mut request = Request::build(&self.url);
        request
            .raw
            .write_function(move |data| Ok(func(data).unwrap()))
            .unwrap();
        request.perform();
        println!("{:?}", request.response_code);
        self.request = Some(request);
        Ok(())
    }
}
