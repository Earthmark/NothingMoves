#[cfg(target_arch = "wasm32")]
pub fn is_wasm() -> bool {
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub fn is_wasm() -> bool {
    false
}