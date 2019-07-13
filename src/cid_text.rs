//! Content-ID-Text
use bit_vec::BitVec;
use itertools::Itertools;

use crate::base58;
use crate::hashes::minimum_hash;
use crate::hashes::sliding_window;
use crate::normalization::text_normalize;

const WINDOW_SIZE_CID_T: usize = 13;

// Component Headers
const HEAD_CID_T: u8 = 0x10;
const HEAD_CID_T_PCF: u8 = 0x11;

/// Generates the id from extracted and normalized plain-text
///
/// * `partial` - The last bit of the header byte of the Content-ID is the
///   "Partial Content Flag". It designates if the Content-ID applies to the
///   full content or just some part of it.
pub fn content_id_text(text: &str, partial: bool) -> String {
    let text = text_normalize(text, false);

    let n_grams: Vec<String> = sliding_window(&text, WINDOW_SIZE_CID_T)
        .iter()
        .map(|w| w.chars().intersperse('\u{0020}').collect())
        .collect();

    let features: Vec<u32> = n_grams
        .iter()
        .map(|n| xxhash2::hash32(n.as_bytes(), 0))
        .collect();

    let minhash = minimum_hash(features, 64);

    let lsb: BitVec = minhash.iter().map(|x| (x & 1) == 1).collect();
    let lsb_bytes = lsb.to_bytes();

    let mut content_id_digest = if partial {
        vec![HEAD_CID_T_PCF]
    } else {
        vec![HEAD_CID_T]
    };
    content_id_digest.extend(&lsb_bytes);

    base58::encode(&content_id_digest)
}
