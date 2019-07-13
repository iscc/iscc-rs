//! Meta-ID
use crate::base58::encode;
use crate::hashes::{similarity_hash, sliding_window};
use crate::normalization::text_normalize;

const WINDOW_SIZE_MID: usize = 4;
const HEAD_MID: u8 = 0x00;
const INPUT_TRIM: usize = 128;

/// The Meta-ID component starts with a 1-byte header `00000000`. The first
/// nibble `0000` indicates that this is a Meta-ID component type. The second
/// nibble is reserved for future extended features of the Meta-ID.
///
/// The Meta-ID body is built from a 64-bit `similarity_hash` over 4-character
/// n-grams of the basic metadata of the content to be identified.  The basic
/// metadata supplied to the Meta-ID generating function is assumed to be UTF-8
/// encoded.
///
/// * `title` - The title of an intangible creation.
/// * `extra` - An optional short statement that distinguishes this intangible
///   creation from another one for the purpose of forced Meta-ID uniqueness.
pub fn meta_id(title: &str, extra: &str) -> (String, String, String) {
    let title_norm = text_normalize(title, true);
    let extra_norm = text_normalize(extra, true);

    let title_trimmed = text_trim(&title_norm);
    let extra_trimmed = text_trim(&extra_norm);

    let concat = format!("{} {}", title_trimmed, extra_trimmed);
    // trim in case `title` or `extra` is an empty string
    let concat = concat.trim();

    let n_grams = sliding_window(concat, WINDOW_SIZE_MID);

    let hash_digests: Vec<u64> = n_grams
        .iter()
        .map(|n| xxhash2::hash64(n.as_bytes(), 0))
        .collect();

    let simhash_digest = similarity_hash(hash_digests);

    let mut meta_id_digest = vec![HEAD_MID];
    meta_id_digest.extend(simhash_digest);

    let meta_id = encode(&meta_id_digest);
    (meta_id, title_trimmed, extra_trimmed)
}

/// Trim text such that its UTF-8 encoded byte representation does not exceed
/// 128-bytes each. Remove leading and trailing whitespace.
pub fn text_trim(text: &str) -> String {
    let input_trim = text.len().min(INPUT_TRIM);
    String::from_utf8_lossy(&text.as_bytes()[..input_trim])
        .replace("�", "")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_text() {
        let multibyte_2 = "ü".repeat(128);
        let trimmed = text_trim(&multibyte_2);
        assert_eq!(trimmed.chars().count(), 64);
        assert_eq!(trimmed.len(), 128);
        let multibyte_3 = "驩".repeat(128);
        let trimmed2 = text_trim(&multibyte_3);
        assert_eq!(trimmed2.chars().count(), 42);
        assert_eq!(trimmed2.len(), 126);
        let mixed = "Iñtërnâtiônàlizætiøn☃💩".repeat(6);
        let trimmed3 = text_trim(&mixed);
        assert_eq!(trimmed3.chars().count(), 85);
        assert_eq!(trimmed3.len(), 128);
    }
}
