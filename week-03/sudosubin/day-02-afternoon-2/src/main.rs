// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]
use std::cmp;

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let prefix_tokens = Vec::from_iter(prefix.split("/"));
    let request_path_tokens = Vec::from_iter(request_path.split("/"));

    if prefix_tokens.len() > request_path_tokens.len() {
        return false;
    }

    for i in 0..cmp::min(prefix_tokens.len(), request_path_tokens.len()) {
        let prefix_token = match prefix_tokens.get(i) {
            Some(prefix_token) if prefix_token.ne(&"*") => prefix_token,
            Some(prefix_token) if prefix_token.eq(&"*") => continue,
            _ => return false,
        };

        let request_path_token = match request_path_tokens.get(i) {
            Some(request_path_token) => request_path_token,
            _ => return false,
        };

        if prefix_token != request_path_token {
            return false;
        }
    }

    return true;
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {}
