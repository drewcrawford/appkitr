use coregraphicsr::CGFloat;
use objr::bindings::*;
use foundationr::NSInteger;
objc_enum! {
    pub struct NSStringDrawingOptions<NSInteger>;
    impl NSStringDrawingOptions {
        UsesLineFragmentOrigin = 1 << 0,
        UsesFontLeading = 1 << 1,
        UsesDeviceMetrics = 1 << 3,
        TruncatesLastVisibleLine  = 1 << 5
    }
}
objc_class! {
    pub struct NSStringDrawingContext {
        @class(NSStringDrawingContext)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("minimumScaleFactor")
        @selector("setMinimumScaleFactor:")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl NSStringDrawingContext {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
            let raw: *const Self = Class::<Self>::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::alloc(), pool,());
            Self::assume_nonnil(raw).assume_retained().assume_mut()
        }
    }
    pub fn minimumScaleFactor(&self, pool: &ActiveAutoreleasePool) -> CGFloat {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::minimumScaleFactor(), pool,())
        }
    }
    pub fn setMinimumScaleFactor(&mut self, minimumScaleFactor: CGFloat, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setMinimumScaleFactor_(),pool, (minimumScaleFactor,))
        }
    }

}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        let mut context = NSStringDrawingContext::new(pool);
        context.setMinimumScaleFactor(0.5,pool);
        assert_eq!(context.minimumScaleFactor(pool), 0.5);
        println!("{}", context);
    })
}
