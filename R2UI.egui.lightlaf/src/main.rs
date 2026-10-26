//! FlatLaf Light / Dark 风格 egui 控件演示 demo 入口。
//!
//! 分层结构：
//!   theme/     FlatLaf Light / Dark 两套视觉值
//!   widgets/   自绘 flatlaf_tab / path_row / LogView / file_dialog / dialog
//!   panels/    控件展示页（基础控件 + 布局演示）
//!
//! 启动流程：
//!   1. fluent-egui Fluent 浅色主题打底
//!   2. 覆盖 FlatLaf Light（默认）
//!   3. 加载系统 CJK 字体
//!   4. 运行 DemoApp，顶部亮色/暗色按钮随时切换

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod theme;
mod widgets;
mod panels;

use eframe::egui;
use fluent_egui::winui::{FluentTheme, ThemeConfig, apply_theme};

use panels::Tab;
use panels::Panel;
use panels::{ControlsBasic, ControlsLayout};

// =============================================================================
// 应用主体
// =============================================================================

struct DemoApp {
    /// 当前选中的控件展示分类
    tab: Tab,
    /// 当前主题（Light = false / Dark = true）
    dark: bool,
    basic: ControlsBasic,
    layout: ControlsLayout,
}

impl DemoApp {
    fn new() -> Self {
        Self {
            tab: Tab::Basic,
            dark: false,
            basic: ControlsBasic::new(),
            layout: ControlsLayout::new(),
        }
    }

    /// 切换主题：重新 apply_*_theme，egui 的 Context 会立即生效。
    fn apply_current_theme(&self, ctx: &egui::Context) {
        if self.dark {
            theme::apply_flatlaf_dark(ctx);
        } else {
            theme::apply_flatlaf_tweaks(ctx);
        }
    }
}

impl eframe::App for DemoApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // 窗口边缘颜色必须与 panel_fill 一致，否则会出现一条异色边框
        if self.dark {
            [43.0 / 255.0, 43.0 / 255.0, 43.0 / 255.0, 1.0]
        } else {
            [242.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0, 1.0]
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 每帧 apply 一次主题：允许用户点按钮后立即生效
        let ctx = ui.ctx().clone();
        self.apply_current_theme(&ctx);

        egui::CentralPanel::default().show(ui, |ui| {
            // —— 顶栏：亮色 / 暗色 主题切换 ——
            ui.horizontal(|ui| {
                let light_btn = egui::Button::new("亮色主题");
                let dark_btn = egui::Button::new("暗色主题");

                // 让当前主题按钮带更突出的描边（用 add_enabled + add_enabled(false) 切换是 FlatLaf 原生行为）
                if ui.add_enabled(!self.dark, light_btn).clicked() {
                    self.dark = false;
                }
                if ui.add_enabled(self.dark, dark_btn).clicked() {
                    self.dark = true;
                }
            });
            ui.add_space(4.0);

            // —— 自绘 Tab 栏 ——
            ui.horizontal(|ui| {
                for &tab in &Tab::ALL {
                    let selected = self.tab == tab;
                    if widgets::flatlaf_tab(ui, selected, tab.label()) {
                        self.tab = tab;
                    }
                }
            });
            ui.separator();

            // Tab body：锁死在 CentralPanel 剩余高度内 + 内外边距
            let ctx = ui.ctx().clone();
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(14, 10))
                .show(ui, |ui| {
                    let body_size = ui.available_size();
                    ui.allocate_ui(body_size, |ui| {
                        match self.tab {
                            Tab::Basic => self.basic.show(ui, &ctx),
                            Tab::Layout => self.layout.show(ui, &ctx),
                        }
                    });
                });
        });
    }
}

// =============================================================================
// 入口
// =============================================================================

fn main() -> eframe::Result<()> {
    let viewport = egui::ViewportBuilder::default()
        .with_title("R2UI FlatLaf 控件演示")
        .with_inner_size([900.0, 600.0])
        .with_min_inner_size([600.0, 400.0])
        .with_maximize_button(false);

    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "R2UI FlatLaf Controls Demo",
        options,
        Box::new(|cc| {
            // 1) fluent-egui Fluent 浅色打底
            apply_theme(&cc.egui_ctx, ThemeConfig::new(FluentTheme::Light));
            // 2) FlatLaf Light 覆盖
            theme::apply_flatlaf_tweaks(&cc.egui_ctx);
            // 3) CJK 字体
            theme::setup_chinese_fonts(&cc.egui_ctx);
            Ok(Box::new(DemoApp::new()))
        }),
    )
}
