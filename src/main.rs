#![cfg_attr(not(test), no_main)]
use objc_rust::TestObject;

#[cfg_attr(not(test), no_mangle)]
pub fn main(_argc: i32, _argv: *const *const u8) -> isize {
    let o = TestObject::new();
    let code = o.get_value(100);
    o.release();
    code as _
}
