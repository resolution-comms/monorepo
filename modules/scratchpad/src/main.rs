use std::error::Error;

use base64::Engine;

fn main() -> Result<(), Box<dyn Error>> {
    let context = resolution_common::CryptoContext::new()?;
    println!(
        "{}",
        base64::prelude::BASE64_URL_SAFE.encode(context.as_public().as_bytes())
    );

    Ok(())
}
