/*!
Base58-ISCC

The ISCC uses a custom per-component data encoding similar to the
[zbase62](https://github.com/simplegeo/zbase62) encoding by [Zooko
Wilcox-O'Hearn](https://en.wikipedia.org/wiki/Zooko_Wilcox-O%27Hearn) but
with a 58-character symbol table.

`SYMBOLS = "C23456789rB1ZEFGTtYiAaVvMmHUPWXKDNbcdefghLjkSnopRqsJuQwxyz"`
 */
const SYMBOLS: [char; 58] = [
    'C', '2', '3', '4', '5', '6', '7', '8', '9', 'r', 'B', '1', 'Z', 'E', 'F', 'G', 'T', 't', 'Y',
    'i', 'A', 'a', 'V', 'v', 'M', 'm', 'H', 'U', 'P', 'W', 'X', 'K', 'D', 'N', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'L', 'j', 'k', 'S', 'n', 'o', 'p', 'R', 'q', 's', 'J', 'u', 'Q', 'w', 'x', 'y',
    'z',
];

/// The `encode` function accepts a 9-byte **ISCC Component Digest** and returns
/// the Base58-ISCC encoded alphanumeric string of 13 characters which we call
/// the **ISCC-Component Code**.
pub fn encode(digest: &[u8]) -> String {
    if digest.len() == 9 {
        let full_encode = format!(
            // Pad with C, base58 "zeros"
            "{:C>2}{:C>11}",
            encode(&digest[..1]),
            encode(&digest[1..])
        );
        return full_encode;
    }
    assert!(
        digest.len() == 1 || digest.len() == 8,
        "Digest must be 1, 8 or 9 bytes long"
    );
    let mut array: [u8; 16] = [0; 16];
    array[16 - digest.len()..].copy_from_slice(&digest);
    let mut num = u128::from_be_bytes(array);

    let mut chars: Vec<char> = Vec::new();
    while num > 0 {
        let rem = (num % 58) as usize;
        num /= 58;
        chars.push(SYMBOLS[rem]);
    }
    chars.into_iter().rev().collect::<String>()
}

/// The `decode` function accepts a 13-character **ISCC-Component Code** and
/// returns the corresponding 9-byte **ISCC-Component Digest**.
pub fn decode(code: &str) -> Vec<u8> {
    let n = code.len();
    if n == 13 {
        return decode(&code[..2])
            .into_iter()
            .chain(decode(&code[2..]).into_iter())
            .collect();
    }
    assert!(
        n == 2 || n == 11,
        "Code must be 2, 11 or 13 chars. Not {}",
        n
    );
    let base: usize = 58;
    let mut num: u64 = 0;

    for (i, chr) in code.chars().enumerate() {
        let power = n - (i + 1);
        num += (SYMBOLS.iter().position(|c| c == &chr).unwrap() * (base.pow(power as u32))) as u64;
    }
    if n == 2 {
        assert!(
            num < 256,
            "The first two characters encode the 1-byte component header and \
             have to be < 256. But '{}' is {}.",
            code,
            num,
        );
        let bytes: [u8; 1] = (num as u8).to_be_bytes();
        bytes.to_vec()
    } else {
        let bytes: [u8; 8] = num.to_be_bytes();
        bytes.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        // TODO: property based testing would be nice here
        let code = "5GcQF7sC3iY2i";
        assert_eq!(encode(&decode(code)), code);
    }
    #[test]
    fn test_encode() {
        let digest: Vec<u8> = vec![0xF7, 0xD3, 0xA5, 0xB2, 0x01, 0xDC, 0x92, 0xF7, 0xA7];
        let code = encode(&digest);
        assert_eq!(code, "5GcvF7s13LK2L");
    }
    #[test]
    fn test_decode() {
        let code = "5GcQF7sC3iY2i";
        let digest = decode(code);
        let expected_digest: Vec<u8> = vec![0xF7, 0xD6, 0xBD, 0x58, 0x7D, 0x22, 0xA7, 0xCB, 0x6D];
        assert_eq!(digest, expected_digest);
    }
    #[test]
    #[should_panic]
    fn test_decode_invalid_component_header() {
        // "1H" is 644, but the component header has to be < 256
        let code = "1H";
        decode(code);
    }

}
