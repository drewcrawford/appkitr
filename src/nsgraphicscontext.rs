use core_foundationr::CFTypeBehavior;
use objr::bindings::*;
objc_class! {
    pub struct NSGraphicsContext {
        @class(NSGraphicsContext)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("graphicsContextWithCGContext:flipped:")
        @selector("currentContext")
        @selector("setCurrentContext:")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl NSGraphicsContext {
    pub fn withCGContext(context: &coregraphicsr::CGContext, flipped: bool, pool: &ActiveAutoreleasePool) ->StrongCell<Self> {
        unsafe {
            let sel = Sel::graphicsContextWithCGContext_flipped();
            let raw = Class::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), sel, pool, (context.as_ptr(), flipped));
            Self::assume_nonnil(raw).assume_retained()
        }
    }
    pub fn currentContext(pool: &ActiveAutoreleasePool) -> Option<StrongCell<Self>> {
        unsafe {
            let sel = Sel::currentContext();
            let raw = Class::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), sel, pool, ());
            Self::nullable(raw).assume_retained()
        }
    }
    pub fn setCurrentContext(context: Option<&Self>, pool: &ActiveAutoreleasePool) {
        unsafe {
            let sel = Sel::setCurrentContext_();
            Class::<Self>::perform_primitive(Self::class().assume_nonmut_perform(), sel, pool, (context.assume_nonmut_perform(),))
        }
    }
}

#[test] fn smoke() {
    autoreleasepool(|pool| {
       let context = unsafe{coregraphicsr::CGContext::createBitmap(std::ptr::null_mut(), 100, 100, 8, 400, &coregraphicsr::CGColorSpace::with_name(coregraphicsr::Name::generic_rgb()), coregraphicsr::CGImageAlphaInfo::PREMULTIPLIED_LAST.0).unwrap()};
        let context = NSGraphicsContext::withCGContext(&context, false, pool);
        NSGraphicsContext::setCurrentContext(Some(&context), pool);
        let current = NSGraphicsContext::currentContext(pool).unwrap();
        let context_ref: &NSGraphicsContext = &context;
        let current_ref: &NSGraphicsContext = &current;
        assert_eq!(context_ref as *const NSGraphicsContext, current_ref as *const NSGraphicsContext);
    });
}