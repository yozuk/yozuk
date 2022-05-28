#[cfg(target_arch = "wasm32")]
pub mod time {
    use time_crate::OffsetDateTime;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export function get_time() { return Date.now(); }")]
    extern "C" {
        fn get_time() -> f64;
    }

    pub fn now_utc() -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp_nanos(get_time() as i128 * 1000000).unwrap()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod time {
    use time_crate::OffsetDateTime;

    pub fn now_utc() -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }
}
