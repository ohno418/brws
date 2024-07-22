#[derive(Debug)]
struct URL {
    scheme: String,
    host: String,
    path: String,
}

impl URL {
    fn new(url: &str) -> Result<Self, String> {
        let mut scheme_and_rest: Vec<String> = url.split("://").map(|s| s.into()).collect();
        if scheme_and_rest.len() != 2 {
            return Err("Excpected an URL that has a scheme".into());
        }

        let (scheme, rest) = (
            std::mem::replace(&mut scheme_and_rest[0], "".into()),
            std::mem::replace(&mut scheme_and_rest[1], "".into()),
        );

        let mut host_and_rest: Vec<String> = rest.split("/").map(|s| s.into()).collect();
        if host_and_rest.len() < 1 {
            return Err("Excpected an URL that has a scheme".into());
        }

        let host = std::mem::replace(&mut host_and_rest[0], "".into());
        let path = host_and_rest.join("/");

        Ok(URL { scheme, host, path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_succeed() {
        let url = URL::new("http://example.com/path/to/somewhere.html").unwrap();
        assert_eq!(url.scheme, "http");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.path, "/path/to/somewhere.html");

        let url = URL::new("http://example.com").unwrap();
        assert_eq!(url.scheme, "http");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.path, "");
    }

    #[test]
    fn new_fail() {
        // No scheme.
        let url = URL::new("example.com");
        assert!(url.is_err());

        // Multiple schemes.
        let url = URL::new("http://http://example.com");
        assert!(url.is_err());
    }
}

fn main() {
    println!("Hello, world!");
}
