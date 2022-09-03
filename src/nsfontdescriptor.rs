use objr::bindings::*;
use coretextr::CTFontDescriptor;
use core::mem::forget;

#[cfg(feature="bridge")]
use core_foundationr::CTFontDescriptor;
objc_class! {
    pub struct NSFontDescriptor {
        @class(NSFontDescriptor)
    }
}

impl NSFontDescriptor {
    #[cfg(feature="coretext")]
    pub fn bridge(ctfontdescriptor: coretextr::StrongCell<CTFontDescriptor>) -> StrongCell<Self> {
        unsafe {
            let as_ref: &CTFontDescriptor = &*ctfontdescriptor;
            let as_ptr: *const CTFontDescriptor = as_ref as *const CTFontDescriptor;
            let pun = as_ptr as *const NSFontDescriptor;
            forget(ctfontdescriptor);
            Self::assume_nonnil(pun).assume_retained()
        }
    }
}
