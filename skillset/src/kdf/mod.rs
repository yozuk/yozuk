use clap::Parser;
use itertools::iproduct;
use std::collections::BTreeMap;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

mod algorithm;
use algorithm::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"jZfCk5tCjLb5jW0oTzuQ5",
    init: |_| {
        Skill::builder()
            .add_corpus(KdfCorpus)
            .add_suggestions(KdfSuggestions)
            .add_translator(KdfTranslator)
            .set_command(KdfCommand)
            .build()
    },
};

pub struct KdfSuggestions;

impl Suggestions for KdfSuggestions {
    fn suggestions(&self, _seed: u64, args: &[Token], _streams: &[InputStream]) -> Vec<String> {
        let inputs = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .map(|arg| arg.as_str())
            .collect::<Vec<_>>();
        let joined = shell_words::join(if inputs.is_empty() {
            vec!["Hello World!"]
        } else {
            inputs
        });
        ENTRIES
            .iter()
            .filter_map(|entry| entry.keywords.iter().next())
            .map(|s| format!("{joined} to {s}"))
            .collect()
    }
}

pub struct KdfCorpus;

impl Corpus for KdfCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        let inputs = vec![
            "Hello World",
            "😍😗😋",
            "quick brown fox jumps over the lazy dog",
            "Veterinarian",
        ];
        iproduct!(
            inputs.clone(),
            ["as", "to", "in", "into"],
            ENTRIES.iter().flat_map(|entry| entry.keywords)
        )
        .map(|(data, prefix, alg)| {
            tk!([
                data; "input:data",
                prefix,
                *alg; "keyword"
            ])
        })
        .chain(
            ENTRIES
                .iter()
                .flat_map(|entry| entry.keywords)
                .flat_map(|key| {
                    [
                        tk!([key.to_string(); "keyword"]),
                        tk!([key.to_string(); "keyword", key.to_string(); "keyword"]),
                    ]
                }),
        )
        .collect()
    }
}

pub struct KdfTranslator;

impl Translator for KdfTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let input = args
            .iter()
            .filter(|arg| arg.tag == "input:data")
            .flat_map(|arg| ["--input", arg.as_str()]);

        let keywords = args
            .iter()
            .filter(|arg| arg.tag == "keyword")
            .collect::<Vec<_>>();

        if !keywords.is_empty()
            && keywords.iter().all(|arg| {
                ENTRIES
                    .iter()
                    .any(|entry| normalized_eq(arg.as_str(), entry.keywords, 0))
            })
        {
            return Some(
                CommandArgs::new().add_args_iter(input).add_args_iter(
                    keywords
                        .iter()
                        .flat_map(|arg| ["--algorithm", arg.as_str()]),
                ),
            );
        }

        None
    }
}

pub struct KdfCommand;

impl Command for KdfCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _user: &UserContext,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;

        let mut entries = BTreeMap::new();
        for name in &args.algorithm {
            let matched = ENTRIES
                .iter()
                .filter(|entry| normalized_eq(name, entry.keywords, 0))
                .collect::<Vec<_>>();

            if matched.is_empty() {
                return Err(Output::new()
                    .set_title("KDF")
                    .add_block(
                        block::Comment::new().set_text(format!("Unsupprted algorithm: {}", name)),
                    )
                    .into());
            }

            for entry in matched {
                entries.entry(entry.name).or_insert_with(|| (entry.init)());
            }
        }

        let docs = Metadata::docs("https://docs.yozuk.com/docs/skills/kdf/")?;
        if let [input, ..] = &args.input[..] {
            if let Some(output) = compute_hash(input.as_bytes(), entries) {
                return Ok(output.add_metadata(docs));
            }
        }
        Err(Output::new()
            .set_title("KDF")
            .add_block(block::Comment::new().set_text("No valid input source provided"))
            .add_metadata(docs)
            .into())
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, multiple_occurrences(true))]
    pub algorithm: Vec<String>,
    #[clap(short, long, multiple_occurrences(true))]
    pub input: Vec<String>,
}

fn compute_hash(
    password: &[u8],
    entries: BTreeMap<&'static str, Box<dyn Algorithm>>,
) -> Option<Output> {
    let entries = entries.into_iter().collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    let result = if entries.len() == 1 {
        vec![entries[0].1.hash_default(password, &mut rng)]
    } else {
        entries
            .into_iter()
            .map(|(name, hash)| format!("{}: `{}`", name, hash.hash_default(password, &mut rng)))
            .collect::<Vec<_>>()
    };

    Some(Output::new().set_title("KDF").add_block(
        block::Data::new().set_highlighted_text_data(result.join("\n"), &Default::default()),
    ))
}
