use magnus::{function, prelude::*, Error, Ruby};
use pdqhash;
use image::io::Reader as ImageReader;
use hex;

fn hash(ruby: &Ruby, filepath: String) -> Result<String, Error> {
    let image_result = ImageReader::open(filepath);
    let image = match image_result {
        Ok(f) => f.decode(),
        Err(e) => return Err(Error::new(
            ruby.exception_io_error(),
            format!("Could not open image: {}", e)
        )),
    };
    let hash = pdqhash::generate_pdq_full_size(&image.unwrap()).0;
    Ok(hex::encode(hash))
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Pdqhash")?;
    module.define_singleton_method("hash", function!(hash, 1))?;
    Ok(())
}
