use std::cmp;
use std::io::{self, BufRead};

extern crate unicode_width;

use unicode_width::UnicodeWidthStr;

fn main() {
    let (lines, widths) = read_lines();
    align(lines, widths);
}

fn align(lines: Vec<Vec<String>>, widths: Vec<usize>) {
    for line in lines {
        let mut adjusted: Vec<String> = vec![];
        for (i, part) in line.iter().enumerate() {
            let n = widths[i] - UnicodeWidthStr::width(part.as_str());
            let sep: String = " ".repeat(n);
            if is_number(part) {
                adjusted.push(format!("{}{}", sep, part));
            } else {
                adjusted.push(format!("{}{}", part, sep));
            }
        }
        println!("{}", adjusted.join(&" ".repeat(2)));
    }
}

fn is_number(s: &str) -> bool {
    if s.parse::<f64>().is_ok() {
        return true;
    }
    if s.parse::<i64>().is_ok() {
        return true;
    }
    false
}

fn read_lines() -> (Vec<Vec<String>>, Vec<usize>) {
    let mut lines: Vec<Vec<String>> = vec![];
    let mut widths: Vec<usize> = vec![];
    let stdin = io::stdin();
    for result in stdin.lock().lines() {
        let line = result.unwrap();
        let mut parts: Vec<String> = vec![];
        for (i, part) in line.split(",").enumerate() {
            let w = UnicodeWidthStr::width(part);
            if i >= widths.len() {
                widths.push(w);
            } else {
                widths[i] = cmp::max(widths[i], w);
            }
            parts.push(part.to_string());
        }
        lines.push(parts);
    }
    (lines, widths)
}
