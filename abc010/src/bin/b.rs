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
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    for n in a {
        if n >= 9 {
            result += n - 9;
        } else if n >= 7 {
            result += n - 7;
        } else if n >= 3 {
            result += n - 3;
        } else if n >= 1 {
            result += n - 1;
        } else {
            unreachable!()
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
                r#"3
5 8 2
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"9
1 2 3 4 5 6 7 8 9
"#
            ),
            r#"8"#
        );
    }
}
