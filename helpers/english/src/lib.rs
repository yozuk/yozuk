#![forbid(unsafe_code)]
#![deny(clippy::all)]

use inflector::cases::snakecase;
use std::borrow::Cow;

mod numeral;
pub use numeral::*;

pub fn singularize(word: &str) -> String {
    singularize::singularize(word)
}

pub fn pluralize(word: &str, n: usize) -> Cow<'_, str> {
    if n == 1 {
        Cow::Borrowed(word)
    } else {
        Cow::Owned(singularize::pluralize(word))
    }
}

pub fn normalize(text: &str) -> String {
    let norm_text = text.trim_end_matches(|c: char| c.is_ascii_punctuation());
    let norm_text = deunicode::deunicode(norm_text);
    let norm_text = singularize(&norm_text.to_lowercase());
    let norm_text = snakecase::to_snake_case(&norm_text);
    if norm_text.is_empty() {
        text.to_string()
    } else {
        norm_text
    }
}

pub fn normalized_eq<A, B, T>(a: A, b: B, levenshtein_tolerance: usize) -> bool
where
    A: AsRef<str>,
    B: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    let a_normalized = normalize(a.as_ref());
    b.into_iter().any(|s| {
        distance::damerau_levenshtein(&normalize(s.as_ref()), &a_normalized)
            <= levenshtein_tolerance
    })
}
