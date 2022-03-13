use anyhow::Result;
use std::{env, fs::File, io::Read, path::Path};
use yozuk::ModelSet;
use yozuk_sdk::prelude::*;

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let model_path = Path::new(&out_dir).join("model.crfsuite");

    if let Ok(mut file) = File::open(&model_path) {
        let mut data = Vec::new();
        let _ = file.read_to_end(&mut data);
        if ModelSet::from_data(data).is_ok() {
            return Ok(());
        }
    }

    let model = yozuk::modelgen(&Environment::new())?;
    let file = File::create(model_path)?;
    model.write(file)?;

    Ok(())
}
