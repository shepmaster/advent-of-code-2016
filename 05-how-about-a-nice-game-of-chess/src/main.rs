extern crate md5;
extern crate hex;

use hex::ToHex;

fn md5hex(s: &str) -> String {
    md5::compute(s.as_bytes()).to_hex()
}

fn password(door_id: &str) -> String {
    (0..)
        .map(|i| format!("{}{}", door_id, i))
        .map(|input| md5hex(&input))
        .filter(|hex| hex.starts_with("00000"))
        .flat_map(|hex| hex.chars().nth(5))
        .take(8)
        .collect()
}

fn main() {
    println!("password: {}", password("cxdnnyjw"));
}

#[test]
fn example_1() {
    assert!(md5hex("abc3231929").starts_with("000001"));
    assert!(md5hex("abc5017308").starts_with("000008f82"));
    assert!(md5hex("abc5278568").starts_with("00000f"));

    assert_eq!(password("abc"), "18f47a30");
}
