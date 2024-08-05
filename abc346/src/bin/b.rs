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
        W: usize,
        B: usize,
    }

    let cs = "wbwbwwbwbwbw".chars().collect::<Vec<_>>();

    for i in 0..cs.len() {
        let mut w = 0;
        let mut b = 0;

        for j in 0..(W + B) {
            if cs[(i + j) % cs.len()] == 'w' {
                w += 1;
            } else {
                b += 1;
            }
        }

        if w == W && b == B {
            return "Yes".to_string();
        }
    }

    "No".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3 2
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3 0
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"92 66
"#
            ),
            r#"Yes"#
        );
    }
}
