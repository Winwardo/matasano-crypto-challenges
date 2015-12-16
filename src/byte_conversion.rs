pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    assert!(hex.len() % 2 == 0, "Hex string length is not even.");
    let mut result: Vec<u8> = Vec::new();

    let mut first_byte = true;

    for c in hex.chars() {
        let val = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' | 'a' => 10,
            'B' | 'b' => 11,
            'C' | 'c' => 12,
            'D' | 'd' => 13,
            'E' | 'e' => 14,
            'F' | 'f' => 15,
            _ => unreachable!(),
        };

        if first_byte {
            result.push(val << 4);
        } else {
            let length = result.len();
            result[length - 1] = result[length - 1] + val;
        }
        first_byte = !first_byte;
    }

    result
}

pub fn bytes_to_readable_text(bytes: &Vec<u8>) -> String {
    bytes.iter().map(|x| *x as char).collect()
}

pub fn readable_text_to_bytes(text: &str) -> Vec<u8> {
    text.bytes().collect()
}

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    let mut result: String = String::new();
    for byte in bytes {

        let bin_to_char = |x| {
            match x {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                15 => 'f',
                _ => unreachable!(),
            }
        };

        result.push(bin_to_char(byte >> 4));
        result.push(bin_to_char(byte % 16));
    }

    result
}

/// Read up more on the base64 algorithm here:
/// https://en.wikibooks.org/wiki/Algorithm_Implementation/Miscellaneous/Base64
pub fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    let base64chars: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
                                      'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
                                      'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
                                      'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                                      'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
                                      '8', '9', '+', '/'];
    assert_eq!(64, base64chars.len());

    let mut padding = String::new();

    // Pad our bytes to a multiple of 3 in length
    let mut padded_bytes = bytes.clone();
    while padded_bytes.len() % 3 != 0 {
        padded_bytes.push(0);
        padding.push('=');
    }
    assert_eq!(0, padded_bytes.len() % 3);

    let mut result = String::new();

    // Step through our bytes 3 at a time
    let mut i = 0;
    while i < padded_bytes.len() {
        // We're using direct access as we know we won't be accessing invalid indices
        // Bring the three bytes into one larger word
        let mut chunk: u64 = 0;
        chunk += (padded_bytes[i] as u64) << 16;
        chunk += (padded_bytes[i + 1] as u64) << 8;
        chunk += padded_bytes[i + 2] as u64;

        // And split it out into 4, appending to our string
        result.push(base64chars[((chunk >> 3 * 6) & 63) as usize]);
        result.push(base64chars[((chunk >> 2 * 6) & 63) as usize]);
        result.push(base64chars[((chunk >> 1 * 6) & 63) as usize]);
        result.push(base64chars[((chunk >> 0 * 6) & 63) as usize]);

        i += 3;
    }

    let truncated_bytes: Vec<u8> = result.bytes().take(result.len() - padding.len()).collect();
    let truncated_string = match String::from_utf8(truncated_bytes) {
        Ok(x) => x,
        Err(_) => unreachable!(),
    };

    let final_output = format!("{}{}", truncated_string, padding);
    assert_eq!(0, final_output.len() % 4);

    final_output
}

pub fn base64_to_bytes(base64: &str) -> Vec<u8> {
    // We use .bytes() and not .char() because base64 should be ASCII
    let mut bytes: Vec<u8> = base64.bytes()
                                   .filter(|x| *x != '\n' as u8)
                                   .collect();

    // Reverse padding at the end from = to 0
    let mut length = bytes.len();
    if length == 0 {
        return vec![];
    }
    assert_eq!(0, length % 4); // Length should be a multiple of 4

    if bytes[length - 1] == 61 {
        bytes[length - 1] = 0;
    }
    if bytes[length - 2] == 61 {
        bytes[length - 2] = 0;
    }

    //
    let get_byte_from_char = |character: char| -> u64 {
        let r = match character {
            x @ 'A'...'Z' => x as u64 + 0 - 65,
            x @ 'a'...'z' => x as u64 + 26 - 97,
            x @ '0'...'9' => x as u64 + 53 - 49,
            '+' => 62,
            '/' => 63,
            '\0' => 0,
            e @ _ => {
                println!("unreachable:: {}, {}", e, e as u8);
                unreachable!()
            }
        };
        r
    };

    // Operate on the bytes 4 at a time
    let mut result: Vec<u8> = vec![];
    let mut i = 0;
    while i < bytes.len() {
        let mut chunk: u64 = 0;
        chunk += get_byte_from_char(bytes[i + 0 as usize] as char) << 18;
        chunk += get_byte_from_char(bytes[i + 1 as usize] as char) << 12;
        chunk += get_byte_from_char(bytes[i + 2 as usize] as char) << 06;
        chunk += get_byte_from_char(bytes[i + 3 as usize] as char) << 00;

        result.push((chunk >> 16 & 255) as u8);
        result.push((chunk >> 08 & 255) as u8);
        result.push((chunk >> 00 & 255) as u8);

        i += 4;
    }

    // Truncate the last two characters if they're not useful
    length = result.len();
    if result[length - 1] == 0 {
        result.truncate(length - 1);
    }
    length = result.len();
    if result[length - 1] == 0 {
        result.truncate(length - 1);
    }

    result
}

pub fn hex_to_base64(hex: &str) -> String {
    let bytes = hex_to_bytes(hex);
    bytes_to_base64(&bytes)
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_to_bytes_i_empty() {
        let x: Vec<u8> = vec![];
        assert_eq!(x, hex_to_bytes(""));
    }

    #[test]
    fn hex_to_bytes_i_simple() {
        let x: Vec<u8> = vec![73, 74];
        assert_eq!(x, hex_to_bytes("494a"));
    }

    #[test]
    fn hex_to_bytes_i_simple_uppercase() {
        let x: Vec<u8> = vec![73, 74];
        assert_eq!(x, hex_to_bytes("494A"));
    }

    #[test]
    fn bytes_to_readable_text_example() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let bytes = hex_to_bytes(hex);
        let expected = "I'm killing your brain like a poisonous mushroom";
        assert_eq!(expected, bytes_to_readable_text(&bytes));
    }

    #[test]
    fn bytes_to_readable_symmetricality() {
        let text = "I'm killing your brain like a poisonous mushroom";
        let bytes = readable_text_to_bytes(text);
        let text_out = bytes_to_readable_text(&bytes);

        assert_eq!(text_out, text);
    }

    #[test]
    fn bytes_to_hex_i_empty() {
        assert_eq!("".to_string(), bytes_to_hex(&vec![]));
    }

    #[test]
    fn bytes_to_hex_i_simple() {
        let a: Vec<u8> = vec![73, 74];
        assert_eq!("494a".to_string(), bytes_to_hex(&a));
    }

    #[test]
    fn bytes_and_hex_symmetricality() {
        let hex = "45ab";
        assert_eq!(hex, bytes_to_hex(&hex_to_bytes(hex)));
    }

    #[test]
    fn bytes_to_base64_i_empty() {
        let a: Vec<u8> = vec![];
        assert_eq!("".to_string(), bytes_to_base64(&a));
    }

    #[test]
    fn bytes_to_base64_i_simple() {
        let a: Vec<u8> = vec![97];
        let actual = bytes_to_base64(&a);
        assert_eq!("YQ==".to_string(), actual);
    }

    #[test]
    fn bytes_to_base64_i_fits() {
        let a: Vec<u8> = vec![72, 101, 108, 108, 111, 33];
        let actual = bytes_to_base64(&a);
        assert_eq!("SGVsbG8h".to_string(), actual);
    }

    #[test]
    fn bytes_to_base64_i_padded() {
        let a: Vec<u8> = vec![72, 101, 108, 108, 111];
        assert_eq!("SGVsbG8=".to_string(), bytes_to_base64(&a));
    }

    #[test]
    fn bytes_to_base64_i_double_padded() {
        let a: Vec<u8> = vec![72, 101, 108, 108];
        assert_eq!("SGVsbA==".to_string(), bytes_to_base64(&a));
    }

    #[test]
    fn base64_to_bytes_empty() {
        let a: Vec<u8> = vec![];
        assert_eq!(a, base64_to_bytes(""));
    }

    #[test]
    fn base64_to_bytes_simple() {
        let a: Vec<u8> = vec![97];
        assert_eq!(a, base64_to_bytes("YQ=="));
    }

    #[test]
    fn base64_to_bytes_i_fits() {
        let a: Vec<u8> = vec![72, 101, 108, 108, 111, 33];
        assert_eq!(a, base64_to_bytes("SGVsbG8h"));
    }

    #[test]
    fn base64_to_bytes_i_with_new_lines() {
        let a: Vec<u8> = vec![72, 101, 108, 108, 111, 33];
        assert_eq!(a, base64_to_bytes("SG\nVsbG8\nh"));
    }

    #[test]
    fn bytes_and_base64_symmetricality_1() {
        let base64_1 = "aGVsbG8=";
        assert_eq!(base64_1, bytes_to_base64(&base64_to_bytes(base64_1)));
    }

    #[test]
    fn bytes_and_base64_symmetricality_2() {
        let base64_2 = "";
        assert_eq!(base64_2, bytes_to_base64(&base64_to_bytes(base64_2)));
    }

    #[test]
    fn bytes_and_base64_symmetricality_3() {
        let base64_3 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(base64_3, bytes_to_base64(&base64_to_bytes(base64_3)));
    }

    #[test]
    fn hex_to_base64_i_standard() {
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let actual: String = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

        assert_eq!(expected, actual);
    }
}
