use std::os::raw::c_ushort;
use objr::bindings::*;
use crate::NSWindow;
use foundationr::NSPoint;
objc_class! {
    pub struct NSEvent {
        @class(NSEvent)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("keyCode")
        @selector("window")
        @selector("locationInWindow")
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
    pub fn window(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSWindow>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::window(), pool, ());
            NSWindow::nullable(raw).assume_retained()
        }
    }
    pub fn locationInWindow(&self, pool: &ActiveAutoreleasePool) -> NSPoint {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::locationInWindow(), pool, ())
        }
    }

}