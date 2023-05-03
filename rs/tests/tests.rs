#[cfg(test)]
use super::*;

#[test]
fn test_addition() { assert_eq!(addition(10,111), 121); }

#[test]
fn test_div() {
    // Test case 1: Valid inputs
    assert_eq!(div(10, 2), Ok(5));

    // Test case 2: Dividing by zero (denominator is zero)
    assert_eq!(div(5, 0), Err("Error: Numerator is zero!"));

    // Test case 3: Numerator is zero
    assert_eq!(div(0, 5), Err("Error: Can not divide by zero!"));

    // Test case 4: Both numerator and denominator are zero
    assert_eq!(div(0, 0), Err("Error: Can not divide by zero!"));
}
