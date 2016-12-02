use std::str::FromStr;
use std::error::Error;

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

fn algorithm(input: &str) -> (u64, u64) {
    let mut direction = Direction::North;
    let mut blocks_north_south = 0;
    let mut blocks_east_west = 0;

    for command in input.split(",").map(|i| i.parse::<Command>().expect("Couldn't parse input")) {
        direction = direction.turn(command.turn);
        let blocks = command.blocks as i64;

        match direction {
            Direction::North => blocks_north_south += blocks,
            Direction::South => blocks_north_south -= blocks,
            Direction::East => blocks_east_west += blocks,
            Direction::West => blocks_east_west -= blocks,
        }
    }

    // Only care about magnitude
    (blocks_north_south.abs() as u64, blocks_east_west.abs() as u64)
}

fn main() {
    let input = "R4, R4, L1, R3, L5, R2, R5, R1, L4, R3, L5, R2, L3, L4, L3, R1, R5, R1, L3, L1, \
                 R3, L1, R2, R2, L2, R5, L3, L4, R4, R4, R2, L4, L1, R5, L1, L4, R4, L1, R1, L2, \
                 R5, L2, L3, R2, R1, L194, R2, L4, R49, R1, R3, L5, L4, L1, R4, R2, R1, L5, R3, \
                 L5, L4, R4, R4, L2, L3, R78, L5, R4, R191, R4, R3, R1, L2, R1, R3, L1, R3, R4, \
                 R2, L2, R1, R4, L5, R2, L2, L4, L2, R1, R2, L3, R5, R2, L3, L3, R3, L1, L1, R5, \
                 L4, L4, L2, R5, R1, R4, L3, L5, L4, R5, L4, R5, R4, L3, L2, L5, R4, R3, L3, R1, \
                 L5, R5, R1, L3, R2, L5, R5, L3, R1, R4, L5, R4, R2, R3, L4, L5, R3, R4, L5, L5, \
                 R4, L4, L4, R1, R5, R3, L1, L4, L3, L4, R1, L5, L1, R2, R2, R4, R4, L5, R4, R1, \
                 L1, L1, L3, L5, L2, R4, L3, L5, L4, L1, R3";

    let (blocks_north_south, blocks_east_west) = algorithm(input);
    let total = blocks_north_south + blocks_east_west;

    println!("{} + {} => {}", blocks_north_south, blocks_east_west, total);
}

#[test]
fn example_1() {
    assert_eq!(algorithm("R2, L3"), (3, 2));
}

#[test]
fn example_2() {
    assert_eq!(algorithm("R2, R2, R2"), (2, 0));
}

#[test]
fn example_3() {
    let (ns, ew) = algorithm("R5, L5, R5, R3");
    assert_eq!(ns + ew, 12) ;
}
