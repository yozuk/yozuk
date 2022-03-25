#![forbid(unsafe_code)]
#![deny(clippy::all)]

use anyhow::Error;
use rayon::{iter::Either, prelude::*};
use slog::{debug, error, Logger};
use sloggers::null::NullLoggerBuilder;
use sloggers::Build;
use std::{iter, mem};
use thiserror::Error;
use yozuk_sdk::prelude::*;

mod labeler;
mod model;
mod modelgen;
mod skill;
mod tagger;

use labeler::*;
use tagger::*;

pub use model::*;
pub use skill::*;

#[cfg(feature = "modelgen")]
pub use modelgen::*;

pub struct Yozuk {
    model: ModelSet,
    skills: Vec<SkillCache>,
    labelers: Vec<Box<dyn Labeler>>,
    commands: Vec<Option<CommandCache>>,
    logger: Logger,
}

impl Yozuk {
    pub fn builder() -> YozukBuilder {
        Default::default()
    }

    pub fn get_commands(
        &self,
        tokens: &[Token],
        streams: &[InputStream],
    ) -> Result<Vec<CommandArgs>, YozukError> {
        debug!(self.logger, "{:?}", tokens);

        let labeler = FeatureLabeler::new(&self.labelers);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus::get().max(2))
            .build()
            .unwrap();

        pool.install(|| {
            let mut commands = self
                .commands
                .par_iter()
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
                        .par_iter()
                        .find_map_first(|tr| tr.parse(&args, streams))
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

            if commands.is_empty() {
                let suggest = self.suggest(tokens);
                return Err(YozukError::UnintelligibleRequest { suggest });
            }

            commands.sort_by_key(|command| -command.0);
            Ok(commands.into_iter().map(|command| command.1).collect())
        })
    }

    pub fn run_commands(
        &self,
        commands: Vec<CommandArgs>,
        streams: &mut [InputStream],
    ) -> Result<Output, YozukError> {
        let commands = commands.into_iter().filter_map(|args| {
            self.model
                .get_index(&args.args[0])
                .and_then(|index| self.commands[index].as_ref())
                .map(|cmd| (args, &cmd.command))
        });

        let mut errors = Vec::new();
        for (args, command) in commands {
            match command.run(args, streams) {
                Ok(result) => return Ok(result),
                Err(err) => errors.push(err),
            }
        }

        Err(YozukError::CommandError { errors })
    }

    fn suggest(&self, tokens: &[Token]) -> Option<String> {
        let words = tokens
            .iter()
            .map(|token| token.as_utf8())
            .collect::<Vec<_>>();
        let string = words.join(" ");

        self.skills
            .par_iter()
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

pub struct YozukBuilder {
    config: Config,
    logger: Logger,
}

impl YozukBuilder {
    pub fn logger(mut self, logger: Logger) -> Self {
        self.logger = logger;
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn build(self, model: ModelSet) -> Yozuk {
        let build_info = build_info::format!(
            r#"{{"version": "{}", "compiler": "{}", "timestamp": "{}", "git": "{}"}}"#, 
            $.crate_info.version,
            $.compiler,
            $.timestamp,
            $.version_control);

        let env = Environment::new()
            .logger(self.logger.clone())
            .build_info(build_info);

        let (mut skills, errors): (Vec<_>, Vec<(&'static NamedSkillEntry, Error)>) = skill::SKILLS
            .par_iter()
            .map(|entry| {
                let config = self
                    .config
                    .skills
                    .get(entry.key)
                    .zip(entry.entry.config_schema)
                    .map(|(value, schema)| SkillConfig::new(value, schema))
                    .unwrap_or_else(|| Ok(Default::default()))
                    .map_err(|err| (entry, err))?;
                Ok(SkillCache {
                    entry,
                    skill: (entry.entry.init)(&env, &config).map_err(|err| (entry, err))?,
                })
            })
            .partition_map(|result| match result {
                Ok(skill) => Either::Left(skill),
                Err(err) => Either::Right(err),
            });

        for (entry, err) in errors {
            error!(self.logger, "Failed to initialize {}: {}", entry.key, err);
        }

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
                        model: model.get(skill.entry.key),
                        translators: mem::take(&mut skill.skill.translators),
                        preprocessors: mem::take(&mut skill.skill.preprocessors),
                        command,
                    });
                }
            }
        }

        Yozuk {
            model,
            skills,
            labelers,
            commands,
            logger: self.logger,
        }
    }
}

impl Default for YozukBuilder {
    fn default() -> Self {
        Self {
            config: Default::default(),
            logger: NullLoggerBuilder.build().unwrap(),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum YozukError {
    #[error("Unable to understand the request")]
    UnintelligibleRequest { suggest: Option<String> },

    #[error("Faild to run commands")]
    CommandError { errors: Vec<Output> },
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
