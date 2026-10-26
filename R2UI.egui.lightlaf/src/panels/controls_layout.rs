//! 布局演示页：展示 FlatLaf 风格下 widgets 层原子组件的组合。
//!
//! 重点展示：
//!   - widgets::flatlaf_tab（自绘 Tab 按钮，展示选中/hover/pressed 三态）
//!   - widgets::path_row（标签 + 自适应输入框 + 浏览按钮）
//!   - Grid + Frame::group 嵌套
//!   - ScrollArea + CollapsingHeader 组合
//!   - Vertical / Horizontal 布局差异

use eframe::egui;

use crate::widgets::{flatlaf_tab, path_row, LogView, TE_V_MARGIN};
use crate::panels::Panel;

pub struct ControlsLayout {
    // path_row 演示状态
    pub dir_a: String,
    pub dir_b: String,
    pub file_a: String,
    pub file_b: String,

    // flatlaf_tab 演示：选中第几个
    tab_selected: usize,
    tab_labels: [&'static str; 4],

    // CollapsingHeader 演示
    show_header_1: bool,
    show_header_2: bool,

    log: LogView,
}

impl ControlsLayout {
    pub fn new() -> Self {
        Self {
            dir_a: String::from("D:/workspace/project_a"),
            dir_b: String::from("D:/workspace/project_b"),
            file_a: String::from("D:/workspace/data.csv"),
            file_b: String::from("D:/workspace/config.json"),
            tab_selected: 0,
            tab_labels: ["Home", "Projects", "Settings", "About"],
            show_header_1: false,
            show_header_2: false,
            log: LogView::new(),
        }
    }
}

impl Default for ControlsLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel for ControlsLayout {
    fn show(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 10.0);

                // —— flatlaf_tab 展示三态 ——
                section(ui, "flatlaf_tab（自绘 Tab 按钮）", |ui| {
                    ui.label("鼠标悬停、按下、切换观察 hover / pressed / selected 三态：");
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        for (i, label) in self.tab_labels.iter().enumerate() {
                            if flatlaf_tab(ui, self.tab_selected == i, label) {
                                self.tab_selected = i;
                            }
                        }
                    });
                    ui.separator();
                    ui.label(format!("当前选中: {}", self.tab_labels[self.tab_selected]));
                });

                // —— path_row ——
                section(ui, "path_row（标签 + 自适应输入框 + 浏览按钮）", |ui| {
                    ui.label("关键：扣除浏览按钮宽度再分配输入框剩余空间，窄窗口不溢出。");
                    ui.add_space(6.0);
                    let mut log_action = |line: &str| self.log.push_line(line);

                    if path_row(ui, "项目目录 A：", &mut self.dir_a, "可拖放目录", true) {
                        log_action("[demo] path_row 点击浏览按钮（实际 pick_folder_async 已剥离于 demo）");
                    }
                    if path_row(ui, "项目目录 B：", &mut self.dir_b, "可拖放目录", true) {
                        log_action("[demo] path_row 点击浏览按钮 B");
                    }
                    if path_row(ui, "输入文件 A：", &mut self.file_a, "可拖放文件", true) {
                        log_action("[demo] path_row 点击浏览按钮 A");
                    }
                    if path_row(ui, "输入文件 B：", &mut self.file_b, "可拖放文件", true) {
                        log_action("[demo] path_row 点击浏览按钮 B");
                    }
                    let _ = log_action;
                });

                // —— Grid ——
                section(ui, "Grid 布局", |ui| {
                    egui::Grid::new("demo_grid")
                        .num_columns(3)
                        .spacing([8.0, 6.0])
                        .show(ui, |ui| {
                            for r in 0..3 {
                                for c in 0..3 {
                                    ui.label(format!("({}, {})", r, c));
                                }
                                ui.end_row();
                            }
                        });
                });

                // —— CollapsingHeader ——
                section(ui, "CollapsingHeader 可折叠分组", |ui| {
                    ui.collapsing("点击展开 / 收起第一组", |ui| {
                        ui.label("展开后的内容区域。FlatLaf 风格下 collapsing_header_frame = false，无边框，仅折叠箭头。");
                        ui.add_space(4.0);
                        let h = crate::widgets::text_edit_height(ui);
                        ui.add(
                            egui::TextEdit::singleline(&mut String::from("在折叠分组里也能用 path_row"))
                                .desired_width(f32::INFINITY)
                                .margin(egui::Margin::symmetric(4, TE_V_MARGIN))
                                .min_size(egui::vec2(0.0, h)),
                        );
                    });
                    ui.add_space(6.0);
                    ui.collapsing("点击展开 / 收起第二组", |ui| {
                        ui.label("另一组内容。");
                    });
                    let _ = self.show_header_1; // 演示 manual collapse 可选，这里直接用 ui.collapsing 简化
                    let _ = self.show_header_2;
                });

                // —— Vertical vs Horizontal ——
                section(ui, "Vertical / Horizontal 布局对比", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Horizontal：");
                        ui.button("A");
                        ui.button("B");
                        ui.button("C");
                    });
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        ui.label("Vertical：");
                        ui.button("第一行按钮");
                        ui.button("第二行按钮");
                        ui.button("第三行按钮");
                    });
                });

                // —— 嵌套 Frame::group ——
                section(ui, "Frame::group 嵌套", |ui| {
                    egui::Frame::group(ui.style())
                        .inner_margin(egui::Margin::symmetric(10, 6))
                        .show(ui, |ui| {
                            ui.label("外层分组");
                            ui.add_space(6.0);
                            egui::Frame::group(ui.style())
                                .inner_margin(egui::Margin::symmetric(8, 4))
                                .show(ui, |ui| {
                                    ui.label("内层分组");
                                    ui.horizontal(|ui| {
                                        ui.button("按钮 X");
                                        ui.button("按钮 Y");
                                    });
                                });
                        });
                });

                // —— 底部 LogView ——
                section(ui, "LogView 实时日志", |ui| {
                    ui.label("来自 widgets::LogView 的日志区，记录了 path_row 浏览按钮点击：");
                    ui.add_space(4.0);
                    self.log.show(ui);
                });

                ui.add_space(12.0);
            });
    }
}

// ----------------------------------------------------------------------------
// 内部辅助：section = Frame::group + 粗体标题
// ----------------------------------------------------------------------------

fn section(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::symmetric(8, 6))
        .show(ui, |ui| {
            ui.label(egui::RichText::new(title).strong());
            ui.add_space(4.0);
            content(ui);
        });
}
