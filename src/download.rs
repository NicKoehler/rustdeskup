use std::fs::File;
use std::io::copy;

pub fn download_from_url(url: String, filename: &str) -> () {
    match reqwest::blocking::get(url) {
        Ok(mut response) => {
            let mut file = File::create(filename).unwrap();
            copy(&mut response, &mut file).unwrap();
        }
        Err(e) => panic!("Unable to update rustdesk:\n{}", e),
    }
}
