#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

fn decompress_len(mut input: &str, recursive: bool) -> usize {
    lazy_static! {
        static ref PARENS: Regex = Regex::new(
            r"\((\d+)x(\d+)\)"
        ).expect("Invalid regular expression");
    }

    let mut result = 0;

    while let Some(captures) = PARENS.captures(input) {
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

        result += before_match.len();
        let length_of_repeated_section = if recursive {
            decompress_len(to_repeat, recursive)
        } else {
            to_repeat.len()
        };
        result += length_of_repeated_section * count;

        input = remainder;
    }

    result += input.len();

    result
}

fn main() {
    let input = include_str!("input.txt");
    let decompressed = decompress_len(input, false);
    println!("Version 1 decompressed data was {} bytes", decompressed);
    let decompressed = decompress_len(input, true);
    println!("Version 2 decompressed data was {} bytes", decompressed);
}

#[test]
fn example_1() {
    assert_eq!(decompress_len("ADVENT", false), 6);
}

#[test]
fn example_2() {
    assert_eq!(decompress_len("A(1x5)BC", false), 7);
}

#[test]
fn example_3() {
    assert_eq!(decompress_len("(3x3)XYZ", false), 9);
}

#[test]
fn example_4() {
    assert_eq!(decompress_len("A(2x2)BCD(2x2)EFG", false), 11);
}

#[test]
fn example_5() {
    assert_eq!(decompress_len("(6x1)(1x3)A", false), 6);
}

#[test]
fn example_6() {
    assert_eq!(decompress_len("X(8x2)(3x3)ABCY", false), 18);
}

#[test]
fn example_7() {
    assert_eq!(decompress_len("(3x3)XYZ", true), 9);
}

#[test]
fn example_8() {
    assert_eq!(decompress_len("X(8x2)(3x3)ABCY", true), 20);
}

#[test]
fn example_9() {
    assert_eq!(decompress_len("(27x12)(20x12)(13x14)(7x10)(1x12)A", true), 241920);
}

#[test]
fn example_10() {
    assert_eq!(decompress_len("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", true), 445);
}
