#![recursion_limit = "1024"] // `error_chain!` can recurse deeply
extern crate error_chain;
extern crate herstory;
#[macro_use]
extern crate rouille;

use rouille::Response;

fn main() {
    rouille::start_server("0.0.0.0:3008", move |request| {
        router!(request,
            (GET) (/{user: String}/{_key: String}) => {
                let photoset = herstory::run();

                match photoset {
                    Ok(photoset) => {
                        let text = format!("Hai {}, {:?}", user, &photoset);
                        Response::text(text)
                    }

                    Err(e) => {
                        use error_chain::ChainedError;
                        Response::text(format!("Something went wrong: {}", e.display_chain()))
                    }
                }
            },

            _ => Response::text("Oww my bones")
        )
    });
}
