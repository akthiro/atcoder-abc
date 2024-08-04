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
        AC: [(usize, usize); N],
    }

    let mut ac = AC.iter().enumerate().collect::<Vec<_>>();
    ac.sort_by(|a, b| a.1 .1.cmp(&b.1 .1));

    let mut result = vec![];
    let mut tmp = 0;

    for (i, (a, _)) in ac {
        if *a > tmp {
            tmp = *a;
            result.push(i + 1);
        }
    }

    result.sort();

    format!(
        "{}\n{}",
        result.len(),
        result
            .iter()
            .map(ToString::to_string)
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
2 4
1 1
3 2
"#
            ),
            r#"2
2 3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"5
1 1
10 2
100 3
1000 4
10000 5
"#
            ),
            r#"5
1 2 3 4 5"#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"6
32 101
65 78
2 29
46 55
103 130
52 40
"#
            ),
            r#"4
2 3 5 6"#
        );
    }
}
