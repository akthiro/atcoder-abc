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
    }

    let mut result = vec![];

    for x in 0..=N {
        for y in 0..=N {
            for z in 0..=N {
                if x + y + z <= N {
                    result.push(format!("{} {} {}", x, y, z));
                }
            }
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"3
"#
            ),
            r#"0 0 0
0 0 1
0 0 2
0 0 3
0 1 0
0 1 1
0 1 2
0 2 0
0 2 1
0 3 0
1 0 0
1 0 1
1 0 2
1 1 0
1 1 1
1 2 0
2 0 0
2 0 1
2 1 0
3 0 0"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"4
"#
            ),
            r#"0 0 0
0 0 1
0 0 2
0 0 3
0 0 4
0 1 0
0 1 1
0 1 2
0 1 3
0 2 0
0 2 1
0 2 2
0 3 0
0 3 1
0 4 0
1 0 0
1 0 1
1 0 2
1 0 3
1 1 0
1 1 1
1 1 2
1 2 0
1 2 1
1 3 0
2 0 0
2 0 1
2 0 2
2 1 0
2 1 1
2 2 0
3 0 0
3 0 1
3 1 0
4 0 0"#
        );
    }
}
