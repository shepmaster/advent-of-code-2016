#![feature(conservative_impl_trait)]

extern crate itertools;

use std::collections::{VecDeque, BTreeSet};
use itertools::{Itertools, Either};

type Element = &'static str;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Component {
    Microchip(Element),
    Generator(Element),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Floor {
    chips: BTreeSet<Element>,
    generators: BTreeSet<Element>,
}

impl Floor {
    fn new(components: Vec<Component>) -> Self {
        use Component::*;

        let (chips, generators) = components.into_iter().partition_map(|c| {
            match c {
                Microchip(element) => Either::Left(element),
                Generator(element) => Either::Right(element),
            }
        });

        Floor {
            chips: chips,
            generators: generators,
        }
    }

    fn is_empty(&self) -> bool {
        self.chips.is_empty() && self.generators.is_empty()
    }

    fn fried(&self) -> bool {
        if self.generators.is_empty() { return false }
        self.chips.difference(&self.generators).next().is_some()
    }

    fn components<'a>(&'a self) -> impl Iterator<Item = Component> + 'a{
        let chips = self.chips.iter().map(|&e| Component::Microchip(e));
        let generators = self.generators.iter().map(|&e| Component::Generator(e));

        chips.chain(generators)
    }

    fn components_to_move(&self) -> Vec<Vec<Component>> {
        self.components().combinations(1)
            .chain(self.components().combinations(2))
            .collect()
    }

    fn remove(&mut self, components: &[Component]) {
        use Component::*;

        for &c in components {
            match c {
                Microchip(element) => self.chips.remove(element),
                Generator(element) => self.generators.remove(element),
            };
        }
    }

    fn add(&mut self, components: &[Component]) {
        use Component::*;

        for &c in components {
            match c {
                Microchip(element) => self.chips.insert(element),
                Generator(element) => self.generators.insert(element),
            };
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FloorState {
    floors: Vec<Floor>,
    elevator: usize,
}

impl FloorState {
    fn complete(&self) -> bool {
        self.floors[0..3].iter().all(Floor::is_empty)
    }

    fn fried(&self) -> bool {
        self.floors.iter().any(Floor::fried)
    }

    fn components_to_move(&self) -> Vec<Vec<Component>> {
        self.floors[self.elevator].components_to_move()
    }

    fn potential_floors(&self) -> &'static [usize] {
        static FLOOR_0: &'static [usize] = &[1];
        static FLOOR_1: &'static [usize] = &[0, 2];
        static FLOOR_2: &'static [usize] = &[1, 3];
        static FLOOR_3: &'static [usize] = &[2];

        match self.elevator {
            0 => FLOOR_0,
            1 => FLOOR_1,
            2 => FLOOR_2,
            3 => FLOOR_3,
            _ => unreachable!(),
        }
    }

    fn move_components_to_floor(&self, components: &[Component], to: usize) -> FloorState {
        assert!(to != self.elevator);

        let mut new_floors = self.floors.clone();

        new_floors[self.elevator].remove(components);
        new_floors[to].add(components);

        FloorState {
            floors: new_floors,
            elevator: to,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    floor_state: FloorState,
    prev: Vec<FloorState>,
}

impl State {
    fn new(floors: Vec<Floor>) -> Self {
        State {
            floor_state: FloorState {
                floors: floors,
                elevator: 0,
            },
            prev: Vec::new(),
        }
    }

    fn complete(&self) -> bool {
        self.floor_state.complete()
    }

    fn components_to_move(&self) -> Vec<Vec<Component>> {
        self.floor_state.components_to_move()
    }

    fn potential_floors(&self) -> &'static [usize] {
        self.floor_state.potential_floors()
    }

    fn depth(&self) -> usize {
        self.prev.len()
    }
}

fn run_problem(initial_state: State) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut seen = BTreeSet::new();

    let mut cnt = 0;
    let mut already_seen_cnt = 0;

    seen.insert(initial_state.floor_state.clone());
    queue.push_back(initial_state);

    println!("-----");

    while let Some(state) = queue.pop_front() {
        cnt += 1;

        if cnt % 250 == 0 {
            println!("{}: queue is {}, processed {}", state.depth(), queue.len(), cnt);
        }

        let potential_components = state.components_to_move();

        for &potential_floor in state.potential_floors() {
            for components in &potential_components {
                let next_floor_state = state.floor_state.move_components_to_floor(components, potential_floor);

                if next_floor_state.fried() {
                    continue;
                }

                if seen.contains(&next_floor_state) {
                    already_seen_cnt += 1
                } else {
                    let mut next_prev = state.prev.clone();
                    next_prev.push(state.floor_state.clone());

                    let next_state = State {
                        floor_state: next_floor_state.clone(),
                        prev: next_prev,
                    };

                    if next_state.complete() {
                        println!(
                            "Processed {} states (skipped {} duplicates)",
                            cnt, already_seen_cnt
                        );
                        println!("{:?}", next_state);
                        return Some(next_state.depth());
                    }

                    seen.insert(next_floor_state);
                    queue.push_back(next_state);
                }
            }
        }
    }

    None
}

fn main() {
    use Component::*;

    let floors = vec![
        Floor::new(vec![Generator("promethium"), Microchip("promethium")]),
        Floor::new(vec![Generator("cobalt"), Generator("curium"), Generator("ruthenium"), Generator("plutonium")]),
        Floor::new(vec![Microchip("cobalt"), Microchip("curium"), Microchip("ruthenium"), Microchip("plutonium")]),
        Floor::new(vec![]),
    ];

    let state = State::new(floors);

    let steps = run_problem(state);

    println!("{:?}", steps);
}

#[test]
fn example_1() {
    use Component::*;

    let floors = vec![
        Floor::new(vec![Microchip("hydrogen"), Microchip("lithium")]),
        Floor::new(vec![Generator("hydrogen")]),
        Floor::new(vec![Generator("lithium")]),
        Floor::new(vec![]),
    ];

    let state = State::new(floors);

    let steps = run_problem(state);

    assert_eq!(steps, Some(11));
}
