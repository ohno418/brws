#[derive(Debug)]
struct URL {
    scheme: String,
}

impl URL {
    fn new(url: &str) -> Result<Self, String> {
        let mut scheme_and_rest: Vec<String> = url.split("://").map(|s| s.into()).collect();
        if scheme_and_rest.len() != 2 {
            return Err("Excpected an URL that has a scheme".into());
        }

        let (scheme, _rest) = (
            std::mem::replace(&mut scheme_and_rest[0], "".into()),
            std::mem::replace(&mut scheme_and_rest[1], "".into()),
        );

        Ok(URL { scheme })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_succeed() {
        let url = URL::new("http://example.com").unwrap();
        assert_eq!(url.scheme, "http");

        let url = URL::new("https://example.com").unwrap();
        assert_eq!(url.scheme, "https");
    }

    #[test]
    fn new_fail() {
        let url = URL::new("example.com");
        assert!(url.is_err());

        let url = URL::new("http://http://example.com");
        assert!(url.is_err());
    }
}

fn main() {
    println!("Hello, world!");
}
