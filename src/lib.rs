/// @notice This is a doc with a doc test
/// @example
/// ```
/// assert!(true);
/// ```
pub fn my_fn() {
    println!("world!");
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
