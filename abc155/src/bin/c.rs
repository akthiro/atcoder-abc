#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashMap,
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
        N: usize,
        S: [String; N],
    }

    let mut map = HashMap::new();

    for s in S {
        map.entry(s).and_modify(|n| *n += 1).or_insert(1);
    }

    let result = map.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>();
    let max = result.iter().map(|(_, v)| *v).max().unwrap();

    let mut result = result
        .iter()
        .filter(|(_, v)| *v >= max)
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();
    result.sort();

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7
beat
vet
beet
bed
vet
bet
beet
"#
            ),
            r#"beet
vet"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"8
buffalo
buffalo
buffalo
buffalo
buffalo
buffalo
buffalo
buffalo
"#
            ),
            r#"buffalo"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"7
bass
bass
kick
kick
bass
kick
kick
"#
            ),
            r#"kick"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"4
ushi
tapu
nichia
kun
"#
            ),
            r#"kun
nichia
tapu
ushi"#
        );
    }
}
