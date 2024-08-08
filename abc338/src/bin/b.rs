#![allow(non_snake_case)]

use proconio::{input, marker::Chars, source::once::OnceSource};
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
        S: Chars,
    }

    let mut map = HashMap::new();

    for c in S {
        map.entry(c).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut v = map.iter().collect::<Vec<_>>();
    v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    let result = v[0].0;

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"frequency
"#
            ),
            r#"e"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"atcoder
"#
            ),
            r#"a"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"pseudopseudohypoparathyroidism
"#
            ),
            r#"o"#
        );
    }
}
