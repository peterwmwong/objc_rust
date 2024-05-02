#![cfg_attr(not(test), no_main)]
use test_objc_provider::TestObject;

#[cfg_attr(not(test), no_mangle)]
pub fn main(_argc: i32, _argv: *const *const u8) -> isize {
    let o = TestObject::new();
    let code = o.get_value(2);
    o.release();
    code as _
}
