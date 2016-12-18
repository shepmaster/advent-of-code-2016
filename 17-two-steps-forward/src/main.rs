extern crate hex;
extern crate md5;

use hex::ToHex;
use std::collections::VecDeque;

fn open(c: char) -> bool {
    match c {
        'b' | 'c' | 'd' | 'e' | 'f' => true,
        _ => false,
    }
}

fn md5hex(s: &str) -> String {
    md5::compute(s.as_bytes()).to_hex()
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn to_char(&self) -> char {
        use Move::*;

        match *self {
            Up => 'U',
            Down => 'D',
            Left => 'L',
            Right => 'R',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct OpenDoors {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn whats_open(passcode: &str, path: &[Move]) -> OpenDoors {
    let mut a = passcode.to_owned();
    a.extend(path.iter().map(Move::to_char));
    let hashed = md5hex(&a);
    let mut open = hashed.chars().take(4).map(open);
    OpenDoors {
        up: open.next().expect("Missing up door"),
        down: open.next().expect("Missing down door"),
        left: open.next().expect("Missing left door"),
        right: open.next().expect("Missing right door"),
    }
}

#[derive(Debug)]
struct State {
    path: Vec<Move>,
    position: Position,
}

impl State {
    fn move_dir<F>(&self, dir: Move, f: F) -> State
        where F: FnOnce(usize, usize) -> (usize, usize)
    {
        let mut path = self.path.clone();
        path.push(dir);
        let (x, y) = f(self.position.0, self.position.1);
        State {
            path: path,
            position: Position(x, y),
        }
    }

    fn move_left(&self) -> State {
        self.move_dir(Move::Left, |x, y| (x - 1, y))
    }

    fn move_right(&self) -> State {
        self.move_dir(Move::Right, |x, y| (x + 1, y))
    }

    fn move_up(&self) -> State {
        self.move_dir(Move::Up, |x, y| (x, y - 1))
    }

    fn move_down(&self) -> State {
        self.move_dir(Move::Down, |x, y| (x, y + 1))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position(usize, usize);

const LAST_X: usize = 3;
const LAST_Y: usize = 3;
const TARGET_POSITION: Position = Position(LAST_X, LAST_Y);

impl Position {
    fn adapt(&self, mut doors: OpenDoors) -> OpenDoors {
        if self.0 == 0 {
            doors.left = false;
        }

        if self.0 == LAST_X {
            doors.right = false;
        }

        if self.1 == 0 {
            doors.up = false;
        }

        if self.1 == LAST_Y {
            doors.down = false;
        }

        doors
    }
}

fn puzzle(passcode: &str) -> Vec<Move> {
    let mut queue = VecDeque::new();

    queue.push_back(State { path: Vec::new(), position: Position(0, 0) });

    while let Some(step) = queue.pop_front() {
        let doors = whats_open(passcode, &step.path);
        let doors = step.position.adapt(doors);

        let mut new_states = Vec::new();

        if doors.left  { new_states.push(step.move_left())  }
        if doors.right { new_states.push(step.move_right()) }
        if doors.up    { new_states.push(step.move_up())    }
        if doors.down  { new_states.push(step.move_down())  }

        for new_state in new_states {
            if new_state.position == TARGET_POSITION {
                return new_state.path;
            }

            queue.push_back(new_state);
        }
    }

    Vec::new()
}

fn move_string(moves: &[Move]) -> String {
    moves.iter().map(Move::to_char).collect()
}

fn main() {
    let path = puzzle("udskfozm");
    println!("{}", move_string(&path));
}

#[test]
fn example_1() {
    assert_eq!(
        whats_open("hijkl", &[]),
        OpenDoors {
            up: true,
            down: true,
            left: true,
            right: false,
        }
    );
}

#[test]
fn example_2() {
    assert_eq!(move_string(&puzzle("ihgpwlah")), "DDRRRD");
    assert_eq!(move_string(&puzzle("kglvqrro")), "DDUDRLRRUDRD");
    assert_eq!(move_string(&puzzle("ulqzkmiv")), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
}
