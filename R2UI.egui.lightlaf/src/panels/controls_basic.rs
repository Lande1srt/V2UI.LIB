//! 基础控件展示页：按"按钮 / 输入框 / 下拉框 / 复选框 / 单选 / 进度条 / 滑块 / 色板"分组。
//!
//! 设计对齐参考图：每个分组用 egui::Frame::group 包裹，分组标题用粗体 Label，
//! 分组内排列同名控件的多种状态（普通 / 禁用 / hover）。

use eframe::egui;

use crate::widgets::{LogView, TE_V_MARGIN};
use crate::panels::Panel;

pub struct ControlsBasic {
    // —— 输入框演示状态 ——
    pub input_normal: String,
    pub input_disabled: String,

    // —— 下拉框演示状态 ——
    pub combo_idx: usize,
    combo_open: bool,

    // —— 复选框 ——
    pub cb_unchecked: bool,
    pub cb_checked: bool,

    // —— 单选组 ——
    pub radio: usize,

    // —— 进度条 ——
    pub progress: f32,

    // —— 滑块 ——
    pub slider: f32,

    // —— 色板 ——
    pub color: egui::Color32,

    // —— LogView（复用 widgets 原子组件） ——
    log: LogView,
}

impl ControlsBasic {
    pub fn new() -> Self {
        Self {
            input_normal: String::from("普通输入框"),
            input_disabled: String::from("禁用输入框"),
            combo_idx: 0,
            combo_open: false,
            cb_unchecked: false,
            cb_checked: true,
            radio: 1,
            progress: 0.45,
            slider: 0.30,
            color: egui::Color32::from_rgb(38, 117, 191),
            log: LogView::new(),
        }
    }
}

impl Default for ControlsBasic {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel for ControlsBasic {
    fn show(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 10.0);

                // —— 按钮 ——
                group(ui, "按钮", |ui| {
                    ui.horizontal(|ui| {
                        ui.button("普通按钮");
                    ui.add_enabled(false, egui::Button::new("禁用按钮"));
                    ui.add(
                        egui::Button::new("悬停效果")
                            .min_size(egui::vec2(88.0, 0.0)),
                    ).on_hover_text("鼠标悬停观察 hover 背景与琥珀黄描边");
                    });
                });

                // —— 输入框 ——
                group(ui, "输入框", |ui| {
                    let h = crate::widgets::text_edit_height(ui);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.input_normal)
                            .hint_text("普通输入框，可编辑")
                            .desired_width(f32::INFINITY)
                            .margin(egui::Margin::symmetric(4, TE_V_MARGIN))
                            .min_size(egui::vec2(0.0, h)),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.input_disabled)
                            .desired_width(f32::INFINITY)
                            .margin(egui::Margin::symmetric(4, TE_V_MARGIN))
                            .min_size(egui::vec2(0.0, h))
                            .interactive(false),
                    );
                });

                // —— 下拉框 ——
                group(ui, "下拉框", |ui| {
                    let options = ["选项1", "选项2", "选项3", "选项4"];

                    egui::ComboBox::from_id_salt("demo_combo_normal")
                        .selected_text(options[self.combo_idx])
                        .show_ui(ui, |ui| {
                            for (i, opt) in options.iter().enumerate() {
                                ui.selectable_value(&mut self.combo_idx, i, *opt);
                            }
                        });
                    ui.label(
                        egui::RichText::new("下拉框支持键盘导航、滚动条、hover 高亮")
                            .size(10.0)
                            .color(ui.visuals().weak_text_color()),
                    );
                    let _ = self.combo_open;
                });

                // —— 复选框 ——
                group(ui, "复选框", |ui| {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.cb_unchecked, "未选中");
                        ui.checkbox(&mut self.cb_checked, "已选中");
                        ui.add_enabled(false, egui::Checkbox::new(&mut false, "禁用"));
                    });
                });

                // —— 单选 ——
                group(ui, "单选按钮", |ui| {
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.radio, 0, "选项 A");
                        ui.radio_value(&mut self.radio, 1, "选项 B");
                        ui.radio_value(&mut self.radio, 2, "选项 C");
                    });
                });

                // —— 进度条 ——
                group(ui, "进度条", |ui| {
                    ui.horizontal(|ui| {
                        ui.add(egui::ProgressBar::new(self.progress).text(format!("{:.0}%", self.progress * 100.0)));
                    });
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("值:");
                        ui.add(egui::Slider::new(&mut self.progress, 0.0..=1.0));
                    });
                });

                // —— 滑块 ——
                group(ui, "滑块", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("音量:");
                        ui.add(egui::Slider::new(&mut self.slider, 0.0..=1.0));
                        ui.label(format!("{:.0}%", self.slider * 100.0));
                    });
                });

                // —— 色板 ——
                group(ui, "色板 / ColorPicker", |ui| {
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut self.color);
                        ui.label(format!("当前颜色: {:?}", self.color));
                    });
                });

                // —— 日志区（复用 widgets::LogView） ——
                group(ui, "LogView（widgets 原子组件）", |ui| {
                    ui.label("来自 widgets::LogView，吃满剩余空间，自动滚到底部：");
                    ui.add_space(4.0);
                    // 演示几个 push_line
                    self.log.show(ui);
                });

                ui.add_space(12.0);
            });
    }
}

// ----------------------------------------------------------------------------
// 内部辅助：Frame::group + 标题 Label
// ----------------------------------------------------------------------------

fn group(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(title).strong());
            ui.add_space(4.0);
            content(ui);
        });
}
