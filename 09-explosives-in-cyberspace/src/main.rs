extern crate regex;

use std::iter;
use regex::Regex;

fn decompress(mut input: &str) -> String {
    let parens = Regex::new(r"\((\d+)x(\d+)\)").expect("Invalid regular expression");

    let mut result = String::new();

    while let Some(captures) = parens.captures(input) {
        let bytes: usize = captures.at(1)
            .expect("digit 1 missing")
            .parse()
            .expect("digit 1 invalid");
        let count: usize = captures.at(2)
            .expect("digit 2 missing")
            .parse()
            .expect("digit 2 invalid");

        let (start, end) = captures.pos(0).expect("There must always be a total capture");

        let before_match = &input[..start];
        let after_match = &input[end..];
        let (to_repeat, remainder) = after_match.split_at(bytes);

        result.push_str(before_match);
        result.extend(iter::repeat(to_repeat).take(count));

        input = remainder;
    }

    result.push_str(input);

    result
}

fn main() {
    let input = include_str!("input.txt");
    let decompressed = decompress(input);
    println!("Decompressed data was {} bytes", decompressed.len());
}

#[test]
fn example_1() {
    assert_eq!(decompress("ADVENT"), "ADVENT");
}

#[test]
fn example_2() {
    assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
}

#[test]
fn example_3() {
    assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
}

#[test]
fn example_4() {
    assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
}

#[test]
fn example_5() {
    assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
}

#[test]
fn example_() {
    assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
}
