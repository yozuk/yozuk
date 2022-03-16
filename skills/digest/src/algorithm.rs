use digest::Digest;

pub const ENTRIES: &[AlgorithmEntry] = &[AlgorithmEntry {
    name: "MD5",
    keywords: &["md5"],
    init: || Box::new(Md5::new()),
}];

pub struct AlgorithmEntry {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub init: fn() -> Box<dyn Algorithm>,
}

pub trait Algorithm {
    fn update(&mut self, data: &[u8]);
    fn finalize(&mut self) -> Vec<u8>;
}

pub struct Md5(md5::Md5);

impl Md5 {
    pub fn new() -> Self {
        Self(md5::Md5::new())
    }
}

impl Algorithm for Md5 {
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    fn finalize(&mut self) -> Vec<u8> {
        self.0.finalize_reset().to_vec()
    }
}
