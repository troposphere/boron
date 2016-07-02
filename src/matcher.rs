use regex::Regex;

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
