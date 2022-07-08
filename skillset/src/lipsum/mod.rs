use clap::Parser;
use lipsum::lipsum;
use rand::Rng;
use yozuk_helper_english::{normalized_eq, NumeralTokenParser};
use yozuk_helper_preprocessor::TokenMerger;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"WKQjgQTbySg0_NOxuUHBD",
    init: |_| {
        Skill::builder()
            .add_preprocessor(TokenMerger::new(NumeralTokenParser))
            .add_corpus(LipsumCorpus)
            .add_translator(LipsumTranslator)
            .set_command(LipsumCommand)
            .add_suggests(LipsumSuggests)
            .build()
    },
};

#[derive(Debug)]
pub struct LipsumSuggests;

impl Suggests for LipsumSuggests {
    fn suggests(&self, _args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen_range(5..=10) * 10;
        vec![format!("{} words dummy text", n)]
    }
}

#[derive(Debug)]
pub struct LipsumCorpus;

impl Corpus for LipsumCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        vec![
            tk!([
                "Lorem"; "keyword",
                "ipsum"; "keyword",
                "dolor",
                "sit",
                "amet"
            ]),
            tk!([
                "Lorem"; "keyword",
                "ipsum"; "keyword",
                "dolor",
                "sit",
                "amet",
                "100"; "input:count"
            ]),
            tk!([
                "Lorem"; "keyword",
                "ipsum"; "keyword",
                "dolor",
                "sit",
                "amet",
                "100"; "input:count",
                "words"
            ]),
            tk!([
                "lorem"; "keyword",
                "ipsum"; "keyword"
            ]),
            tk!([
                "lorem"; "keyword",
                "ipsum"; "keyword",
                "100"; "input:count"
            ]),
            tk!([
                "lorem"; "keyword",
                "ipsum"; "keyword",
                "100"; "input:count",
                "words"
            ]),
            tk!(["lipsum,"; "keyword"]),
            tk!([
                "lipsum,"; "keyword",
                "100"; "input:count"
            ]),
            tk!([
                "lipsum,"; "keyword",
                "100"; "input:count",
                "words"
            ]),
            tk!([
                "dummy"; "keyword",
                "text"; "keyword"
            ]),
            tk!([
                "Generate",
                "dummy"; "keyword",
                "text"; "keyword"
            ]),
            tk!([
                "dummy"; "keyword",
                "text"; "keyword",
                "100"; "input:count"
            ]),
            tk!([
                "dummy"; "keyword",
                "text"; "keyword",
                "100"; "input:count",
                "words"
            ]),
            tk!([
                "Generate",
                "100"; "input:count",
                "words",
                "dummy"; "keyword",
                "text"; "keyword"
            ]),
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct LipsumTranslator;

impl Translator for LipsumTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let count = args
            .iter()
            .find(|arg| arg.tag == "input:count")
            .and_then(|arg| arg.as_str().parse::<usize>().ok())
            .map(|n| ["-n".to_string(), n.to_string()]);

        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "keyword")
            .collect::<Vec<_>>();

        if let [lorem, ipsum] = keywords[..] {
            if normalized_eq(lorem.as_str(), &["lorem"], 1)
                && normalized_eq(ipsum.as_str(), &["ipsum"], 1)
            {
                return Some(CommandArgs::new().add_args(count));
            }
        }

        if let [dummy, text] = keywords[..] {
            if normalized_eq(dummy.as_str(), &["dummy"], 1)
                && normalized_eq(text.as_str(), &["text"], 1)
            {
                return Some(CommandArgs::new().add_args(count));
            }
        }

        if let [lipsum] = keywords[..] {
            if normalized_eq(lipsum.as_str(), &["lipsum"], 1) {
                return Some(CommandArgs::new().add_args(count));
            }
        }

        None
    }
}

#[cfg(feature = "wild")]
const MAX_COUNT: usize = u16::MAX as _;

#[cfg(not(feature = "wild"))]
const MAX_COUNT: usize = 320;

#[derive(Debug)]
pub struct LipsumCommand;

impl Command for LipsumCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        if args.n > MAX_COUNT {
            return Err(Output::new()
                .set_title("Lorem ipsum")
                .add_block(block::Comment::new().set_text(format!(
                    "Too large number of the requested words (Limit: {}).",
                    MAX_COUNT
                )))
                .into());
        }

        Ok(Output::new()
            .set_title("Lorem ipsum")
            .add_block(block::Data::new().set_text_data(lipsum(args.n))))
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, default_value_t = 30)]
    pub n: usize,
}
