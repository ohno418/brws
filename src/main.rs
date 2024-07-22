mod url;

use crate::url::URL;

fn main() {
    let url = URL::new("http://example.org/path/to/somewhere.html");
    println!("{:?}", url);
}
