use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Locale {
    pub timezone: Option<String>,
    pub location: Option<(f64, f64)>,
}
