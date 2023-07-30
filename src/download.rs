use std::fs::File;
use std::io::copy;

pub fn download_from_url(url: String, filename: &str) -> () {
    match reqwest::blocking::get(url) {
        Ok(mut response) => match response.status() {
            reqwest::StatusCode::OK => {
                let mut file = File::create(filename).unwrap();
                copy(&mut response, &mut file).unwrap();
            }
            _ => panic!("Unable to update rustdesk:\n{}", response.status()),
        },
        Err(e) => panic!("Unable to update rustdesk:\n{}", e),
    }
}
