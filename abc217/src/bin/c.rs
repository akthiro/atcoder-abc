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
        p: [usize; N],
    }

    let mut result = vec![0; N];

    for (i, n) in p.iter().enumerate() {
        result[*n - 1] = i + 1;
    }

    result
        .iter()
        .map(ToString::to_string)
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
                r#"3
2 3 1
"#
            ),
            r#"3 1 2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
1 2 3
"#
            ),
            r#"1 2 3"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5
5 3 2 4 1
"#
            ),
            r#"5 3 2 4 1"#
        );
    }
}
