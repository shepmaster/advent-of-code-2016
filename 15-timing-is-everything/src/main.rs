use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Disc {
    current: usize,
    positions: usize,
}

impl Disc {
    fn passes_at(&self, time: usize) -> bool {
        (self.current + time) % self.positions == 0
    }
}

impl FromStr for Disc {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        let mut parts = s.split_whitespace().fuse();
        let positions = parts.by_ref().nth(3).ok_or("Missing disc positions count")?.parse()?;
        let current = parts.by_ref().nth(7).ok_or("Missing disc current position")?.trim_right_matches(".").parse()?;

        Ok(Disc {
            positions: positions,
            current: current
        })
    }
}

fn puzzle(discs: &[Disc]) -> Option<usize> {
    (0..).find(|time| {
        discs.iter().enumerate().all(|(idx, disc)| {
            disc.passes_at(time + idx + 1) // Add time for the first fall
        })
    })
}

fn main() {
    let input = include_str!("input.txt");
    let discs: Result<Vec<Disc>, _> = input.lines().map(str::parse).collect();
    let discs = discs.expect("Unable to parse discs");
    let time = puzzle(&discs);
    println!("Drop at {:?}", time);
}

#[test]
fn example_1() {
    let disc = Disc { current: 4, positions: 5 };
    assert!(disc.passes_at(1));
    assert!(disc.passes_at(6));
}

#[test]
fn example_2() {
    let disc = Disc { current: 1, positions: 2 };
    assert!(!disc.passes_at(2));
    assert!(disc.passes_at(7));
}

#[test]
fn example_3() {
    let discs = [
        Disc { current: 4, positions: 5 },
        Disc { current: 1, positions: 2 },
    ];

    assert_eq!(puzzle(&discs), Some(5));
}

#[test]
fn parsing() {
    let disc = "Disc #1 has 5 positions; at time=0, it is at position 4.".parse::<Disc>();
    assert_eq!(disc.unwrap(), Disc { positions: 5, current: 4 });
}
