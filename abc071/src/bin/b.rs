#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        S: String,
    }

    let alphabets: HashSet<char> = HashSet::from_iter('a'..='z');
    let chars: HashSet<char> = HashSet::from_iter(S.chars());

    let mut diff = alphabets.difference(&chars).collect::<Vec<_>>();
    diff.sort();

    if diff.is_empty() {
        "None".to_string()
    } else {
        diff[0].to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"atcoderregularcontest
"#
            ),
            r#"b"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"abcdefghijklmnopqrstuvwxyz
"#
            ),
            r#"None"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"fajsonlslfepbjtsaayxbymeskptcumtwrmkkinjxnnucagfrg
"#
            ),
            r#"d"#
        );
    }
}
