use std::collections::BTreeMap;

fn decode_message(input: &str) -> String {
    let mut by_position = BTreeMap::new();

    for line in input.lines() {
        for (i, chr) in line.trim().chars().enumerate() {
            let by_letter = by_position.entry(i).or_insert_with(BTreeMap::new);
            *by_letter.entry(chr).or_insert(0) += 1;
        }
    }

    by_position.into_iter()
        .flat_map(|(_, by_letter)| {
            by_letter.into_iter()
                .max_by_key(|&(_, cnt)| cnt)
                .map(|(chr, _)| chr)
        })
        .collect()
}

fn main() {
    println!("The message is {}", decode_message(include_str!("input.txt")));
}

#[test]
fn example_1() {
    assert_eq!(decode_message(include_str!("test-input.txt")), "easter");
}
