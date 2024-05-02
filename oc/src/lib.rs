#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ObjCObject(core::ptr::NonNull<core::ffi::c_void>);
impl ObjCObject {
    #[inline]
    pub fn as_ptr(&self) -> *const core::ffi::c_void {
        self.0.as_ptr()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ObjCClass(core::ptr::NonNull<core::ffi::c_void>);
impl ObjCClass {
    #[inline]
    pub fn as_ptr(&self) -> *const core::ffi::c_void {
        self.0.as_ptr()
    }
}

#[macro_export]
macro_rules! objc_bind_class {
    ($objc_class_name:ident) => {
        #[repr(C)]
        pub struct $objc_class_name($crate::ObjCObject);
        impl $objc_class_name {
            #[inline(always)]
            pub fn class() -> $crate::ObjCClass {
                extern "C" {
                    #[link_name = concat!("OBJC_CLASS_$_", stringify!($objc_class_name))]
                    static OBJC_CLASS: [u8; 0];
                }
                unsafe { core::mem::transmute(OBJC_CLASS.as_ptr()) }
            }

            #[inline]
            pub fn as_ptr(&self) -> *const core::ffi::c_void {
                self.0.as_ptr()
            }
        }
    };
}

#[macro_export]
macro_rules! msg_send {
    ($rtnTy:ty, $obj:expr, $name:ident) => {{
        extern "C" {
            #[link_name = concat!("objc_msgSend$",stringify!($name))]
            fn msg_send_fn(_: *const ::core::ffi::c_void) -> $rtnTy;
        }
        let objc_ptr: *const core::ffi::c_void = $obj.as_ptr();
        msg_send_fn(objc_ptr)
    }};
    ($rtnTy:ty, $obj:expr, $($name:ident : $ty:ty = $arg:expr)+) => {{
        extern "C" {
            #[link_name = concat!("objc_msgSend$",$(stringify!($name), ':'),+)]
            fn msg_send_fn(_: *const ::core::ffi::c_void, unused: ::core::mem::MaybeUninit<::core::ffi::c_long> $(,_ : $ty)+) -> $rtnTy;
        }
        let objc_ptr: *const core::ffi::c_void = $obj.as_ptr();
        msg_send_fn(objc_ptr, ::core::mem::MaybeUninit::uninit() $(,$arg)+)
    }};
}
