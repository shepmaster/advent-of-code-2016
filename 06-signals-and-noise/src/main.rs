use std::collections::BTreeMap;
use std::collections::btree_map;

type CharacterFrequency = (char, usize);

fn decode_message<F>(input: &str, ordering: F) -> String
    where F: Fn(btree_map::IntoIter<char, usize>) -> Option<CharacterFrequency>,
{
    let mut by_position = BTreeMap::new();

    for line in input.lines() {
        for (i, chr) in line.trim().chars().enumerate() {
            let by_letter = by_position.entry(i).or_insert_with(BTreeMap::new);
            *by_letter.entry(chr).or_insert(0) += 1;
        }
    }

    by_position.into_iter()
        .flat_map(|(_, by_letter)| {
            ordering(by_letter.into_iter())
                .map(|(chr, _)| chr)
        })
        .collect()
}

fn decode_message_max(input: &str) -> String {
    decode_message(input, |i| Iterator::max_by_key(i, |&(_, i)| i))
}

fn decode_message_min(input: &str) -> String {
    decode_message(input, |i| Iterator::min_by_key(i, |&(_, i)| i))
}

fn main() {
    let input = include_str!("input.txt");

    println!("The message is {}", decode_message_max(input));
    println!("The message is really {}", decode_message_min(input));
}

#[cfg(test)]
mod test {
    use super::{decode_message_max, decode_message_min};

    const TEST_INPUT: &'static str = include_str!("test-input.txt");

    #[test]
    fn example_1() {
        assert_eq!(decode_message_max(TEST_INPUT), "easter");
    }

    #[test]
    fn example_2() {
        assert_eq!(decode_message_min(TEST_INPUT), "advent");
    }
}
