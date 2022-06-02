#[cfg(not(target_os = "wasi"))]
mod locale_locale {
    pub fn locale() -> Option<String> {
        sys_locale::get_locale()
    }
}

#[cfg(target_os = "wasi")]
mod locale_locale {
    pub fn locale() -> Option<String> {
        None
    }
}

pub use locale_locale::*;
