#![forbid(unsafe_code)]
#![deny(clippy::all)]

use inflector::cases::snakecase;
use std::borrow::Cow;

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
    let text = text.trim_end_matches(|c: char| c.is_ascii_punctuation());
    let text = singularize(&text.to_lowercase());
    snakecase::to_snake_case(&text)
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
