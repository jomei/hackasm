use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

const COMMENT_MARKER: &str = "//";

// из за того что есть Read, можно подложить что угодно, имплементирующее  Read, например Cursor
pub fn call<R: Read>(content: R) -> Vec<String> {
    return BufReader::new(content)
        .lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .filter(|l| !(l.is_empty() || l.starts_with(COMMENT_MARKER)))
        .map(|l| clear_inline_comment(l))
        .collect();
}

fn clear_inline_comment(line: String) -> String {
    let split: Vec<&str> =  line.split(COMMENT_MARKER).collect();
    split[0].trim().to_string()
}
