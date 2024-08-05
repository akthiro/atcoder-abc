#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
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
        AC: [(usize, usize); N],
    }

    let mut map = HashMap::new();

    for (a, c) in AC {
        map.entry(c)
            .and_modify(|n| *n = cmp::min(*n, a))
            .or_insert(a);
    }

    let v = map.values().collect::<Vec<_>>();
    let result = v.iter().max().unwrap();

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
100 1
20 5
30 5
40 1
"#
            ),
            r#"40"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10
68 3
17 2
99 2
92 4
82 4
10 3
100 2
78 1
3 1
35 4
"#
            ),
            r#"35"#
        );
    }
}
