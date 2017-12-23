extern crate regex;
use regex::Regex;

fn main() {
    let check_data = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("did u date match? {}", check_data.is_match("2014-02-01"));
}
