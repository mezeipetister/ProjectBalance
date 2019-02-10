/// Main function
///

fn main() {
    println!("HelloWorld!");
    for item in 1..100 {
        let _a: i32 = item;
        println!("{}", _a);
    }
    println!("HelloBello");
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
