/// Renverse la chaîne de caractères passée en argument.
///
/// # Exemple
/// ```
/// use string_tools::reverse;
///
/// assert_eq!(reverse("hello"), "olleh");
/// assert_eq!(reverse("Rust"), "tsuR");
/// assert_eq!(reverse("12345"), "54321");
/// ```
pub fn reverse(s: &str) -> String {
    "".to_string()
}

/// Vérifie si la chaîne de caractères est un palindrome.
///
/// # Exemple
/// ```
/// use string_tools::is_palindrome;
///
/// assert!(is_palindrome("radar"));
/// assert!(!is_palindrome("hello"));
/// assert!(is_palindrome("level"));
/// assert!(is_palindrome("A man a plan a canal Panama"));
/// ```
pub fn is_palindrome(s: &str) -> String {
    "".to_string()
}

/// Supprime toutes les voyelles (a, e, i, o, u) de la chaîne de caractères.
///
/// # Exemple
/// ```
/// use string_tools::remove_vowels;
///
/// assert_eq!(remove_vowels("hello"), "hll");
/// assert_eq!(remove_vowels("Rust programming"), "Rst prgrmmng");
/// assert_eq!(remove_vowels("AEIOUaeiou"), "");
/// ```
pub fn remove_vowels(s: &str) -> String {
    "".to_string()
}

/// Applique la compression Run-Length Encoding (RLE) à la chaîne.
///
/// # Exemple
/// ```
/// use string_tools::rle_encode;
///
/// assert_eq!(rle_encode("aaabbc"), "a3b2c1");
/// assert_eq!(rle_encode("xxxxxyyyzz"), "x5y3z2");
/// assert_eq!(rle_encode("abcdef"), "a1b1c1d1e1f1");
/// ```
pub fn rle_encode(s: &str) -> String {
    "".to_string()
}

pub enum txtToCase {
    SnakeCase,
    CamelCase,
    KebabCase,
}

/// Convertit une chaîne en snake_case, CamelCase, ou kebab-case en fonction du paramètre.
///
/// # Exemple
/// ```
/// use string_tools::{to_case, txtToCase};
///
/// assert_eq!(to_case("Hello World", txtToCase::SnakeCase), "hello_world");
/// assert_eq!(to_case("Hello World", txtToCase::CamelCase), "helloWorld");
/// assert_eq!(to_case("Hello World", txtToCase::KebabCase), "hello-world");
/// ```
pub fn to_case(s: &str, c: txtToCase) -> String {
    "".to_string()
}
