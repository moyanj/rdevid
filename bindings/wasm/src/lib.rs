use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn device_id() -> String {
    rdevid::device_id()
}

#[wasm_bindgen]
pub fn device_info() -> String {
    rdevid::device_info()
}
