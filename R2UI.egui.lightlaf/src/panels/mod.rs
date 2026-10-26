//! 控件展示页：按 Tab 切分「基础控件」和「布局演示」。
//!
//! 与原业务骨架的主要差异：
//!   - Tab 枚举改成控件展示分类
//!   - 每个面板只负责渲染自己的那一类控件
//!   - Panel trait 签名不变，main.rs 统一分发

pub mod controls_basic;
pub mod controls_layout;

pub use controls_basic::ControlsBasic;
pub use controls_layout::ControlsLayout;

/// 顶部标签页 —— 控件展示分类。
#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Basic,
    Layout,
}

impl Tab {
    pub const ALL: [Tab; 2] = [Tab::Basic, Tab::Layout];

    pub fn label(self) -> &'static str {
        match self {
            Tab::Basic => "基础控件",
            Tab::Layout => "布局演示",
        }
    }
}

/// 统一 show 签名：所有控件展示面板都接受当前 Ui 和 Context。
pub trait Panel {
    fn show(&mut self, ui: &mut eframe::egui::Ui, ctx: &eframe::egui::Context);
}
