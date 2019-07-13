//! Content-ID Image
use bit_vec::BitVec;
use image::{FilterType, ImageResult};

use crate::base58::encode;

const HEAD_CID_I: u8 = 0x12;
const HEAD_CID_I_PCF: u8 = 0x13;

/// Generates the id from normalized grayscale pixel data
///
/// * `partial` - The last bit of the header byte of the Content-ID is the
///   "Partial Content Flag". It designates if the Content-ID applies to the
///   full content or just some part of it.
pub fn content_id_image(img_path: &str, partial: bool) -> ImageResult<String> {
    let pixels = image_normalize(img_path)?;
    let hash_digest = image_hash(&pixels);

    let mut content_id_digest = if partial {
        vec![HEAD_CID_I_PCF]
    } else {
        vec![HEAD_CID_I]
    };
    content_id_digest.extend(&hash_digest);

    Ok(encode(&content_id_digest))
}

pub fn image_normalize(img_path: &str) -> ImageResult<Vec<Vec<u8>>> {
    let img = image::open(img_path)?;

    let img = img.grayscale();

    // TODO: Not the same as in pillow, see https://stackoverflow.com/a/23209568
    let img = img.resize_exact(32, 32, FilterType::Triangle);

    let two_dim_image = img
        .raw_pixels()
        .chunks(32)
        .map(|row| row.to_vec())
        .collect();
    Ok(two_dim_image)
}

pub fn image_hash(pixels: &[Vec<u8>]) -> Vec<u8> {
    let mut dct_row_lists: Vec<Vec<f64>> = Vec::new();
    for row in pixels.iter() {
        let mut row: Vec<f64> = row.iter().map(|&n| f64::from(n)).collect();
        dct(&mut row);
        dct_row_lists.push(row);
    }

    let mut dct_col_lists_t: Vec<Vec<f64>> = transpose(&dct_row_lists);
    for mut col in dct_col_lists_t.iter_mut() {
        dct(&mut col);
    }
    let dct_lists: Vec<Vec<f64>> = transpose(&dct_col_lists_t);

    // Extract upper left 8x8 corner
    let flat_list: Vec<f64> = dct_lists
        .into_iter()
        .take(8)
        .map(|l| l.into_iter().take(8))
        .flatten()
        .collect();

    let med = median(&flat_list);

    let bv: BitVec = flat_list.into_iter().map(|v| v > med).collect();
    bv.to_bytes()
}

fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut m_t = Vec::new();
    for i in 0..m.len() {
        let col: Vec<f64> = m.iter().map(|row| row[i]).collect();
        m_t.push(col)
    }
    m_t
}

fn median(xs: &[f64]) -> f64 {
    let mut xs = xs.to_owned();
    xs.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let n = xs.len();
    if n % 2 == 0 {
        (xs[n / 2] + xs[n / 2 - 1]) / 2.0
    } else {
        xs[n / 2]
    }
}

/// Computes the unscaled DCT type II on the specified array in place.
/// The array length must be a power of 2.
/// For the formula, see https://en.wikipedia.org/wiki/Discrete_cosine_transform#DCT-II .
///
/// Discrete cosine transform algorithm by Project Nayuki. (MIT License)
/// See: https://www.nayuki.io/page/fast-discrete-cosine-transform-algorithms
pub fn dct(vector: &mut [f64]) {
    // TODO: Try https://github.com/ejmahler/rust_dct
    let n: usize = vector.len();
    assert_eq!(n.count_ones(), 1, "Length must be power of 2");
    _transform_recursive(vector, &mut vec![0.0f64; n]);
}

fn _transform_recursive(vector: &mut [f64], temp: &mut [f64]) {
    // Algorithm by Byeong Gi Lee, 1984. For details, see:
    // See: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.118.3056&rep=rep1&type=pdf#page=34
    let len: usize = vector.len();
    if len == 1 {
        return;
    }
    let halflen: usize = len / 2;
    for i in 0..halflen {
        let x = vector[i];
        let y = vector[len - 1 - i];
        temp[i] = x + y;
        temp[i + halflen] =
            (x - y) / ((((i as f64) + 0.5) * std::f64::consts::PI / (len as f64)).cos() * 2.0);
    }
    _transform_recursive(&mut temp[0..halflen], vector);
    _transform_recursive(&mut temp[halflen..len], vector);
    for i in 0..halflen - 1 {
        vector[i * 2] = temp[i];
        vector[i * 2 + 1] = temp[i + halflen] + temp[i + halflen + 1];
    }
    vector[len - 2] = temp[halflen - 1];
    vector[len - 1] = temp[len - 1];
}
