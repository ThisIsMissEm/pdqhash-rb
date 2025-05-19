use hex;
use image::io::Reader as ImageReader;
use magnus::{function, prelude::*, Error, Ruby};
use pdqhash;

/// Returns the hex-encoded PDQ hash for the image at `filepath`.
///
/// Returns a wrapped Ruby IO exception if `filepath` could not be opened.
///
/// Returns a wrapped Ruby Encoding exception if `filepath` could not be decoded.
fn hash(ruby: &Ruby, filepath: String) -> Result<String, Error> {
    let reader = ImageReader::open(filepath)
        .map_err(|e| Error::new(ruby.exception_io_error(), format!("open: {}", e)))?;

    let dynamic_image = reader
        .decode()
        .map_err(|e| Error::new(ruby.exception_encoding_error(), format!("decode: {}", e)))?;

    Ok(hex::encode(
        pdqhash::generate_pdq_full_size(&dynamic_image).0,
    ))
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Pdqhash")?;
    module.define_singleton_method("hash", function!(hash, 1))?;
    Ok(())
}
