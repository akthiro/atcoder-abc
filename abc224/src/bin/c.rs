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

    let mut result = 0;

    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                let x1 = XY[i].0;
                let x2 = XY[j].0;
                let x3 = XY[k].0;
                let y1 = XY[i].1;
                let y2 = XY[j].1;
                let y3 = XY[k].1;

                if (x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3) != 0 {
                    result += 1;
                }
            }
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
                r#"4
0 1
1 3
1 1
-1 -1
"#
            ),
            r#"3"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"20
224 433
987654321 987654321
2 0
6 4
314159265 358979323
0 0
-123456789 123456789
-1000000000 1000000000
124 233
9 -6
-4 0
9 5
-7 3
333333333 -333333333
-9 -1
7 -10
-1 5
324 633
1000000000 -1000000000
20 0
"#
            ),
            r#"1124"#
        );
    }
}
