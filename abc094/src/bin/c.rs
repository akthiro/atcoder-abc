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
        X: [usize; N],
    }

    let mut x = X.clone();
    x.sort();

    let big_mid = x[N / 2];
    let small_mid = x[N / 2 - 1];

    let mut result = vec![];

    for n in X {
        if n < big_mid {
            result.push(big_mid);
        } else {
            result.push(small_mid);
        }
    }

    result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
2 4 4 3
"#
            ),
            r#"4
3
3
4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2
1 2
"#
            ),
            r#"2
1"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6
5 5 4 4 3 3
"#
            ),
            r#"4
4
4
4
4
4"#
        );
    }
}
