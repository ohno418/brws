mod url;

use crate::url::URL;

pub fn request(url: &str) {
    let url = URL::new(url);
    println!("{:?}", url);
}
