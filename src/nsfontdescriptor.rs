use objr::bindings::*;
#[cfg(feature="coretext")]
use coretextr::CTFontDescriptor;
use coregraphicsr::{CGFloat};

#[cfg(feature="bridge")]
use core_foundationr::CTFontDescriptor;
objc_class! {
    pub struct NSFontDescriptor {
        @class(NSFontDescriptor)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("fontDescriptorWithName:size:")
    }
    impl Selectors for Sel {}
}

#[allow(non_snake_case)]
impl NSFontDescriptor {
    #[cfg(feature="coretext")]
    pub fn bridge(ctfontdescriptor: coretextr::StrongCell<CTFontDescriptor>) -> StrongCell<Self> {
        unsafe {
            let as_ref: &CTFontDescriptor = &*ctfontdescriptor;
            let as_ptr: *const CTFontDescriptor = as_ref as *const CTFontDescriptor;
            let pun = as_ptr as *const NSFontDescriptor;
            core::mem::forget(ctfontdescriptor);
            Self::assume_nonnil(pun).assume_retained()
        }
    }
    ///`+ (NSFontDescriptor *)fontDescriptorWithName:(NSString *)fontName size:(CGFloat)size;`
    pub fn withNameSize(name: &NSString, size: CGFloat, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let raw: *const Self = Class::<Self>::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::fontDescriptorWithName_size(), pool,(name.assume_nonmut_perform(), size));
            Self::assume_nonnil(raw).assume_retained()
        }
    }
}

#[test] fn test() {
    autoreleasepool(|pool| {
        let nsstring = NSString::with_str_copy("Helvetica", pool);
        let font = NSFontDescriptor::withNameSize(&nsstring, 12.0, pool);
        println!("{}", font);
    })

}