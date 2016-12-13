use std::str::FromStr;
use std::ops::{Index, IndexMut};

type Error = Box<::std::error::Error>;
type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Default)]
struct Machine {
    registers: Registers,
}

impl Machine {
    fn new() -> Self {
        Self::default()
    }

    fn value_of_register_or_immediate(&self, src: RegisterOrImmediate) -> i32 {
        use RegisterOrImmediate::*;

        match src {
            Immediate(v) => v,
            Register(r) => self.registers[r],
        }
    }

    fn run(&mut self, program: &[Instruction]) {
        use Instruction::*;

        let mut pc = 0;

        while let Some(&instruction) = program.get(pc) {
            let mut pc_delta = 1;

            match instruction {
                Cpy { src, dst } => {
                    self.registers[dst] = self.value_of_register_or_immediate(src);
                },
                Inc { reg } => self.registers[reg] += 1,
                Dec { reg } => self.registers[reg] -= 1,
                Jnz { src, offset } => {
                    if self.value_of_register_or_immediate(src) != 0 {
                        pc_delta = offset;
                    }
                },
            }

            let next_pc = if pc_delta < 0 {
                pc.overflowing_sub(pc_delta.abs() as usize)
            } else {
                pc.overflowing_add(pc_delta.abs() as usize)
            };

            pc = match next_pc {
                (_, true) => break,
                (v, false) => v,
            };
        }
    }

    fn register(&self, register: Register) -> i32 {
        self.registers[register]
    }

    fn set_register(&mut self, register: Register, value: i32) {
        self.registers[register] = value;
    }
}

#[derive(Debug, PartialEq, Default)]
struct Registers([i32; 4]);

impl Registers {
    fn index_for(register: Register) -> usize {
        use Register::*;

        match register {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }
}

impl Index<Register> for Registers {
    type Output = i32;

    fn index(&self, i: Register) -> &Self::Output {
        self.0.index(Registers::index_for(i))
    }
}


impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, i: Register) -> &mut Self::Output {
        self.0.index_mut(Registers::index_for(i))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Cpy { src: RegisterOrImmediate, dst: Register },
    Inc { reg: Register },
    Dec { reg: Register },
    Jnz { src: RegisterOrImmediate, offset: i32 },
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use Instruction::*;

        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("cpy") => {
                Ok(Cpy {
                    src: parts.next().ok_or("cpy instruction missing source")?.parse()?,
                    dst: parts.next().ok_or("cpy instruction missing destination")?.parse()?,
                })
            },
            Some("inc") => {
                Ok(Inc {
                    reg: parts.next().ok_or("inc instruction missing register")?.parse()?,
                })
            },
            Some("dec") => {
                Ok(Dec {
                    reg: parts.next().ok_or("dec instruction missing register")?.parse()?,
                })
            },
            Some("jnz") => {
                Ok(Jnz {
                    src: parts.next().ok_or("jnz instruction missing src")?.parse()?,
                    offset: parts.next().ok_or("jnz instruction missing offset")?.parse()?,
                })
            },
            Some(i) => Err(format!("unknown instruction '{}' (in '{}')", i, s).into()),
            None => Err("no instruction found".into()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RegisterOrImmediate {
    Register(Register),
    Immediate(i32),
}

impl FromStr for RegisterOrImmediate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use RegisterOrImmediate::*;

        s.parse().map(Immediate)
            .or_else(|_| s.parse().map(Register))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use Register::*;

        match s {
            "a" => Ok(A),
            "b" => Ok(B),
            "c" => Ok(C),
            "d" => Ok(D),
            _ => Err(format!("unknown register '{}'", s).into())
        }
    }
}

fn parse_code(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(|l| l.trim().parse()).collect()
}

fn run_code(input: &str) -> Result<Machine> {
    run_code_extended(input, |_| {})
}

fn run_code_extended<F>(input: &str, initialize: F) -> Result<Machine>
    where F: FnOnce(&mut Machine)
{
    parse_code(input).map(|program| {
        let mut machine = Machine::new();
        initialize(&mut machine);
        machine.run(&program);
        machine
    })
}

fn main() {
    let input = include_str!("input.txt");

    let machine = run_code(input).expect("Unable to execute program");
    println!("{}", machine.register(Register::A));

    let machine = run_code_extended(input, |machine| machine.set_register(Register::C, 1))
        .expect("Unable to execute program");
    println!("{}", machine.register(Register::A));
}

#[test]
fn example_1() {
    let input = "cpy 41 a\n\
                 inc a\n\
                 inc a\n\
                 dec a\n\
                 jnz a 2\n\
                 dec a";

    let machine = run_code(input).expect("Unable to execute program");

    assert_eq!(machine.register(Register::A), 42);
}
