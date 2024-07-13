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
        H: usize,
        W: usize,
        N: usize,
    }

    let mut result = vec![vec!['.'; W]; H];

    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];

    let h = H as isize;
    let w = W as isize;

    let mut dir = 0;
    let mut y = 0;
    let mut x = 0;

    for _ in 0..N {
        if result[y][x] == '.' {
            result[y][x] = '#';
            dir += 1;
        } else {
            result[y][x] = '.';
            dir += 3;
        }

        dir %= 4;

        let mut tmp_y = y as isize;
        let mut tmp_x = x as isize;

        tmp_y += dy[dir];
        tmp_x += dx[dir];

        if tmp_y < 0 {
            tmp_y += h;
        }

        if tmp_y >= h {
            tmp_y -= h;
        }

        if tmp_x < 0 {
            tmp_x += w;
        }

        if tmp_x >= w {
            tmp_x -= w;
        }

        y = tmp_y as usize;
        x = tmp_x as usize;
    }

    result
        .iter()
        .map(|chars| chars.iter().collect::<String>())
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
                r#"3 4 5
"#
            ),
            r#".#..
##..
...."#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve(
                r#"2 2 1000
"#
            ),
            r#"..
.."#
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            solve(
                r#"10 10 10
"#
            ),
            r#"##........
##........
..........
..........
..........
..........
..........
..........
..........
#........#"#
        );
    }
}
