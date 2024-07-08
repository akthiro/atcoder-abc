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
        A: [isize; N],
    }

    let mut a = A
        .iter()
        .enumerate()
        .map(|(i, n)| n - i as isize + 1)
        .collect::<Vec<_>>();
    a.sort();

    let mid = a[N / 2];

    let mut result = 0;

    for n in a {
        result += (mid - n).abs();
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
                r#"5
2 2 3 5 5
"#
            ),
            r#"2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"9
1 2 3 4 5 6 7 8 9
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6
6 5 4 3 2 1
"#
            ),
            r#"18"#
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            solve(
                r#"7
1 1 1 1 2 3 4
"#
            ),
            r#"6"#
        );
    }
}
