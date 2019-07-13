#[macro_use]
extern crate doc_comment;

doctest!("../README.md");

pub use crate::cid_image::content_id_image;
pub use crate::cid_mixed::content_id_mixed;
pub use crate::cid_text::content_id_text;
pub use crate::did::data_id;
pub use crate::iid::instance_id;
pub use crate::mid::meta_id;

pub mod base58;
pub mod cid_image;
pub mod cid_mixed;
pub mod cid_text;
#[doc(hidden)]
pub mod constants;
pub mod did;
pub mod hashes;
pub mod iid;
pub mod mid;
pub mod normalization;
