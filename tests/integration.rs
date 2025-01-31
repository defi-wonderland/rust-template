use core::assert;

#[test]
#[allow(clippy::assertions_on_constants)]
fn test_empty_input() {
    template::my_fn();

    assert!(true);
}
