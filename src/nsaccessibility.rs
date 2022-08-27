use objr::bindings::*;
use foundationr::{NSArray,NSRect};
use crate::{NSAccessibilityRole,NSAccessibilityNotificationName};
objc_selector_group! {
    trait Selectors {
        @selector("accessibilityChildren")
        @selector("setAccessibilityChildren:")
        @selector("accessibilityParent")
        @selector("setAccessibilityParent:")
        @selector("accessibilityFrame")
        @selector("accessibilityRole")
        @selector("setAccessibilityRole:")
        @selector("setAccessibilityFrame:")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
pub unsafe trait NSAccessibility: ObjcInstance + Sized {
    fn accessibilityChildren(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSArray<NSObject>>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::accessibilityChildren(), pool, ());
            NSArray::nullable(raw).assume_retained()
        }
    }
    fn setAccessibilityChildren(&mut self, children: Option<&NSArray<NSObject>>, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setAccessibilityChildren_(), pool, (children.assume_nonmut_perform(),))
        }
    }
    fn accessibilityParent(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSObject>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::accessibilityParent(), pool, ());
            NSObject::nullable(raw).assume_retained()
        }
    }
    fn setAccessibilityParent(&mut self, parent: &NSObject, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setAccessibilityParent_(), pool, (parent.assume_nonmut_perform(),))
        }
    }
    fn accessibilityFrame(&self, pool: &ActiveAutoreleasePool) -> NSRect {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::accessibilityFrame(), pool, ())
        }
    }
    fn accessibilityRole(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSAccessibilityRole>> {
        unsafe {
            let raw = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::accessibilityRole(), pool, ());
            NSAccessibilityRole::nullable(raw).assume_retained()
        }
    }
    fn setAccessibilityRole(&mut self, role: Option<&NSAccessibilityRole>, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setAccessibilityRole_(), pool, (role.assume_nonmut_perform(),))
        }
    }
    unsafe fn setAccessibilityFrame(&mut self, frame: NSRect, pool: &ActiveAutoreleasePool) {
        Self::perform_primitive(self, Sel::setAccessibilityFrame_(), pool, (frame,))
    }

}

objc_instance! {
    /**
    An erased type that can be used to represent any NSAccessibility object.
    */
    pub struct AnyNSAccessibility;
}

unsafe impl NSAccessibility for AnyNSAccessibility {}

extern "C" {
    fn NSAccessibilityPostNotification(element: &NSObject, notification: NSAccessibilityNotificationName);
}

pub fn post_notification<E: NSAccessibility>(element: &E, notification: NSAccessibilityNotificationName) {
    unsafe {
        NSAccessibilityPostNotification(element.as_nsobject(), notification);
    }
}