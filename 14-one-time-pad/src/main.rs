extern crate md5;
extern crate hex;

use hex::ToHex;

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::iter;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct RandomStream<'a> {
    salt: &'a str,
    stretches: usize,
    idx: usize,
    cache: Rc<RefCell<BTreeMap<usize, String>>>,
}

impl<'a> RandomStream<'a> {
    fn new(salt: &'a str, stretches: usize) -> Self {
        RandomStream {
            salt: salt,
            stretches: stretches,
            idx: 0,
            cache: Default::default(),
        }
    }
}

impl<'a> Iterator for RandomStream<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut RandomStream { salt, idx, stretches, .. } = self;

        let hash = self.cache.borrow_mut().entry(idx).or_insert_with(|| {
            let original_hash = md5hex(&format!("{}{}", salt, idx));
            stretch_hash(original_hash, stretches)
        }).clone();

        self.idx += 1;

        Some(hash)
    }
}

fn md5hex(s: &str) -> String {
    md5::compute(s.as_bytes()).to_hex()
}

fn stretch_hash<'a, S>(s: S, stretches: usize) -> String
    where S: Into<Cow<'a, str>>
{
    let mut s = s.into();
    for _ in 0..stretches {
        s = md5hex(&s).into();
    }
    s.into_owned()
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
        while let Some(hash) = self.iter.by_ref().next() {
            if let Some(c) = contains_three(hash.as_ref()) {
                let mut next_thousand = self.iter.clone().take(1000);
                if next_thousand.any(|hash| contains_five(hash.as_ref(), c)) {
                    return Some(c)
                }
            }
        }
        None
    }
}

fn puzzle(salt: &str, stretches: usize) -> (String, usize) {
    let rs = RandomStream::new(salt, stretches);
    let mut ks = KeyStream::new(rs);
    let key: String = ks.by_ref().take(64).collect();
    let rs = ks.into_inner();
    (key, rs.idx - 1) // We've already incremented for the next iteration
}

fn main() {
    println!("{:?}", puzzle("cuanljph", 0));
    println!("{:?}", puzzle("cuanljph", 2016));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_data() {
        let mut rs = RandomStream::new("abc", 0);
        let data = rs.nth(18).unwrap();
        assert!(data.contains("cc38887a5"));
    }

    #[test]
    fn first_key() {
        let rs = RandomStream::new("abc", 0);
        let mut ks = KeyStream::new(rs);
        assert_eq!(ks.nth(0), Some('e'));
    }

    #[test]
    fn second_key() {
        let rs = RandomStream::new("abc", 0);
        let mut ks = KeyStream::new(rs);
        assert_eq!(ks.nth(1), Some('9'));
    }

    #[test]
    fn index_for_last_key() {
        let (_, idx) = puzzle("abc", 0);
        assert_eq!(idx, 22728);
    }

    #[test]
    fn stretch_hash_works() {
        assert_eq!(
            stretch_hash("577571be4de9dcce85a041ba0410f29f", 2016),
            "a107ff634856bb300138cac6568c0f24"
        );
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
