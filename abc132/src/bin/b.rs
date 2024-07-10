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
        n: usize,
        p: [usize; n],
    }

    let mut result = 0;

    for i in 0..n - 2 {
        if (p[i] < p[i + 1] && p[i + 1] < p[i + 2]) || (p[i + 2] < p[i + 1] && p[i + 1] < p[i]) {
            result += 1;
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
                r#"5
1 3 5 4 2
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
9 6 3 2 5 8 7 4 1
"#
            ),
            r#"5"#
        );
    }
}
