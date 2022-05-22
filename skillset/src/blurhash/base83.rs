fn char_to_value(c: char) -> Option<u8> {
    Some(match c {
        '0'..='9' => c as u8 - b'0',
        'A'..='Z' => c as u8 - b'A' + 10,
        'a'..='z' => c as u8 - b'a' + 36,
        '#' => 62,
        '$' => 63,
        '%' => 64,
        '*' => 65,
        '+' => 66,
        ',' => 67,
        '-' => 68,
        '.' => 69,
        ':' => 70,
        ';' => 71,
        '=' => 72,
        '?' => 73,
        '@' => 74,
        '[' => 75,
        ']' => 76,
        '^' => 77,
        '_' => 78,
        '{' => 79,
        '|' => 80,
        '}' => 81,
        '~' => 82,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_value() {
        assert_eq!(char_to_value('5'), Some(5));
        assert_eq!(char_to_value('J'), Some(19));
        assert_eq!(char_to_value('e'), Some(40));
        assert_eq!(char_to_value('%'), Some(64));
        assert_eq!(char_to_value('!'), None);
    }
}
