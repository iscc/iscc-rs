//! Instance-ID
use std::fs::File;
use std::io::Read;

use hex;
use sha2::{Digest, Sha256};

use crate::base58;

// Component Headers
const HEAD_IID: u8 = 0x30;

/// The Instance-ID is built from the raw data of the media object to be
/// identified and serves as checksum for the media object. The raw data of the
/// media object is split into 64-kB data-chunks. Then we build a hash-tree from
/// those chunks and use the truncated tophash (merkle root) as component body
/// of the Instance-ID.
pub fn instance_id(data_path: &str) -> std::io::Result<(String, String)> {
    let mut data = File::open(data_path)?;
    let mut leaf_node_digests: Vec<Vec<u8>> = Vec::new();
    let mut chunk = [0; 64000];
    loop {
        let n = data.read(&mut chunk).unwrap();
        if n == 0 {
            break;
        }
        let zero = &[0];
        let hash = sha256d(&[zero, &chunk[..n]].concat());
        leaf_node_digests.push(hash);
    }

    let top_hash_digest = top_hash(&leaf_node_digests);

    let mut instance_id_digest = vec![HEAD_IID];
    instance_id_digest.extend(&top_hash_digest[..8]);

    let code = base58::encode(&instance_id_digest);
    let hex_hash = hex::encode(top_hash_digest);

    Ok((code, hex_hash))
}

pub fn top_hash(hashes: &[Vec<u8>]) -> Vec<u8> {
    let size = hashes.len();
    if size == 1 {
        return hashes[0].clone();
    }

    let mut pairwise_hashed: Vec<Vec<u8>> = Vec::new();
    for hash_pair in hashes.chunks(2) {
        let a = &hash_pair[0];
        let b = hash_pair.get(1).unwrap_or(a);
        pairwise_hashed.push(hash_inner_nodes(a, b));
    }

    top_hash(&pairwise_hashed)
}

fn hash_inner_nodes(a: &[u8], b: &[u8]) -> Vec<u8> {
    let one = &[1];
    sha256d(&[one, a, b].concat())
}

pub fn sha256d(data: &[u8]) -> Vec<u8> {
    Sha256::digest(&Sha256::digest(data)).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_hash() {
        let input: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected: Vec<u8> = vec![
            0x21, 0x09, 0x0f, 0x23, 0xeb, 0xe2, 0x2e, 0xfc, 0x65, 0xe3, 0xe7, 0xd8, 0x01, 0x79,
            0x69, 0xe5, 0xba, 0x31, 0x71, 0xb6, 0x48, 0xd6, 0x1c, 0x36, 0x3a, 0x01, 0x39, 0xc9,
            0x37, 0x6f, 0x2e, 0xd8,
        ];
        assert_eq!(top_hash(&input), expected)
    }
}
