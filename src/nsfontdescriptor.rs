use objr::bindings::*;
#[cfg(feature="coretext")]
use coretextr::CTFontDescriptor;
use coregraphicsr::{CGFloat};
use foundationr::{NSCopying, NSDictionary};

objc_class_newtype! {
    pub struct NSFontDescriptorAttributeName: NSString;
}
impl NSCopying for NSFontDescriptorAttributeName {}
extern "C" {
    static NSFontFamilyAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontNameAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontFaceAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontSizeAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontVisibleNameAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontMatrixAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontVariationAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontCharacterSetAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontCascadeListAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontTraitsAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontFixedAdvanceAttribute: &'static NSFontDescriptorAttributeName;
    static NSFontFeatureSettingsAttribute: &'static NSFontDescriptorAttributeName;
}

impl NSFontDescriptorAttributeName {
    #[inline] pub fn family() -> &'static Self { unsafe { &NSFontFamilyAttribute } }
    #[inline] pub fn name() -> &'static Self { unsafe { &NSFontNameAttribute } }
    #[inline] pub fn face() -> &'static Self { unsafe { &NSFontFaceAttribute } }
    #[inline] pub fn size() -> &'static Self { unsafe { &NSFontSizeAttribute } }
    #[inline] pub fn visible_name() -> &'static Self { unsafe { &NSFontVisibleNameAttribute } }
    #[inline] pub fn matrix() -> &'static Self { unsafe { &NSFontMatrixAttribute } }
    #[inline] pub fn variation() -> &'static Self { unsafe { &NSFontVariationAttribute } }
    #[inline] pub fn character_set() -> &'static Self { unsafe { &NSFontCharacterSetAttribute } }
    #[inline] pub fn cascade_list() -> &'static Self { unsafe { &NSFontCascadeListAttribute } }
    #[inline] pub fn traits() -> &'static Self { unsafe { &NSFontTraitsAttribute } }
    #[inline] pub fn fixed_advance() -> &'static Self { unsafe { &NSFontFixedAdvanceAttribute } }
    #[inline] pub fn feature_settings() -> &'static Self { unsafe { &NSFontFeatureSettingsAttribute } }
}

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
        @selector("initWithFontAttributes:")
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
    pub fn withFontAttributes(attributes: &NSDictionary<NSFontDescriptorAttributeName,NSObject>, pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        unsafe {
            let alloc = Self::class().alloc(pool);
            let raw: *const Self = Self::perform_autorelease_to_retain(alloc, Sel::initWithFontAttributes_(), pool,(attributes.assume_nonmut_perform(),));
            Self::assume_nonnil(raw).assume_retained()
        }
    }
}

#[cfg(test)] mod tests {
    use foundationr::{objc_nsstring,NSDictionary,NSObjectTrait};
    use objr::bindings::autoreleasepool;
    use crate::nsfontdescriptor::{NSFontDescriptor,NSFontDescriptorAttributeName};
    use objr::foundation::{NSString};


    #[test] fn test() {
        autoreleasepool(|pool| {
            let nsstring = NSString::with_str_copy("Helvetica", pool);
            let font = NSFontDescriptor::withNameSize(&nsstring, 12.0, pool);
            println!("{}", font);
        })

    }
    #[test] fn attributes() {
        autoreleasepool(|pool| {
            let helvetica = objc_nsstring!("Helvetica");
            let size = foundationr::NSNumber::with_int(12,pool);
            let attributes = NSDictionary::withObjectsForKeys(&[helvetica.as_nsobject(),size.as_nsobject()], &[NSFontDescriptorAttributeName::name(), NSFontDescriptorAttributeName::size()],pool);
            let font = NSFontDescriptor::withFontAttributes(&attributes, pool);
            println!("{}", font);
        })
    }
}


