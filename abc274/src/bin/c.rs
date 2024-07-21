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

    let mut result = vec![0_usize; 2 * N + 1];

    for (i, n) in A.iter().enumerate() {
        result[2 * i + 1] = result[n - 1] + 1;
        result[2 * i + 2] = result[n - 1] + 1;
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
                r#"2
1 2
"#
            ),
            r#"0
1
1
2
2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
1 3 5 2
"#
            ),
            r#"0
1
1
2
2
3
3
2
2"#
        );
    }
}
