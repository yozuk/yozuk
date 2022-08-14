use time::format_description::well_known::{Rfc2822, Rfc3339};
use time::OffsetDateTime;

pub struct Format {
    pub name: &'static str,
    pub keywords: &'static [&'static str],
    pub format: fn(&OffsetDateTime) -> Option<String>,
}

pub const ENTRIES: &[Format] = &[
    Format {
        name: "UNIX",
        keywords: &["unix"],
        format: |t| Some(t.unix_timestamp().to_string()),
    },
    Format {
        name: "UNIX Milliseconds",
        keywords: &[
            "unixms",
            "unix ms",
            "unix milli",
            "unix millisec",
            "unix milliseconds",
        ],
        format: |t| Some((t.unix_timestamp_nanos() * 1000000).to_string()),
    },
    Format {
        name: "UNIX Microseconds",
        keywords: &[
            "unixus",
            "unix us",
            "unixμs",
            "unix μs",
            "unix μsec",
            "unix μseconds",
            "unix micro",
            "unix microsec",
            "unix microseconds",
        ],
        format: |t| Some((t.unix_timestamp_nanos() * 1000).to_string()),
    },
    Format {
        name: "UNIX Nanoseconds",
        keywords: &[
            "unixns",
            "unix ns",
            "unix nano",
            "unix nanosec",
            "unix nanoseconds",
        ],
        format: |t| Some(t.unix_timestamp_nanos().to_string()),
    },
    Format {
        name: "RFC-2822",
        keywords: &["rfc-2822", "rfc2822", "2822"],
        format: |t| t.format(&Rfc2822).ok(),
    },
    Format {
        name: "RFC-3339",
        keywords: &["rfc-3339", "rfc3339", "3339", "iso", "iso-8601", "8601"],
        format: |t| t.format(&Rfc3339).ok(),
    },
    Format {
        name: "NTP",
        keywords: &["ntp"],
        format: |t| {
            t.unix_timestamp()
                .checked_add(NTP_OFFSET)
                .map(|t| t.to_string())
        },
    },
    Format {
        name: "TAI-64",
        keywords: &["tai64", "tai-64"],
        format: |t| {
            unix_to_tai64(t.unix_timestamp()).map(|tai64| {
                format!(
                    "@{}",
                    hex::encode((tai64 + TAI64_LABEL_OFFSET).to_be_bytes())
                )
            })
        },
    },
];

pub const TAI64_LABEL_OFFSET: u64 = 0x4000000000000000 - NTP_OFFSET as u64;
pub const NTP_OFFSET: i64 = 2208988800;

pub fn unix_to_tai64(unix: i64) -> Option<u64> {
    let ntp = (unix + NTP_OFFSET) as u64;
    let index = match TAI64_LEAP_SECONDS.binary_search_by_key(&ntp, |(ts, _)| *ts) {
        Ok(n) => n,
        Err(n) => n,
    };
    TAI64_LEAP_SECONDS.get(index).map(|(_, leap)| ntp + leap)
}

// https://www.ietf.org/timezones/data/leap-seconds.list
pub const TAI64_LEAP_SECONDS: &[(u64, u64)] = &[
    (2272060800, 10),
    (2287785600, 11),
    (2303683200, 12),
    (2335219200, 13),
    (2366755200, 14),
    (2398291200, 15),
    (2429913600, 16),
    (2461449600, 17),
    (2492985600, 18),
    (2524521600, 19),
    (2571782400, 20),
    (2603318400, 21),
    (2634854400, 22),
    (2698012800, 23),
    (2776982400, 24),
    (2840140800, 25),
    (2871676800, 26),
    (2918937600, 27),
    (2950473600, 28),
    (2982009600, 29),
    (3029443200, 30),
    (3076704000, 31),
    (3124137600, 32),
    (3345062400, 33),
    (3439756800, 34),
    (3550089600, 35),
    (3644697600, 36),
    (3692217600, 37),
    (3881174400 - 1, 37),
];
