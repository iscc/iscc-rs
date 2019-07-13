//! Content Normalization
use unic_ucd_category::GeneralCategory;
use unicode_normalization::UnicodeNormalization;

/// We define a text normalization function that is specific to our application.
/// It takes text and an optional boolean `keep_ws` parameter as an input and
/// returns *normalized* Unicode text for further algorithmic processing. The
/// `text_normalize` function performs the following operations in the given
/// order while each step works with the results of the previous operation:
///
/// 1. Decode to native Unicode if text is a byte string
/// 2. Remove leading and trailing whitespace
/// 3. Transform text to lower case
/// 4. Decompose the lower case text by applying [Unicode Normalization Form D
///    (NFD)](http://www.unicode.org/reports/tr15/#Norm_Forms).
/// 5. Filter out all characters that are neither alphanumeric, whitespace
///    or of unicode category symbol.
/// 6. Keep or remove whitespace depending on `keep_ws` parameter
/// 7. Re-Combine the text by applying `Unicode Normalization Form KC (NFKC)`.
pub fn text_normalize(text: &str, keep_ws: bool) -> String {
    let text_filtered: String = text
        .trim()
        .to_lowercase()
        .nfd()
        .collect::<String>()
        .chars()
        .filter(|&ch| {
            ch.is_alphanumeric() || ch.is_whitespace() || GeneralCategory::of(ch).is_symbol()
        })
        .nfkc()
        .collect();
    let ws_char = if keep_ws { " " } else { "" };
    text_filtered
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(ws_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_normalize() {
        assert_eq!(text_normalize(" ", false), "");
        assert_eq!(text_normalize("Hello\nWorld", true), "hello world");
        assert_eq!(text_normalize("  Hello  World ? ", true), "hello world");

        let text = "  Iñtërnâtiôn\nàlizætiøn☃💩 –  is a tric\t ky\u{00A0} thing!\r";
        let mut normalized = text_normalize(text, false);
        assert_eq!(normalized, "internationalizætiøn☃💩isatrickything");

        normalized = text_normalize(text, true);
        assert_eq!(
            normalized,
            "internation alizætiøn☃💩 is a tric ky thing"
        );
    }
}
