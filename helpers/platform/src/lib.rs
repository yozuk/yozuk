pub mod time {
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    mod time_now_utc {
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

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    mod time_now_utc {
        use time_crate::OffsetDateTime;

        pub fn now_utc() -> OffsetDateTime {
            OffsetDateTime::now_utc()
        }
    }

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    mod time_timezone {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen(
            inline_js = "export function get_timezone() { return Intl.DateTimeFormat().resolvedOptions().timeZone; }"
        )]
        extern "C" {
            fn get_timezone() -> String;
        }

        pub fn timezone() -> Option<String> {
            Some(get_timezone())
        }
    }

    #[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
    mod time_timezone {
        pub fn timezone() -> Option<String> {
            None
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod time_timezone {
        pub fn timezone() -> Option<String> {
            use time_tz::TimeZone;

            time_tz::system::get_timezone()
                .map(|tz| tz.name().to_string())
                .ok()
        }
    }

    pub use time_now_utc::*;
    pub use time_timezone::*;
}
