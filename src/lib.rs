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
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        assert!(true);
    }
}