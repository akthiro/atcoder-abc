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
        W: [String; N],
    }

    let mut map = HashMap::new();
    map.insert(W[0].clone(), true);

    let mut prev = W[0].clone();

    for cur in W.iter().skip(1) {
        if map.contains_key(cur) {
            return "No".to_string();
        }

        let prev_chars = prev.chars().collect::<Vec<_>>();
        let cur_chars = cur.chars().collect::<Vec<_>>();

        if prev_chars[prev_chars.len() - 1] != cur_chars[0] {
            return "No".to_string();
        }

        map.insert(cur.clone(), true);
        prev.clone_from(cur);
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
hoge
english
hoge
enigma
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"9
basic
c
cpp
php
python
nadesico
ocaml
lua
assembly
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"8
a
aa
aaa
aaaa
aaaaa
aaaaaa
aaa
aaaaaaa
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"3
abc
arc
agc
"#
            ),
            r#"No"#
        );
    }
}
