
extern crate herstory;

fn main() {
    let mut photoset = herstory::photoset();
    photoset.perform_requests();
    println!("{:?}", &photoset)
}
