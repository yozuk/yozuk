fn char_to_value(c: u8) -> Option<u8> {
    Some(match c {
        b'0'..=b'9' => c - b'0',
        b'A'..=b'Z' => c - b'A' + 10,
        b'a'..=b'z' => c - b'a' + 36,
        b'#' => 62,
        b'$' => 63,
        b'%' => 64,
        b'*' => 65,
        b'+' => 66,
        b',' => 67,
        b'-' => 68,
        b'.' => 69,
        b':' => 70,
        b';' => 71,
        b'=' => 72,
        b'?' => 73,
        b'@' => 74,
        b'[' => 75,
        b']' => 76,
        b'^' => 77,
        b'_' => 78,
        b'{' => 79,
        b'|' => 80,
        b'}' => 81,
        b'~' => 82,
        _ => return None,
    })
}

fn decode_digits(digits: &[u8]) -> Option<u32> {
    let mut result = 0u32;
    let mut base = 1u32;
    for digit in digits.iter().rev() {
        if let Some(value) = char_to_value(*digit) {
            result += (value as u32) * base;
            base *= 83;
        } else {
            return None;
        }
    }
    Some(result)
}

const HEADER_LENGTH: usize = 6;

pub fn validate_blurhash(hash: &[u8]) -> bool {
    if hash.len() < HEADER_LENGTH {
        return false;
    }

    let components = match decode_digits(&hash[0..1]) {
        Some(value) => value,
        None => return false,
    };
    let nx = components % 9 + 1;
    let ny = components / 9 + 1;

    if decode_digits(&hash[1..2]).is_none() {
        return false;
    }

    let rgb24 = match decode_digits(&hash[2..6]) {
        Some(value) => value,
        None => return false,
    };
    if rgb24 > 0xFFFFFF {
        return false;
    }

    let data = &hash[HEADER_LENGTH..];
    let length = (nx as usize * ny as usize - 1) * 2;
    data.len() == length && data.iter().all(|&c| char_to_value(c).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_value() {
        assert_eq!(char_to_value(b'5'), Some(5));
        assert_eq!(char_to_value(b'J'), Some(19));
        assert_eq!(char_to_value(b'e'), Some(40));
        assert_eq!(char_to_value(b'%'), Some(64));
        assert_eq!(char_to_value(b'!'), None);
    }

    #[test]
    fn test_validate_blurhash() {
        assert!(!validate_blurhash(&[]));
        assert!(!validate_blurhash(&b"LlMF%n00%#MwS|WCWEM{R*bbWB"[..]));
        assert!(!validate_blurhash(&b"LlMF%n00%#MwS|WCWEM{R*bbWBbHbH"[..]));
        assert!(validate_blurhash(&b"LlMF%n00%#MwS|WCWEM{R*bbWBbH"[..]));
    }
}
