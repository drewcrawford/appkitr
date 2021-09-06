use objr::bindings::*;
objc_class! {
    pub struct NSApplication {
        @class(NSApplication)
    }
}
objc_selector_group! {
    pub trait NSApplicationSelectors {
        @selector("sharedApplication")
        @selector("run")
    }
    impl NSApplicationSelectors for Sel{}
}

#[allow(non_snake_case)]
impl NSApplication {
    pub fn sharedApplication(pool: &ActiveAutoreleasePool) -> StrongMutCell<NSApplication> {
        unsafe {
            let ptr = Class::perform_autorelease_to_retain(Self::class().assume_nonmut_perform(), Sel::sharedApplication(),pool, () );
            Self::assume_nonnil(ptr).assume_retained().assume_mut()
        }
    }
    pub fn run(&mut self, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::run(), pool, () )
        }
    }
}
