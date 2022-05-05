#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::{iter, mem};
use yozuk_sdk::model::*;
use yozuk_sdk::prelude::*;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

mod labeler;
mod model;
mod modelgen;
mod skill;
mod tagger;
mod tokenizer;

use labeler::*;
use tagger::*;

pub use model::*;
pub use skill::*;

#[cfg(feature = "modelgen")]
pub use modelgen::*;

pub const SKILLS_DIGEST: [u8; 20] = skill::skills_digest();

pub struct Yozuk {
    model: ModelSet,
    i18n: I18n,
    skills: Vec<SkillCache>,
    labelers: Vec<Box<dyn Labeler>>,
    commands: Vec<Option<CommandCache>>,
    redirections: Vec<(Vec<Token>, Vec<String>)>,
}

impl Yozuk {
    pub fn builder() -> YozukBuilder {
        Default::default()
    }

    pub fn parse_tokens(text: &str) -> Vec<Token> {
        tokenizer::tokenize(text)
    }

    pub fn get_commands(&self, tokens: &[Token], streams: &[InputStream]) -> Vec<CommandArgs> {
        let filter = |(redirect, _): &&(Vec<Token>, Vec<String>)| {
            redirect.len() == tokens.len()
                && redirect
                    .iter()
                    .map(|token| token.as_str())
                    .zip(tokens.iter().map(|token| token.as_str()))
                    .all(|(a, b)| yozuk_helper_english::normalized_eq(a, [b], 0))
        };

        #[cfg(feature = "rayon")]
        let redirection = self.redirections.par_iter().find_first(filter);

        #[cfg(not(feature = "rayon"))]
        let redirection = self.redirections.iter().find(filter);

        if let Some((_, args)) = redirection {
            return vec![CommandArgs::new()
                .add_args(["yozuk-redirect"])
                .add_args_iter(args)];
        }

        let labeler = FeatureLabeler::new(&self.labelers);

        #[cfg(feature = "rayon")]
        let iter = self.commands.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.commands.iter();

        let mut commands = iter
            .filter_map(|cache| cache.as_ref())
            .map(|cache| {
                (
                    cache,
                    cache
                        .preprocessors
                        .iter()
                        .fold(tokens.to_vec(), |tokens, prep| prep.preprocess(tokens)),
                )
            })
            .map(|(cache, tokens)| {
                (
                    cache,
                    if let Some(model) = &cache.model {
                        model.tag_tokens(&labeler, &tokens)
                    } else {
                        tokens
                    },
                )
            })
            .filter_map(|(cache, args)| {
                cache
                    .translators
                    .iter()
                    .find_map(|tr| tr.parse(&args, streams))
                    .map(|args| {
                        (
                            cache.command.priority(),
                            CommandArgs {
                                args: iter::once(cache.name.to_string())
                                    .chain(args.args.into_iter())
                                    .collect(),
                                data: args.data,
                            },
                        )
                    })
            })
            .collect::<Vec<_>>();

        commands.sort_by_key(|command| -command.0);
        commands.into_iter().map(|command| command.1).collect()
    }

    pub fn run_commands(
        &self,
        commands: Vec<CommandArgs>,
        streams: &mut [InputStream],
        i18n: Option<&I18n>,
    ) -> Result<Vec<Output>, Vec<Output>> {
        let commands = commands.into_iter().filter_map(|args| {
            self.model
                .get_index(&args.args[0])
                .and_then(|index| self.commands[index].as_ref())
                .map(|cmd| (args, &cmd.command))
        });

        let mut primary = None;
        let mut results = Vec::new();
        let mut errors = Vec::new();
        for (args, command) in commands {
            let name = args.args[0].clone();
            match command.run(args, streams, i18n.unwrap_or(&self.i18n)) {
                Ok(result) => {
                    if result.mode == OutputMode::Primary {
                        if primary.is_none() {
                            primary = Some(result);
                        }
                    } else {
                        results.push(result);
                    }
                }
                Err(err) => errors.push(err.into_output(name)),
            }
        }

        if errors.is_empty() {
            Ok(primary.into_iter().chain(results).collect())
        } else {
            Err(errors)
        }
    }

    pub fn suggest(&self, tokens: &[Token]) -> Option<String> {
        let words = tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<_>>();
        let string = words.join(" ");

        self.skills
            .iter()
            .flat_map(|cache| &cache.skill.suggests)
            .flat_map(|suggests| suggests.suggests(tokens))
            .map(|s| {
                (
                    distance::sift3(&string.to_lowercase(), &s.to_lowercase()),
                    s,
                )
            })
            .filter(|&(dist, _)| dist <= 3.5)
            .min_by_key(|(dist, _)| (dist * 100.0) as u32)
            .map(|(_, s)| s)
    }
}

#[derive(Default)]
pub struct YozukBuilder {
    config: Config,
    i18n: I18n,
    redirections: Vec<(Vec<Token>, Vec<String>)>,
}

impl YozukBuilder {
    pub fn set_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn set_i18n(mut self, i18n: I18n) -> Self {
        self.i18n = i18n;
        self
    }

    pub fn add_redirection<T, TI, S, SI>(mut self, tokens: TI, args: SI) -> Self
    where
        T: Into<Token>,
        TI: IntoIterator<Item = T>,
        S: Into<String>,
        SI: IntoIterator<Item = S>,
    {
        self.redirections.push((
            tokens.into_iter().map(Into::into).collect(),
            args.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn build(self, model: ModelSet) -> Yozuk {
        let build_info = concat!(r#"{"version": ""#, env!("CARGO_PKG_VERSION"), r#""}"#);

        let env = Environment::new().build_info(build_info);

        #[cfg(feature = "rayon")]
        let iter = skill::SKILLS.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = skill::SKILLS.iter();

        let results = iter
            .map(|entry| {
                let config = self
                    .config
                    .skills
                    .get(entry.key)
                    .zip(entry.entry.config_schema)
                    .map(|(value, _)| SkillConfig::new(value))
                    .unwrap_or_else(|| Ok(Default::default()))
                    .map_err(|err| (entry, err))?;
                Ok(SkillCache {
                    entry,
                    skill: (entry.entry.init)(&env, &config).map_err(|err| (entry, err))?,
                })
            })
            .collect::<Vec<Result<_, (&NamedSkillEntry, _)>>>();

        let mut skills = results
            .into_iter()
            .filter_map(|result| result.ok())
            .collect::<Vec<_>>();

        let labelers = skills
            .iter_mut()
            .flat_map(|cache| std::mem::take(&mut cache.skill.labelers))
            .collect::<Vec<_>>();

        let mut commands = Vec::new();
        for skill in &mut skills {
            if let Some(index) = model.get_index(skill.entry.key) {
                if let Some(command) = skill.skill.command.take() {
                    if commands.len() <= index {
                        commands.resize_with(index + 1, || None);
                    }
                    commands[index] = Some(CommandCache {
                        name: skill.entry.key,
                        model: model.get(skill.entry.key).map(ModelEntry::new),
                        translators: mem::take(&mut skill.skill.translators),
                        preprocessors: mem::take(&mut skill.skill.preprocessors),
                        command,
                    });
                }
            }
        }

        Yozuk {
            model,
            i18n: self.i18n,
            skills,
            labelers,
            commands,
            redirections: self.redirections,
        }
    }
}

struct SkillCache {
    entry: &'static NamedSkillEntry,
    skill: Skill,
}

struct CommandCache {
    name: &'static str,
    model: Option<ModelEntry>,
    preprocessors: Vec<Box<dyn Preprocessor>>,
    translators: Vec<Box<dyn Translator>>,
    command: Box<dyn Command>,
}
