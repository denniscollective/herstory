// use std::io;
use std::time;
use std::thread;

mod request;
mod serialization;

use models::serialization::DeserializedPhotoset;
use models::request::Request;

use Config;

#[derive(Debug)]
pub struct Photoset {
    pub name: String,
    pub images: Option<Vec<Image>>,
    pub state: PhotosetState,
}

#[derive(Debug)]
pub enum PhotosetState {
    Deserialized,
    Downloading,
    Downoaded,
}

impl Photoset {
    pub fn from_json(json: &str) -> Photoset {
        DeserializedPhotoset::from_json(json)
    }

    pub fn download_and_save(mut self) -> Photoset {
        let threadpool: Threadpool<Image> = Threadpool::new(4);

        let handles: Vec<thread::JoinHandle<()>> = self.images
            .unwrap()
            .into_iter()
            .map(|mut image: Image| {
                threadpool.execute(move || { image.request = Some(image.spawn_request()); })
            })
            .collect(); //collect here to spawn all threads

        for handle in handles {
            handle.join().unwrap();
        }

        self.images = threadpool.values();
        self
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

    pub fn spawn_request(&mut self) -> Request {
        let request = Request::build(&self.url, &self.filename());
        request.perform_and_save()
    }
}

struct Threadpool<T> {
    values: Option<Vec<T>>,
}

impl<T> Threadpool<T> {
    pub fn new(_worker_size: u32) -> Threadpool<T> {
        Threadpool { values: None }
    }

    pub fn execute<F>(&self, fun: F) -> thread::JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(fun)
    }

    pub fn values(self) -> Option<Vec<T>> {
        self.values
    }
}
