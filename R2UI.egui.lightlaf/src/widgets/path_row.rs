//! 路径选择行：标签 + 自适应宽度输入框 + "浏览"按钮。
//!
//! 不能在同一 horizontal 里给 TextEdit 传 `desired_width(f32::INFINITY)`：
//! 输入框会吃掉整行剩余宽度，把"浏览"按钮挤出裁剪区。
//! 这里先扣除按钮宽度与控件间距，输入框取剩余空间，窄窗口下也不会溢出。

use eframe::egui;

/// "浏览"按钮预留宽度（2 个汉字 + 内边距）。
pub const BROWSE_BTN_W: f32 = 56.0;

/// 单行输入框的上下内边距（整数像素）。
///
/// egui 0.35 的 [`egui::Margin`] 四边都是 `i8`（只支持整数像素），
/// 因此输入框高度只能按 1px 步长调整：`行高 + 2*TE_V_MARGIN + 1px 描边`。
/// 同行按钮用 [`text_edit_height`] 算出的同一高度 `min_size` 钉死，保证齐平。
pub const TE_V_MARGIN: i8 = 6;

/// 计算当前字体下单行 TextEdit 的实际外框高度，供同行 Button 对齐。
pub fn text_edit_height(ui: &egui::Ui) -> f32 {
    let row_h = ui.text_style_height(&egui::TextStyle::Body);
    row_h + 2.0 * TE_V_MARGIN as f32 + 1.0
}

/// 路径选择行。返回是否点击了"浏览"（`enabled=false` 时恒为 false）。
pub fn path_row(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut String,
    hint: &str,
    enabled: bool,
) -> bool {
    let mut browse = false;
    ui.horizontal(|ui| {
        ui.label(label);
        // 扣除"浏览"按钮宽度与控件间距，输入框取本行剩余空间
        let field_w =
            (ui.available_width() - BROWSE_BTN_W - ui.spacing().item_spacing.x).max(0.0);
        let h = text_edit_height(ui);
        ui.add_enabled(
            enabled,
            egui::TextEdit::singleline(value)
                .hint_text(hint)
                .desired_width(field_w)
                .margin(egui::Margin::symmetric(4, TE_V_MARGIN)),
        );
        if ui
            .add_enabled(
                enabled,
                egui::Button::new("浏览").min_size(egui::vec2(BROWSE_BTN_W, h)),
            )
            .clicked()
        {
            browse = true;
        }
    });
    browse
}
