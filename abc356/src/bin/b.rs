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
        X: [[usize; M]; N],
    }

    for (i, required) in A.iter().enumerate() {
        let mut total = 0;

        for row in X.iter() {
            total += row[i];
        }

        if total < *required {
            return "No".to_string();
        }
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2 3
10 20 30
20 0 10
0 100 100
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 4
10 20 30 40
20 0 10 30
0 100 100 0
"#
            ),
            r#"No"#
        );
    }
}
