use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub version: String,
    pub status: String,
    pub explanation: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

// TODO: Add test.
/// Parses a HTTP response and creates a new `Response`.
pub fn parse_response(res: &str) -> Result<Response, String> {
    let mut res_lines = res.split("\r\n");

    // status line
    let status_line = res_lines
        .next()
        .ok_or("Failed to get HTTP response status line".to_string())?;
    let (version, rest) = status_line
        .split_once(" ")
        .ok_or("Expected a space in a status line".to_string())?;
    let (status, explanation) = rest
        .split_once(" ")
        .ok_or("Expected a space in a status line".to_string())?;

    // other headers
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in res_lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (key, value) = line
            .split_once(" ")
            .ok_or("Expected a space in a resonse headers".to_string())?;
        headers.insert(
            // Headers are case-insensitive. Normalize them to lower case.
            key.to_lowercase(),
            // Strip off extra whitespaces from header values.
            value.trim().to_string(),
        );
    }

    // Make sure that the response wasn't sent in an unusual way.
    assert!(headers.get("transfer-encoding").is_none());
    assert!(headers.get("content-encoding").is_none());

    // body
    let body = res_lines.collect::<Vec<&str>>().join("\r\n");

    return Ok(Response {
        version: version.into(),
        status: status.into(),
        explanation: explanation.into(),
        headers,
        body,
    });
}
