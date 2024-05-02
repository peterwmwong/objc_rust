mod objc_runtime;

objc_bind_class!(TestObject);
impl TestObject {
    #[inline]
    pub fn new() -> Self {
        unsafe { msg_send![TestObject, TestObject::class(), new] }
    }
    #[inline]
    pub fn get_value(&self, v: u32) -> u32 {
        unsafe { msg_send![u32, self, getValue:u32 = v] }
    }
    #[inline]
    pub fn release(self) {
        unsafe { msg_send![(), self, release] }
    }
}
