/// Parses HTML and returns a string to show.
pub fn parse_html(html: &str) -> String {
    let mut buf = String::new();
    let mut in_tag = false;

    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ => {
                if !in_tag {
                    buf.push(c);
                }
            }
        }
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_string_within_tags() {
        let ret = parse_html("<sometag>inner text</sometag>");
        assert_eq!(ret, "inner text");

        let ret = parse_html(
            r"<!doctype html><html><head><title>Example Domain</title></head>\n<body><div><h1>Example Domain</h1>\n<p>This domain is for use in illustrative examples in documents</p></div></body></html>",
        );
        assert_eq!(
            ret,
            r"Example Domain\nExample Domain\nThis domain is for use in illustrative examples in documents"
        );
    }
}
