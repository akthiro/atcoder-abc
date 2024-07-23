#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp::Ordering,
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
        a: [usize; N],
    }

    let mut map = HashMap::new();

    for n in a {
        map.entry(n).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut result = 0;

    for (k, v) in map {
        match k.cmp(&v) {
            Ordering::Greater => {
                result += v;
            }
            Ordering::Less => {
                result += v - k;
            }
            _ => {}
        }
    }

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
3 3 3 3
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
2 4 1 4 2
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6
1 2 2 3 3 3
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"1
1000000000
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            solve(
                r#"8
2 7 1 8 2 8 1 8
"#
            ),
            r#"5"#
        );
    }
}
