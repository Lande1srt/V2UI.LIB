//! FlatLaf Light 风格调色板。
//!
//! 颜色值来自 FlatLaf 官方 `FlatLightLaf.properties`：
//!   @background   = #f2f2f2
//!   @buttonBackground / @componentBackground = lighten(@background, 5%)
//!   @accentBaseColor = #2675BF
//!   Component.borderColor = shade(@background, 20%)
//!   Component.focusedBorderColor = shade(Component.focusColor, 10%)
//!   Separator.foreground = shade(@background, 15%)
//!   Button.hoverBackground / pressedBackground = darken(Button.background, 3% / 10%)
//!   TabbedPane.hoverColor = darken(@background, 7%)
//!   TabbedPane.underlineColor = tint(@accentBaseColor, 10%)

use eframe::egui::Color32;

/// 窗口/面板主背景 FlatLaf `@background = #F2F2F2`。
pub const BG: Color32 = Color32::from_rgb(242, 242, 242);
/// 按钮/组件背景 `lighten(BG, 5%)`：#F3F3F3，比窗口稍亮。
pub const COMPONENT_BG: Color32 = Color32::from_rgb(243, 243, 243);
/// Tab hover 背景 `darken(BG, 7%)`：#E2E2E2。
pub const TAB_HOVER: Color32 = Color32::from_rgb(225, 225, 225);
/// 按钮 hover `darken(COMPONENT_BG, 3%)`：#ECECEC。
pub const BTN_HOVER: Color32 = Color32::from_rgb(236, 236, 236);
/// 按钮 pressed `darken(COMPONENT_BG, 10%)`：#DBDBDB。
pub const BTN_PRESSED: Color32 = Color32::from_rgb(219, 219, 219);
/// 描边：天蓝调（比纯灰更有辨识度），与 focus/hover 态自然过渡。
pub const BORDER: Color32 = Color32::from_rgb(122, 167, 218);
/// 焦点/hover 描边：温暖琥珀黄，聚焦输入框时黄色边框形成对比。
pub const FOCUS_BORDER: Color32 = Color32::from_rgb(255, 183, 77);
/// 分隔线：天蓝调的浅灰，柔和不抢戏。
pub const SEPARATOR: Color32 = Color32::from_rgb(195, 210, 225);
/// FlatLaf accent 蓝 `@accentBaseColor = #2675BF`。
pub const ACCENT: Color32 = Color32::from_rgb(38, 117, 191);
/// Tab 选中下划线 `tint(ACCENT, 10%)`。
pub const ACCENT_UNDERLINE: Color32 = Color32::from_rgb(60, 131, 197);
/// 主文字 `@foreground = #000`。
pub const TEXT: Color32 = Color32::BLACK;
/// 次要文字 `tint(@foreground, 50%)` 即 50% 灰。
pub const WEAK_TEXT: Color32 = Color32::from_rgb(127, 127, 127);

/// 错误文字（偏红）。
pub const WARN_FG: Color32 = Color32::from_rgb(230, 125, 43);
pub const ERROR_FG: Color32 = Color32::from_rgb(200, 50, 50);
pub const SUCCESS_FG: Color32 = Color32::from_rgb(0x21, 0x7a, 0x34);
