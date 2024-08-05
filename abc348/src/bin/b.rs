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
        XY: [(isize, isize); N],
    }

    let mut result = vec![];

    for (i, (x1, y1)) in XY.iter().enumerate() {
        let mut max_val = 0.0;
        let mut max_idx = 0;

        for (j, (x2, y2)) in XY.iter().enumerate() {
            if i == j {
                continue;
            }

            let tmp = (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();

            if max_val < tmp {
                max_val = tmp;
                max_idx = j;
            }
        }

        result.push(max_idx);
    }

    result
        .iter()
        .map(|n| (n + 1).to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"4
0 0
2 4
5 0
3 4
"#
            ),
            r#"3
3
1
1"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"6
3 2
1 6
4 5
1 3
5 5
9 8
"#
            ),
            r#"6
6
6
6
6
4"#
        );
    }
}
