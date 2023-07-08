const MULTIPLE_OF_THREE_DIVIDENT: i64 = 3;
const MULTIPLE_OF_FIVE_DIVIDENT: i64 = 5;
const ZERO_REMINDER: i64 = 0;

/// Returns a string based on the FizzBuzz problem for the given number.
///
/// # Arguments
///
/// * `number` - The input number.
///
/// # Returns
///
/// A string based on the following conditions:
/// - If the `number` is divisible by both 3 and 5, it returns "FizzBuzz".
/// - If the `number` is divisible by 3, it returns "Fizz".
/// - If the `number` is divisible by 5, it returns "Buzz".
/// - Otherwise, it returns the string representation of the input number.
///
/// # Examples
///
/// ```
/// let result = fizz_buzz(15);
/// assert_eq!(result, "FizzBuzz");
///
/// let result = fizz_buzz(9);
/// assert_eq!(result, "Fizz");
///
/// let result = fizz_buzz(10);
/// assert_eq!(result, "Buzz");
///
/// let result = fizz_buzz(7);
/// assert_eq!(result, "7");
/// ```
fn fizz_buzz(number: i64) -> String {
    let multiple_of_three: bool = number % MULTIPLE_OF_THREE_DIVIDENT == ZERO_REMINDER;
    let multiple_of_five: bool = number % MULTIPLE_OF_FIVE_DIVIDENT == ZERO_REMINDER;

    if multiple_of_three && multiple_of_five {
        return "FizzBuzz".to_string();
    }

    if multiple_of_three {
        return "Fizz".to_string();
    }

    if multiple_of_five {
        return "Buzz".to_string();
    }

    return number.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_string() {
        assert_eq!(fizz_buzz(16), "16");
    }

    #[test]
    fn test_get_string_multiple_of_three() {
        assert_eq!(fizz_buzz(12), "Fizz");
        assert_eq!(fizz_buzz(63), "Fizz");
        assert_eq!(fizz_buzz(24), "Fizz");
    }

    #[test]
    fn test_get_string_multiple_of_five() {
        assert_eq!(fizz_buzz(100), "Buzz");
        assert_eq!(fizz_buzz(250), "Buzz");
        assert_eq!(fizz_buzz(125), "Buzz");
    }

    #[test]
    fn test_get_string_multiple_of_three_and_five() {
        assert_eq!(fizz_buzz(105), "FizzBuzz");
        assert_eq!(fizz_buzz(90), "FizzBuzz");
        assert_eq!(fizz_buzz(60), "FizzBuzz");
    }
}
