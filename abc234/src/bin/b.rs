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
        xy: [(isize, isize); N],
    }

    let mut result = 0.0;

    for i in 0..N {
        for j in i + 1..N {
            let i_x = xy[i].0;
            let i_y = xy[i].1;
            let j_x = xy[j].0;
            let j_y = xy[j].1;
            result = f64::max(
                result,
                (((i_x - j_x).abs().pow(2) + (i_y - j_y).abs().pow(2)) as f64).sqrt(),
            )
        }
    }

    format!("{}", (result * 10000000000.0).round() / 10000000000.0)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
0 0
0 1
1 1
"#
            ),
            r#"1.4142135624"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
315 271
-2 -621
-205 -511
-952 482
165 463
"#
            ),
            r#"1455.7159750446"#
        );
    }
}
