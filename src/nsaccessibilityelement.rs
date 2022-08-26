/* ! NSAccessibility.h */
use foundationr::NSRect;
use objr::bindings::*;
use crate::NSAccessibilityRole;

objc_class! {
    pub struct NSAccessibilityElement {
        @class(NSAccessibilityElement)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("accessibilityElementWithRole:frame:label:parent:")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl NSAccessibilityElement {
    fn accessibilityElementWithRoleFrameLabelParent(role: &NSAccessibilityRole, frame: NSRect, label: Option<&NSString>, parent: Option<&NSAccessibilityElement>, pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
            let initialized = Class::<Self>::perform(Self::class().assume_nonmut_perform(), Sel::accessibilityElementWithRole_frame_label_parent(), pool, (role.assume_nonmut_perform(), frame, label.assume_nonmut_perform(), parent.assume_nonmut_perform()));
            Self::assume_nonnil(initialized).retain().assume_mut()
        }
    }
}

#[test] fn smoke() {
    autoreleasepool(|pool| {
        let element = NSAccessibilityElement::accessibilityElementWithRoleFrameLabelParent(NSAccessibilityRole::unknown(), NSRect::ZERO, None, None, pool);
        // let element = element.assume_nonmut_perform();
        // let role = element.accessibilityRole(pool);
        // assert_eq!(role, NSAccessibilityRole::AXUnknownRole);
    });
}