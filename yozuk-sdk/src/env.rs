#[derive(Debug, Default, Clone)]
pub struct Environment {
    pub build_info: &'static str,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_info(mut self, build_info: &'static str) -> Self {
        self.build_info = build_info;
        self
    }
}
