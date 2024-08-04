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
        SC: [(String, usize); N],
    }

    let total = SC.iter().map(|(_, n)| *n).sum::<usize>();

    let mut s = SC.iter().map(|(s, _)| s.clone()).collect::<Vec<_>>();
    s.sort();

    s[total % N].to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
takahashi 2
aoki 6
snuke 5
"#
            ),
            r#"snuke"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
takahashi 2813
takahashixx 1086
takahashix 4229
"#
            ),
            r#"takahashix"#
        );
    }
}
