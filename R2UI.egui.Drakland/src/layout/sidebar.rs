//! 双分区侧边栏：顶部“工具分类”与底部“链接分类”，
//! 中间通过一条可上下拖拽的分割条调整两区高度比例。

use egui::Ui;

use crate::theme::ThemeColors;

/// 拖拽分割条后回传的状态。
#[derive(Default)]
pub struct SplitSidebarResponse {
    /// 新的上半区高度比例（0.0..=1.0），仅在拖拽帧变化时有效。
    pub new_ratio: Option<f32>,
}

/// 双分区侧栏构造器。
pub struct SplitSidebar<'a> {
    pub id_suffix: &'a str,
    pub tc: &'a ThemeColors,
    /// 上半区标题（如“工具分类”）。
    pub top_title: &'a str,
    /// 下半区标题（如“链接分类”）。
    pub bottom_title: &'a str,
    /// 上半区当前高度比例。
    pub ratio: f32,
}

impl<'a> SplitSidebar<'a> {
    pub fn new(id_suffix: &'a str, tc: &'a ThemeColors, top_title: &'a str, bottom_title: &'a str, ratio: f32) -> Self {
        Self {
            id_suffix,
            tc,
            top_title,
            bottom_title,
            ratio: ratio.clamp(0.1, 0.9),
        }
    }

    /// 在已有的 SidePanel `ui` 内绘制双分区。
    ///
    /// `top_section` / `bottom_section` 分别绘制两个可滚动列表的内容
    /// （通常在内部使用 `ScrollArea::vertical`）。
    pub fn show<R1, R2>(
        self,
        ui: &mut Ui,
        top_section: impl FnOnce(&mut Ui) -> R1,
        bottom_section: impl FnOnce(&mut Ui) -> R2,
    ) -> (SplitSidebarResponse, R1, R2) {
        let tc = self.tc;
        ui.add_space(12.0);

        let sidebar_height = ui.available_height();
        let top_height = (sidebar_height * self.ratio) as f32;

        // ---- 上半区 ----
        let top_ret = ui
            .vertical(|ui| {
                ui.set_max_height(top_height - 6.0);
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    crate::widgets::category_label(ui, tc, self.top_title);
                });
                ui.add_space(4.0);
                top_section(ui)
            })
            .inner;

        // ---- 分割条 ----
        let separator_rect = egui::Rect::from_min_size(
            ui.next_widget_position(),
            egui::vec2(ui.available_width(), 1.0),
        );
        let separator_response = ui.interact(
            separator_rect,
            ui.id().with(("sidebar_separator", self.id_suffix)),
            egui::Sense::drag(),
        );

        let painter = ui.painter();
        painter.rect_filled(separator_rect, egui::Rounding::same(0.0), tc.sidebar_separator);
        painter.rect_stroke(
            separator_rect,
            egui::Rounding::same(0.0),
            egui::Stroke::new(1.0, tc.sidebar_separator_stroke),
        );

        ui.add_space(12.0);

        let mut response = SplitSidebarResponse::default();
        if separator_response.dragged() {
            let delta_y = separator_response.drag_delta().y;
            let new_top = (top_height + delta_y).max(60.0).min(sidebar_height - 60.0);
            response.new_ratio = Some(new_top / sidebar_height);
        }

        // ---- 下半区 ----
        let bottom_ret = ui
            .vertical(|ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    crate::widgets::category_label(ui, tc, self.bottom_title);
                });
                ui.add_space(4.0);
                bottom_section(ui)
            })
            .inner;

        (response, top_ret, bottom_ret)
    }
}
