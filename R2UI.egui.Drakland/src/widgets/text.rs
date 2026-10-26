//! 文字辅助：常用 RichText 构造，统一字号与语义颜色。

use egui::{RichText, Ui};

use crate::theme::ThemeColors;

/// 主要正文（白 / 深色）。
pub fn text_primary(tc: &ThemeColors, s: impl Into<String>) -> RichText {
    RichText::new(s).color(tc.text_primary)
}

/// 次级正文。
pub fn text_secondary(tc: &ThemeColors, s: impl Into<String>) -> RichText {
    RichText::new(s).color(tc.text_secondary)
}

/// 弱化 / 提示文字。
pub fn text_muted(tc: &ThemeColors, s: impl Into<String>) -> RichText {
    RichText::new(s).color(tc.text_muted)
}

/// 元信息行（11px，弱化色），用于顶栏版本/更新时间等。
pub fn meta_line(ui: &mut Ui, tc: &ThemeColors, s: impl Into<String>) {
    ui.label(RichText::new(s).size(11.0).color(tc.meta_text));
}

/// 状态行（13px）。
pub fn status_line(ui: &mut Ui, tc: &ThemeColors, s: impl Into<String>) {
    ui.label(RichText::new(s).size(13.0).color(tc.status_text));
}

/// 表单字段下方的说明文字（11px，弱化色）。
pub fn hint(ui: &mut Ui, tc: &ThemeColors, s: impl Into<String>) {
    ui.label(RichText::new(s).color(tc.hint_text).size(11.0));
}
