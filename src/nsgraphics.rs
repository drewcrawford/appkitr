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

objc_class_newtype! {
    pub struct NSColorSpaceName: NSString;
}
extern "C" {
    static NSCalibratedWhiteColorSpace: &'static NSColorSpaceName;
    static NSCalibratedRGBColorSpace: &'static NSColorSpaceName;
    static NSDeviceWhiteColorSpace: &'static NSColorSpaceName;
    static NSDeviceRGBColorSpace: &'static NSColorSpaceName;
    static NSDeviceCMYKColorSpace: &'static NSColorSpaceName;
    static NSNamedColorSpace: &'static NSColorSpaceName;
    static NSPatternColorSpace: &'static NSColorSpaceName;
    static NSCustomColorSpace: &'static NSColorSpaceName;
    static NSCalibratedBlackColorSpace: &'static NSColorSpaceName;
    static NSDeviceBlackColorSpace: &'static NSColorSpaceName;
}

#[allow(non_snake_case)]
impl NSColorSpaceName {
    pub fn calibrated_white() -> &'static NSColorSpaceName { unsafe{ NSCalibratedWhiteColorSpace.cast() }}
    pub fn calibrated_rgb() -> &'static NSColorSpaceName { unsafe{ NSCalibratedRGBColorSpace.cast() }}
    pub fn device_white() -> &'static NSColorSpaceName { unsafe{ NSDeviceWhiteColorSpace.cast() }}
    pub fn device_rgb() -> &'static NSColorSpaceName { unsafe{ NSDeviceRGBColorSpace.cast() }}
    pub fn device_cmyk() -> &'static NSColorSpaceName { unsafe{ NSDeviceCMYKColorSpace.cast() }}
    pub fn named() -> &'static NSColorSpaceName { unsafe{ NSNamedColorSpace.cast() }}
    pub fn pattern() -> &'static NSColorSpaceName { unsafe{ NSPatternColorSpace.cast() }}
    pub fn custom() -> &'static NSColorSpaceName { unsafe{ NSCustomColorSpace.cast() }}
    pub fn calibrated_black() -> &'static NSColorSpaceName { unsafe{ NSCalibratedBlackColorSpace.cast() }}
    pub fn device_black() -> &'static NSColorSpaceName { unsafe{ NSDeviceBlackColorSpace.cast() }}
}

objc_enum! {
    pub struct NSBitmapFormat<NSUInteger>;
    impl NSBitmapFormat {
        AlphaFirst = 1 << 0,
        AlphaNonpremultiplied = 1 << 1,
        FloatingPointSamples = 1 << 2,
        SixteenBitLittleEndian = 1 << 8,
        ThirtyTwoBitLittleEndian = 1 << 9,
        SixteenBitBigEndian = 1 << 10,
        ThirtyTwoBitBigEndian = 1 << 11
    }
}
