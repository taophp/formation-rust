/// Evaluates mathematical and logical expressions using `eval` and `meval` crates
/// Optionnaly, plays the [24 puzzle](https://en.wikipedia.org/wiki/24_(puzzle))
///

use serde_json::Value;

#[warn(unused_variables)]

///
/// Checks if the given expression can be evaluated with meval or, otherwise, eval.
///
/// # Arguments
///
/// * `expression` - The expression to check.
///
/// # Returns
///
/// * `bool` - `true` if the expression can be evaluated, `false` otherwise.
///
/// # Examples
///
/// ```
/// use calc::check;
///
/// assert!(check("2 + 2"));
/// assert!(check("3 * 4"));
/// assert!(check("(5 + 6) * 7"));
/// assert!(check("2 == 2 && 3 != 4"));
/// assert!(check("true || false"));
/// assert!(check("sin(0) == 0"));
/// assert!(check("cos(0) == 1"));
/// assert!(!check("2 +"));
/// assert!(!check("3 *"));
/// ```
///
pub fn check(expression: &str) -> bool {
    false
}

/// Evaluates mathematical and logical expressions using `eval` and `meval` crates.
/// Optionally, plays the [24 puzzle](https://en.wikipedia.org/wiki/24_(puzzle)).
///
/// # Arguments
///
/// * `expression` - The expression to evaluate.
///
/// # Returns
///
/// * `Result<json_serde::Value, CalcError>` - The result of the evaluation, or a `CalcError` if the evaluation fails.
///
/// # Examples
///
/// ```
/// use calc::calc;
/// use serde_json::json;
///
/// assert_eq!(calc("2 + 2").unwrap(), json!(4.0));
/// assert_eq!(calc("3 * 4").unwrap(), json!(12.0));
/// assert_eq!(calc("(5 + 6) * 7").unwrap(), json!(77.0));
/// assert_eq!(calc("2 == 2 && 3 != 4").unwrap(), json!(true));
/// assert_eq!(calc("true || false").unwrap(), json!(true));
/// assert_eq!(calc("sin(0)").unwrap(), json!(0.0));
/// assert_eq!(calc("cos(0)").unwrap(), json!(1.0));
/// assert!(calc("2 +").is_err());
/// assert!(calc("3 *").is_err());
/// assert!(calc("2 +").is_err());
/// assert!(calc("3 *").is_err());
/// ```
///
/// In this example, we check that the `calc` function returns the correct result for valid expressions, and returns an error for invalid expressions.
///
pub fn calc(expression: &str) -> Result<Value, CalcError> {
    Err(CalcError::InvalidExpression)
}

pub enum CalcError {
  InvalidExpression,
  ChallengeFalseItems,
  UnsolvedChallenge,
  Other
}

/// Compares two mathematical or logical expressions using `eval` and `meval` crates
///
/// # Arguments
///
/// * `expression1` - The first expression to compare.
/// * `expression2` - The second expression to compare.
///
/// # Returns
///
/// * `Result<i8, CalcError>` - `-1` if `expression1` is less than `expression2`, `0` if they are equal, `1` if `expression1` is greater than `expression2`, or a `CalcError` if the comparison fails.
///
/// # Examples
///
/// ```
/// use calc::are_equal;
///
/// assert_eq!(are_equal("2 + 2", "4").unwrap(), 0);
/// assert_eq!(are_equal("3 * 4", "13").unwrap(), -1);
/// assert_eq!(are_equal("(5 + 6) * 7", "91").unwrap(), 1);
/// assert_eq!(are_equal("2 == 2 && 3 != 4", "true").unwrap(), 0);
/// assert_eq!(are_equal("true || false", "true").unwrap(), 0);
/// assert_eq!(are_equal("sin(0)", "0").unwrap(), 0);
/// assert_eq!(are_equal("cos(0)", "1").unwrap(), 0);
/// assert!(are_equal("2 +", "3").is_err());
/// assert!(are_equal("3 *", "6").is_err());
/// ```
///
/// In this example, we check that the `are_equal` function returns the correct result for valid expressions, and returns an error for invalid expressions.
pub fn are_equal(expression1: &str, expression2: &str) -> Result<i8, CalcError> {
    Ok(1)
}

/// Generates a random set of four numbers for the [24 puzzle](https://en.wikipedia.org/wiki/24_(puzzle))
///
/// # Returns
///
/// * `Result<[u8; 4], CalcError>` - An array of four numbers, or a `CalcError` if the generation fails.
///
/// # Examples
///
/// ```
/// use calc::generate_24_challenge;
///
/// let challenge = generate_24_challenge().unwrap();
/// println!("{:?}", challenge);
/// ```
///
/// In this example, we generate a random set of four numbers for the 24 puzzle and print the array containing the numbers.
pub fn generate_24_challenge() -> Result<[u8; 4], CalcError> {
    Err(CalcError::Other)
}

/// Checks if a set of four numbers and a mathematical expression can be used to solve the [24 puzzle](https://en.wikipedia.org/wiki/24_(puzzle))
///
/// # Arguments
///
/// * `numbers` - An array of four numbers.
/// * `expression` - A mathematical expression that uses the four numbers to solve the 24 puzzle.
///
/// # Returns
///
/// * `Result<(), CalcError>` - `Ok(())` if the set of numbers and the mathematical expression can be used to solve the 24 puzzle, or a `CalcError` if the check fails.
///
/// # Errors
///
/// * `CalcError::ChallengeFalseItems` - If the mathematical expression contains numbers that are not in the array, or if it uses a number more than once.
/// * `CalcError::UnsolvedChallenge` - If the mathematical expression does not solve the 24 puzzle.
///
/// # Examples
///
/// ```
/// use calc::check_24_challenge;
///
/// let numbers = [8, 8, 4, 3];
/// let expression = "((8 * 8) - 4) / 3";
/// assert!(check_24_challenge(&numbers, expression).is_ok());
///
/// let numbers = [1, 2, 3, 4];
/// let expression = "1 + 2 + 3 + 4";
/// assert!(check_24_challenge(&numbers, expression).is_err());
///
/// let numbers = [8, 8, 4, 3];
/// let expression = "((8 * 8) - 4) / 3 + 8";
/// assert!(check_24_challenge(&numbers, expression).is_err());
///
/// let numbers = [8, 8, 4, 3];
/// let expression = "((8 * 8) - 4) / 3 * 8";
/// assert!(check_24_challenge(&numbers, expression).is_err());
/// ```
///
/// In this example, we check if two sets of four numbers and mathematical expressions can be used to solve the 24 puzzle. We also check that the mathematical expression contains only the four numbers given in the array, and that it uses each number only once.
pub fn check_24_challenge(numbers: &[u8; 4], expression: &str) -> Result<(), CalcError> {
    Err(CalcError::Other)
}
