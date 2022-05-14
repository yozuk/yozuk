use anyhow::Result;
use std::io::Write;
use std::{env, fs::File, io::Read, path::Path};
use yozuk_sdk::model::*;
use yozuk_sdk::prelude::*;

fn main() -> Result<()> {
    build_data::set_GIT_COMMIT();
    build_data::set_BUILD_TIMESTAMP();
    build_data::set_RUSTC_VERSION();
    build_data::no_debug_rebuilds();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let model_path = Path::new(&out_dir).join("model.data");

    if let Ok(mut file) = File::open(&model_path) {
        let mut data = Vec::new();
        let _ = file.read_to_end(&mut data);
        if data.ends_with(&yozuk_core_skillset::skills_digest())
            && ModelSet::from_data(data).is_ok()
        {
            return Ok(());
        }
    }

    let model = yozuk_model::modelgen(yozuk_core_skillset::SKILLS, &Environment::new())?;
    let mut file = File::create(model_path)?;
    model.write(&mut file)?;
    file.write_all(&yozuk_core_skillset::skills_digest())?;

    Ok(())
}
