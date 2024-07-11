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
        v: [usize; N],
    }

    let mut v = v;
    v.sort();

    let mut result = (v[0] + v[1]) as f64 / 2.0;

    for n in v.iter().skip(2) {
        result = (result + *n as f64) / 2.0;
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
                r#"2
3 4
"#
            ),
            r#"3.5"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
500 300 200
"#
            ),
            r#"375"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"5
138 138 138 138 138
"#
            ),
            r#"138"#
        );
    }
}
