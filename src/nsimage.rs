use coregraphicsr::{CGSize, CGImage};
use objr::bindings::*;
use foundationr::{NSData};
objc_class! {
    pub struct NSImage {
        @class(NSImage)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("initWithCGImage:size:")
        @selector("lockFocus")
        @selector("unlockFocus")
        @selector("TIFFRepresentation")
        @selector("CGImageForProposedRect:context:hints:")
        @selector("initWithSize:")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl NSImage {
    pub fn initWithSize(size: CGSize, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let alloc = Self::class().alloc(pool);
            Self::assume_nonnil(Self::perform(alloc, Sel::initWithSize_(), pool, (size,))).assume_retained()
        }
    }
    pub fn initWithCGImageSize(image: &mut CGImage, size: CGSize, pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
            let alloc = Self::class().alloc(pool);
            Self::assume_nonnil(Self::perform_autorelease_to_retain(alloc, Sel::initWithCGImage_size(), pool, (image, size))).assume_retained().assume_mut()
        }
    }
    pub fn lockFocus(&mut self,pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::lockFocus(), pool, ())
        }
    }
    pub fn unlockFocus(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::unlockFocus(), pool, ())
        }
    }
    pub fn TIFFRepresentation(&mut self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSData>> {
        unsafe {
            let raw: *const NSData = Self::perform_autorelease_to_retain(self, Sel::TIFFRepresentation(), pool, ());
            NSData::nullable(raw).assume_retained()
        }
    }
    #[cfg(feature="corefoundationr")]
    pub fn CGImageForProposedRectContextHints(&mut self, rect: Option<&foundationr::NSRect>, context: Option<&crate::NSGraphicsContext>, hints: Option<&foundationr::NSDictionary<crate::NSImageHintKey,NSObject>>, pool: &ActiveAutoreleasePool) -> Option<core_foundationr::StrongCell<CGImage>> {
        unsafe {
            let raw: *const std::os::raw::c_void = Self::perform_primitive(self, Sel::CGImageForProposedRect_context_hints(), pool, (rect.assume_nonmut_perform(), context.assume_nonmut_perform(), hints.assume_nonmut_perform()));

            if raw.is_null() {
                None
            }
            else {
                Some(core_foundationr::StrongCell::retain_assuming_nonnull(raw as *const CGImage))
            }
        }
    }
}

#[cfg(test)] mod tests {
    use coregraphicsr::{CGColorRenderingIntent, CGColorSpace, CGImage, CGImageAlphaInfo, CGSize, Name, SliceProvider};
    use objr::bindings::autoreleasepool;
    use crate::NSImage;

    #[test] fn smoke() {
        autoreleasepool(|pool| {
            let mut image = [0u8; 4];
            let mut provider = SliceProvider::new(&mut image).unwrap();
            let mut image = unsafe{CGImage::create(1, 1, 8, 32, 4, &CGColorSpace::with_name(Name::generic_rgb()), CGImageAlphaInfo::NONE, provider.as_provider_mut(), None, false, CGColorRenderingIntent::DEFAULT)}.unwrap();
            let mut image = NSImage::initWithCGImageSize(&mut image, CGSize{width: 1.0, height: 1.0}, pool);
            image.lockFocus(pool);
            image.unlockFocus(pool);
            let _ = image.TIFFRepresentation(pool).unwrap();

            #[cfg(feature="corefoundationr")]
            {
                let _ = image.CGImageForProposedRectContextHints(None,None,None,pool).unwrap();
            }

        });
    }
    #[test] fn desginated_init() {
        autoreleasepool( |pool| {
            let _ = NSImage::initWithSize(CGSize{width: 1.0, height: 1.0}, pool);
        })
    }
}