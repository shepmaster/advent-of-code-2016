#![feature(conservative_impl_trait)]

extern crate md5;
extern crate hex;

use std::collections::BTreeMap;

use hex::ToHex;

fn md5hex(s: &str) -> String {
    md5::compute(s.as_bytes()).to_hex()
}

fn code_stream<'a>(door_id: &'a str) -> impl Iterator<Item = String> + 'a {
    (0..)
        .map(move |i| format!("{}{}", door_id, i))
        .map(|input| md5hex(&input))
        .filter(|hex| hex.starts_with("00000"))
}

const PASSWORD_LENGTH: u8 = 8;

fn password(door_id: &str) -> String {
    code_stream(door_id)
        .flat_map(|hex| hex.chars().nth(5))
        .take(PASSWORD_LENGTH as usize)
        .collect()
}

fn password2(door_id: &str) -> Option<String> {
    fn sixth_and_seventh_characters(hex: String) -> Option<(char, char)> {
        let mut c = hex.chars().skip(5).fuse();
        match (c.next(), c.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None
        }
    }

    let valid_position_char_pairs = code_stream(door_id)
        .flat_map(sixth_and_seventh_characters)
        .flat_map(|(pos, chr)| pos.to_digit(10).map(|pos| (pos, chr)))
        .filter(|&(pos, _)| pos < PASSWORD_LENGTH as u32);

    let mut code = BTreeMap::new();
    for (pos, chr) in valid_position_char_pairs {
        code.entry(pos).or_insert(chr);
        if code.len() == PASSWORD_LENGTH as usize {
            // We know that BTreeMap will iterate in order
            return Some(code.into_iter().map(|(_, chr)| chr).collect())
        }
    }
    None
}

fn main() {
    println!("password: {}", password("cxdnnyjw"));
    println!("password2: {:?}", password2("cxdnnyjw"));
}

#[test]
fn example_1() {
    assert!(md5hex("abc3231929").starts_with("000001"));
    assert!(md5hex("abc5017308").starts_with("000008f82"));
    assert!(md5hex("abc5278568").starts_with("00000f"));

    assert_eq!(password("abc"), "18f47a30");
}

#[test]
fn example_2() {
    assert_eq!(password2("abc").as_ref().map(String::as_str), Some("05ace8e3"));
}
