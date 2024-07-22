#[derive(Debug)]
pub struct URL {
    pub scheme: String,
    pub host: String,
    pub path: String,
}

impl URL {
    /// Parses a URL string and creates a new `URL`.
    pub fn new(url: &str) -> Result<Self, String> {
        let (scheme, rest) = match url.split_once("://") {
            Some((scheme, rest)) => (scheme.into(), rest),
            None => return Err("Missing scheme".into()),
        };

        // Error with multiple schemes.
        if rest.find("://").is_some() {
            return Err("Multiple schemes found.".into());
        }

        let (host, path) = match rest.split_once("/") {
            Some((host, path)) => (host.to_string(), format!("/{}", path)),
            None => (rest.to_string(), "/".into()),
        };

        // Error with no host.
        if host.is_empty() {
            return Err("Missing host".into());
        }

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
        assert_eq!(url.path, "/");
    }

    #[test]
    fn new_fail() {
        // No scheme.
        let url = URL::new("example.com");
        assert!(url.is_err());

        // No host.
        let url = URL::new("http://");
        assert!(url.is_err());

        // Multiple schemes.
        let url = URL::new("http://http://example.com");
        assert!(url.is_err());
    }
}
