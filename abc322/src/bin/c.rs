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
        M: usize,
        A: [usize; M],
    }

    let mut l = vec![false; N];

    for a in A {
        l[a - 1] = true;
    }

    let mut result = vec![0; N];
    result[N - 1] = 0;

    if N >= 2 {
        for i in (0..=N - 2).rev() {
            if l[i] {
                result[i] = 0;
            } else {
                result[i] = result[i + 1] + 1;
            }
        }
    }

    result
        .iter()
        .map(ToString::to_string)
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
                r#"3 2
2 3
"#
            ),
            r#"1
0
0"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"8 5
1 3 4 7 8
"#
            ),
            r#"0
1
0
0
2
1
0
0"#
        );
    }
}
