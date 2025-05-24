use std::io::Write;
use serde::Serialize;

use flate2::{Compression, write::GzEncoder};

pub fn write<T: Serialize, W: Write>(data: &T, dst: W) -> Result<(), rmp_serde::encode::Error> {
    let mut enc = GzEncoder::new(dst, Compression::default());
    rmp_serde::encode::write(&mut enc, data)
}
