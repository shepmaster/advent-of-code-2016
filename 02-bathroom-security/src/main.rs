use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Result<Self, Box<Error>> {
        use Direction::*;

        Ok(match c {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => return Err("Not a valid direction".into()),
        })
    }
}

trait Keypad {
    fn next(&self, d: Direction) -> Self;

    fn as_str(&self) -> &'static str;
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum StandardKeypad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl StandardKeypad {
    fn up(&self) -> StandardKeypad {
        use StandardKeypad::*;

        match *self {
            Four  => One,
            Five  => Two,
            Six   => Three,
            Seven => Four,
            Eight => Five,
            Nine  => Six,
            other => other,
        }
    }

    fn down(&self) -> StandardKeypad {
        use StandardKeypad::*;

        match *self {
            One   => Four,
            Two   => Five,
            Three => Six,
            Four  => Seven,
            Five  => Eight,
            Six   => Nine,
            other => other,
        }
    }

    fn left(&self) -> StandardKeypad {
        use StandardKeypad::*;

        match *self {
            Two   => One,
            Three => Two,
            Five  => Four,
            Six   => Five,
            Eight => Seven,
            Nine  => Eight,
            other => other,
        }
    }

    fn right(&self) -> StandardKeypad {
        use StandardKeypad::*;

        match *self {
            One   => Two,
            Two   => Three,
            Four  => Five,
            Five  => Six,
            Seven => Eight,
            Eight => Nine,
            other => other,
        }
    }
}

impl Keypad for StandardKeypad {
    fn next(&self, d: Direction) -> StandardKeypad {
        use Direction::*;

        match d {
            Up    => self.up(),
            Down  => self.down(),
            Left  => self.left(),
            Right => self.right()
        }
    }

    fn as_str(&self) -> &'static str {
        use StandardKeypad::*;

        match *self {
            One   => "1",
            Two   => "2",
            Three => "3",
            Four  => "4",
            Five  => "5",
            Six   => "6",
            Seven => "7",
            Eight => "8",
            Nine  => "9",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum FancyKeypad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl FancyKeypad {
    fn up(&self) -> FancyKeypad {
        use FancyKeypad::*;

        match *self {
            Three => One,
            Six   => Two,
            Seven => Three,
            Eight => Four,
            A     => Six,
            B     => Seven,
            C     => Eight,
            D     => B,
            other => other,
        }
    }

    fn down(&self) -> FancyKeypad {
        use FancyKeypad::*;

        match *self {
            One   => Three,
            Two   => Six,
            Three => Seven,
            Four  => Eight,
            Six   => A,
            Seven => B,
            Eight => C,
            B     => D,
            other => other,
        }
    }

    fn left(&self) -> FancyKeypad {
        use FancyKeypad::*;

        match *self {
            Three => Two,
            Four  => Three,
            Six   => Five,
            Seven => Six,
            Eight => Seven,
            Nine  => Eight,
            B     => A,
            C     => B,
            other => other,
        }
    }

    fn right(&self) -> FancyKeypad {
        use FancyKeypad::*;

        match *self {
            Two   => Three,
            Three => Four,
            Five  => Six,
            Six   => Seven,
            Seven => Eight,
            Eight => Nine,
            A     => B,
            B     => C,
            other => other,
        }
    }
}

impl Keypad for FancyKeypad {
    fn next(&self, d: Direction) -> FancyKeypad {
        use Direction::*;

        match d {
            Up    => self.up(),
            Down  => self.down(),
            Left  => self.left(),
            Right => self.right()
        }
    }

    fn as_str(&self) -> &'static str {
        use FancyKeypad::*;

        match *self {
            One   => "1",
            Two   => "2",
            Three => "3",
            Four  => "4",
            Five  => "5",
            Six   => "6",
            Seven => "7",
            Eight => "8",
            Nine  => "9",
            A     => "A",
            B     => "B",
            C     => "C",
            D     => "D",
        }
    }
}

fn the_code<K>(input: &str, initial_key: K) -> String
    where K: Keypad
{
    let mut key = initial_key;
    input.lines().map(|l| {
        for direction in l.trim().chars().map(Direction::from_char) {
            key = key.next(direction.expect("Invalid direction"));
        }
        key.as_str()
    }).collect()
}

fn main() {
    let input = include_str!("input.txt");

    println!("{:?}", the_code(input, StandardKeypad::Five));
    println!("{:?}", the_code(input, FancyKeypad::Five));
}

#[test]
fn example_1() {
    let input = "ULL
                 RRDDD
                 LURDL
                 UUUUD";

    assert_eq!(the_code(input, StandardKeypad::Five), "1985");
}

#[test]
fn example_2() {
    let input = "ULL
                 RRDDD
                 LURDL
                 UUUUD";

    assert_eq!(the_code(input, FancyKeypad::Five), "5DB3");
}
