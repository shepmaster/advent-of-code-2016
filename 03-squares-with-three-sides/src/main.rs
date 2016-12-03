extern crate itertools;

use std::str::FromStr;
use std::error::Error;

use itertools::Itertools;

struct MaybeTriangle {
    a: u32,
    b: u32,
    c: u32,
}

impl MaybeTriangle {
    /// To be valid, the sum of any two sides must be larger than the
    /// remaining side
    fn is_valid(&self) -> bool {
        let MaybeTriangle { a, b, c } = *self;
        a + b > c && b + c > a && a + c > b
    }

    fn from_three_optional_strings(a: Option<&str>,
                                   b: Option<&str>,
                                   c: Option<&str>)
                                   -> Result<Self, Box<Error>> {
        let a = a.ok_or("Number missing".to_string())?;
        let b = b.ok_or("Number missing".to_string())?;
        let c = c.ok_or("Number missing".to_string())?;

        let a = a.parse()?;
        let b = b.parse()?;
        let c = c.parse()?;

        Ok(MaybeTriangle { a: a, b: b, c: c })
    }
}

impl FromStr for MaybeTriangle {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().fuse();
        MaybeTriangle::from_three_optional_strings(parts.next(), parts.next(), parts.next())
    }
}

fn horizontal_algorithm(input: &str) -> usize {
    input.lines()
        .filter_map(|l| l.parse::<MaybeTriangle>().ok())
        .filter(MaybeTriangle::is_valid)
        .count()
}

fn vertical_algorithm(input: &str) -> usize {
    input.lines().tuples().map(|(line1, line2, line3)| {
        let mut line1 = line1.split_whitespace().fuse();
        let mut line2 = line2.split_whitespace().fuse();
        let mut line3 = line3.split_whitespace().fuse();

        (0..3).filter_map(|_| {
            let a = line1.next();
            let b = line2.next();
            let c = line3.next();

            MaybeTriangle::from_three_optional_strings(a, b, c).ok()
        }).filter(MaybeTriangle::is_valid).count()
    }).sum()
}

fn main() {
    let input = include_str!("input.txt");

    let count = horizontal_algorithm(input);
    println!("There were {} valid triangles", count);

    let count = vertical_algorithm(input);
    println!("There were {} valid triangles", count);
}

#[test]
fn example_1() {
    assert_eq!(0, horizontal_algorithm("5 10 25"));
}

#[test]
fn example_2() {
    let input = "101 301 501
                 102 302 502
                 103 303 503
                 201 401 601
                 202 402 602
                 203 403 603";
    assert_eq!(6, vertical_algorithm(input));
}
