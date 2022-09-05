use std::os::raw::c_void;
use foundationr::NSInteger;
use objr::bindings::*;
use crate::nsgraphics::{NSBitmapFormat,NSColorSpaceName};
objc_class! {
    pub struct NSBitmapImageRep {
        @class(NSBitmapImageRep)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bitmapFormat:bytesPerRow:bitsPerPixel:")
    }
    impl Selectors for Sel {}
}
impl NSBitmapImageRep {
    pub fn init(bitmap_data_planes: *mut *mut c_void, pixels_wide: NSInteger, pixels_high: NSInteger, bits_per_sample: NSInteger, samples_per_pixel: NSInteger, has_alpha: bool, is_planar: bool,
                color_space_name: &NSColorSpaceName, bitmap_format: NSBitmapFormat, bytes_per_row: NSInteger, bits_per_pixel: NSInteger, pool: &ActiveAutoreleasePool) -> Option<StrongCell<Self>> {
        unsafe {
            let sel = Sel::initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bitmapFormat_bytesPerRow_bitsPerPixel();
            let alloc = Self::class().alloc(pool);
            let raw = Self::perform(alloc, sel, pool, (bitmap_data_planes, pixels_wide, pixels_high, bits_per_sample, samples_per_pixel, has_alpha, is_planar, color_space_name.assume_nonmut_perform(),
                                             bitmap_format.field(), bytes_per_row, bits_per_pixel));
            Self::nullable(raw).assume_retained()
        }
    }
}
#[test] fn smoke() {
    autoreleasepool(|pool| {
        let rep = NSBitmapImageRep::init(std::ptr::null_mut(), 1, 1, 8, 3, false, false, &NSColorSpaceName::device_rgb(), NSBitmapFormat(0), 4, 32, pool).unwrap();
        let desc = rep.description(pool);
        println!("{}", desc);
    });
}