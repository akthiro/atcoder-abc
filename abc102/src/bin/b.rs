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
        A: [usize; N],
    }

    let mut a = A;
    a.sort();

    let result = a[a.len() - 1] - a[0];

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
1 4 6 3
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
1000000000 1
"#
            ),
            r#"999999999"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5
1 1 1 1 1
"#
            ),
            r#"0"#
        );
    }
}
