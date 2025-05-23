use std::io::Write;
use flate2::{
    write::GzEncoder, 
    Compression,
};

pub fn gzip_data(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}