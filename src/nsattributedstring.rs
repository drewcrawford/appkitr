/*! AppKit NSAttributedString.h
*/
use foundationr::NSAttributedStringKey;
extern "C" {
    static NSFontAttributeName: &'static NSAttributedStringKey;
    static NSParagraphStyleAttributeName: &'static NSAttributedStringKey;
    static NSForegroundColorAttributeName: &'static NSAttributedStringKey;
    static NSBackgroundColorAttributeName: &'static NSAttributedStringKey;
    static NSLigatureAttributeName: &'static NSAttributedStringKey;
    static NSKernAttributeName: &'static NSAttributedStringKey;
    static NSTrackingAttributeName: &'static NSAttributedStringKey;
    static NSStrikethroughStyleAttributeName: &'static NSAttributedStringKey;
    static NSUnderlineStyleAttributeName: &'static NSAttributedStringKey;
    static NSStrokeColorAttributeName: &'static NSAttributedStringKey;
    static NSStrokeWidthAttributeName: &'static NSAttributedStringKey;
    static NSShadowAttributeName: &'static NSAttributedStringKey;
    static NSTextEffectAttributeName: &'static NSAttributedStringKey;
    static NSAttachmentAttributeName: &'static NSAttributedStringKey;
    static NSLinkAttributeName: &'static NSAttributedStringKey;
    static NSBaselineOffsetAttributeName: &'static NSAttributedStringKey;
    static NSUnderlineColorAttributeName: &'static NSAttributedStringKey;
    static NSStrikethroughColorAttributeName: &'static NSAttributedStringKey;
    static NSObliquenessAttributeName: &'static NSAttributedStringKey;
    static NSExpansionAttributeName: &'static NSAttributedStringKey;
    static NSWritingDirectionAttributeName: &'static NSAttributedStringKey;
    static NSVerticalGlyphFormAttributeName: &'static NSAttributedStringKey;
    static NSCursorAttributeName: &'static NSAttributedStringKey;
    static NSToolTipAttributeName: &'static NSAttributedStringKey;
    static NSMarkedClauseSegmentAttributeName: &'static NSAttributedStringKey;
    static NSTextAlternativesAttributeName: &'static NSAttributedStringKey;
    static NSSpellingStateAttributeName: &'static NSAttributedStringKey;
    static NSSuperscriptAttributeName: &'static NSAttributedStringKey;
    static NSGlyphInfoAttributeName: &'static NSAttributedStringKey;
}
pub trait NSAttributedStringKeyAppKit {
    fn font() -> &'static NSAttributedStringKey {
        unsafe { &NSFontAttributeName }
    }
    fn paragraph_style() -> &'static NSAttributedStringKey {
        unsafe { &NSParagraphStyleAttributeName }
    }
    fn foreground_color() -> &'static NSAttributedStringKey {
        unsafe { &NSForegroundColorAttributeName }
    }
    fn background_color() -> &'static NSAttributedStringKey {
        unsafe { &NSBackgroundColorAttributeName }
    }
    fn ligature() -> &'static NSAttributedStringKey {
        unsafe { &NSLigatureAttributeName }
    }
    fn kern() -> &'static NSAttributedStringKey {
        unsafe { &NSKernAttributeName }
    }
    fn tracking() -> &'static NSAttributedStringKey {
        unsafe { &NSTrackingAttributeName }
    }
    fn strikethrough_style() -> &'static NSAttributedStringKey {
        unsafe { &NSStrikethroughStyleAttributeName }
    }
    fn underline_style() -> &'static NSAttributedStringKey {
        unsafe { &NSUnderlineStyleAttributeName }
    }
    fn stroke_color() -> &'static NSAttributedStringKey {
        unsafe { &NSStrokeColorAttributeName }
    }
    fn stroke_width() -> &'static NSAttributedStringKey {
        unsafe { &NSStrokeWidthAttributeName }
    }
    fn shadow() -> &'static NSAttributedStringKey {
        unsafe { &NSShadowAttributeName }
    }
    fn text_effect() -> &'static NSAttributedStringKey {
        unsafe { &NSTextEffectAttributeName }
    }
    fn attachment() -> &'static NSAttributedStringKey {
        unsafe { &NSAttachmentAttributeName }
    }
    fn link() -> &'static NSAttributedStringKey {
        unsafe { &NSLinkAttributeName }
    }
    fn baseline_offset() -> &'static NSAttributedStringKey {
        unsafe { &NSBaselineOffsetAttributeName }
    }
    fn underline_color() -> &'static NSAttributedStringKey {
        unsafe { &NSUnderlineColorAttributeName }
    }
    fn strikethrough_color() -> &'static NSAttributedStringKey {
        unsafe { &NSStrikethroughColorAttributeName }
    }
    fn obliqueness() -> &'static NSAttributedStringKey {
        unsafe { &NSObliquenessAttributeName }
    }
    fn expansion() -> &'static NSAttributedStringKey {
        unsafe { &NSExpansionAttributeName }
    }
    fn writing_direction() -> &'static NSAttributedStringKey {
        unsafe { &NSWritingDirectionAttributeName }
    }
    fn vertical_glyph_form() -> &'static NSAttributedStringKey {
        unsafe { &NSVerticalGlyphFormAttributeName }
    }
    fn cursor() -> &'static NSAttributedStringKey {
        unsafe { &NSCursorAttributeName }
    }
    fn tool_tip() -> &'static NSAttributedStringKey {
        unsafe { &NSToolTipAttributeName }
    }
    fn marked_clause_segment() -> &'static NSAttributedStringKey {
        unsafe { &NSMarkedClauseSegmentAttributeName }
    }
    fn text_alternatives() -> &'static NSAttributedStringKey {
        unsafe { &NSTextAlternativesAttributeName }
    }
    fn spelling_state() -> &'static NSAttributedStringKey {
        unsafe { &NSSpellingStateAttributeName }
    }
    fn superscript() -> &'static NSAttributedStringKey {
        unsafe { &NSSuperscriptAttributeName }
    }
    fn glyph_info() -> &'static NSAttributedStringKey {
        unsafe { &NSGlyphInfoAttributeName }
    }
}
impl NSAttributedStringKeyAppKit for NSAttributedStringKey {}
