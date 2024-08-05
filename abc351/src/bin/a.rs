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
        A: [usize; 9],
        B: [usize; 8],
    }

    let a = A.iter().sum::<usize>();
    let b = B.iter().sum::<usize>();

    if a < b {
        0.to_string()
    } else {
        (a - b + 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"0 1 0 1 2 2 0 0 1
1 1 0 0 0 0 1 0
"#
            ),
            r#"5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
"#
            ),
            r#"1"#
        );
    }
}
