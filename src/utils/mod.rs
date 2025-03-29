#[cfg(debug_assertions)]
pub fn debug_mode() -> bool {
    true
}

#[cfg(not(debug_assertions))]
pub fn debug_mode() -> bool {
    false
}
