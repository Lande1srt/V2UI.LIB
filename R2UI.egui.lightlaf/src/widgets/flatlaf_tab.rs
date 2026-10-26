//! FlatLaf 风格的 Tab 按钮 —— 自绘控件，不依赖 egui 默认 Tab 实现。
//!
//! - 未选中：纯黑文字，hover 时画淡色背景
//! - 选中：纯黑文字，底部 2px accent 色下划线
//!
//! 返回 `true` 表示这一帧被点击。

use eframe::egui::{self, FontId, Sense, Stroke};

use crate::theme::palette;

pub fn flatlaf_tab(ui: &mut egui::Ui, selected: bool, label: &str) -> bool {
    // 使用 Button 字号
    let font_size = ui
        .style()
        .text_styles
        .get(&egui::TextStyle::Button)
        .map(|s| s.size)
        .unwrap_or(13.0);
    let font_id = FontId::proportional(font_size);

    // 布局文字
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_string(), font_id, palette::TEXT);

    let padding_x = 10.0;
    let padding_y = 6.0;
    let desired_size = egui::vec2(
        galley.size().x + padding_x * 2.0,
        galley.size().y + padding_y * 2.0,
    );

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    // —— 绘制 ——
    let painter = ui.painter_at(rect);

    // 背景：hover → TAB_HOVER；按下 → BTN_PRESSED
    let is_down = response.hovered() && ui.ctx().input(|i| i.pointer.primary_down());
    if is_down {
        painter.rect_filled(rect, 0.0, palette::BTN_PRESSED);
    } else if response.hovered() {
        painter.rect_filled(rect, 0.0, palette::TAB_HOVER);
    }

    // 文字（垂直居中）
    let text_pos = egui::pos2(rect.min.x + padding_x, rect.min.y + padding_y);
    painter.galley(text_pos, galley, palette::TEXT);

    // 选中下划线
    if selected {
        let underline_y = rect.max.y - 1.0;
        let stroke = Stroke::new(2.0, palette::ACCENT);
        painter.line_segment(
            [
                egui::pos2(rect.min.x + 4.0, underline_y),
                egui::pos2(rect.max.x - 4.0, underline_y),
            ],
            stroke,
        );
    }

    response.clicked()
}
