extern crate md5;
extern crate hex;

use hex::ToHex;
use std::iter;

#[derive(Debug, Clone)]
struct RandomStream<'a> {
    salt: &'a str,
    idx: usize,
}

impl<'a> RandomStream<'a> {
    fn new(salt: &'a str) -> Self {
        RandomStream {
            salt: salt,
            idx: 0,
        }
    }
}

impl<'a> Iterator for RandomStream<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let a = format!("{}{}", self.salt, self.idx);
        self.idx += 1;
        Some(md5::compute(a.as_bytes()).to_hex())
    }
}

fn contains_three(s: &str) -> Option<char> {
    let chars: Vec<_> = s.chars().collect();
    for three_char_window in chars.windows(3) {
        if three_char_window.windows(2).all(|w| w[0] == w[1]) {
            return Some(three_char_window[0])
        }
    }
    None
}

fn contains_five(s: &str, c: char) -> bool {
    let needle: String = iter::repeat(c).take(5).collect();
    s.contains(&needle)
}

struct KeyStream<I> {
    iter: I,
}

impl<I> KeyStream<I> {
    fn new(iter: I) -> Self {
        KeyStream {
            iter: iter,
        }
    }

    fn into_inner(self) -> I {
        self.iter
    }
}

impl<I> Iterator for KeyStream<I>
    where I: Iterator + Clone,
          I::Item: AsRef<str>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.by_ref().next() {
            if let Some(c) = contains_three(x.as_ref()) {
                let mut xxx = self.iter.clone().take(1000);
                if xxx.any(|x2| contains_five(x2.as_ref(), c)) {
                    return Some(c)
                }
            }
        }
        None
    }
}

fn puzzle(salt: &str) -> (String, usize) {
    let rs = RandomStream::new(salt);
    let mut ks = KeyStream::new(rs);
    let key: String = ks.by_ref().take(64).collect();
    let rs = ks.into_inner();
    (key, rs.idx - 1) // We've already incremented for the next iteration
}

fn main() {
    println!("{:?}", puzzle("cuanljph"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_data() {
        let mut rs = RandomStream::new("abc");
        let data = rs.nth(18).unwrap();
        assert!(data.contains("cc38887a5"));
    }

    #[test]
    fn first_key() {
        let rs = RandomStream::new("abc");
        let mut ks = KeyStream::new(rs);
        assert_eq!(ks.nth(0), Some('e'));
    }

    #[test]
    fn second_key() {
        let rs = RandomStream::new("abc");
        let mut ks = KeyStream::new(rs);
        assert_eq!(ks.nth(1), Some('9'));
    }

    #[test]
    fn index_for_last_key() {
        let (_, idx) = puzzle("abc");
        assert_eq!(idx, 22728);
    }

    #[test]
    fn contains_three_works() {
        assert_eq!(contains_three("cc38887a5"), Some('8'))
    }

    #[test]
    fn contains_five_works() {
        assert!(contains_five("cc99999a5", '9'))
    }
}
