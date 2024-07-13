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
        XA: isize,
        YA: isize,
        XB: isize,
        YB: isize,
        XC: isize,
        YC: isize,
    }

    let a = (YA - YB).abs().pow(2) + (XA - XB).abs().pow(2);
    let b = (YB - YC).abs().pow(2) + (XB - XC).abs().pow(2);
    let c = (YC - YA).abs().pow(2) + (XC - XA).abs().pow(2);

    let mut l = [a, b, c];
    l.sort();

    if l[0] + l[1] == l[2] {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"0 0
4 0
0 3
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"-4 3
2 1
3 4
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2 4
-3 2
1 -2
"#
            ),
            r#"No"#
        );
    }
}
