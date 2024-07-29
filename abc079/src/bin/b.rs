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
    }

    let mut l = vec![0_usize; N + 1];
    l[0] = 2;
    l[1] = 1;

    for i in 2..N + 1 {
        l[i] = l[i - 1] + l[i - 2];
    }

    l.last().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"5
"#
            ),
            r#"11"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"86
"#
            ),
            r#"939587134549734843"#
        );
    }
}
