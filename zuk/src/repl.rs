#![cfg(not(target_arch = "wasm32"))]

use owo_colors::OwoColorize;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, Validator};
use rustyline::{Context, Editor};
use rustyline_derive::Helper;
use std::borrow::Cow;

pub struct Repl {
    editor: Editor<YozukHelper>,
}

const PROMPT: &str = "» ";

impl Repl {
    pub fn new() -> Self {
        let mut editor = Editor::new();

        let helper = YozukHelper {
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
        };
        editor.set_helper(Some(helper));
        editor.helper_mut().expect("No helper").colored_prompt =
            format!("{}", PROMPT.bold().blue());

        Self { editor }
    }

    pub fn readline(&mut self) -> Option<String> {
        let readline = self.editor.readline(PROMPT);
        match readline {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                return Some(line);
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("Bye.");
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
        None
    }
}

#[derive(Helper)]
struct YozukHelper {
    highlighter: MatchingBracketHighlighter,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for YozukHelper {
    type Candidate = Pair;
}

impl Hinter for YozukHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for YozukHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Cow::Borrowed(&self.colored_prompt)
        } else {
            Cow::Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for YozukHelper {
    fn validate(
        &self,
        _ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        Ok(validate::ValidationResult::Valid(None))
    }

    fn validate_while_typing(&self) -> bool {
        false
    }
}
