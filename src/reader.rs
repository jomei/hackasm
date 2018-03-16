use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use line::Line;

// из за того что есть Read, можно подложить что угодно, имплементирующее  Read, например Cursor
pub fn call<R: Read>(content: R) -> Vec<Line> {
    return BufReader::new(content)
        .lines()
        .map(|l| l.expect("Could not parse line").trim().to_string())
        .filter(|l| !(l.is_empty() || l.starts_with("//")))
        .enumerate()
        .map(|(i, line)| Line::new(line, i))
        .collect();
}
