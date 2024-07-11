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
        X: usize,
    }

    let sum = A.iter().sum::<usize>();

    let mut result = N * (X / sum);
    let mut total: usize = sum * (X / sum);

    for n in A {
        if total > X {
            break;
        }

        result += 1;
        total += n
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
                r#"3
3 5 2
26
"#
            ),
            r#"8"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
12 34 56 78
1000
"#
            ),
            r#"23"#
        );
    }
}
