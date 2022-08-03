use clap::Parser;
use unicode_reader::{CodePoints, Graphemes};
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"kGNawrrhjv7zXXJ5XDqA9",
    init: |_| {
        Skill::builder()
            .add_suggestions(UnicodeSuggestions)
            .add_translator(UnicodeTranslator)
            .set_command(UnicodeCommand)
            .build()
    },
};

pub struct UnicodeSuggestions;

impl Suggestions for UnicodeSuggestions {
    fn suggestions(&self, _seed: u64, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        vec!["😶‍🌫️", "😵‍💫", "🐈‍⬛", "🐻‍❄️", "👩‍👩‍👦‍👦", "🏳️‍⚧️", "🏳️‍🌈"]
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

pub struct UnicodeTranslator;

impl Translator for UnicodeTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_urlencoded = !args.is_empty()
            && args.iter().all(|arg| {
                let mut data = arg.as_str().as_bytes();
                let mut graphemes = Graphemes::from(&mut data);
                graphemes.next().is_some() && graphemes.next().is_none()
            });
        if is_urlencoded {
            return Some(CommandArgs::new().add_args_iter(args.iter().map(|arg| arg.as_str())));
        }
        None
    }
}

pub struct UnicodeCommand;

impl Command for UnicodeCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let blocks = args
            .inputs
            .iter()
            .flat_map(|arg| {
                let mut data = arg.as_str().as_bytes();
                Graphemes::from(&mut data).collect::<Vec<_>>()
            })
            .filter_map(|graph| graph.ok())
            .map(|graph| {
                let escaped = graph
                    .chars()
                    .map(|c| {
                        if c == '`' {
                            format!("\\{c}")
                        } else {
                            format!("{}", c.escape_default()).replace("u{", "\\u{")
                        }
                    })
                    .collect::<Vec<_>>();
                let codepoints = graph
                    .chars()
                    .flat_map(|c| {
                        let data = c.to_string();
                        let mut data = data.as_str().as_bytes();
                        CodePoints::from(&mut data).collect::<Vec<_>>()
                    })
                    .filter_map(|c| c.ok())
                    .map(|c| {
                        if c == '`' {
                            format!("`\\{c}` (U+{:04X})", c as u32)
                        } else {
                            format!("`{c}` (U+{:04X})", c as u32)
                        }
                    })
                    .collect::<Vec<_>>();
                let utf8 = hex::encode(graph.as_bytes());
                let utf16 = graph
                    .encode_utf16()
                    .map(|u| format!("{u:04x}"))
                    .collect::<Vec<_>>();
                format!(
                    "`\"{}\"`\n{}\nUTF-8: `{utf8}`\nUTF-16: `{}`",
                    escaped.join(""),
                    codepoints.join(" "),
                    utf16.join(""),
                )
            })
            .map(|data| block::Data::new().set_highlighted_text_data(data, &Default::default()));
        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/unicode/")?;
        Ok(Output::new()
            .set_title("Unicode Decoder")
            .add_blocks_iter(blocks)
            .add_metadata(docs))
    }

    fn priority(&self) -> i32 {
        -100
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}
