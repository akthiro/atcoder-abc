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
        H: [usize; N],
    }

    let mut result = 0;
    let mut m = M;

    for h in H {
        if m < h {
            break;
        }

        result += 1;
        m -= h;
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
                r#"5 10
2 3 2 5 3
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5 10
2 3 2 3 5
"#
            ),
            r#"4"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"1 5
1
"#
            ),
            r#"1"#
        );
    }
}
