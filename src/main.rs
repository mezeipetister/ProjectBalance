// Copyright 2019 Peter Mezei
// https://github.com/mezeipetister
// For more details please check the readme and LICENSE files enclosed.

/// Main function
/// Hello bello
///
/// # Examples
/// ```rust
/// let a = 1;
/// let b = 2;
/// assert_eq!(a+b,3);
/// ```
// Hello Bello
use std::fs::{File, OpenOptions};
use std::path::Path;

fn main() {
    println!("HelloWorld!");
    for item in 1..100 {
        let _a: i32 = item;
        println!("{}", _a);
    }
    let a = 1 + 2;
    println!("HelloBello");
    OpenOptions::new().create(true).write(true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn demo_test() {
        let a = 1;
        let b = 2;
        assert_eq!(a + b, 3);
    }
}
