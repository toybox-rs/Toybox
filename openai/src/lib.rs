#![crate_type = "dylib"]

#[no_mangle]
pub extern fn get_state() -> String {
    String::from("It worked!")
}