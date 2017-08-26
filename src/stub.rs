use models;

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
