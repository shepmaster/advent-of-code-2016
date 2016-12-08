fn has_abba(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    chars.windows(4).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn supports_tls(input: &str) -> bool {
    // Assumes brackets are always in the correct pairing and never nested
    let delimiters = &['[', ']'][..];
    let parts = input.split(delimiters);
    let outside_hypernet = vec![true, false].into_iter().cycle();

    let mut has_one_abba = false;

    for (part, outside_hypernet) in parts.zip(outside_hypernet) {
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

fn main() {
    let input = include_str!("input.txt");
    let count = input.lines().filter(|s| supports_tls(s)).count();
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
