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
        K: usize,
    }

    let mut result = N;

    for _ in 0..K {
        let mut g1 = result.to_string().chars().collect::<Vec<_>>();
        g1.sort_by(|a, b| b.cmp(a));
        let g1 = g1.iter().collect::<String>().parse::<usize>().unwrap();

        let mut g2 = result.to_string().chars().collect::<Vec<_>>();
        g2.sort();
        let g2 = g2.iter().collect::<String>().parse::<usize>().unwrap();

        result = g1 - g2;
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
                r#"314 2
"#
            ),
            r#"693"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1000000000 100
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6174 100000
"#
            ),
            r#"6174"#
        );
    }
}
