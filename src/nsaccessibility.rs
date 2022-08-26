use objr::bindings::*;
use foundationr::NSArray;
objc_selector_group! {
    trait Selectors {
        @selector("accessibilityChildren")
        @selector("setAccessibilityChildren:")
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
}