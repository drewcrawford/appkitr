use objr::bindings::*;
use coregraphicsr::CGFloat;
use crate::NSDeviceDescriptionKey;
use foundationr::NSDictionary;
objc_class! {
    pub struct NSScreen {
        @class(NSScreen)
    }
}
objc_selector_group! {
    trait NSScreenSelectors {
        @selector("mainScreen")
        @selector("backingScaleFactor")
        @selector("deviceDescription")
    }
    impl NSScreenSelectors for Sel {}
}

#[allow(non_snake_case)]
impl NSScreen {
    pub fn mainScreen(pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSScreen>> {
        unsafe {
            let ptr = Class::perform_autorelease_to_retain(NSScreen::class().assume_nonmut_perform(), Sel::mainScreen(), pool, ());
            Self::nullable(ptr).assume_retained()
        }
    }
    pub fn backingScaleFactor(&self, pool: &ActiveAutoreleasePool) -> CGFloat {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::backingScaleFactor(), pool, ())
        }
    }
    pub fn deviceDescription(&self, pool: &ActiveAutoreleasePool) -> StrongCell<NSDictionary<NSDeviceDescriptionKey, NSObject>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::deviceDescription(), pool, ());
            NSDictionary::assume_nonnil(ptr).assume_retained()
        }
    }
}
#[test] fn smoke_test() {
    autoreleasepool(|pool |{
        let m = NSScreen::mainScreen(pool).unwrap();
        let factor = m.backingScaleFactor(pool);
        println!("{}",factor);

        m.deviceDescription(pool).objectForKey(NSDeviceDescriptionKey::NSDeviceIsScreen(), pool);
    })
}