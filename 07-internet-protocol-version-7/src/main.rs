#![feature(conservative_impl_trait)]

use std::collections::HashSet;

fn has_abba(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    chars.windows(4).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

static DELIMITERS: &'static [char] = &['[', ']'];

// Assumes brackets are always in the correct pairing and never nested
// Should we create an enum instead of a boolean?
fn ip_chunks<'a>(input: &'a str) -> impl Iterator<Item = (&'a str, bool)> + 'a {
    let parts = input.split(DELIMITERS);
    let outside_hypernet = vec![true, false].into_iter().cycle();

    parts.zip(outside_hypernet)
}

fn supports_tls(input: &str) -> bool {
    let mut has_one_abba = false;

    for (part, outside_hypernet) in ip_chunks(input) {
        if has_abba(part) {
            if outside_hypernet {
                has_one_abba = true;
            } else {
                return false;
            }
        }
    }

    has_one_abba
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct AbaBabPair {
    a: char,
    b: char,
}

impl AbaBabPair {
    fn from_aba(aba: &[char]) -> AbaBabPair {
        AbaBabPair { a: aba[0], b: aba[1] }
    }

    fn from_bab(bab: &[char]) -> AbaBabPair {
        AbaBabPair { a: bab[1], b: bab[0] }
    }
}

fn find_aba_bab_pairs<F>(input: &str, collection: &mut HashSet<AbaBabPair>, f: F)
    where F: Fn(&[char]) -> AbaBabPair
{
    let chars: Vec<_> = input.chars().collect();
    let newly_found = chars.windows(3)
        .filter(|w| w[0] == w[2] && w[0] != w[1])
        .map(f);
    collection.extend(newly_found)
}

fn supports_ssl(input: &str) -> bool {
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();

    for (part, outside_hypernet) in ip_chunks(input) {
        if outside_hypernet {
            find_aba_bab_pairs(part, &mut abas, AbaBabPair::from_aba)
        } else {
            find_aba_bab_pairs(part, &mut babs, AbaBabPair::from_bab)
        }
    }

    !abas.is_disjoint(&babs)
}

fn main() {
    let input = include_str!("input.txt");
    let count = input.lines().filter(|s| supports_tls(s)).count();
    println!("There were {}", count);
    let count = input.lines().filter(|s| supports_ssl(s)).count();
    println!("There were {}", count);
}

#[test]
fn example_1() {
    assert!(supports_tls("abba[mnop]qrst"));
}

#[test]
fn example_2() {
    assert!(!supports_tls("abcd[bddb]xyyx"));
}

#[test]
fn example_3() {
    assert!(!supports_tls("aaaa[qwer]tyui"));
}

#[test]
fn example_4() {
    assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn example_5() {
    assert!(supports_ssl("aba[bab]xyz"));
}

#[test]
fn example_6() {
    assert!(!supports_ssl("xyx[xyx]xyx"));
}

#[test]
fn example_7() {
    assert!(supports_ssl("aaa[kek]eke"));
}

#[test]
fn example_8() {
    assert!(supports_ssl("zazbz[bzb]cdb"));
}
