use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Feature {
    pub name: String,
    pub non_entity: bool,
    pub pos: isize,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.name, self.pos)
    }
}
