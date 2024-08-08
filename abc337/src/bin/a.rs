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
        XY: [(usize, usize); N],
    }

    let mut takahashi = 0;
    let mut aoki = 0;

    for (x, y) in XY {
        takahashi += x;
        aoki += y;
    }

    #[allow(clippy::comparison_chain)]
    if takahashi > aoki {
        "Takahashi".to_string()
    } else if takahashi < aoki {
        "Aoki".to_string()
    } else {
        "Draw".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
10 2
10 1
10 2
3 2
"#
            ),
            r#"Takahashi"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"6
5 4
4 5
2 4
1 6
7 1
3 2
"#
            ),
            r#"Draw"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"4
0 0
10 10
50 50
0 100
"#
            ),
            r#"Aoki"#
        );
    }
}
