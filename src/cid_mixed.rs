//! Content-ID-Mixed

use crate::base58;
use crate::hashes::similarity_hash;

const HEAD_CID_M: u8 = 0x18;
const HEAD_CID_M_PCF: u8 = 0x19;

/// The Content-ID-Mixed aggregates multiple Content-IDs of the same or
/// different types. It may be used for digital media objects that embed
/// multiples types of media or for collections of contents of the same type.
///
/// * `partial` - The last bit of the header byte of the Content-ID is the
///   "Partial Content Flag". It designates if the Content-ID applies to the
///   full content or just some part of it.
pub fn content_id_mixed(cids: &[&str], partial: bool) -> String {
    let decoded: Vec<Vec<u8>> = cids.iter().map(|cid| base58::decode(cid)).collect();

    // Extract first 8-bytes
    let mut array = [0; 8];
    let mut truncated: Vec<u64> = Vec::new();
    for code in decoded {
        array.copy_from_slice(&code[..8]);
        truncated.push(u64::from_be_bytes(array));
    }

    let simhash_digest = similarity_hash(truncated);

    let mut content_id_mixed = if partial {
        vec![HEAD_CID_M_PCF]
    } else {
        vec![HEAD_CID_M]
    };
    content_id_mixed.extend(&simhash_digest);

    base58::encode(&content_id_mixed)
}
