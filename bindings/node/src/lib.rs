use node_bindgen::derive::node_bindgen;

#[node_bindgen]
pub fn device_id() -> String {
    rdevid::device_id()
}

#[node_bindgen]
pub fn device_info() -> String {
    rdevid::device_info()
}
