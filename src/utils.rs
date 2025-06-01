pub fn without_trailing_slash(url: &str) -> String {
    if url.ends_with('/') {
        url[..url.len() - 1].to_string()
    } else {
        url.to_string()
    }
}
