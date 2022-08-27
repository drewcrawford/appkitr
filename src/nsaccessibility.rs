use objr::bindings::*;
use foundationr::{NSArray,NSRect};
objc_selector_group! {
    trait Selectors {
        @selector("accessibilityChildren")
        @selector("setAccessibilityChildren:")
        @selector("accessibilityParent")
        @selector("accessibilityFrame")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
pub unsafe trait NSAccessibility: PerformablePointer + Sized + Arguable {
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
    fn accessibilityFrame(&self, pool: &ActiveAutoreleasePool) -> NSRect {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::accessibilityFrame(), pool, ())
        }
    }

}