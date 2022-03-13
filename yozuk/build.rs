use globwalk::GlobWalkerBuilder;
use serde_derive::Deserialize;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::Path,
};

#[derive(Deserialize)]
struct Toml {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
}

fn main() {
    build_info_build::build_script();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("skill_list.rs");
    let mut out = File::create(dest_path).unwrap();

    out.write_all(b"skills![").unwrap();

    for toml in GlobWalkerBuilder::from_patterns("../skills", &["/*/Cargo.toml"])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
    {
        let mut file = File::open(toml.path()).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let toml: Toml = toml::from_str(&data).unwrap();
        out.write_all(
            format!(
                "[{}, \"{}\"],",
                toml.package.name.replace('-', "_"),
                toml.package.name
            )
            .as_bytes(),
        )
        .unwrap();
    }

    out.write_all(b"];").unwrap();

    println!("cargo:rerun-if-changed=../skills");
}
