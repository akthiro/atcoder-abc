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
        a: [isize; N],
    }

    let mut map = HashMap::new();

    for n in a {
        map.entry(n + 1).and_modify(|n| *n += 1).or_insert(1);
        map.entry(n - 1).and_modify(|n| *n += 1).or_insert(1);
        map.entry(n).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut l = map.values().collect::<Vec<_>>();
    l.sort_by(|a, b| b.cmp(a));

    let result = l[0];

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"7
3 1 4 1 5 9 2
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"10
0 1 2 3 4 5 6 7 8 9
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1
99999
"#
            ),
            r#"1"#
        );
    }
}
