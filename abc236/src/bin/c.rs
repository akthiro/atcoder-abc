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
        S: [String; N],
        T: [String; M],
    }

    let mut i = 0;
    let mut result = vec!["No"; N];

    for (j, s) in S.iter().enumerate() {
        if *s == T[i] {
            result[j] = "Yes";
            i += 1;
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 3
tokyo kanda akiba okachi ueno
tokyo akiba ueno
"#
            ),
            r#"Yes
No
Yes
No
Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"7 7
a t c o d e r
a t c o d e r
"#
            ),
            r#"Yes
Yes
Yes
Yes
Yes
Yes
Yes"#
        );
    }
}
