/// Renverse la chaîne de caractères passée en argument.
///
/// # Exemple
/// ```
/// use txt::reverse;
///
/// assert_eq!(reverse("hello"), "olleh");
/// assert_eq!(reverse("Rust"), "tsuR");
/// assert_eq!(reverse("12345"), "54321");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Vérifie si la chaîne de caractères est un palindrome.
///
/// # Exemple
/// ```
/// use txt::is_palindrome;
///
/// assert!(is_palindrome("radar"));
/// assert!(!is_palindrome("hello"));
/// assert!(is_palindrome("level"));
/// assert!(is_palindrome("A man a plan a canal Panama"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    cleaned.eq_ignore_ascii_case(&cleaned.chars().rev().collect::<String>())
}

/// Supprime toutes les voyelles (a, e, i, o, u) de la chaîne de caractères.
///
/// # Exemple
/// ```
/// use txt::remove_vowels;
///
/// assert_eq!(remove_vowels("hello"), "hll");
/// assert_eq!(remove_vowels("Rust programming"), "Rst prgrmmng");
/// assert_eq!(remove_vowels("AEIOUaeiou"), "");
/// ```
pub fn remove_vowels(s: &str) -> String {
    s.chars().filter(|&c| !"aeiouAEIOU".contains(c)).collect()
}

/// Applique la compression Run-Length Encoding (RLE) à la chaîne.
///
/// # Exemple
/// ```
/// use txt::rle_encode;
///
/// assert_eq!(rle_encode("aaabbc"), "a3b2c");
/// assert_eq!(rle_encode("xxxxxyyyzz"), "x5y3z2");
/// assert_eq!(rle_encode("abcdef"), "abcdef");
/// ```
pub fn rle_encode(s: &str) -> String {
    let mut encoded = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        let mut count = 1;
        while chars.peek() == Some(&c) {
            count += 1;
            chars.next();
        }
        encoded.push(c);
        if count > 1 {
            encoded.push_str(&count.to_string());
        }
    }
    encoded
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
/// use txt::{to_case, txtToCase};
///
/// assert_eq!(to_case("Hello World", txtToCase::SnakeCase), "hello_world");
/// assert_eq!(to_case("Hello World", txtToCase::CamelCase), "helloWorld");
/// assert_eq!(to_case("Hello World", txtToCase::KebabCase), "hello-world");
/// ```
pub fn to_case(s: &str, case: txtToCase) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    match case {
        txtToCase::SnakeCase => words.join("_").to_lowercase(),
        txtToCase::CamelCase => {
            let mut camel_case = String::new();
            for (i, word) in words.iter().enumerate() {
                if i == 0 {
                    camel_case.push_str(&word.to_lowercase());
                } else {
                    camel_case.push_str(&capitalize_first_letter(word));
                }
            }
            camel_case
        }
        txtToCase::KebabCase => words.join("-").to_lowercase(),
    }
}

fn capitalize_first_letter(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
