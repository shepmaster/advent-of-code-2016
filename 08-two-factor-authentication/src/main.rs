#![feature(field_init_shorthand)]

extern crate regex;

use std::str::FromStr;
use std::error::Error;
use std::fmt;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Rectangle { x: usize, y: usize },
    RotateColumn { idx: usize, amount: usize },
    RotateRow { idx: usize, amount: usize },
}

#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref PARSE_RECTANGLE: Regex = Regex::new(
        r"rect (\d+)x(\d+)"
    ).expect("Could not compile rectangle regex");

    static ref PARSE_ROTATE_COLUMN: Regex = Regex::new(
        r"rotate column x=(\d+) by (\d+)"
    ).expect("Could not compile column regex");

    static ref PARSE_ROTATE_ROW: Regex = Regex::new(
        r"rotate row y=(\d+) by (\d+)"
    ).expect("Could not compile row regex");
}

impl FromStr for Instruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = PARSE_RECTANGLE.captures(s) {
            match (captures.at(1), captures.at(2)) {
                (Some(x), Some(y)) => {
                    let x = x.parse()?;
                    let y = y.parse()?;
                    Ok(Instruction::Rectangle { x, y })
                },
                _ => Err("Not enough parameters for rectangle".into())
            }
        } else if let Some(captures) = PARSE_ROTATE_COLUMN.captures(s) {
            match (captures.at(1), captures.at(2)) {
                (Some(idx), Some(amount)) => {
                    let idx = idx.parse()?;
                    let amount = amount.parse()?;
                    Ok(Instruction::RotateColumn { idx, amount })
                },
                _ => Err("Not enough parameters for column".into())
            }
        } else if let Some(captures) = PARSE_ROTATE_ROW.captures(s) {
            match (captures.at(1), captures.at(2)) {
                (Some(idx), Some(amount)) => {
                    let idx = idx.parse()?;
                    let amount = amount.parse()?;
                    Ok(Instruction::RotateRow { idx, amount })
                },
                _ => Err("Not enough parameters for row".into())
            }
        } else {
            Err("Unknown instruction".into())
        }
    }
}

#[derive(Clone, PartialEq)]
struct Display {
    leds: Vec<Vec<bool>>,
}

impl Display {
    fn new(x: usize, y: usize) -> Self {
        Display {
            leds: vec![vec![false; x]; y],
        }
    }

    fn process(&mut self, instruction: Instruction) {
        use Instruction::*;

        match instruction {
            Rectangle { x, y } => {
                for line in &mut self.leds[0..y] {
                    for pixel in &mut line[0..x] {
                        *pixel = true;
                    }
                }
            }
            RotateColumn { idx, amount } => {
                let h = self.height();
                let amount = amount % h;
                let mut new = vec![false; h];

                for (i, row) in self.leds.iter().enumerate() {
                    new[(i + amount) % h] = row[idx];
                }

                for (row, val) in self.leds.iter_mut().zip(new) {
                    row[idx] = val;
                }
            }
            RotateRow { idx, amount } => {
                let w = self.width();
                let amount = amount % w;
                let mut new = vec![false; w];
                for (i, &on) in self.leds[idx].iter().enumerate() {
                    new[(i + amount) % w] = on;
                }
                self.leds[idx] = new;
            }
        }
    }

    fn width(&self) -> usize {
        self.leds[0].len()
    }

    fn height(&self) -> usize {
        self.leds.len()
    }

    fn count(&self) -> usize {
        self.leds.iter().flat_map(|line| line).filter(|&&on| on).count()
    }
}

impl FromStr for Display {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.lines().map(|l| {
            l.chars().map(|c| c == '#').collect()
        }).collect();

        // TODO: assert rectangular

        Ok(Display {
            leds: v,
        })
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.leds {
            for &pixel in line {
                write!(fmt, "{}", if pixel { "#" } else { "." })?;
            }
            writeln!(fmt, "")?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut display = Display::new(50, 6);
    for line in input.lines() {
        display.process(line.parse().expect("Coldn't parse instruction"));
    }
    println!("{:?}", display);
    println!("There are {} lights", display.count());
}

#[cfg(test)]
mod test {
    use super::{Display, Instruction};

    fn quick_display(s: &str) -> Display {
        s.parse().expect("Unable to parse test display")
    }

    #[test]
    fn example_1() {
        let mut display = Display::new(7, 3);

        display.process(Instruction::Rectangle { x: 3, y: 2 });

        let expected = quick_display("###....\n\
                                      ###....\n\
                                      .......");

        assert_eq!(display, expected);
    }

    #[test]
    fn example_2() {
        let mut display = quick_display("###....\n\
                                         ###....\n\
                                         .......");

        display.process(Instruction::RotateColumn { idx: 1, amount: 1 });

        let expected = quick_display("#.#....\n\
                                      ###....\n\
                                      .#.....");

        assert_eq!(display, expected);
    }

    #[test]
    fn example_3() {
        let mut display = quick_display("#.#....\n\
                                         ###....\n\
                                         .#.....");

        display.process(Instruction::RotateRow { idx: 0, amount: 4 });

        let expected = quick_display("....#.#\n\
                                      ###....\n\
                                      .#.....");

        assert_eq!(display, expected);
    }

    #[test]
    fn example_4() {
        let mut display = quick_display("....#.#\n\
                                         ###....\n\
                                         .#.....");

        display.process(Instruction::RotateColumn { idx: 1, amount: 1 });

        let expected = quick_display(".#..#.#\n\
                                      #.#....\n\
                                      .#.....\n");

        assert_eq!(display, expected);
    }

    #[test]
    fn instruction_1() {
        let actual: Instruction = "rect 100x50".parse().expect("Could not parse instruction");
        assert_eq!(actual, Instruction::Rectangle { x: 100, y: 50 });
    }

    #[test]
    fn instruction_2() {
        let actual: Instruction = "rotate row y=99 by 42".parse().expect("Could not parse instruction");
        assert_eq!(actual, Instruction::RotateRow { idx: 99, amount: 42 });
    }

    #[test]
    fn instruction_3() {
        let actual: Instruction = "rotate column x=42 by 99".parse().expect("Could not parse instruction");
        assert_eq!(actual, Instruction::RotateColumn { idx: 42, amount: 99 });
    }
}
