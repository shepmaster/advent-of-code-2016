use std::str::FromStr;
use std::error::Error;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl FromStr for Turn {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Turn::Right,
            "L" => Turn::Left,
            _ => return Err("Not a direction".into()),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Command {
    turn: Turn,
    blocks: u32,
}

impl FromStr for Command {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (t, b) = s.trim().split_at(1); // FIXME: error handling
        let t = t.parse()?;
        let b = b.parse()?;
        Ok(Command {
            turn: t,
            blocks: b,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        use Direction::*;
        use Turn::*;

        match (*self, turn) {
            (North, Left)  => West,
            (North, Right) => East,
            (East,  Left)  => North,
            (East,  Right) => South,
            (South, Left)  => East,
            (South, Right) => West,
            (West,  Left)  => South,
            (West,  Right) => North,
        }
    }
}
type Coordinates = (i64, i64);

#[derive(Debug)]
struct Tracker {
    blocks_north_south: i64,
    blocks_east_west: i64,
    visited: HashSet<Coordinates>,
    first_intersection: Option<Coordinates>,
}

/// Exclusive because we've already visited the starting location,
/// inclusive so we visit the ending location.
macro_rules! iterate_in_either_direction_exclusive_inclusive {
    ($a:expr, $b:expr) => {
        if $a < $b {
            Box::new($a..($b+1)) as Box<Iterator<Item = i64>>
        } else {
            Box::new((($b+1)..$a).rev())
        }.skip(1)
    }
}

impl Tracker {
    fn new() -> Tracker {
        let mut visited = HashSet::default();
        // We start at the origin
        visited.insert((0, 0));

        Tracker {
            blocks_north_south: 0,
            blocks_east_west: 0,
            visited: visited,
            first_intersection: None,
        }
    }

    fn travel_north_south(&mut self, blocks: i64) {
        let start = self.blocks_north_south;
        let end = start + blocks;

        let x = self.blocks_east_west;
        for y in iterate_in_either_direction_exclusive_inclusive!(start, end) {
            self.track_visited(x, y);
        }

        self.blocks_north_south = end;
    }

    fn travel_east_west(&mut self, blocks: i64) {
        let start = self.blocks_east_west;
        let end = start + blocks;

        let y = self.blocks_north_south;
        for x in iterate_in_either_direction_exclusive_inclusive!(start, end) {
            self.track_visited(x, y);
        }

        self.blocks_east_west = end;
    }

    fn track_visited(&mut self, x: i64, y: i64) {
        if self.first_intersection.is_some() { return }

        let coord = (x, y);

        if !self.visited.insert(coord) {
            self.first_intersection = Some(coord);
        }
    }

    fn end_position(&self) -> (u64, u64) {
        // Only care about magnitude
        (self.blocks_east_west.abs() as u64, self.blocks_north_south.abs() as u64)
    }

    fn first_intersection(&self) -> Option<(u64, u64)> {
        // Only care about magnitude
        self.first_intersection.map(|(x, y)| (x.abs() as u64, y.abs() as u64))
    }
}

fn track_commands(input: &str) -> Tracker {
    let mut direction = Direction::North;
    let mut tracker = Tracker::new();

    for command in input.split(",").map(|i| i.parse::<Command>().expect("Couldn't parse input")) {
        direction = direction.turn(command.turn);
        let blocks = command.blocks as i64;

        match direction {
            Direction::North => tracker.travel_north_south(blocks),
            Direction::South => tracker.travel_north_south(-blocks),
            Direction::East => tracker.travel_east_west(blocks),
            Direction::West => tracker.travel_east_west(-blocks),
        }
    }

    tracker
}

fn main() {
    let input = include_str!("input.txt");

    let tracker = track_commands(input);

    let (blocks_north_south, blocks_east_west) = tracker.end_position();
    let total = blocks_north_south + blocks_east_west;

    println!("{} + {} => {}", blocks_north_south, blocks_east_west, total);

    let (blocks_east_west, blocks_north_south) = tracker.first_intersection().expect("No intersection found");
    let total = blocks_north_south + blocks_east_west;

    println!("{} + {} => {}", blocks_north_south, blocks_east_west, total);
}

#[test]
fn example_1() {
    assert_eq!(track_commands("R2, L3").end_position(), (2, 3));
}

#[test]
fn example_2() {
    assert_eq!(track_commands("R2, R2, R2").end_position(), (0, 2));
}

#[test]
fn example_3() {
    let (ns, ew) = track_commands("R5, L5, R5, R3").end_position();
    assert_eq!(ns + ew, 12) ;
}

#[test]
fn example_4() {
    assert_eq!(track_commands("R8, R4, R4, R8").first_intersection(), Some((4, 0)));
}
