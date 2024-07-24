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
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }

    let mut a = A;
    let mut c = C;

    loop {
        if c <= B {
            return "Yes".to_string();
        }

        c -= B;

        if a <= D {
            return "No".to_string();
        }

        a -= D;
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"10 9 10 10
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"46 4 40 5
"#
            ),
            r#"Yes"#
        );
    }
}
