use std::str::FromStr;
use std::error::Error;

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
}

impl FromStr for MaybeTriangle {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().fuse();

        let a = parts.next().ok_or("Number missing".to_string())?;
        let b = parts.next().ok_or("Number missing".to_string())?;
        let c = parts.next().ok_or("Number missing".to_string())?;

        let a = a.parse()?;
        let b = b.parse()?;
        let c = c.parse()?;

        Ok(MaybeTriangle { a: a, b: b, c: c })
    }
}

fn algorithm(input: &str) -> usize {
    input.lines()
        .filter_map(|l| l.parse::<MaybeTriangle>().ok())
        .filter(MaybeTriangle::is_valid)
        .count()
}

fn main() {
    let input = include_str!("input.txt");

    let count = algorithm(input);

    println!("There were {} valid triangles", count);
}

#[test]
fn example_1() {
    assert_eq!(0, algorithm("5 10 25"));
}
