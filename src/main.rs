
extern crate herstory;

fn main() {
    let mut photoset = herstory::photoset();
    photoset.download_and_save();
    println!("{:?}", &photoset)
}
