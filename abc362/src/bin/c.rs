#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    cmp,
    io::{self, Read},
};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    input! {
        from OnceSource::from(src),
        N: usize,
        LR: [(isize, isize); N],
    }

    let min = LR.iter().map(|(min, _)| *min).sum::<isize>();
    let max = LR.iter().map(|(_, max)| *max).sum::<isize>();

    if min > 0 || max < 0 {
        return "No".to_string();
    }

    let mut result = LR.iter().map(|(min, _)| *min).collect::<Vec<_>>();
    let mut total = result.iter().sum::<isize>();

    for i in 0..N {
        let add = cmp::min(0 - total, LR[i].1 - LR[i].0);
        result[i] += add;
        total += add;
    }

    format!(
        "Yes\n{}",
        result
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
3 5
-4 1
-2 3
"#
            ),
            r#"Yes
5 -3 -2"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"3
1 2
1 2
1 2
"#
            ),
            r#"No"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6
-87 12
-60 -54
2 38
-76 6
87 96
-17 38
"#
            ),
            r#"Yes
12 -54 38 -66 87 -17"#
        );
    }
}
