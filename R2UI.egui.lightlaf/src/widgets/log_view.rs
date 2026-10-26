/// 只读多行日志区，自动滚动到底部。
///
/// 布局约束依赖**父容器先限死可用高度**，然后 `show()` 用 `ui.available_size()`
/// 取剩余高度把 ScrollArea 固定在这个矩形里——文字再多只在框内滚动，绝不撑出窗口。

pub struct LogView {
    text: String,
}

impl LogView {
    pub fn new() -> Self {
        Self {
            text: "日志模块监听中：应用程序已初始化完成\n".to_owned(),
        }
    }

    /// 追加一行，自动补换行符。
    pub fn push_line(&mut self, line: &str) {
        self.text.push_str(line);
        self.text.push('\n');
    }

    /// 清空日志。
    pub fn clear(&mut self) {
        self.text.clear();
    }

    /// 占满剩余空间的内部滚动日志区。
    ///
    /// 用 `top_down_justified` 让 min_rect == max_rect，强制 LogView 吃满剩余高度，
    /// 否则子 Ui 只按内容收缩，ScrollArea 就没有真正的可视高度。
    pub fn show(&mut self, ui: &mut eframe::egui::Ui) {
        let size = ui.available_size();
        ui.allocate_ui_with_layout(
            size,
            eframe::egui::Layout::top_down_justified(eframe::egui::Align::LEFT),
            |ui| {
                eframe::egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add(
                            eframe::egui::TextEdit::multiline(&mut self.text)
                                .interactive(false)
                                .desired_width(f32::INFINITY)
                                .desired_rows(1)
                                .font(eframe::egui::TextStyle::Monospace),
                        );
                    });
            },
        );
    }
}

impl Default for LogView {
    fn default() -> Self {
        Self::new()
    }
}
