#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate curl;


mod models;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut photoset = photoset();
        photoset.perform_requests();
        println!("{:?}", &photoset)
    }
}
