#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        N: usize,
    }

    let mut l = vec![];

    for n in 1..=N {
        if n == 1 {
            l.push(vec![1]);
        } else {
            let prev = l.last().unwrap().clone();
            l.push(
                [prev.clone(), vec![n], prev]
                    .iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>(),
            );
        }
    }

    l.last()
        .unwrap()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2
"#
            ),
            r#"1 2 1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4
"#
            ),
            r#"1 2 1 3 1 2 1 4 1 2 1 3 1 2 1"#
        );
    }
}
