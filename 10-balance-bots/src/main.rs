#![feature(field_init_shorthand)]

extern crate regex;

use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;
use std::str::FromStr;
use std::error::Error;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BinNumber(usize);
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Value(usize);
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BotId(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Target {
    Bot(BotId),
    Bin(BinNumber),
}

impl FromStr for Target {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Target::*;

        let mut parts = s.splitn(2, " ");
        let target = parts.next().ok_or("missing target")?;
        let id = parts.next().ok_or("missing target id")?;
        let id = id.parse()?;

        match target {
            "bot" => Ok(Bot(BotId(id))),
            "output" => Ok(Bin(BinNumber(id))),
            _ => Err(format!("unknown target: {}", s).into()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Instruction {
    InitialValue { id: BotId, value: Value },
    Transfer { id: BotId, low: Target, high: Target },
}

impl FromStr for Instruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        let initial_value_re = Regex::new(r"value (\d+) goes to bot (\d+)").expect("Bad initial value regex");
        let transfer_re = Regex::new(r"bot (\d+) gives low to (.*) and high to (.*)").expect("Bad transfer regex");

        if let Some(captures) = initial_value_re.captures(s) {
            let value = captures.at(1).ok_or("missing value")?;
            let id = captures.at(2).ok_or("missing id")?;

            let value = Value(value.parse()?);
            let id = BotId(id.parse()?);

            Ok(InitialValue { id, value })
        } else if let Some(captures) = transfer_re.captures(s) {
            let id = captures.at(1).ok_or("missing id")?;
            let low = captures.at(2).ok_or("missing low")?;
            let high = captures.at(3).ok_or("missing high")?;

            let id = BotId(id.parse()?);
            let low = low.parse()?;
            let high = high.parse()?;

            Ok(Transfer { id, low, high })
        } else {
            Err(format!("unknown instruction: {}", s).into())
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Bot {
    input_a: Option<Value>,
    input_b: Option<Value>,
    low: Option<Target>,
    high: Option<Target>,
}

impl Bot {
    fn add_input(&mut self, value: Value) {
        match (self.input_a, self.input_b) {
            (None, _) => self.input_a = Some(value),
            (Some(_), None) => self.input_b = Some(value),
            _ => panic!("Too many inputs added to the bot"),
        }
    }

    fn transfer_to(&mut self, low: Target, high: Target) {
        self.low = Some(low);
        self.high = Some(high);
    }

    fn process(&self) -> Option<((Target, Value), (Target, Value))> {
        use std::cmp::Ordering::*;

        match (self.input_a, self.input_b) {
            (Some(a), Some(b)) => {
                let low = self.low.expect("No low yet");
                let high = self.high.expect("No high yet");

                match a.cmp(&b) {
                    Less => Some(((low, a), (high, b))),
                    Greater => Some(((low, b), (high, a))),
                    _ => panic!("Input values were equal"),
                }
            }
            _ => None
        }
    }

    fn inputs_are(&self, a: Value, b: Value) -> bool {
        let a = Some(a);
        let b = Some(b);

        (self.input_a == a && self.input_b == b) || (self.input_a == b && self.input_b == a)
    }
}

#[derive(Debug, Clone, Default)]
struct Factory {
    bots: BTreeMap<BotId, Bot>,
    processed: BTreeSet<BotId>,
    outputs: BTreeMap<BinNumber, Value>,
}

impl Factory {
    fn new() -> Self {
        Default::default()
    }

    fn initialize(&mut self, instruction: Instruction) {
        use Instruction::*;

        match instruction {
            InitialValue { id, value } => {
                self.bots.entry(id).or_insert_with(Default::default).add_input(value);
            },
            Transfer { id, low, high } => {
                self.bots.entry(id).or_insert_with(Default::default).transfer_to(low, high);
            },
        }
    }

    fn process(&mut self) {
        while self.bots.len() != self.processed.len() {
            let mut new_results = Vec::new();

            for (&id, bot) in self.bots.iter_mut() {
                if self.processed.contains(&id) { continue; }

                if let Some((low, high)) = bot.process() {
                    new_results.push(low);
                    new_results.push(high);
                    self.processed.insert(id);
                }
            }

            for (target, value) in new_results {
                match target {
                    Target::Bot(id) => {
                        self.bots.get_mut(&id).expect("Target bot missing").add_input(value);
                    },
                    Target::Bin(id) => {
                        self.outputs.entry(id).or_insert(value);
                    },
                }
            }
        }
    }

    fn output(&self, bin_number: BinNumber) -> Option<Value> {
        self.outputs.get(&bin_number).cloned()
    }

    fn bot_for(&self, a: Value, b: Value) -> Option<BotId> {
        self.bots.iter()
            .filter(|&(_, bot)| bot.inputs_are(a, b))
            .map(|(&id, _)| id)
            .next()
    }
}

impl FromIterator<Instruction> for Factory {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = Instruction>
    {
        let mut f = Factory::new();
        for instruction in iter {
            f.initialize(instruction)
        }
        f.process();
        f
    }
}

fn main() {
    let input = include_str!("input.txt");

    let factory: Result<Factory, _> = input.lines().map(|l| l.trim().parse()).collect();
    let factory = factory.expect("Unable to parse factory");

    let bot = factory.bot_for(Value(61), Value(17));
    println!("{:?}", bot);
}

#[test]
fn example_1() {
    let input = "value 5 goes to bot 2
                 bot 2 gives low to bot 1 and high to bot 0
                 value 3 goes to bot 1
                 bot 1 gives low to output 1 and high to bot 0
                 bot 0 gives low to output 2 and high to output 0
                 value 2 goes to bot 2";

    let factory: Result<Factory, _> = input.lines().map(|l| l.trim().parse()).collect();
    let factory = factory.expect("Unable to parse factory");

    assert_eq!(factory.output(BinNumber(0)), Some(Value(5)));
    assert_eq!(factory.output(BinNumber(1)), Some(Value(2)));
    assert_eq!(factory.output(BinNumber(2)), Some(Value(3)));

    assert_eq!(factory.bot_for(Value(2), Value(5)), Some(BotId(2)));
}
