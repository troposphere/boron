use regex::{Captures, Regex};

struct Matcher {
    pattern: Regex
}

impl Matcher {
    pub fn new(pattern: &str) -> Matcher {
        Matcher { pattern: Regex::new(format!("^{}$", pattern).as_str()).unwrap() }
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.pattern.is_match(text)
    }

    pub fn matched<'t>(&self, text: &'t str) -> Captures<'t> {
        assert!(self.is_match(text));
        self.pattern.captures(text).unwrap()
    }
}

#[test]
fn test_url_exact() {
    let m = Matcher::new("/abc/hello");
    assert!(m.is_match("/abc/hello"));
}

#[test]
fn test_url_preceed() {
    let m = Matcher::new("/abc/hello");
    assert_eq!(m.is_match("/v1/abc/hello"), false);
}

#[test]
fn test_url_follow() {
    let m = Matcher::new("/abc/hello");
    assert_eq!(m.is_match("/abc/hello_"), false);
}

#[test]
fn test_url_pattern_match() {
    let m = Matcher::new(r"/abc/\d*/hello");
    assert!(m.is_match("/abc/1/hello"));
    assert!(m.is_match("/abc/23/hello"));
    assert_eq!(m.is_match("/abc/1/hello/lol"), false);
}

#[test]
fn test_url_pattern_match_typesafe() {
    let m = Matcher::new(r"/abc/\d*/hello");
    assert_eq!(m.is_match("/abc/random/hello"), false);
}

#[test]
fn test_matched_groups() {
    let m = Matcher::new(r"/api/(\d{1})/user/(\d*)");
    let test_path = "/api/1/user/32571";
    assert!(m.is_match(test_path));
    let caps = m.matched(test_path);
    assert_eq!(caps.at(1), Some("1"));
    assert_eq!(caps.at(2), Some("32571"));
}

#[test]
fn test_matched_groups_name() {
    let m = Matcher::new(r"/api/(?P<version>\d{1})/user/(?P<id>\d*)");
    let test_path = "/api/1/user/32571";
    assert!(m.is_match(test_path));
    let caps = m.matched(test_path);
    assert_eq!(caps.name("version"), Some("1"));
    assert_eq!(caps.name("id"), Some("32571"));
}
