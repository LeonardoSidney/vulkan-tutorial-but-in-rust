#[cfg(debug_assertions)]
pub fn debug_mode() -> bool {
    println!("Debugging enabled");
    true
}

#[cfg(not(debug_assertions))]
pub fn debug_mode() -> bool {
    false
}
