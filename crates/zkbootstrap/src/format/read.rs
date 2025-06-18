use std::io::Read;
use serde::de::DeserializeOwned;

use flate2::read::GzDecoder;

pub fn read<T: DeserializeOwned, R: Read>(src: R) -> Result<T, rmp_serde::decode::Error> {
            let mut dec = GzDecoder::new(src);
            rmp_serde::decode::from_read(&mut dec)
}
