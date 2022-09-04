use std::ops::BitOr;
use coregraphicsr::{CGFloat, CGRect, CGSize};
use objr::bindings::*;
use foundationr::NSInteger;
use foundationr::NSAttributedString;

objc_enum! {
    pub struct NSStringDrawingOptions<NSInteger>;
    impl NSStringDrawingOptions {
        UsesLineFragmentOrigin = 1 << 0,
        UsesFontLeading = 1 << 1,
        UsesDeviceMetrics = 1 << 3,
        TruncatesLastVisibleLine  = 1 << 5
    }
}
impl BitOr for NSStringDrawingOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
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
        @selector("boundingRectWithSize:options:context:")
        @selector("drawWithRect:options:context:")
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

#[allow(non_snake_case)]
pub trait NSAttributedStringDrawing: Sized + PerformablePointer + Arguable {
    fn boundingRectWithSizeOptionsContext(&self, size: CGSize, options: NSStringDrawingOptions, context: &mut NSStringDrawingContext, pool: &ActiveAutoreleasePool) -> CGRect {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::boundingRectWithSize_options_context(), pool, (size, options.field(), context))
        }
    }
    fn drawWithRectOptionsContext(&self, rect: CGRect, options: NSStringDrawingOptions, context: &mut NSStringDrawingContext, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::drawWithRect_options_context(), pool, (rect, options.field(), context))
        }
    }
}
impl NSAttributedStringDrawing for NSAttributedString {

}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        let mut context = NSStringDrawingContext::new(pool);
        context.setMinimumScaleFactor(0.5,pool);
        assert_eq!(context.minimumScaleFactor(pool), 0.5);
        println!("{}", context);

        let s = NSAttributedString::withStringAttributes(objc_nsstring!("Hello world"), None, pool);
        let r = s.boundingRectWithSizeOptionsContext(CGSize{width: 100.0,height: 100.0}, NSStringDrawingOptions::UsesLineFragmentOrigin | NSStringDrawingOptions::TruncatesLastVisibleLine, &mut context, pool);
        s.drawWithRectOptionsContext(r, NSStringDrawingOptions::UsesLineFragmentOrigin | NSStringDrawingOptions::TruncatesLastVisibleLine, &mut context, pool);
        println!("{:?}", r);
    })
}
