use crate::prelude::*;
use anyhow::Result;
use std::fmt;

#[derive(Clone, Copy)]
pub struct SkillEntry {
    pub model_id: &'static [u8],
    pub config_schema: Option<&'static str>,
    pub init: fn(&Environment, &SkillConfig) -> Result<Skill>,
}

#[derive(Clone, Copy)]
pub struct NamedSkillEntry {
    pub key: &'static str,
    pub entry: SkillEntry,
}

pub trait Labeler: fmt::Debug + Send + Sync + 'static {
    fn label_features(&self, input: &[Token]) -> Vec<Vec<Feature>>;
}

pub trait Corpus: fmt::Debug + Send + Sync + 'static {
    fn training_data(&self) -> Vec<Vec<Token>>;
    fn weight(&self) -> f64 {
        1.0
    }
}

pub trait Suggests: fmt::Debug + Send + Sync + 'static {
    fn suggests(&self, input: &[Token]) -> Vec<String>;
}

pub trait Preprocessor: fmt::Debug + Send + Sync + 'static {
    fn preprocess(&self, input: Vec<Token>) -> Vec<Token>;
}

pub trait Translator: fmt::Debug + Send + Sync + 'static {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs>;
}

pub trait Command: fmt::Debug + Send + Sync + 'static {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError>;
    fn priority(&self) -> i32 {
        0
    }
}

#[derive(Debug, Default)]
pub struct Skill {
    pub corpora: Vec<Box<dyn Corpus>>,
    pub suggests: Vec<Box<dyn Suggests>>,
    pub labelers: Vec<Box<dyn Labeler>>,
    pub preprocessors: Vec<Box<dyn Preprocessor>>,
    pub translators: Vec<Box<dyn Translator>>,
    pub command: Option<Box<dyn Command>>,
}

impl Skill {
    pub fn builder() -> SkillBuilder {
        Default::default()
    }
}

#[derive(Default)]
pub struct SkillBuilder {
    skill: Skill,
}

impl SkillBuilder {
    pub fn add_corpus<T: Corpus>(mut self, item: T) -> Self {
        self.skill.corpora.push(Box::new(item));
        self
    }

    pub fn add_suggests<T: Suggests>(mut self, item: T) -> Self {
        self.skill.suggests.push(Box::new(item));
        self
    }

    pub fn add_labeler<T: Labeler>(mut self, item: T) -> Self {
        self.skill.labelers.push(Box::new(item));
        self
    }

    pub fn add_translator<T: Translator>(mut self, item: T) -> Self {
        self.skill.translators.push(Box::new(item));
        self
    }

    pub fn add_preprocessor<T: Preprocessor>(mut self, item: T) -> Self {
        self.skill.preprocessors.push(Box::new(item));
        self
    }

    pub fn set_command<T: Command>(mut self, item: T) -> Self {
        self.skill.command = Some(Box::new(item));
        self
    }

    pub fn build(self) -> Result<Skill> {
        Ok(self.skill)
    }
}
