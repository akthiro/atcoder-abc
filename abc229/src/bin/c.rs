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
        W: usize,
        AB: [(isize, isize); N],
    }

    let mut ab = AB;
    ab.sort_by(|a, b| b.0.cmp(&a.0));

    let mut result = 0;
    let mut w = W as isize;

    for (a, b) in ab {
        if w <= 0 {
            break;
        }

        if b <= w {
            result += a * b;
            w -= b;
        } else {
            result += a * w;
            w -= w;
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
                r#"3 5
3 1
4 2
2 3
"#
            ),
            r#"15"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4 100
6 2
1 5
3 9
8 7
"#
            ),
            r#"100"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 3141
314944731 649
140276783 228
578012421 809
878510647 519
925326537 943
337666726 611
879137070 306
87808915 39
756059990 244
228622672 291
"#
            ),
            r#"2357689932073"#
        );
    }
}
