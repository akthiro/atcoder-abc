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
        Q: usize,
        T: [usize; Q],
    }

    let mut result = vec![1; N];

    for n in T.iter().map(|n| n - 1) {
        if result[n] == 1 {
            result[n] = 0;
        } else {
            result[n] = 1;
        }
    }

    result.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"30 6
2 9 18 27 18 9
"#
            ),
            r#"28"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1 7
1 1 1 1 1 1 1
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"9 20
9 5 1 2 2 2 8 9 2 1 6 2 6 5 8 7 8 5 9 8
"#
            ),
            r#"5"#
        );
    }
}
