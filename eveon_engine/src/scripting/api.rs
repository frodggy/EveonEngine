use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;

#[derive(WrapperApi)]
pub struct Api<'a> {
    rust_str: &'a &'static str,
    
}
