use objr::bindings::*;
use foundationr::{NSUInteger, NSCopying};
objc_enum! {
    #[derive(Debug)]
    pub struct NSBackingStoreType<NSUInteger>;
    impl NSBackingStoreType {
        Buffered = 2
    }
}

objc_class_newtype! {
    pub struct NSDeviceDescriptionKey: NSString;
}
impl NSCopying for NSDeviceDescriptionKey {}

extern {
    #[link_name="NSDeviceResolution"]
    static NSDeviceResolution: &'static NSString;
    #[link_name="NSDeviceColorSpaceName"]
    static NSDeviceColorSpaceName: &'static NSString;
    #[link_name="NSDeviceBitsPerSample"]
    static NSDeviceBitsPerSample: &'static NSString;
    #[link_name="NSDeviceIsScreen"]
    static NSDeviceIsScreen: &'static NSString;
    #[link_name="NSDeviceIsPrinter"]
    static NSDeviceIsPrinter: &'static NSString;
    #[link_name="NSDeviceSize"]
    static NSDeviceSize: &'static NSString;
}

#[allow(non_snake_case)]
impl NSDeviceDescriptionKey {
    pub fn NSDeviceResolution() -> &'static NSDeviceDescriptionKey { unsafe{ NSDeviceResolution.cast() }}
    pub fn NSDeviceColorSpaceName() -> &'static NSDeviceDescriptionKey { unsafe{ NSDeviceColorSpaceName.cast() }}
    pub fn NSDeviceBitsPerSample() -> &'static NSDeviceDescriptionKey { unsafe{ NSDeviceBitsPerSample.cast() }}
    pub fn NSDeviceIsScreen() -> &'static NSDeviceDescriptionKey { unsafe{ NSDeviceIsScreen.cast() }}
    pub fn NSDeviceIsPrinter() -> &'static NSDeviceDescriptionKey { unsafe{ NSDeviceIsPrinter.cast() }}
    pub fn NSDeviceSize() -> &'static NSDeviceDescriptionKey { unsafe{ NSDeviceSize.cast() }}
    ///Converts some other key to NSDeviceDescriptionKey.
    ///
    /// For example, `<https://developer.apple.com/documentation/appkit/nsscreen/1388360-devicedescription>`
    ///
    /// # Safety
    /// There is no guarantee this is actually a valid key
    pub unsafe fn undocumented_key(key: &NSString) -> &NSDeviceDescriptionKey {
        key.cast()
    }
}