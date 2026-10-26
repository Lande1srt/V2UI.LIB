//! 卡片组件：统一圆角边框容器、工具卡片、链接卡片。

use egui::{Margin, RichText, Rounding, Ui};

use crate::theme::ThemeColors;
use crate::widgets::badge::{parse_title_badges, Badge};
use crate::widgets::inline_link;

/// 下载状态三态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadState {
    NotDownloaded,
    Downloaded,
    Downloading,
}

/// 卡片底部可触发的动作（由业务侧匹配处理）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardAction {
    /// 下载（未下载卡片）
    Download,
    /// 启动（已下载卡片）
    Launch,
    /// 修复
    Repair,
    /// 打开所在路径
    OpenPath,
    /// 删除
    Delete,
    /// 取消下载
    CancelDownload,
    /// 打开链接（链接卡片）
    OpenLink,
}

/// 统一的卡片外框：group 样式、12 内边距、8 圆角。
pub struct CardFrame;

impl CardFrame {
    pub fn show<R>(ui: &mut Ui, width: f32, add_contents: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        ui.vertical(|ui| {
            ui.set_width(width);

            let frame = egui::Frame::group(ui.style())
                .inner_margin(Margin::same(12.0))
                .rounding(Rounding::same(8.0));

            frame.show(ui, |ui| {
                ui.set_width((width - 24.0).max(0.0));
                add_contents(ui)
            }).inner
        })
    }
}

/// 工具卡片的展示数据（与业务模型解耦）。
pub struct ToolCardData<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub state: DownloadState,
    /// 已下载时展示的体积文本（如 "12.3 MB"）。
    pub downloaded_size: Option<&'a str>,
    /// 下载中：0.0..=100.0
    pub progress_percent: f64,
    /// 下载中：已下载 / 总大小文本。
    pub progress_detail: Option<&'a str>,
}

/// 绘制工具卡片，返回被点击的动作（若有）。
pub fn tool_card(ui: &mut Ui, tc: &ThemeColors, width: f32, data: &ToolCardData<'_>) -> Option<CardAction> {
    let mut action: Option<CardAction> = None;

    CardFrame::show(ui, width, |ui| {
        // 标题行（含徽标，右对齐排列）
        ui.horizontal(|ui| {
            let (title_text, badges) = parse_title_badges(data.title);
            ui.label(RichText::new(title_text).color(tc.card_title).size(14.0).strong());
            if !badges.is_empty() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    for badge in badges.iter().rev() {
                        Badge::new(badge).show(ui, tc);
                        ui.add_space(4.0);
                    }
                });
            }
        });
        ui.add_space(4.0);

        ui.label(RichText::new(data.description).size(12.0).weak().color(tc.card_desc));
        ui.add_space(8.0);

        // 状态行
        match data.state {
            DownloadState::Downloaded => {
                let size = data.downloaded_size.unwrap_or("");
                ui.label(
                    RichText::new(format!("已下载 ({})", size))
                        .color(tc.downloaded)
                        .size(12.0),
                );
            }
            DownloadState::Downloading => {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("下载中...").color(tc.downloading).size(12.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if inline_link(ui, "取消下载", tc.cancel_text).clicked() {
                            action = Some(CardAction::CancelDownload);
                        }
                    });
                });
            }
            DownloadState::NotDownloaded => {
                ui.label(RichText::new("未下载").color(tc.not_downloaded).size(12.0));
            }
        }

        ui.add_space(8.0);

        // 进度 / 操作按钮
        match data.state {
            DownloadState::Downloading => {
                let progress = (data.progress_percent / 100.0) as f32;
                ui.add(egui::ProgressBar::new(progress).show_percentage());
                ui.add_space(4.0);
                let detail = data.progress_detail.unwrap_or("");
                ui.label(format!("{:.1}% - {}", data.progress_percent, detail));
            }
            DownloadState::Downloaded => {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        if ui.button("启动").clicked() {
                            action = Some(CardAction::Launch);
                        }
                        ui.add_space(6.0);
                        if ui.button("修复").clicked() {
                            action = Some(CardAction::Repair);
                        }
                        ui.add_space(6.0);
                        if ui.button("路径").clicked() {
                            action = Some(CardAction::OpenPath);
                        }
                        ui.add_space(6.0);
                        if ui.button("删除").clicked() {
                            action = Some(CardAction::Delete);
                        }
                    });
                });
            }
            DownloadState::NotDownloaded => {
                if ui.button("下载").clicked() {
                    action = Some(CardAction::Download);
                }
            }
        }
    });

    action
}

/// 链接卡片的展示数据。
pub struct LinkCardData<'a> {
    pub title: &'a str,
    pub description: &'a str,
}

/// 绘制链接卡片，返回是否点击“打开链接”。
pub fn link_card(ui: &mut Ui, tc: &ThemeColors, width: f32, data: &LinkCardData<'_>) -> bool {
    let mut clicked = false;

    CardFrame::show(ui, width, |ui| {
        ui.label(RichText::new(data.title).color(tc.card_title).size(14.0).strong());
        ui.add_space(4.0);

        ui.label(RichText::new(data.description).size(12.0).weak().color(tc.card_desc));
        ui.add_space(10.0);

        if ui
            .add_sized([(width - 48.0).max(0.0), 30.0], egui::Button::new("打开链接"))
            .clicked()
        {
            clicked = true;
        }
    });

    clicked
}
