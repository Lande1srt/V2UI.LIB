//! 基础组件：徽标、卡片、状态文字、搜索框、小节标题等。
//!
//! 所有组件均以自由函数 + `&mut egui::Ui` 形式提供，返回交互响应，
//! 不绑定任何业务模型，便于在任意 egui 项目中复用。

use egui::{Color32, RichText, Ui};

use crate::theme::ThemeColors;

pub mod badge;
pub mod card;
pub mod text;

pub use badge::Badge;
pub use card::{CardAction, CardFrame, DownloadState};
pub use text::*;

// ------------------------------------------------------------------
// 小节标题 / 分类标签
// ------------------------------------------------------------------

/// 分类标题（侧边栏分组名样式：12px、强调色、加粗）。
pub fn category_label(ui: &mut Ui, tc: &ThemeColors, label: &str) {
    ui.label(RichText::new(label).color(tc.category_label).size(12.0).strong());
}

/// 内容区小节标题（16px、分类色）。
pub fn section_title(ui: &mut Ui, tc: &ThemeColors, title: &str) {
    ui.label(RichText::new(title).color(tc.category_label).size(16.0));
}

/// 页面大标题（18px、强调色）。
pub fn page_title(ui: &mut Ui, tc: &ThemeColors, title: &str) {
    ui.label(RichText::new(title).size(18.0).color(tc.accent));
}

// ------------------------------------------------------------------
// 搜索框
// ------------------------------------------------------------------

/// 全局搜索框的返回结果。
pub struct SearchResponse {
    pub response: egui::Response,
}

/// 绘制单行搜索框，样式与源项目一致（250 宽、8/6 内边距）。
///
/// 输入法（IME）处理保持源项目行为：获得焦点时上报光标矩形。
pub fn search_box(ui: &mut Ui, tc: &ThemeColors, query: &mut String, id_source: &str) -> egui::Response {
    let text_edit = egui::TextEdit::singleline(query)
        .hint_text("搜索...")
        .text_color(tc.search_text)
        .margin(egui::Margin::symmetric(8.0, 6.0))
        .id_source(id_source)
        .desired_width(250.0);
    let response = ui.add(text_edit);

    if response.has_focus() {
        let rect = response.rect;
        ui.output_mut(|o| {
            o.mutable_text_under_cursor = true;
            o.ime = Some(egui::output::IMEOutput {
                rect,
                cursor_rect: egui::Rect::from_min_size(
                    egui::pos2(rect.left(), rect.top()),
                    egui::vec2(2.0, rect.height()),
                ),
            });
        });
    }

    response
}

// ------------------------------------------------------------------
// 状态徽标式文字
// ------------------------------------------------------------------

/// 三态下载状态文字颜色。
pub fn download_state_color(tc: &ThemeColors, state: DownloadState) -> Color32 {
    match state {
        DownloadState::NotDownloaded => tc.not_downloaded,
        DownloadState::Downloaded => tc.downloaded,
        DownloadState::Downloading => tc.downloading,
    }
}

/// 可点击的下划线文字链接（如“取消下载”），返回点击响应。
pub fn inline_link(ui: &mut Ui, label: &str, color: Color32) -> egui::Response {
    let rich = RichText::new(label).color(color).size(12.0).underline();
    ui.add(egui::Label::new(rich).sense(egui::Sense::click()))
}
