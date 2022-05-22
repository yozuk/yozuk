#![forbid(unsafe_code)]
#![deny(clippy::all)]

use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"881SN07tdT529jbaAYLwX",
    config_schema: None,
    init: |_, _| Skill::builder().build(),
};
