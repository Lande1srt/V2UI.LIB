//! 简易模态弹窗（类似 Java JOptionPane / WinForms MessageBox）。

use eframe::egui;

/// 错误弹窗：居中、不可缩放、带"确定"按钮。
///
/// `error` 为 `Some(message)` 时弹窗显示；点击确定后自动置为 `None`。
pub fn show_error_window(ctx: &egui::Context, error: &mut Option<String>) {
    let Some(message) = error.clone() else {
        return;
    };
    let mut close = false;
    egui::Window::new("错误")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .min_width(280.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.label(message);
            ui.add_space(8.0);
            ui.vertical_centered(|ui| {
                if ui.button("确定").clicked() {
                    close = true;
                }
            });
        });
    if close {
        *error = None;
    }
}
