//! 布局框架：
//! - [`top_bar`]       : 72px 顶部状态/操作栏
//! - [`SplitSidebar`]  : 中间带可拖拽分割条的双分区侧边栏
//! - [`adaptive_grid`] : 按可用宽度自适应 1~4 列的卡片网格
//! - [`selectable_row`]: 主从结构（Master-Detail）中的可选中行列表

use egui::{Color32, RichText, Ui};

use crate::theme::ThemeColors;

pub mod sidebar;

pub use sidebar::{SplitSidebar, SplitSidebarResponse};

/// 顶栏高度（与源项目一致）。
pub const TOP_BAR_HEIGHT: f32 = 72.0;

/// 绘制顶部栏（固定 72px、不可调整高度）。
///
/// `left_content` 在左侧垂直排列（状态行 + 元信息），
/// `right_content` 在右侧垂直居中排列（操作按钮）。
pub fn top_bar<R>(
    ctx: &egui::Context,
    id_source: &str,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::TopBottomPanel::top(egui::Id::new(id_source))
        .resizable(false)
        .exact_height(TOP_BAR_HEIGHT)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            let ret = add_contents(ui);
            ui.add_space(8.0);
            ret
        })
}

/// 顶栏常用的左右两栏布局：左侧状态，右侧按钮。
pub fn top_bar_contents<R>(
    ui: &mut Ui,
    left: impl FnOnce(&mut Ui),
    right: impl FnOnce(&mut Ui) -> R,
) -> R {
    let mut right_result: Option<R> = None;
    ui.horizontal(|ui| {
        ui.add_space(16.0);
        ui.vertical(|ui| {
            left(ui);
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(16.0);
            right_result = Some(right(ui));
            ui.add_space(12.0);
        });
    });
    right_result.expect("right content always runs")
}

/// 自适应卡片网格：根据可用宽度计算列数（卡片基准宽 280，1~4 列，间距 12）。
///
/// 对每个条目调用 `item`，传入 `(ui, computed_item_width, index)`。
pub fn adaptive_grid<T>(
    ui: &mut Ui,
    id_source: &str,
    items: &[T],
    mut item: impl FnMut(&mut Ui, f32, &T),
) {
    let available_width = ui.available_width();
    let columns = ((available_width / 280.0) as usize).clamp(1, 4);
    let item_width = (available_width - (columns as f32 - 1.0) * 12.0) / columns as f32;

    egui::Grid::new(id_source)
        .spacing([12.0, 12.0])
        .show(ui, |ui| {
            for (i, data) in items.iter().enumerate() {
                if i > 0 && i % columns == 0 {
                    ui.end_row();
                }
                item(ui, item_width, data);
            }
        });
}

// ------------------------------------------------------------------
// 主从列表（Master-Detail）
// ------------------------------------------------------------------

/// 列表行的描述数据。
pub struct ListRow<'a> {
    pub id: &'a str,
    pub title: &'a str,
    /// 右侧计数/附加文本（如 "12 项"）。
    pub badge: Option<&'a str>,
}

/// 绘制一条可选中的列表行，返回是否被点击。
///
/// 选中行使用强调色半透明底，标题变为强调色。
pub fn selectable_row(ui: &mut Ui, tc: &ThemeColors, row: &ListRow<'_>, selected: bool, right_badge_width: f32) -> bool {
    let row_bg = if selected { tc.selected_row_bg() } else { Color32::TRANSPARENT };

    let frame = egui::Frame::default()
        .fill(row_bg)
        .inner_margin(egui::Margin::symmetric(4.0, 3.0));

    let resp = frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add_sized(
                [ui.available_width() - right_badge_width, 22.0],
                egui::Label::new(
                    RichText::new(row.title)
                        .color(if selected { tc.accent } else { tc.text_primary })
                        .size(11.0),
                ),
            );
            if let Some(badge) = row.badge {
                ui.add(
                    egui::Label::new(
                        RichText::new(badge).color(tc.text_secondary).size(10.0).monospace(),
                    ),
                );
            }
        });
    });

    resp.response.interact(egui::Sense::click()).clicked()
}

/// 侧边栏分类项（`名称 (数量)` 的 selectable_label 风格）。
pub fn category_item(ui: &mut Ui, name: &str, count: usize, selected: bool) -> bool {
    // 源项目使用 6px 左缩进 + selectable_label
    let mut clicked = false;
    ui.horizontal(|ui| {
        ui.add_space(6.0);
        let label = format!("{} ({})", name, count);
        if ui.selectable_label(selected, label).clicked() {
            clicked = true;
        }
        ui.allocate_space(ui.available_size());
    });
    clicked
}
