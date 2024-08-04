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
        T: usize,
        P: usize,
        L: [usize; N],
    }

    let mut l = L;
    l.sort_by(|a, b| b.cmp(a));

    if T <= l[P - 1] {
        0.to_string()
    } else {
        (T - l[P - 1]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5 10 3
3 11 1 6 2
"#
            ),
            r#"7"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 5 2
10 10
"#
            ),
            r#"0"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"3 10 1
1 2 3
"#
            ),
            r#"7"#
        );
    }
}
