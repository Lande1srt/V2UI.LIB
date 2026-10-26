//! 主题系统：全局 egui Style（深/浅色）与语义化颜色板。
//!
//! 颜色层级（由深到浅 / 由底到面）：
//!   extreme_bg_color ← 最底层背景（窗口 desktop 级）
//!   faint_bg_color   ← 弱化容器背景（ScrollArea 内嵌、分组区）
//!   panel_fill       ← 侧面板背景（SidePanel / TopBottomPanel）
//!   window_fill      ← 主窗口背景（Viewport 根）

use egui::Color32;

/// 主题种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeKind {
    Dark,
    Light,
}

impl ThemeKind {
    /// 从配置字符串解析，非 `"light"` 一律视为深色。
    pub fn from_config_str(theme: &str) -> Self {
        if theme == "light" {
            Self::Light
        } else {
            Self::Dark
        }
    }

    pub fn is_dark(self) -> bool {
        self == Self::Dark
    }

    pub fn as_config_str(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }
}

/// 语义化颜色板：供业务代码直接取用，与全局 visuals 保持一致。
#[derive(Clone)]
pub struct ThemeColors {
    pub is_dark: bool,
    pub window_fill: Color32,
    pub sidebar_fill: Color32,
    pub sidebar_separator: Color32,
    pub sidebar_separator_stroke: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub text_bright: Color32,
    pub accent: Color32,
    pub accent_green: Color32,
    pub accent_red: Color32,
    pub accent_yellow: Color32,
    pub card_title: Color32,
    pub card_desc: Color32,
    pub search_text: Color32,
    pub category_label: Color32,
    pub status_text: Color32,
    pub meta_text: Color32,
    pub not_downloaded: Color32,
    pub downloaded: Color32,
    pub downloading: Color32,
    pub cancel_text: Color32,
    pub badge_fill: Color32,
    pub badge_text: Color32,
    pub hint_text: Color32,
}

impl ThemeColors {
    /// 根据主题种类取得对应颜色板。
    pub fn new(kind: ThemeKind) -> Self {
        match kind {
            ThemeKind::Dark => Self::dark(),
            ThemeKind::Light => Self::light(),
        }
    }

    pub fn dark() -> Self {
        Self {
            is_dark: true,
            window_fill: Color32::from_rgb(33, 36, 43),
            sidebar_fill: Color32::from_rgb(30, 33, 39),
            sidebar_separator: Color32::from_rgb(60, 63, 70),
            sidebar_separator_stroke: Color32::from_rgb(80, 83, 90),
            text_primary: Color32::WHITE,
            text_secondary: Color32::from_rgb(200, 200, 210),
            text_muted: Color32::from_rgb(150, 150, 165),
            text_bright: Color32::from_rgb(220, 220, 230),
            accent: Color32::from_rgb(102, 192, 242),
            accent_green: Color32::from_rgb(80, 200, 100),
            accent_red: Color32::from_rgb(220, 80, 80),
            accent_yellow: Color32::from_rgb(230, 180, 60),
            card_title: Color32::from_rgb(102, 192, 242),
            card_desc: Color32::from_rgb(180, 180, 190),
            search_text: Color32::from_rgb(220, 220, 230),
            category_label: Color32::from_rgb(102, 180, 220),
            status_text: Color32::from_rgb(220, 220, 230),
            meta_text: Color32::from_rgb(150, 150, 165),
            not_downloaded: Color32::from_rgb(220, 80, 80),
            downloaded: Color32::from_rgb(80, 200, 100),
            downloading: Color32::from_rgb(230, 180, 60),
            cancel_text: Color32::from_rgb(220, 80, 80),
            badge_fill: Color32::from_rgb(245, 158, 11),
            badge_text: Color32::from_rgb(30, 30, 30),
            hint_text: Color32::from_rgb(150, 150, 165),
        }
    }

    pub fn light() -> Self {
        Self {
            is_dark: false,
            window_fill: Color32::from_rgb(250, 250, 252),
            sidebar_fill: Color32::from_rgb(242, 244, 248),
            sidebar_separator: Color32::from_rgb(210, 215, 222),
            sidebar_separator_stroke: Color32::from_rgb(195, 200, 208),
            text_primary: Color32::from_rgb(25, 30, 38),
            text_secondary: Color32::from_rgb(55, 62, 72),
            text_muted: Color32::from_rgb(120, 128, 140),
            text_bright: Color32::from_rgb(35, 40, 50),
            accent: Color32::from_rgb(50, 120, 180),
            accent_green: Color32::from_rgb(40, 130, 70),
            accent_red: Color32::from_rgb(200, 50, 50),
            accent_yellow: Color32::from_rgb(200, 130, 30),
            card_title: Color32::from_rgb(40, 100, 160),
            card_desc: Color32::from_rgb(75, 82, 92),
            search_text: Color32::from_rgb(30, 35, 45),
            category_label: Color32::from_rgb(45, 110, 170),
            status_text: Color32::from_rgb(35, 40, 50),
            meta_text: Color32::from_rgb(110, 118, 130),
            not_downloaded: Color32::from_rgb(190, 40, 40),
            downloaded: Color32::from_rgb(40, 130, 70),
            downloading: Color32::from_rgb(190, 120, 20),
            cancel_text: Color32::from_rgb(190, 40, 40),
            badge_fill: Color32::from_rgb(245, 158, 11),
            badge_text: Color32::from_rgb(30, 30, 30),
            hint_text: Color32::from_rgb(130, 138, 150),
        }
    }

    /// 列表选中行使用的半透明强调底色。
    pub fn selected_row_bg(&self) -> Color32 {
        let base = self.accent;
        Color32::from_rgba_unmultiplied(
            (base.r() as u16 + 40).min(255) as u8,
            (base.g() as u16 + 40).min(255) as u8,
            (base.b() as u16 + 40).min(255) as u8,
            35,
        )
    }
}

/// 应用全局主题：设置深色模式、间距、圆角与各状态颜色。
///
/// 与源项目保持一致的视觉结果，仅做模块化整理。
pub fn apply_theme(ctx: &egui::Context, theme: &str) {
    apply_theme_kind(ctx, ThemeKind::from_config_str(theme));
}

/// 与 [`apply_theme`] 相同，但接收强类型 [`ThemeKind`]。
pub fn apply_theme_kind(ctx: &egui::Context, kind: ThemeKind) {
    let mut style = (*ctx.style()).clone();
    let is_dark = kind.is_dark();
    style.visuals.dark_mode = is_dark;

    // ---- 间距 ----
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(0.0);
    style.spacing.button_padding = egui::vec2(10.0, 6.0);
    style.spacing.text_edit_width = 250.0;

    // ---- 圆角 ----
    style.visuals.window_rounding = 8.0.into();
    style.visuals.menu_rounding = 6.0.into();
    style.visuals.widgets.noninteractive.rounding = 6.0.into();
    style.visuals.widgets.inactive.rounding = 6.0.into();
    style.visuals.widgets.hovered.rounding = 6.0.into();
    style.visuals.widgets.active.rounding = 6.0.into();
    style.visuals.widgets.open.rounding = 6.0.into();

    let visuals = &mut style.visuals;

    if is_dark {
        visuals.extreme_bg_color = Color32::from_rgb(20, 22, 26);
        visuals.panel_fill = Color32::from_rgb(28, 30, 36);
        visuals.window_fill = Color32::from_rgb(33, 36, 43);

        visuals.widgets.noninteractive.bg_stroke.color = Color32::from_rgb(60, 63, 70);
        visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(200, 200, 210);

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(45, 48, 56);
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(45, 48, 56);
        visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(180, 180, 195);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(58, 62, 72);
        visuals.widgets.hovered.fg_stroke.color = Color32::from_rgb(10, 100, 100);

        visuals.widgets.active.bg_fill = Color32::from_rgb(60, 130, 168);
        visuals.widgets.active.fg_stroke.color = Color32::from_rgb(220, 220, 200);

        visuals.selection.bg_fill = Color32::from_rgb(60, 130, 168);
        visuals.selection.stroke = egui::Stroke::new(1.5, Color32::from_rgb(255, 255, 255));

        visuals.hyperlink_color = Color32::from_rgb(102, 192, 242);
        visuals.faint_bg_color = Color32::from_rgb(38, 41, 48);
    } else {
        visuals.extreme_bg_color = Color32::from_rgb(230, 232, 238);
        visuals.panel_fill = Color32::from_rgb(255, 255, 255);
        visuals.window_fill = Color32::from_rgb(246, 247, 250);

        visuals.widgets.noninteractive.bg_stroke.color = Color32::from_rgb(195, 200, 210);
        visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(28, 33, 42);

        visuals.widgets.inactive.bg_fill = Color32::from_rgb(235, 238, 244);
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(235, 238, 244);
        visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(55, 60, 72);

        visuals.widgets.hovered.bg_fill = Color32::from_rgb(218, 224, 232);
        visuals.widgets.hovered.fg_stroke.color = Color32::from_rgb(25, 30, 38);

        visuals.widgets.active.bg_fill = Color32::from_rgb(60, 130, 185);
        visuals.widgets.active.fg_stroke.color = Color32::from_rgb(255, 255, 255);

        visuals.selection.bg_fill = Color32::from_rgb(180, 210, 240);
        visuals.selection.stroke = egui::Stroke::new(1.5, Color32::from_rgb(28, 33, 42));

        visuals.hyperlink_color = Color32::from_rgb(45, 105, 170);
        visuals.faint_bg_color = Color32::from_rgb(228, 232, 238);
    }

    ctx.set_style(style);
}
