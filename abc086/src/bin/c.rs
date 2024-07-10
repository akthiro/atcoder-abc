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
        txy: [(isize, isize, isize); N],
    }

    let txy = [vec![(0, 0, 0)], txy].concat();

    for i in 0..N {
        let movable = (txy[i].0 - txy[i + 1].0).abs();
        let must = (txy[i].1 - txy[i + 1].1).abs() + (txy[i].2 - txy[i + 1].2).abs();

        if movable < must {
            return "No".to_string();
        }

        if (movable - must) % 2 != 0 {
            return "No".to_string();
        }
    }

    "Yes".to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"2
3 1 2
6 1 1
"#
            ),
            r#"Yes"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"1
2 100 100
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"2
5 1 1
100 1 1
"#
            ),
            r#"No"#
        );
    }
}
