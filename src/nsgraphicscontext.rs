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
}

#[test] fn smoke() {
    autoreleasepool(|pool| {
       let context = unsafe{coregraphicsr::CGContext::createBitmap(std::ptr::null_mut(), 100, 100, 8, 400, &coregraphicsr::CGColorSpace::with_name(coregraphicsr::Name::generic_rgb()), coregraphicsr::CGImageAlphaInfo::PREMULTIPLIED_LAST.0).unwrap()};
         let _ = NSGraphicsContext::withCGContext(&context, false, pool);
    });
}