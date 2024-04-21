//! # My Crate
//!
//! `ch14_cargo` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds both numbers together.
///
/// # Examples
///
/// ```
/// let answer = ch14_cargo::add(1, 5);
///
/// assert_eq!(answer, 6)
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Adds one the the number provided.
///
/// # Examples
///
/// ```
/// let answer = ch14_cargo::add_one(2);
///
/// assert_eq!(answer, 3)
/// ```
pub fn add_one(num: usize) -> usize {
    add(num, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_one_works() {
        let result = add_one(3);
        assert_eq!(result, 4);
    }
}
