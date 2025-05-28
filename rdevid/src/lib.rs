#[cfg(not(feature = "wasm"))]
mod native;
#[cfg(feature = "wasm")]
mod wasm;

use sha2::{Digest, Sha256};

pub fn device_info() -> String {
    #[cfg(feature = "wasm")]
    return wasm::get_device_info();
    #[cfg(not(feature = "wasm"))]
    return native::get_device_info();
}

pub fn device_id() -> String {
    let info = device_info();
    let hash = Sha256::digest(info.as_bytes());
    return hex::encode(hash);
}
