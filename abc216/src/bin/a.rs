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
        XY: String,
    }

    let xy = XY.split('.').collect::<Vec<_>>();
    let x = xy[0].parse::<usize>().unwrap();
    let y = xy[1].parse::<usize>().unwrap();

    if (0..=2).contains(&y) {
        format!("{}-", x)
    } else if (3..=6).contains(&y) {
        format!("{}", x)
    } else if (7..=9).contains(&y) {
        format!("{}+", x)
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"15.8
"#
            ),
            r#"15+"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1.0
"#
            ),
            r#"1-"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"12.5
"#
            ),
            r#"12"#
        );
    }
}
