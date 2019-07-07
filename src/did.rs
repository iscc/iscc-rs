use std::fs::File;
use std::io::Read;

use bit_vec::BitVec;
use xxhash2;

use crate::base58::encode;
use crate::constants::CHUNKING_GEAR;
use crate::hashes::minimum_hash;

const GEAR1_NORM: usize = 40;
const GEAR1_MIN: usize = 20;
const GEAR1_MAX: usize = 640;
const GEAR1_MASK1: u64 = 0x0001_6118;
const GEAR1_MASK2: u64 = 0x0000_A0B1;
const GEAR2_NORM: usize = 4096;
const GEAR2_MIN: usize = 2048;
const GEAR2_MAX: usize = 65536;
const GEAR2_MASK1: u64 = 0x0003_5907_0353_0000;
const GEAR2_MASK2: u64 = 0x0000_D900_0353_0000;

// Component Header
const HEAD_DID: u8 = 0x20;

pub fn data_id(data_path: &str) -> std::io::Result<String> {
    let data = File::open(data_path)?;

    //  1. & 2. XxHash32 over CDC-Chunks
    let features: Vec<u32> = data_chunks(data)
        .map(|chunk| xxhash2::hash32(&chunk, 0))
        .collect();

    // 3. Apply minimum_hash
    let minhash = minimum_hash(features, 64);

    // 4. Collect least significant bits
    let lsb: BitVec = minhash.iter().map(|x| (x & 1) == 1).collect();

    // 5. Create 64-bit digests
    let lsb_bytes = lsb.to_bytes();

    // 6. Prepend the 1-byte header
    let mut data_id_digest = vec![HEAD_DID];
    data_id_digest.extend(&lsb_bytes);

    // 7. Encode and return
    Ok(encode(&data_id_digest))
}

struct Chunk {
    // TODO: Generalize with Reader trait
    // TODO: Maybe use BufReader
    data: File,
    counter: usize,
    section: Vec<u8>,
}

impl Chunk {
    fn new(mut data: File) -> Chunk {
        let mut buffer = [0; GEAR1_MAX];
        let n = data.read(&mut buffer).unwrap();
        let mut section: Vec<u8> = Vec::new();
        section.extend(&buffer[..n]);
        Chunk {
            data,
            counter: 0,
            section,
        }
    }
}

pub fn data_chunks(data: File) -> impl Iterator<Item = Vec<u8>> {
    Chunk::new(data)
}

pub fn chunk_length(
    data: &[u8],
    norm_size: usize,
    min_size: usize,
    max_size: usize,
    mask_1: u64,
    mask_2: u64,
) -> usize {
    let data_length = data.len();
    let mut i = min_size;

    if data_length <= min_size {
        return data_length;
    }

    let mut pattern: u64 = 0;
    let barrier_1 = norm_size.min(data_length);
    let barrier_2 = max_size.min(data_length);
    for (mask, barrier) in [(mask_1, barrier_1), (mask_2, barrier_2)].iter() {
        while i < *barrier {
            let gear = CHUNKING_GEAR[data[i] as usize];
            pattern = (pattern << 1).wrapping_add(gear);
            if (pattern & mask) == 0 {
                return i;
            }
            i += 1;
        }
    }
    i
}

impl Iterator for Chunk {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Vec<u8>> {
        let mut buffer = [0; GEAR2_MAX];
        let boundary: usize;

        let counter = self.counter;
        let mut section = self.section.clone();
        let mut data = &self.data;
        if counter < 100 {
            if section.len() < GEAR1_MAX {
                let n = data.read(&mut buffer).unwrap();
                section.extend(&buffer[..n]);
            }
            if section.is_empty() {
                return None;
            }
            boundary = chunk_length(
                &section,
                GEAR1_NORM,
                GEAR1_MIN,
                GEAR1_MAX,
                GEAR1_MASK1,
                GEAR1_MASK2,
            );
        } else {
            if section.len() < GEAR2_MAX {
                let n = data.read(&mut buffer).unwrap();
                section.extend(&buffer[..n]);
            }
            if section.is_empty() {
                return None;
            }
            boundary = chunk_length(
                &section,
                GEAR2_NORM,
                GEAR2_MIN,
                GEAR2_MAX,
                GEAR2_MASK1,
                GEAR2_MASK2,
            );
        }
        self.section = section[boundary..].to_vec();
        self.counter += 1;
        Some(section[..boundary].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_data_id() {
        assert_eq!(
            data_id("test_data/cat.jpg").unwrap(),
            "CDC7Lg4oHA8DC".to_string()
        );
        assert_eq!(
            data_id("test_data/cat.png").unwrap(),
            "CDCx1AzhDGcT7".to_string()
        );
        assert_eq!(
            data_id("test_data/cat.gif").unwrap(),
            "CDcLVF7es2AEP".to_string()
        );
    }
    #[test]
    fn test_data_chunks() {
        let f = File::open("test_data/lenna.jpg").expect("Unable to open file");
        let chunks1 = Vec::from_iter(data_chunks(f));
        assert_eq!(chunks1.len(), 112);
        assert_eq!(chunks1[0].len(), 38);
        assert_eq!(chunks1.last().unwrap().len(), 2840);
    }
}
