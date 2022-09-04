use coregraphicsr::{CGSize,CGImage};
use objr::bindings::*;
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
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl NSImage {
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
        });
    }
}