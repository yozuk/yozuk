use serde_derive::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Highlight {
    pub kind: HighlightKind,
    pub range: Range<usize>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum HighlightKind {
    Value,
}

pub struct Highlighter {
    quotation: char,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new('`')
    }
}

impl Highlighter {
    pub fn new(quotation: char) -> Self {
        Self { quotation }
    }

    pub fn highlight(&self, s: &str) -> (String, Vec<Highlight>) {
        let mut plain = String::with_capacity(s.len());
        let mut highlights = vec![];
        let mut start: Option<usize> = None;
        let mut escaped = false;
        for c in s.chars() {
            if escaped {
                escaped = false;
                plain.push(c);
            } else if c == '\\' {
                escaped = true;
            } else if c == self.quotation {
                if let Some(start) = start.take() {
                    highlights.push(Highlight {
                        kind: HighlightKind::Value,
                        range: start..plain.len(),
                    });
                } else {
                    start = Some(plain.len());
                }
            } else {
                plain.push(c);
            }
        }
        (plain, highlights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlighter() {
        let text = "`quick brown` 🦊 \\`jumps\\`\nover `the` lazy `dog`.";
        let (plain, highlights) = Highlighter::default().highlight(text);
        assert_eq!(plain, "quick brown 🦊 `jumps`\nover the lazy dog.");
        assert_eq!(
            highlights,
            vec![
                Highlight {
                    kind: HighlightKind::Value,
                    range: 0..11
                },
                Highlight {
                    kind: HighlightKind::Value,
                    range: 30..33
                },
                Highlight {
                    kind: HighlightKind::Value,
                    range: 39..42
                }
            ]
        );
    }
}
