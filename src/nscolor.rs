use coregraphicsr::CGColor;
use objr::bindings::*;
objc_class! {
    pub struct NSColor {
        @class(NSColor)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("colorWithCGColor:")
        @selector("CGColor")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl NSColor {
    pub fn colorWithCGColor(cgcolor: &CGColor, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSColor>> {
        unsafe {
            let ptr = Class::<Self>::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::colorWithCGColor_(), pool, (cgcolor.assume_nonmut_perform(),));
            Self::nullable(ptr).assume_retained()
        }
    }

    pub fn CGColor(&self, pool: &ActiveAutoreleasePool) -> StrongCell<CGColor> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::CGColor(), pool, ());
            CGColor::assume_nonnil(ptr).assume_retained()
        }
    }
}

#[test] fn test() {
    autoreleasepool(|pool| {
        let color = NSColor::colorWithCGColor(&CGColor::grey(1.0,1.0), &pool).unwrap();
        let _ = color.CGColor(&pool);
    })

}
