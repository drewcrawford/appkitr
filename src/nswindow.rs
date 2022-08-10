use objr::bindings::*;
use coregraphicsr::*;
use bitflags::bitflags;
use crate::{NSBackingStoreType, NSView, NSScreen};
use foundationr::{NSUInteger,NSRect};
bitflags! {
    #[repr(transparent)]
    pub struct NSWindowOcclusionState: NSUInteger {
        const VISIBLE = 1 << 1;
    }
}
//safe due to repr-transparent
unsafe impl Arguable for NSWindowOcclusionState{}
unsafe impl Primitive for NSWindowOcclusionState {}

objc_class! {
    pub struct NSWindow {
        @class(NSWindow)
    }
}
objc_selector_group! {
    pub trait NSWindowSelectors {
        @selector("initWithContentRect:styleMask:backing:defer:")
        @selector("makeKeyAndOrderFront:")
        @selector("setContentView:")
        @selector("screen")
        @selector("setDelegate:")
        @selector("occlusionState")
        @selector("backingScaleFactor")
        @selector("acceptsMouseMovedEvents")
        @selector("setAcceptsMouseMovedEvents:")
        @selector("frame")
    }
    impl NSWindowSelectors for Sel {}
}
bitflags! {
    pub struct NSWindowStyleMask: NSUInteger {
       const BORDERLESS = 0;
        const TITLED = 1 << 0;
        const CLOSEABLE = 1 << 1;
        const MINIATUREIZABLE = 1 << 2;
        const RESIZABLE	= 1 << 3;
        //NSWindowStyleMaskTexturedBackground is deprecated
        const UNIFIED_TITLE_AND_TOOLBAR = 1 << 12;
        const FULL_SCREEN = 1 << 14;
        const FULL_SIZE_CONTENT_VIEW = 1 << 15;
    }
}

#[allow(non_snake_case)]
impl NSWindow {
    pub fn initWithContentRect(rect: CGRect, style_mask: NSWindowStyleMask, backing: NSBackingStoreType, defer: bool, pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
            let uninitialized = NSWindow::class().alloc(pool);
            let initialized = Self::perform(uninitialized, Sel::initWithContentRect_styleMask_backing_defer(), pool, (rect, style_mask.bits, backing.field(), defer));
            Self::assume_nonnil(initialized).assume_retained().assume_mut()
        }
    }
    pub fn makeKeyAndOrderFront(&self, sender: &NSObject, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::makeKeyAndOrderFront_(), pool, (sender.assume_nonmut_perform(), ))
        }
    }
    pub fn setContentView(&mut self, contentView: Option<&NSView>, pool: &ActiveAutoreleasePool) {

        unsafe {
            Self::perform_primitive(self, Sel::setContentView_(), pool, (contentView.as_ptr().assume_nonmut_perform(),))
        }
    }
    pub fn screen(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSScreen>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::screen(), pool, ());
            NSScreen::nullable(ptr).assume_retained()
        }
    }
    pub fn backingScaleFactor(&self, pool: &ActiveAutoreleasePool) -> CGFloat {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::backingScaleFactor(), pool, ())
        }
    }
    pub fn setDelegate(&mut self, delegate: &NSObject, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setDelegate_(), pool, (delegate.assume_nonmut_perform(),))
        }
    }
    pub fn occlusionState(&self,pool: &ActiveAutoreleasePool) -> NSWindowOcclusionState {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::occlusionState(), pool, ())
        }
    }
    pub fn acceptsMouseMovedEvents(&self, pool: &ActiveAutoreleasePool) -> bool {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::acceptsMouseMovedEvents(), pool, ())
        }
    }
    pub fn setAcceptsMouseMovedEvents(&mut self, value: bool, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setAcceptsMouseMovedEvents_(), pool, (value,))
        }
    }
    pub fn frame(&self,pool: &ActiveAutoreleasePool) -> NSRect {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::frame(), pool, ())
        }
    }
}