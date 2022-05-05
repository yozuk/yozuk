use clap::Parser;
use itertools::iproduct;
use rand::distributions::{Distribution, Uniform};
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"bVGwEx0QkZ7j3EbXF-VXo",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(PasswordCorpus)
            .add_translator(PasswordTranslator)
            .set_command(PasswordCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct PasswordCorpus;

impl Corpus for PasswordCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        iproduct!(["generate", "new"], ["password"])
            .flat_map(|(verb, name)| {
                vec![tk!([
                    verb,
                    name; "command:password"
                ])]
            })
            .chain(["password", "pwgen"].map(|name| tk!([name; "command:password"])))
            .collect()
    }
}

#[derive(Debug)]
pub struct PasswordTranslator;

impl Translator for PasswordTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        if args.iter().any(|arg| {
            arg.tag == "command:password" && normalized_eq(arg.as_utf8(), &["password", "pwgen"], 0)
        }) {
            return Some(CommandArgs::new());
        }
        None
    }
}

const CHARACTERS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*";

#[derive(Debug)]
pub struct PasswordCommand;

impl Command for PasswordCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let between = Uniform::from(0..CHARACTERS.len());
        let mut rng = rand::thread_rng();
        let mut password = String::with_capacity(args.length);
        for _ in 0..args.length {
            let index = between.sample(&mut rng);
            password.push_str(&CHARACTERS[index..index + 1]);
        }
        Ok(Output {
            title: "Password Generator".into(),
            blocks: vec![Block::Spoiler(block::Spoiler::new("Password", password))],
            ..Default::default()
        })
    }
}

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, default_value_t = 20)]
    pub length: usize,
}
