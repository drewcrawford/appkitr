/* ! NSAccessibility.h */
use foundationr::NSRect;
use objr::bindings::*;
use crate::nsaccessibility::NSAccessibility;
use crate::NSAccessibilityRole;

objc_class! {
    pub struct NSAccessibilityElement {
        @class(NSAccessibilityElement)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("accessibilityElementWithRole:frame:label:parent:")
        @selector("accessibilityAddChildElement:")
        @selector("accessibilityFrameInParentSpace")
        @selector("setAccessibilityFrameInParentSpace:")
    }
    impl Selectors for Sel {}
}
#[allow(non_snake_case)]
impl NSAccessibilityElement {
    pub fn accessibilityElementWithRoleFrameLabelParent(role: &NSAccessibilityRole, frame: NSRect, label: Option<&NSString>, parent: Option<&NSAccessibilityElement>, pool: &ActiveAutoreleasePool) -> StrongMutCell<Self> {
        unsafe {
            let initialized = Class::<Self>::perform(Self::class().assume_nonmut_perform(), Sel::accessibilityElementWithRole_frame_label_parent(), pool, (role.assume_nonmut_perform(), frame, label.assume_nonmut_perform(), parent.assume_nonmut_perform()));
            Self::assume_nonnil(initialized).retain().assume_mut()
        }
    }
    pub fn accessibilityAddChildElement(&mut self, child: &NSAccessibilityElement, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::accessibilityAddChildElement_(), pool, (child.assume_nonmut_perform(),))
        }
    }
    pub fn accessibilityFrameInParentSpace(&self, pool: &ActiveAutoreleasePool) -> NSRect {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::accessibilityFrameInParentSpace(), pool, ())
        }
    }
    pub fn setAccessibilityFrameInParentSpace(&mut self, frame: NSRect, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setAccessibilityFrameInParentSpace_(), pool, (frame,))
        }
    }

}

unsafe impl NSAccessibility for NSAccessibilityElement {}


#[test] fn smoke() {
    autoreleasepool(|pool| {
        let mut element = NSAccessibilityElement::accessibilityElementWithRoleFrameLabelParent(NSAccessibilityRole::unknown(), NSRect::ZERO, None, None, pool);
        let element2 = NSAccessibilityElement::accessibilityElementWithRoleFrameLabelParent(NSAccessibilityRole::unknown(), NSRect::ZERO, None, None, pool);
        element.accessibilityAddChildElement(&element2, pool);
        use foundationr::NSArray;
        let r: &NSObject = element2.as_nsobject();
        element.setAccessibilityChildren(Some(&NSArray::with_slice(&[r], pool)), pool);
        assert_eq!(element.accessibilityChildren(pool).unwrap().count(pool), 1);

        element.setAccessibilityFrameInParentSpace(NSRect::ZERO, pool);
        assert_eq!(element.accessibilityFrameInParentSpace(pool), NSRect::ZERO);
    });
}