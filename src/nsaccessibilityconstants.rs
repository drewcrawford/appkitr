use objr::bindings::*;
objc_class_newtype!(
    pub struct NSAccessibilityRole: NSString;
);

extern "C" {
    static NSAccessibilityUnknownRole: &'static NSString;
    static NSAccessibilityButtonRole: &'static NSString;
    static NSAccessibilityRadioButtonRole: &'static NSString;
    static NSAccessibilityCheckBoxRole: &'static NSString;
    static NSAccessibilitySliderRole: &'static NSString;
    static NSAccessibilityTabGroupRole: &'static NSString;
    static NSAccessibilityTextFieldRole: &'static NSString;
    static NSAccessibilityStaticTextRole: &'static NSString;
    static NSAccessibilityTextAreaRole: &'static NSString;
    static NSAccessibilityScrollAreaRole: &'static NSString;
    static NSAccessibilityPopUpButtonRole: &'static NSString;
    static NSAccessibilityMenuButtonRole: &'static NSString;
    static NSAccessibilityTableRole: &'static NSString;
    static NSAccessibilityApplicationRole: &'static NSString;
    static NSAccessibilityGroupRole: &'static NSString;
    static NSAccessibilityRadioGroupRole: &'static NSString;
    static NSAccessibilityListRole: &'static NSString;
    static NSAccessibilityScrollBarRole: &'static NSString;
    static NSAccessibilityValueIndicatorRole: &'static NSString;
    static NSAccessibilityImageRole: &'static NSString;
    static NSAccessibilityMenuBarRole: &'static NSString;
    static NSAccessibilityMenuBarItemRole: &'static NSString;
    static NSAccessibilityMenuRole: &'static NSString;
    static NSAccessibilityMenuItemRole: &'static NSString;
    static NSAccessibilityColumnRole: &'static NSString;
    static NSAccessibilityRowRole: &'static NSString;
    static NSAccessibilityToolbarRole: &'static NSString;
    static NSAccessibilityBusyIndicatorRole: &'static NSString;
    static NSAccessibilityProgressIndicatorRole: &'static NSString;
    static NSAccessibilityWindowRole: &'static NSString;
    static NSAccessibilityDrawerRole: &'static NSString;
    static NSAccessibilitySystemWideRole: &'static NSString;
    static NSAccessibilityOutlineRole: &'static NSString;
    static NSAccessibilityIncrementorRole: &'static NSString;
    static NSAccessibilityBrowserRole: &'static NSString;
    static NSAccessibilityComboBoxRole: &'static NSString;
    static NSAccessibilitySplitGroupRole: &'static NSString;
    static NSAccessibilitySplitterRole: &'static NSString;
    static NSAccessibilityColorWellRole: &'static NSString;
    static NSAccessibilityGrowAreaRole: &'static NSString;
    static NSAccessibilitySheetRole: &'static NSString;
    static NSAccessibilityHelpTagRole: &'static NSString;
    static NSAccessibilityMatteRole: &'static NSString;
    static NSAccessibilityRulerRole: &'static NSString;
    static NSAccessibilityRulerMarkerRole: &'static NSString;
    static NSAccessibilityLinkRole: &'static NSString;
    static NSAccessibilityDisclosureTriangleRole: &'static NSString;
    static NSAccessibilityRelevanceIndicatorRole: &'static NSString;
    static NSAccessibilityLevelIndicatorRole: &'static NSString;
    static NSAccessibilityCellRole: &'static NSString;
    static NSAccessibilityPopoverRole: &'static NSString;
    static NSAccessibilityPageRole: &'static NSString;
}

impl NSAccessibilityRole {
    #[inline] pub fn unknown() -> &'static Self { unsafe { NSAccessibilityUnknownRole.cast() } }
    #[inline] pub fn button() -> &'static Self { unsafe { NSAccessibilityButtonRole.cast() } }
    #[inline] pub fn radio_button() -> &'static Self { unsafe { NSAccessibilityRadioButtonRole.cast() } }
    #[inline] pub fn check_box() -> &'static Self { unsafe { NSAccessibilityCheckBoxRole.cast() } }
    #[inline] pub fn slider() -> &'static Self { unsafe { NSAccessibilitySliderRole.cast() } }
    #[inline] pub fn tab_group() -> &'static Self { unsafe { NSAccessibilityTabGroupRole.cast() } }
    #[inline] pub fn text_field() -> &'static Self { unsafe { NSAccessibilityTextFieldRole.cast() } }
    #[inline] pub fn static_text() -> &'static Self { unsafe { NSAccessibilityStaticTextRole.cast() } }
    #[inline] pub fn text_area() -> &'static Self { unsafe { NSAccessibilityTextAreaRole.cast() } }
    #[inline] pub fn scroll_area() -> &'static Self { unsafe { NSAccessibilityScrollAreaRole.cast() } }
    #[inline] pub fn pop_up_button() -> &'static Self { unsafe { NSAccessibilityPopUpButtonRole.cast() } }
    #[inline] pub fn menu_button() -> &'static Self { unsafe { NSAccessibilityMenuButtonRole.cast() } }
    #[inline] pub fn table() -> &'static Self { unsafe { NSAccessibilityTableRole.cast() } }
    #[inline] pub fn application() -> &'static Self { unsafe { NSAccessibilityApplicationRole.cast() } }
    #[inline] pub fn group() -> &'static Self { unsafe { NSAccessibilityGroupRole.cast() } }

    #[inline] pub fn radio_group() -> &'static Self { unsafe { NSAccessibilityRadioGroupRole.cast() } }
    #[inline] pub fn list() -> &'static Self { unsafe { NSAccessibilityListRole.cast() } }
    #[inline] pub fn scroll_bar() -> &'static Self { unsafe { NSAccessibilityScrollBarRole.cast() } }
    #[inline] pub fn value_indicator() -> &'static Self { unsafe { NSAccessibilityValueIndicatorRole.cast() } }
    #[inline] pub fn image() -> &'static Self { unsafe { NSAccessibilityImageRole.cast() } }

    #[inline] pub fn menu_bar() -> &'static Self { unsafe { NSAccessibilityMenuBarRole.cast() } }
    #[inline] pub fn menu_bar_item() -> &'static Self { unsafe { NSAccessibilityMenuBarItemRole.cast() } }
    #[inline] pub fn menu() -> &'static Self { unsafe { NSAccessibilityMenuRole.cast() } }
    #[inline] pub fn menu_item() -> &'static Self { unsafe { NSAccessibilityMenuItemRole.cast() } }
    #[inline] pub fn column() -> &'static Self { unsafe { NSAccessibilityColumnRole.cast() } }
    #[inline] pub fn row() -> &'static Self { unsafe { NSAccessibilityRowRole.cast() } }
    #[inline] pub fn toolbar() -> &'static Self { unsafe { NSAccessibilityToolbarRole.cast() } }
    #[inline] pub fn busy_indicator() -> &'static Self { unsafe { NSAccessibilityBusyIndicatorRole.cast() } }
    #[inline] pub fn progress_indicator() -> &'static Self { unsafe { NSAccessibilityProgressIndicatorRole.cast() } }
    #[inline] pub fn window() -> &'static Self { unsafe { NSAccessibilityWindowRole.cast() } }
    #[inline] pub fn drawer() -> &'static Self { unsafe { NSAccessibilityDrawerRole.cast() } }
    #[inline] pub fn system_wide() -> &'static Self { unsafe { NSAccessibilitySystemWideRole.cast() } }
    #[inline] pub fn outline() -> &'static Self { unsafe { NSAccessibilityOutlineRole.cast() } }
    #[inline] pub fn incrementor() -> &'static Self { unsafe { NSAccessibilityIncrementorRole.cast() } }
    #[inline] pub fn browser() -> &'static Self { unsafe { NSAccessibilityBrowserRole.cast() } }
    #[inline] pub fn combo_box() -> &'static Self { unsafe { NSAccessibilityComboBoxRole.cast() } }
    #[inline] pub fn split_group() -> &'static Self { unsafe { NSAccessibilitySplitGroupRole.cast() } }
    #[inline] pub fn splitter() -> &'static Self { unsafe { NSAccessibilitySplitterRole.cast() } }
    #[inline] pub fn color_well() -> &'static Self { unsafe { NSAccessibilityColorWellRole.cast() } }
    #[inline] pub fn grow_area() -> &'static Self { unsafe { NSAccessibilityGrowAreaRole.cast() } }
    #[inline] pub fn sheet() -> &'static Self { unsafe { NSAccessibilitySheetRole.cast() } }
    #[inline] pub fn help_tag() -> &'static Self { unsafe { NSAccessibilityHelpTagRole.cast() } }
    #[inline] pub fn matte() -> &'static Self { unsafe { NSAccessibilityMatteRole.cast() } }
    #[inline] pub fn ruler() -> &'static Self { unsafe { NSAccessibilityRulerRole.cast() } }
    #[inline] pub fn ruler_marker() -> &'static Self { unsafe { NSAccessibilityRulerMarkerRole.cast() } }
    #[inline] pub fn link() -> &'static Self { unsafe { NSAccessibilityLinkRole.cast() } }
    #[inline] pub fn disclosure_triangle() -> &'static Self { unsafe { NSAccessibilityDisclosureTriangleRole.cast() } }
    #[inline] pub fn relevance_indicator() -> &'static Self { unsafe { NSAccessibilityRelevanceIndicatorRole.cast() } }
    #[inline] pub fn level_indicator() -> &'static Self { unsafe { NSAccessibilityLevelIndicatorRole.cast() } }
    #[inline] pub fn cell() -> &'static Self { unsafe { NSAccessibilityCellRole.cast() } }
    #[inline] pub fn popover() -> &'static Self { unsafe { NSAccessibilityPopoverRole.cast() } }
    #[inline] pub fn page() -> &'static Self { unsafe { NSAccessibilityPageRole.cast() } }

}

objc_class_newtype!(
    pub struct NSAccessibilityNotificationName: NSString;
);

extern "C" {
    static NSAccessibilityMainWindowChangedNotification: &'static NSString;
    static NSAccessibilityFocusedWindowChangedNotification: &'static NSString;
    static NSAccessibilityFocusedUIElementChangedNotification: &'static NSString;
    static NSAccessibilityApplicationActivatedNotification: &'static NSString;
    static NSAccessibilityApplicationDeactivatedNotification: &'static NSString;
    static NSAccessibilityApplicationHiddenNotification: &'static NSString;
    static NSAccessibilityApplicationShownNotification: &'static NSString;
    static NSAccessibilityWindowCreatedNotification: &'static NSString;
    static NSAccessibilityWindowMovedNotification: &'static NSString;
    static NSAccessibilityWindowResizedNotification: &'static NSString;
    static NSAccessibilityWindowMiniaturizedNotification: &'static NSString;
    static NSAccessibilityWindowDeminiaturizedNotification: &'static NSString;
    static NSAccessibilityDrawerCreatedNotification: &'static NSString;
    static NSAccessibilitySheetCreatedNotification: &'static NSString;
    static NSAccessibilityUIElementDestroyedNotification: &'static NSString;
    static NSAccessibilityValueChangedNotification: &'static NSString;
    static NSAccessibilityTitleChangedNotification: &'static NSString;
    static NSAccessibilityResizedNotification: &'static NSString;
    static NSAccessibilityMovedNotification: &'static NSString;
    static NSAccessibilityCreatedNotification: &'static NSString;
    static NSAccessibilityLayoutChangedNotification: &'static NSString;
    static NSAccessibilityHelpTagCreatedNotification: &'static NSString;
    static NSAccessibilitySelectedTextChangedNotification: &'static NSString;
    static NSAccessibilityRowCountChangedNotification: &'static NSString;
    static NSAccessibilitySelectedChildrenChangedNotification: &'static NSString;
    static NSAccessibilitySelectedRowsChangedNotification: &'static NSString;
    static NSAccessibilitySelectedColumnsChangedNotification: &'static NSString;
    static NSAccessibilityRowExpandedNotification: &'static NSString;
    static NSAccessibilityRowCollapsedNotification: &'static NSString;
    static NSAccessibilitySelectedCellsChangedNotification: &'static NSString;
    static NSAccessibilityUnitsChangedNotification: &'static NSString;
    static NSAccessibilitySelectedChildrenMovedNotification: &'static NSString;
    static NSAccessibilityAnnouncementRequestedNotification: &'static NSString;
}

impl NSAccessibilityNotificationName {
    #[inline] pub fn main_window_changed() -> &'static Self { unsafe { NSAccessibilityMainWindowChangedNotification.cast() } }
    #[inline] pub fn focused_window_changed() -> &'static Self { unsafe { NSAccessibilityFocusedWindowChangedNotification.cast() } }
    #[inline] pub fn focused_ui_element_changed() -> &'static Self { unsafe { NSAccessibilityFocusedUIElementChangedNotification.cast() } }
    #[inline] pub fn application_activated() -> &'static Self { unsafe { NSAccessibilityApplicationActivatedNotification.cast() } }
    #[inline] pub fn application_deactivated() -> &'static Self { unsafe { NSAccessibilityApplicationDeactivatedNotification.cast() } }
    #[inline] pub fn application_hidden() -> &'static Self { unsafe { NSAccessibilityApplicationHiddenNotification.cast() } }
    #[inline] pub fn application_shown() -> &'static Self { unsafe { NSAccessibilityApplicationShownNotification.cast() } }
    #[inline] pub fn window_created() -> &'static Self { unsafe { NSAccessibilityWindowCreatedNotification.cast() } }
    #[inline] pub fn window_moved() -> &'static Self { unsafe { NSAccessibilityWindowMovedNotification.cast() } }
    #[inline] pub fn window_resized() -> &'static Self { unsafe { NSAccessibilityWindowResizedNotification.cast() } }
    #[inline] pub fn window_miniaturized() -> &'static Self { unsafe { NSAccessibilityWindowMiniaturizedNotification.cast() } }
    #[inline] pub fn window_deminiaturized() -> &'static Self { unsafe { NSAccessibilityWindowDeminiaturizedNotification.cast() } }
    #[inline] pub fn drawer_created() -> &'static Self { unsafe { NSAccessibilityDrawerCreatedNotification.cast() } }
    #[inline] pub fn sheet_created() -> &'static Self { unsafe { NSAccessibilitySheetCreatedNotification.cast() } }
    #[inline] pub fn ui_element_destroyed() -> &'static Self { unsafe { NSAccessibilityUIElementDestroyedNotification.cast() } }
    #[inline] pub fn value_changed() -> &'static Self { unsafe { NSAccessibilityValueChangedNotification.cast() } }
    #[inline] pub fn title_changed() -> &'static Self { unsafe { NSAccessibilityTitleChangedNotification.cast() } }
    #[inline] pub fn resized() -> &'static Self { unsafe { NSAccessibilityResizedNotification.cast() } }
    #[inline] pub fn moved() -> &'static Self { unsafe { NSAccessibilityMovedNotification.cast() } }
    #[inline] pub fn created() -> &'static Self { unsafe { NSAccessibilityCreatedNotification.cast() } }
    #[inline] pub fn layout_changed() -> &'static Self { unsafe { NSAccessibilityLayoutChangedNotification.cast() } }

    #[inline] pub fn help_tag_created() -> &'static Self { unsafe { NSAccessibilityHelpTagCreatedNotification.cast() } }
    #[inline] pub fn selected_text_changed() -> &'static Self { unsafe { NSAccessibilitySelectedTextChangedNotification.cast() } }
    #[inline] pub fn row_count_changed() -> &'static Self { unsafe { NSAccessibilityRowCountChangedNotification.cast() } }
    #[inline] pub fn selected_children_changed() -> &'static Self { unsafe { NSAccessibilitySelectedChildrenChangedNotification.cast() } }
    #[inline] pub fn selected_rows_changed() -> &'static Self { unsafe { NSAccessibilitySelectedRowsChangedNotification.cast() } }
    #[inline] pub fn selected_columns_changed() -> &'static Self { unsafe { NSAccessibilitySelectedColumnsChangedNotification.cast() } }
    #[inline] pub fn row_expanded() -> &'static Self { unsafe { NSAccessibilityRowExpandedNotification.cast() } }
    #[inline] pub fn row_collapsed() -> &'static Self { unsafe { NSAccessibilityRowCollapsedNotification.cast() } }
    #[inline] pub fn selected_cells_changed() -> &'static Self { unsafe { NSAccessibilitySelectedCellsChangedNotification.cast() } }
    #[inline] pub fn units_changed() -> &'static Self { unsafe { NSAccessibilityUnitsChangedNotification.cast() } }
    #[inline] pub fn selected_children_moved() -> &'static Self { unsafe { NSAccessibilitySelectedChildrenMovedNotification.cast() } }
    #[inline] pub fn announcement_requested() -> &'static Self { unsafe { NSAccessibilityAnnouncementRequestedNotification.cast() } }

}
