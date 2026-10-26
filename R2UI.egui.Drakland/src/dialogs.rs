//! 对话框与独立视口窗口：
//! - [`center_on_screen`] : 计算窗口居中位置
//! - [`confirm_dialog`]  : 应用内确认对话框（egui::Window，强调色描边）
//! - [`viewport_frame`]  : 独立 viewport 窗口的通用 CentralPanel 外框

use egui::{Margin, Rounding, Stroke};

use crate::theme::ThemeColors;

/// 根据主 viewport 的 screen_rect 计算屏幕中心，返回居中窗口左上角坐标。
pub fn center_on_screen(ctx: &egui::Context, window_size: [f32; 2]) -> [f32; 2] {
    let rect = ctx.screen_rect();
    let w = rect.width();
    let h = rect.height();
    let x = rect.min.x + (w - window_size[0]) * 0.5;
    let y = rect.min.y + (h - window_size[1]) * 0.5;
    [x.max(0.0), y.max(0.0)]
}

/// 请求关闭当前所在的 viewport。
///
/// 对 `show_viewport_immediate` 创建的子窗口，点击标题栏关闭按钮只会产生
/// `close_requested` 输入状态，必须显式发送 [`egui::ViewportCommand::Close`]
/// 才能真正销毁窗口；否则父帧下一帧再次调用 show 会把窗口重建出来。
pub fn close_viewport(ctx: &egui::Context) {
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
}

/// 检测子 viewport 是否请求关闭（标题栏 X 按钮 / Esc）。
pub fn viewport_close_requested(ctx: &egui::Context) -> bool {
    ctx.input(|i| i.viewport().close_requested())
}

/// 独立 viewport 窗口的配置（Builder 风格）。
pub struct ViewportOptions<'a> {
    pub id_hash: &'a str,
    pub title: String,
    pub inner_size: [f32; 2],
    pub min_inner_size: Option<[f32; 2]>,
    pub resizable: bool,
}

impl<'a> ViewportOptions<'a> {
    pub fn new(id_hash: &'a str, title: impl Into<String>, inner_size: [f32; 2]) -> Self {
        Self {
            id_hash,
            title: title.into(),
            inner_size,
            min_inner_size: None,
            resizable: true,
        }
    }

    pub fn min_size(mut self, size: [f32; 2]) -> Self {
        self.min_inner_size = Some(size);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// 构造好 ViewportBuilder（已含标题、尺寸、居中位置）。
    pub fn builder(&self, parent_ctx: &egui::Context) -> egui::ViewportBuilder {
        let min_size = self.min_inner_size.unwrap_or(self.inner_size);
        egui::ViewportBuilder::default()
            .with_title(self.title.clone())
            .with_inner_size(self.inner_size)
            .with_min_inner_size(min_size)
            .with_resizable(self.resizable)
            .with_decorations(true)
            .with_transparent(false)
            .with_position(center_on_screen(parent_ctx, self.inner_size))
    }

    pub fn viewport_id(&self) -> egui::ViewportId {
        egui::ViewportId::from_hash_of(self.id_hash)
    }
}

/// 在独立 viewport 内绘制统一的 CentralPanel 外框（16 内边距、window_fill 底色）。
///
/// 典型用法见 demo：
/// ```ignore
/// let opts = ViewportOptions::new("my_win", "标题", [400.0, 300.0]);
/// parent_ctx.show_viewport_immediate(opts.viewport_id(), opts.builder(parent_ctx), |ctx, _| {
///     if ctx.input(|i| i.viewport().close_requested()) { /* 关闭 */ }
///     viewport_frame(ctx, &tc, |ui| { /* 内容 */ });
/// });
/// ```
pub fn viewport_frame<R>(
    ctx: &egui::Context,
    tc: &ThemeColors,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::CentralPanel::default()
        .frame(
            egui::Frame::default()
                .fill(tc.window_fill)
                .inner_margin(Margin::same(16.0)),
        )
        .show(ctx, add_contents)
}

// ------------------------------------------------------------------
// 应用内确认对话框
// ------------------------------------------------------------------

/// 确认对话框的按钮配置。
pub struct ConfirmOptions<'a> {
    pub prompt: &'a str,
    pub detail: &'a str,
    pub cancel_text: &'a str,
    pub confirm_text: &'a str,
    pub size: [f32; 2],
}

impl<'a> ConfirmOptions<'a> {
    pub fn new(prompt: &'a str, detail: &'a str, confirm_text: &'a str) -> Self {
        Self {
            prompt,
            detail,
            cancel_text: "取消",
            confirm_text,
            size: [360.0, 180.0],
        }
    }
}

/// 确认对话框的结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmOutcome {
    None,
    Cancelled,
    Confirmed,
}

/// 绘制应用内模态确认对话框（egui::Window，强调色 1.5px 描边、8 圆角）。
///
/// `open` 为当前是否显示；返回本帧结果。
pub fn confirm_dialog(
    ctx: &egui::Context,
    tc: &ThemeColors,
    open: &mut bool,
    opts: &ConfirmOptions<'_>,
) -> ConfirmOutcome {
    let mut outcome = ConfirmOutcome::None;

    let window_frame = egui::Frame::default()
        .fill(tc.window_fill)
        .rounding(Rounding::same(8.0))
        .stroke(Stroke::new(1.5, tc.accent));

    egui::Window::new("")
        .open(open)
        .collapsible(false)
        .resizable(false)
        .fixed_size(egui::vec2(opts.size[0], opts.size[1]))
        .frame(window_frame)
        .show(ctx, |ui| {
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    ui.label(opts.prompt);
                    ui.add_space(8.0);
                    ui.label(opts.detail);
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.add_space(40.0);
                        if ui.button(opts.cancel_text).clicked() {
                            outcome = ConfirmOutcome::Cancelled;
                        }
                        ui.add_space(20.0);
                        if ui.button(opts.confirm_text).clicked() {
                            outcome = ConfirmOutcome::Confirmed;
                        }
                    });
                    ui.add_space(16.0);
                });
                ui.add_space(16.0);
            });
        });

    // 用户点了窗口标题栏的关闭按钮
    if !*open && outcome == ConfirmOutcome::None {
        outcome = ConfirmOutcome::Cancelled;
    }

    outcome
}
