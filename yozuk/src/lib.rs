#![forbid(unsafe_code)]
#![deny(clippy::all)]

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::{iter, mem};
use yozuk_model::FeatureLabeler;
use yozuk_sdk::model::*;
use yozuk_sdk::prelude::*;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[cfg(all(feature = "rayon", target_arch = "wasm32"))]
compile_error!("wasm target does not support rayon");

mod model;
mod skill;

pub use model::*;
pub use skill::*;

pub const MODEL_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model.data"));

const MAX_ARG_BYTES_LEN: usize = 10240;

pub struct Yozuk {
    model: ModelSet,
    i18n: I18n,
    labelers: Vec<Box<dyn Labeler>>,
    commands: Vec<Option<CommandCache>>,
    redirections: Vec<(Vec<Token>, Vec<String>)>,
    seed: u64,
}

impl Yozuk {
    pub fn builder() -> YozukBuilder {
        Default::default()
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
                    .find_map(|tr| tr.generate_command(&args, streams))
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
        if commands
            .iter()
            .any(|cmd| cmd.bytes_len() > MAX_ARG_BYTES_LEN)
        {
            return Err(vec![
                Output::new().add_block(block::Comment::new().set_text("Too large arguments"))
            ]);
        }

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

        if primary.is_some() || errors.is_empty() {
            Ok(primary.into_iter().chain(results).collect())
        } else {
            Err(errors)
        }
    }

    pub fn random_suggests(&self, amount: usize) -> Vec<String> {
        let mut suggests = Vec::with_capacity(amount);
        let mut skills = self
            .commands
            .iter()
            .filter_map(|cache| cache.as_ref())
            .flat_map(|cache| &cache.suggests)
            .collect::<Vec<_>>();
        let mut rng = StdRng::seed_from_u64(self.seed);
        skills.shuffle(&mut rng);
        for skill in skills {
            if suggests.len() >= amount {
                break;
            }
            if let Some(suggest) = skill.suggests(self.seed, &[], &[]).choose_mut(&mut rng) {
                suggests.push(mem::take(suggest));
            }
        }
        suggests
    }

    pub fn suggests(&self, amount: usize, args: &[Token], streams: &[InputStream]) -> Vec<String> {
        let tokens = args.iter().map(|arg| arg.as_str()).collect::<Vec<_>>();
        let inputs = deunicode::deunicode(&tokens.join(" "));

        #[cfg(feature = "rayon")]
        let iter = self.commands.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = self.commands.iter();

        let labeler = FeatureLabeler::new(&self.labelers);
        let matcher = SkimMatcherV2::default().ignore_case();

        let mut suggests = iter
            .filter_map(|cache| cache.as_ref())
            .map(|cache| {
                (
                    cache,
                    cache
                        .preprocessors
                        .iter()
                        .fold(args.to_vec(), |tokens, prep| prep.preprocess(tokens)),
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
            .flat_map(|(cache, tokens)| {
                cache
                    .suggests
                    .iter()
                    .flat_map(|skill| skill.suggests(self.seed, &tokens, streams))
                    .enumerate()
                    .filter_map(|(index, text)| {
                        matcher
                            .fuzzy_match(&text, &inputs)
                            .map(|score| (index, cache.command.priority(), score, text))
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        suggests.sort_by_key(|(_, priority, _, _)| -priority);
        suggests.sort_by_key(|(index, _, _, _)| *index);
        suggests.sort_by_key(|(_, _, score, _)| -score);

        suggests
            .into_iter()
            .take(amount)
            .map(|(_, _, _, text)| text)
            .collect()
    }
}

pub struct YozukBuilder {
    i18n: I18n,
    redirections: Vec<(Vec<Token>, Vec<String>)>,
}

impl Default for YozukBuilder {
    fn default() -> Self {
        Self {
            i18n: I18n {
                locale: yozuk_helper_platform::locale::locale(),
                timezone: yozuk_helper_platform::time::timezone(),
                ..Default::default()
            },
            redirections: vec![],
        }
    }
}

impl YozukBuilder {
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

    pub fn build(self) -> Yozuk {
        let model = ModelSet::from_data(MODEL_DATA).unwrap();
        let build_info = concat!(r#"{"version": ""#, env!("CARGO_PKG_VERSION"), r#""}"#);

        let env = Environment::new().build_info(build_info);

        #[cfg(feature = "rayon")]
        let iter = skill::SKILLS.par_iter();
        #[cfg(not(feature = "rayon"))]
        let iter = skill::SKILLS.iter();

        let results = iter
            .map(|entry| Ok((entry, (entry.entry.init)(&env).map_err(|err| (entry, err))?)))
            .collect::<Vec<Result<_, (&NamedSkillEntry, _)>>>();

        let mut skills = results
            .into_iter()
            .filter_map(|result| result.ok())
            .collect::<Vec<_>>();

        let labelers = skills
            .iter_mut()
            .flat_map(|cache| std::mem::take(&mut cache.1.labelers))
            .collect::<Vec<_>>();

        let mut commands = Vec::new();
        for (entry, skill) in &mut skills {
            if let Some(index) = model.get_index(entry.key) {
                if let Some(command) = skill.command.take() {
                    if commands.len() <= index {
                        commands.resize_with(index + 1, || None);
                    }
                    commands[index] = Some(CommandCache {
                        name: entry.key,
                        model: model.get(entry.key).map(ModelEntry::new),
                        translators: mem::take(&mut skill.translators),
                        preprocessors: mem::take(&mut skill.preprocessors),
                        suggests: mem::take(&mut skill.suggests),
                        command,
                    });
                }
            }
        }

        Yozuk {
            model,
            i18n: self.i18n,
            labelers,
            commands,
            redirections: self.redirections,
            seed: rand::random(),
        }
    }
}

struct CommandCache {
    name: &'static str,
    model: Option<ModelEntry>,
    preprocessors: Vec<Box<dyn Preprocessor>>,
    translators: Vec<Box<dyn Translator>>,
    suggests: Vec<Box<dyn Suggests>>,
    command: Box<dyn Command>,
}
