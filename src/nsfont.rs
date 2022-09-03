use coregraphicsr::CGFloat;
use objr::bindings::*;
use crate::NSFontDescriptor;
objc_class! {
    pub struct NSFont {
        @class(NSFont)
    }
}

objc_selector_group! {
    trait Selectors {
        @selector("fontWithDescriptor:size:")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl NSFont {
    pub fn withDescriptorSize(descriptor: &NSFontDescriptor, size: CGFloat, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let raw: *const Self = Class::<Self>::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::fontWithDescriptor_size(), pool,(descriptor.assume_nonmut_perform(), size));
            Self::assume_nonnil(raw).assume_retained()
        }
    }
}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        let nsstring = NSString::with_str_copy("Helvetica", pool);
        let font = NSFontDescriptor::withNameSize(&nsstring, 12.0, pool);
        let font = NSFont::withDescriptorSize(&font, 12.0, pool);
        println!("{}", font);
    })
}