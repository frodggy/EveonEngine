#[no_mangle]
pub static name: &str = "Player";

#[no_mangle]
pub extern "C" fn onUpdate() {
    println!("something else");
}