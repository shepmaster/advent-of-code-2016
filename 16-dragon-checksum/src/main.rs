use std::borrow::Cow;

fn negate(c: char) -> char { if c == '0' { '1' } else { '0' } }

fn dragon_step(a: &str) -> String {
    // Call the data you have at this point "a".
// Make a copy of "a"; call this copy "b".
// Reverse the order of the characters in "b".
// In "b", replace all instances of 0 with 1 and all 1s with 0.
    //The resulting data is "a", then a single 0, then "b".

    let mut result = a.to_owned();
    result.push('0');
    result.extend(a.chars().rev().map(negate));
    result
}

fn dragon<'a, S>(s: S, length: usize) -> String
    where S: Into<Cow<'a, str>>,
{
    let mut s = s.into();

    while s.len() < length {
        s = dragon_step(&s).into();
    }

    let mut result = s.into_owned();
    result.truncate(length);
    result
}

fn checksum(s: &str) -> String {
    let chars: Vec<_> = s.chars().collect();

    let maybe_checksum: String = chars.chunks(2).map(|chunk| {
        if chunk[0] == chunk[1] { '1' } else { '0' }
    }).collect();

    if maybe_checksum.len() % 2 == 0 {
        checksum(&maybe_checksum)
    } else {
        maybe_checksum
    }
}

fn wipe_disk(length: usize, state: &str) -> String {
    let data = dragon(state, length);
    checksum(&data)
}

fn main() {
    println!("Wiping... {}", wipe_disk(272, "01111001100111011"));
    println!("Wiping... {}", wipe_disk(35651584, "01111001100111011"));
}

#[test]
fn test_dragon_step() {
    assert_eq!(dragon_step("1"), "100");
    assert_eq!(dragon_step("0"), "001");
    assert_eq!(dragon_step("11111"), "11111000000");
    assert_eq!(dragon_step("111100001010"), "1111000010100101011110000");
}

#[test]
fn test_dragon() {
    assert_eq!(dragon("10000", 20), "10000011110010000111");
}

#[test]
fn test_checksum() {
    assert_eq!(checksum("110010110100"), "100");
}

#[test]
fn test_wipe_disk() {
    assert_eq!(wipe_disk(20, "10000"), "01100");
}
