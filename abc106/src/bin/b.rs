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

    let mut result = 0;

    for i in 1..=N {
        if i % 2 == 0 {
            continue;
        }

        let mut cnt = 0;

        for j in 1..=N {
            if i % j == 0 {
                cnt += 1;
            }
        }

        if cnt == 8 {
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
                r#"105
"#
            ),
            r#"1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7
"#
            ),
            r#"0"#
        );
    }
}
