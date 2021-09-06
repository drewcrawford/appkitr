use objr::bindings::*;
use coregraphicsr::*;
use foundationr::NSInteger;
use crate::NSWindow;

objc_enum! {
    pub struct NSViewLayerContentsRedrawPolicy<NSInteger>;
    impl NSViewLayerContentsRedrawPolicy {
        Never = 0,
        OnSetNeedsDisplay = 1,
        DuringViewResize = 2,
        BeforeViewResize = 3,
        Crossfade  = 4
    }

}

objc_class! {
    pub struct NSView{
        @class(NSView)
    }
}

objc_selector_group! {
    trait NSViewSelectors {
        @selector("initWithFrame:")
        @selector("setWantsLayer:")
        @selector("setLayerContentsRedrawPolicy:")
        @selector("window")
    }
    impl NSViewSelectors for Sel {}
}

#[allow(non_snake_case)]
impl NSView {
    pub fn initWithFrame( frame: CGRect, pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
            let uninitialized = NSView::class().alloc(pool);
            let initialized = Self::perform(uninitialized, Sel::initWithFrame_(), pool, (frame,));
            Self::assume_nonnil(initialized).assume_retained().assume_mut()
        }

    }
    pub fn setWantsLayer(&mut self, wants_layer: bool, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self,Sel::setWantsLayer_(), pool, (wants_layer,))
        }
    }
    pub fn setLayerContentsRedrawPolicy(&mut self, policy: NSViewLayerContentsRedrawPolicy, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self,Sel::setLayerContentsRedrawPolicy_(), pool, (policy.field(),))
        }
    }
    pub fn window(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<NSWindow>> {
        unsafe {
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::window(), pool, ());
            NSWindow::nullable(ptr).assume_retained()
        }
    }
}

#[test]
fn smoke_test() {
    autoreleasepool(|pool| {
        let mut v = NSView::initWithFrame(CGRect::make(0.,0.,100.,100.), pool);
        v.setWantsLayer( true, pool);
        v.setLayerContentsRedrawPolicy(NSViewLayerContentsRedrawPolicy::OnSetNeedsDisplay, pool);
        assert!(v.window(pool).is_none());
    })
}

