fn main() {
    println!("Hello, ");
    template::my_fn();
}

#[cfg(test)]
mod tests {
    use core::assert;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test() {
        assert!(true);
    }
}
