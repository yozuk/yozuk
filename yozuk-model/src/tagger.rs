use bytes::Bytes;
use crfs::{Attribute, Model};
use yozuk_sdk::prelude::*;

pub struct Tagger {
    data: Bytes,
}

impl Tagger {
    pub fn new(data: Bytes) -> Self {
        Self { data }
    }

    pub fn tag<I, T>(&self, features: I) -> Vec<String>
    where
        I: AsRef<[T]>,
        T: AsRef<[Feature]>,
    {
        let model = Model::new(&self.data).unwrap();
        let xseq = features
            .as_ref()
            .iter()
            .map(|features| {
                features
                    .as_ref()
                    .iter()
                    .map(|feature| Attribute::new(crate::minify_feature(feature), 1.0))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut tagger = model.tagger().unwrap();
        let yseq = tagger.tag(&xseq).unwrap();
        yseq.into_iter().map(Into::into).collect()
    }
}
