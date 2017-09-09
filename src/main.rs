extern crate herstory;

fn main() {
    let photoset = herstory::run().unwrap();
    println!("{:?}", &photoset)
}
