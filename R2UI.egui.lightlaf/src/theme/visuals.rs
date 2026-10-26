//! FlatLaf Light / Dark 双主题覆盖。
//!
//! 两份主题共用同一套 spacing + text_styles（调用私有 apply_common_style），
//! 只把控件颜色 / 窗口背景 / 文字反色等视觉值差异化。

use eframe::egui::{self, Color32, CornerRadius, Margin, Stroke, Visuals};

use super::palette;

/// FlatLaf Dark 调色板（从 FlatLaf DarkLaf.properties 简化推导）。
mod dark {
    use eframe::egui::Color32;

    pub const BG: Color32 = Color32::from_rgb(43, 43, 43);          // @background = #2B2B2B
    pub const COMPONENT_BG: Color32 = Color32::from_rgb(60, 60, 60); // 按钮/输入框背景
    pub const TAB_HOVER: Color32 = Color32::from_rgb(80, 80, 80);
    pub const BTN_HOVER: Color32 = Color32::from_rgb(70, 70, 70);
    pub const BTN_PRESSED: Color32 = Color32::from_rgb(90, 90, 90);
    pub const BORDER: Color32 = Color32::from_rgb(100, 100, 100);
    pub const FOCUS_BORDER: Color32 = Color32::from_rgb(255, 183, 77); // 琥珀黄在 dark 里也清晰
    pub const SEPARATOR: Color32 = Color32::from_rgb(80, 80, 80);
    pub const ACCENT: Color32 = Color32::from_rgb(80, 150, 220);      // accent 蓝偏亮
    pub const TEXT: Color32 = Color32::from_rgb(220, 220, 220);       // 主文字
    pub const WEAK_TEXT: Color32 = Color32::from_rgb(160, 160, 160);
}

// =============================================================================
// 共用 spacing / font 配置
// =============================================================================

fn apply_common_style(ctx: &egui::Context, theme: egui::Theme) {
    let mut style = (*ctx.style_of(theme)).clone();

    // 全局控件最小交互尺寸：TextEdit/Button/Checkbox 统一增高到 28px
    style.spacing.interact_size = egui::vec2(0.0, 28.0);
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.window_margin = Margin::symmetric(10, 8);
    style.spacing.button_padding = egui::vec2(14.0, 5.0);
    style.spacing.indent = 18.0;

    style.spacing.scroll = egui::style::ScrollStyle {
        floating: false,
        bar_width: 6.0,
        handle_min_length: 24.0,
        foreground_color: true,
        ..Default::default()
    };

    let ts = &mut style.text_styles;
    ts.insert(egui::TextStyle::Heading, egui::FontId::proportional(15.0));
    ts.insert(egui::TextStyle::Body, egui::FontId::proportional(12.0));
    ts.insert(egui::TextStyle::Button, egui::FontId::proportional(12.0));
    ts.insert(egui::TextStyle::Small, egui::FontId::proportional(10.0));
    ts.insert(egui::TextStyle::Monospace, egui::FontId::monospace(11.0));

    ctx.set_style_of(theme, style);
}

// =============================================================================
// FlatLaf Light
// =============================================================================

pub fn apply_flatlaf_tweaks(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    visuals.dark_mode = false;

    visuals.window_fill = palette::BG;
    visuals.panel_fill = palette::BG;
    visuals.extreme_bg_color = palette::BG;
    visuals.text_edit_bg_color = Some(palette::BG);
    visuals.faint_bg_color = palette::COMPONENT_BG;
    visuals.code_bg_color = palette::COMPONENT_BG;
    visuals.override_text_color = Some(palette::TEXT);
    visuals.weak_text_color = Some(palette::WEAK_TEXT);

    visuals.widgets.noninteractive.bg_fill = palette::COMPONENT_BG;
    visuals.widgets.noninteractive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(0.5, palette::BORDER);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, palette::TEXT);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(4);

    visuals.widgets.inactive.bg_fill = palette::COMPONENT_BG;
    visuals.widgets.inactive.weak_bg_fill = palette::COMPONENT_BG;
    visuals.widgets.inactive.bg_stroke = Stroke::new(0.5, palette::BORDER);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, palette::TEXT);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(4);

    visuals.widgets.hovered.bg_fill = palette::BTN_HOVER;
    visuals.widgets.hovered.weak_bg_fill = palette::BTN_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(0.5, palette::FOCUS_BORDER);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, palette::TEXT);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(4);

    visuals.widgets.active.bg_fill = palette::BTN_PRESSED;
    visuals.widgets.active.weak_bg_fill = palette::BTN_PRESSED;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, palette::ACCENT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, palette::TEXT);
    visuals.widgets.active.corner_radius = CornerRadius::same(4);

    visuals.selection.bg_fill = palette::ACCENT;
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);

    visuals.hyperlink_color = palette::ACCENT;
    visuals.warn_fg_color = palette::WARN_FG;
    visuals.error_fg_color = palette::ERROR_FG;

    visuals.window_stroke = Stroke::new(0.5, palette::SEPARATOR);
    visuals.window_shadow = egui::Shadow::NONE;
    visuals.window_corner_radius = CornerRadius::same(4);
    visuals.window_highlight_topmost = false;
    visuals.menu_corner_radius = CornerRadius::same(4);

    visuals.button_frame = true;
    visuals.collapsing_header_frame = false;

    ctx.set_visuals(visuals);
    apply_common_style(ctx, egui::Theme::Light);
}

// =============================================================================
// FlatLaf Dark
// =============================================================================

pub fn apply_flatlaf_dark(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    visuals.dark_mode = true;

    visuals.window_fill = dark::BG;
    visuals.panel_fill = dark::BG;
    visuals.extreme_bg_color = dark::BG;
    visuals.text_edit_bg_color = Some(dark::COMPONENT_BG);
    visuals.faint_bg_color = dark::COMPONENT_BG;
    visuals.code_bg_color = dark::COMPONENT_BG;
    visuals.override_text_color = Some(dark::TEXT);
    visuals.weak_text_color = Some(dark::WEAK_TEXT);

    visuals.widgets.noninteractive.bg_fill = dark::COMPONENT_BG;
    visuals.widgets.noninteractive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(0.5, dark::BORDER);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, dark::TEXT);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(4);

    visuals.widgets.inactive.bg_fill = dark::COMPONENT_BG;
    visuals.widgets.inactive.weak_bg_fill = dark::COMPONENT_BG;
    visuals.widgets.inactive.bg_stroke = Stroke::new(0.5, dark::BORDER);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, dark::TEXT);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(4);

    visuals.widgets.hovered.bg_fill = dark::BTN_HOVER;
    visuals.widgets.hovered.weak_bg_fill = dark::BTN_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(0.5, dark::FOCUS_BORDER);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, dark::TEXT);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(4);

    visuals.widgets.active.bg_fill = dark::BTN_PRESSED;
    visuals.widgets.active.weak_bg_fill = dark::BTN_PRESSED;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, dark::ACCENT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, dark::TEXT);
    visuals.widgets.active.corner_radius = CornerRadius::same(4);

    visuals.selection.bg_fill = dark::ACCENT;
    visuals.selection.stroke = Stroke::new(1.0, Color32::BLACK);

    visuals.hyperlink_color = dark::ACCENT;
    visuals.warn_fg_color = palette::WARN_FG;
    visuals.error_fg_color = palette::ERROR_FG;

    visuals.window_stroke = Stroke::new(0.5, dark::SEPARATOR);
    visuals.window_shadow = egui::Shadow::NONE;
    visuals.window_corner_radius = CornerRadius::same(4);
    visuals.window_highlight_topmost = false;
    visuals.menu_corner_radius = CornerRadius::same(4);

    visuals.button_frame = true;
    visuals.collapsing_header_frame = false;

    ctx.set_visuals(visuals);
    apply_common_style(ctx, egui::Theme::Dark);
}
