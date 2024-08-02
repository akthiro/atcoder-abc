#![allow(non_snake_case)]

use proconio::{input, source::once::OnceSource};
use std::{
    collections::HashMap,
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
        S: String,
    }

    let keyboard = "WBWBWWBWBWBW".repeat(7);

    let result = keyboard.find(&S).unwrap();

    let mut map = HashMap::new();
    map.insert(0, "Do");
    map.insert(2, "Re");
    map.insert(4, "Mi");
    map.insert(5, "Fa");
    map.insert(7, "So");
    map.insert(9, "La");
    map.insert(11, "Si");

    map[&result].to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test1() {
        assert_eq!(
            solve(
                r#"WBWBWWBWBWBWWBWBWWBW
"#
            ),
            r#"Do"#
        );
    }
}
