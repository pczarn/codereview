use regex::Captures;
use regex::Regex;

#[derive(Debug)]
struct LineParseError;
#[derive(Debug)]
enum Tag {
    Open(String),
    Close(String),
    Scalar(String, String),
}
fn extract_line_tag(line: &str) -> Result<Tag, LineParseError> {
    let reg: Regex = Regex::new(r"<(?P<close>/)|((?P<open>.+)>(?P<value>.+)?)").unwrap();

    let capture = reg.captures(line);
    match capture {
        None => Err(LineParseError),
        Some(cap) => Ok(extract_tag_from_line_match(cap)),
    }
}

fn extract_tag_from_line_match(capture: Captures) -> Tag {
    let close = capture.name("close");
    let open = capture.name("open");
    let value = capture.name("value");

    let scalar = open.zip(value);
    if let Some((o, v)) = scalar {
        return Tag::Scalar(o.as_str().to_string(), v.as_str().to_string());
    } else if let Some(c) = close {
        return Tag::Close(c.as_str().to_string());
    } else if let Some(o) = open {
        return Tag::Open(o.as_str().to_string());
    } else {
        unreachable!();
    }
}

fn main() {
    println!("{:?}", extract_line_tag("<val>"));
    println!("{:?}", extract_line_tag("</val>"));
}