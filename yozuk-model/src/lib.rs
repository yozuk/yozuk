#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod labeler;
mod modelgen;
mod tagger;

#[cfg(feature = "modelgen")]
pub use modelgen::*;

pub use labeler::*;
pub use tagger::*;

use yozuk_sdk::feature::Feature;

pub(crate) fn minify_feature(feature: &Feature) -> String {
    use blake2::{digest::consts::U2, Blake2b, Digest};
    let mut hasher = Blake2b::<U2>::new();
    hasher.update(&feature.to_string());
    let res = hasher.finalize();
    base64::encode_config(res, base64::STANDARD_NO_PAD)
}
