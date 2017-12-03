use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub fn get_json() -> Result<String, Error> {
    let mut file = File::open("stub.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


