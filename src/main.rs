#![recursion_limit = "1024"] // `error_chain!` can recurse deeply
extern crate error_chain;
extern crate herstory;

fn main() {
    let photoset = herstory::run();
    match photoset {
        Ok(photoset) => println!("{:?}", &photoset),
        Err(ref e) => {
            use std::io::Write;
            use error_chain::ChainedError; // trait which holds `display_chain`
            let stderr = &mut ::std::io::stderr();
            let errmsg = "Error writing to stderr";

            writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
            ::std::process::exit(1);
        }
    }
}
