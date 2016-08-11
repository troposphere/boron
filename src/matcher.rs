use regex::Regex;

pub struct Matcher {
    pattern: Regex
}

impl Matcher {
    pub fn new(pattern: &str) -> Matcher {
        Matcher { pattern: Regex::new(format!("^{}$", pattern).as_str()).unwrap() }
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.pattern.is_match(text)
    }

    pub fn matched_tokens(&self, text: &str) -> Vec<(String, String)> {
        match self.pattern.captures(text) {
            Some(captures) => {
                let mut tokens = vec![];
                for token in captures.iter_named() {
                    tokens.push((token.0.to_string(), token.1.unwrap().to_string()));
                }
                tokens
            },
            None => vec![]
        }
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
fn test_matched_groups_without_name() {
    let m = Matcher::new(r"/api/(\d{1})/user/(\d*)");
    let test_path = "/api/1/user/32571";
    assert!(m.is_match(test_path));
    let tokens = m.matched_tokens(test_path);
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_matched_groups_name() {
    let m = Matcher::new(r"/api/(?P<version>\d{1})/user/(?P<id>\d*)");
    let test_path = "/api/1/user/32571";
    assert!(m.is_match(test_path));
    let tokens = m.matched_tokens(test_path);
    assert!(tokens.contains(&("version".to_string(), "1".to_string())));
    assert!(tokens.contains(&("id".to_string(), "32571".to_string())));
}
