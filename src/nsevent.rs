use std::os::raw::c_ushort;
use objr::bindings::*;
objc_class! {
    pub struct NSEvent {
        @class(NSEvent)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("keyCode")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl NSEvent {
    pub fn keyCode(&self, pool: &ActiveAutoreleasePool) -> c_ushort {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::keyCode(), pool, ())
        }
    }
}