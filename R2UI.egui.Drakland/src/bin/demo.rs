//! R2UI.egui 组件库 Demo
//!
//! 复现源项目 iWinFix R 的界面构成（顶栏 + 双分区侧栏 + 卡片网格 + 对话框），
//! 但完全基于抽离后的 r2ui 组件库构建，用于验证视觉一致性与复用方式。
//!
//! 运行：cargo run --bin r2ui_demo

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::egui;
use r2ui::dialogs::{ConfirmOptions, ConfirmOutcome, ViewportOptions};
use r2ui::layout::{
    adaptive_grid, category_item, selectable_row, top_bar, top_bar_contents, ListRow,
    SplitSidebar,
};
use r2ui::widgets::card::{link_card, tool_card, CardAction, DownloadState, LinkCardData, ToolCardData};
use r2ui::widgets::{page_title, search_box, section_title};
use r2ui::{apply_theme_kind, fonts, ThemeColors, ThemeKind};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("R2UI.egui Demo")
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "R2UI Demo",
        options,
        Box::new(|cc| {
            let kind = ThemeKind::Dark;
            apply_theme_kind(&cc.egui_ctx, kind);
            fonts::load_chinese_fonts(&cc.egui_ctx);
            Ok(Box::new(DemoApp::new(kind)))
        }),
    )
}

// ------------------------------------------------------------------
// 数据模型（演示用，源项目业务模型的极简版）
// ------------------------------------------------------------------

#[derive(Clone)]
struct DemoTool {
    title: String,
    description: String,
    state: DownloadState,
    size: String,
    progress: f64,
}

#[derive(Clone)]
struct DemoLink {
    title: String,
    description: String,
}

#[derive(PartialEq, Clone, Copy)]
enum View {
    Home,
    Tools,
    Links,
    List,
}

struct DemoApp {
    kind: ThemeKind,
    view: View,
    search: String,
    selected_tool_cat: Option<String>,
    selected_link_cat: Option<String>,
    sidebar_ratio: f32,
    selected_rule: Option<String>,
    show_confirm_delete: bool,
    show_about: bool,

    tool_cats: Vec<(String, Vec<DemoTool>)>,
    link_cats: Vec<(String, Vec<DemoLink>)>,
    rules: Vec<(String, String)>,
}

impl DemoApp {
    fn new(kind: ThemeKind) -> Self {
        let tool_cats = vec![
            (
                "系统修复".to_string(),
                vec![
                    DemoTool {
                        title: "DISM 修复 [推荐]".to_string(),
                        description: "使用 DISM 修复系统映像".to_string(),
                        state: DownloadState::Downloaded,
                        size: "12.3 MB".to_string(),
                        progress: 100.0,
                    },
                    DemoTool {
                        title: "SFC 扫描 [新]".to_string(),
                        description: "扫描并修复受保护的系统文件".to_string(),
                        state: DownloadState::NotDownloaded,
                        size: String::new(),
                        progress: 0.0,
                    },
                    DemoTool {
                        title: "网络重置".to_string(),
                        description: "重置 Winsock 与 TCP/IP 协议栈".to_string(),
                        state: DownloadState::Downloading,
                        size: String::new(),
                        progress: 42.0,
                    },
                ],
            ),
            (
                "清理优化".to_string(),
                vec![DemoTool {
                    title: "磁盘清理".to_string(),
                    description: "清理临时文件与系统缓存".to_string(),
                    state: DownloadState::Downloaded,
                    size: "3.1 MB".to_string(),
                    progress: 100.0,
                }],
            ),
        ];

        let link_cats = vec![(
            "官方资源".to_string(),
            vec![
                DemoLink {
                    title: "项目主页".to_string(),
                    description: "访问 R2UI 组件库官方网站".to_string(),
                },
                DemoLink {
                    title: "使用文档".to_string(),
                    description: "查阅组件库 API 与示例".to_string(),
                },
            ],
        )];

        let rules = vec![
            ("rule-dism", "DISM 系统映像修复", "12 项"),
            ("rule-sfc", "SFC 系统文件扫描", "5 项"),
            ("rule-net", "网络组件重置", "8 项"),
            ("rule-raw", "原始注册表脚本", ""),
        ]
        .into_iter()
        .map(|(id, desc, count)| (id.to_string(), format!("{}|{}", desc, count)))
        .collect();

        Self {
            kind,
            view: View::Home,
            search: String::new(),
            selected_tool_cat: None,
            selected_link_cat: None,
            sidebar_ratio: 0.5,
            selected_rule: None,
            show_confirm_delete: false,
            show_about: false,
            tool_cats,
            link_cats,
            rules,
        }
    }

    fn tc(&self) -> ThemeColors {
        ThemeColors::new(self.kind)
    }

    fn toggle_theme(&mut self, ctx: &egui::Context) {
        self.kind = match self.kind {
            ThemeKind::Dark => ThemeKind::Light,
            ThemeKind::Light => ThemeKind::Dark,
        };
        apply_theme_kind(ctx, self.kind);
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let tc = self.tc();

        // ---------------- 顶部栏 ----------------
        top_bar(ctx, "top_bar", |ui| {
            top_bar_contents(
                ui,
                |ui| {
                    r2ui::widgets::status_line(ui, &tc, "就绪 - R2UI 组件库演示");
                    ui.add_space(2.0);
                    r2ui::widgets::meta_line(ui, &tc, "数据版本: 1.0.0 | 更新时间: 2026-10-17 | 提交者: R2UI");
                },
                |ui| {
                    if ui.button("关于").clicked() {
                        self.show_about = true;
                    }
                    ui.add_space(6.0);
                    let theme_label = if self.kind.is_dark() { "浅色模式" } else { "深色模式" };
                    if ui.button(theme_label).clicked() {
                        self.toggle_theme(ctx);
                    }
                },
            );
        });

        // ---------------- 侧边栏 ----------------
        // 闭包内只收集“想做什么”，闭包外再统一修改 self，避免多重借用。
        #[derive(Default)]
        struct SidebarMsg {
            home: bool,
            pick_tool: Option<String>,
            pick_link: Option<String>,
            new_ratio: Option<f32>,
        }
        let mut msg = SidebarMsg::default();

        egui::SidePanel::left("sidebar")
            .default_width(130.0)
            .width_range(124.0..=200.0)
            .resizable(true)
            .frame(egui::Frame::side_top_panel(&ctx.style()).fill(tc.sidebar_fill))
            .show(ctx, |ui| {
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    if ui
                        .add_sized([ui.available_width(), 36.0], egui::Button::new("首页"))
                        .clicked()
                    {
                        msg.home = true;
                    }
                });

                // 双分区（可拖拽分割条）
                let link_cats = self.link_cats.clone();
                let tool_cats = self.tool_cats.clone();
                let ratio = self.sidebar_ratio;
                let sel_tool = self.selected_tool_cat.clone();
                let sel_link = self.selected_link_cat.clone();

                let (split_resp, picked_tool, picked_link) =
                    SplitSidebar::new("main", &tc, "工具分类", "链接分类", ratio).show(
                    ui,
                    |ui| {
                        let mut picked: Option<String> = None;
                        egui::ScrollArea::vertical().id_source("tool_cat_scroll").show(ui, |ui| {
                            for (name, tools) in &tool_cats {
                                if category_item(ui, name, tools.len(), sel_tool.as_deref() == Some(name.as_str())) {
                                    picked = Some(name.clone());
                                }
                            }
                        });
                        picked
                    },
                    |ui| {
                        let mut picked: Option<String> = None;
                        egui::ScrollArea::vertical().id_source("link_cat_scroll").show(ui, |ui| {
                            for (name, links) in &link_cats {
                                if category_item(ui, name, links.len(), sel_link.as_deref() == Some(name.as_str())) {
                                    picked = Some(name.clone());
                                }
                            }
                        });
                        picked
                    },
                );

                msg.pick_tool = picked_tool;
                msg.pick_link = picked_link;
                msg.new_ratio = split_resp.new_ratio;
            });

        // 统一处理侧栏消息
        if msg.home {
            self.view = View::Home;
            self.selected_tool_cat = None;
            self.selected_link_cat = None;
            self.search.clear();
        }
        if let Some(name) = msg.pick_tool {
            self.selected_tool_cat = Some(name);
            self.selected_link_cat = None;
            self.search.clear();
            self.view = View::Tools;
        }
        if let Some(name) = msg.pick_link {
            self.selected_link_cat = Some(name);
            self.selected_tool_cat = None;
            self.search.clear();
            self.view = View::Links;
        }
        if let Some(new_ratio) = msg.new_ratio {
            self.sidebar_ratio = new_ratio;
        }

        // ---------------- 中央内容 ----------------
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().id_source("central_scroll").show(ui, |ui| {
                ui.add_space(12.0);

                if self.view != View::Home {
                    ui.horizontal(|ui| {
                        if self.view != View::Tools || self.search.is_empty() {
                            let title = match self.view {
                                View::Tools => self.selected_tool_cat.clone().unwrap_or_default(),
                                View::Links => self.selected_link_cat.clone().unwrap_or_default(),
                                View::List => "主从列表示例".to_string(),
                                View::Home => String::new(),
                            };
                            page_title(ui, &tc, &title);
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            search_box(ui, &tc, &mut self.search, "global_search");
                        });
                    });
                    ui.add_space(8.0);
                }

                match self.view {
                    View::Home => self.draw_home(ui, &tc),
                    View::Tools => {
                        if let Some(cat) = self.selected_tool_cat.clone() {
                            if let Some(tools) = self.tool_cats.iter().find(|(n, _)| n == &cat).map(|(_, v)| v.clone()) {
                                self.draw_tool_grid(ui, &tc, &tools);
                            }
                        }
                    }
                    View::Links => {
                        if let Some(cat) = self.selected_link_cat.clone() {
                            if let Some(links) = self.link_cats.iter().find(|(n, _)| n == &cat).map(|(_, v)| v.clone()) {
                                self.draw_link_grid(ui, &tc, &links);
                            }
                        }
                    }
                    View::List => self.draw_rule_list(ui, &tc),
                }

                ui.add_space(16.0);
            });
        });

        // ---------------- 确认对话框 ----------------
        if self.show_confirm_delete {
            let mut open = true;
            let opts = ConfirmOptions::new("确定要删除此工具吗？", "此操作将删除工具所在的整个文件夹", "确认删除");
            let outcome = r2ui::dialogs::confirm_dialog(ctx, &tc, &mut open, &opts);
            match outcome {
                ConfirmOutcome::Confirmed | ConfirmOutcome::Cancelled => {
                    self.show_confirm_delete = false;
                }
                ConfirmOutcome::None => {}
            }
        }

        // ---------------- 关于（独立 viewport） ----------------
        if self.show_about {
            let opts = ViewportOptions::new("about_viewport", "关于 R2UI", [440.0, 360.0]).resizable(false);
            let tc2 = self.tc();
            let mut close = false;
            ctx.show_viewport_immediate(opts.viewport_id(), opts.builder(ctx), move |ctx, _| {
                // 标题栏 X 按钮：immediate viewport 必须显式发送 Close 命令才会销毁
                if r2ui::dialogs::viewport_close_requested(ctx) {
                    r2ui::dialogs::close_viewport(ctx);
                    close = true;
                }
                r2ui::dialogs::viewport_frame(ctx, &tc2, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("R2UI.egui").color(tc2.accent).size(20.0).strong());
                        ui.add_space(4.0);
                        ui.label(egui::RichText::new("可复用的 egui 组件库").color(tc2.text_secondary));
                    });
                    ui.add_space(12.0);
                    ui.label("本 Demo 完全基于从 iWinFix R 抽离的组件构建：");
                    ui.label("• theme   深/浅主题与语义色板");
                    ui.label("• fonts   中文字体加载");
                    ui.label("• widgets 徽标 / 卡片 / 搜索框");
                    ui.label("• layout  顶栏 / 双分区侧栏 / 网格");
                    ui.label("• dialogs 确认框 / 居中视口");
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(8.0);
                    if ui.add_sized([100.0, 32.0], egui::Button::new("确定")).clicked() {
                        r2ui::dialogs::close_viewport(ui.ctx());
                        close = true;
                    }
                });
            });
            if close {
                self.show_about = false;
            }
        }
    }
}

impl DemoApp {
    fn draw_home(&mut self, ui: &mut egui::Ui, tc: &ThemeColors) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(egui::RichText::new("R2UI 组件库演示").size(32.0).color(tc.accent));
            ui.label(egui::RichText::new("从 iWinFix R 抽离的可复用 egui UI 构成").color(tc.text_secondary));
            ui.add_space(16.0);
            ui.separator();
        });

        ui.add_space(12.0);
        section_title(ui, tc, "模块构成");
        ui.add_space(8.0);

        egui::Grid::new("module_grid").num_columns(2).spacing([20.0, 6.0]).show(ui, |ui| {
            let rows = [
                ("theme", "深/浅色主题、全局 Style、ThemeColors 语义色板"),
                ("fonts", "Windows 中文字体自动检测与注册"),
                ("widgets", "Badge 徽标、ToolCard/LinkCard 卡片、搜索框、状态文字"),
                ("layout", "顶栏、可拖拽双分区侧栏、自适应卡片网格、主从列表行"),
                ("dialogs", "居中视口辅助、确认对话框"),
            ];
            for (name, desc) in rows {
                ui.label(egui::RichText::new(name).strong().color(tc.accent));
                ui.label(egui::RichText::new(desc).color(tc.text_secondary));
                ui.end_row();
            }
        });

        ui.add_space(16.0);
        section_title(ui, tc, "交互演示入口");
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if ui.button("工具卡片网格").clicked() {
                self.selected_tool_cat = Some("系统修复".to_string());
                self.selected_link_cat = None;
                self.view = View::Tools;
            }
            if ui.button("链接卡片").clicked() {
                self.selected_link_cat = Some("官方资源".to_string());
                self.selected_tool_cat = None;
                self.view = View::Links;
            }
            if ui.button("主从列表").clicked() {
                self.view = View::List;
            }
            if ui.button("确认对话框").clicked() {
                self.show_confirm_delete = true;
            }
        });
    }

    fn draw_tool_grid(&mut self, ui: &mut egui::Ui, tc: &ThemeColors, tools: &[DemoTool]) {
        let items: Vec<ToolCardData<'_>> = tools
            .iter()
            .map(|t| ToolCardData {
                title: &t.title,
                description: &t.description,
                state: t.state,
                downloaded_size: if t.state == DownloadState::Downloaded { Some(&t.size) } else { None },
                progress_percent: t.progress,
                progress_detail: if t.state == DownloadState::Downloading {
                    Some("2.1 MB/5.0 MB")
                } else {
                    None
                },
            })
            .collect();

        let mut pending_delete = false;
        adaptive_grid(ui, "demo_tool_grid", &items, |ui, width, data| {
            if let Some(action) = tool_card(ui, tc, width, data) {
                if matches!(action, CardAction::Delete) {
                    pending_delete = true;
                }
            }
        });
        if pending_delete {
            self.show_confirm_delete = true;
        }
    }

    fn draw_link_grid(&mut self, ui: &mut egui::Ui, tc: &ThemeColors, links: &[DemoLink]) {
        let items: Vec<LinkCardData<'_>> = links
            .iter()
            .map(|l| LinkCardData { title: &l.title, description: &l.description })
            .collect();

        adaptive_grid(ui, "demo_link_grid", &items, |ui, width, data| {
            let _ = link_card(ui, tc, width, data);
        });
    }

    fn draw_rule_list(&mut self, ui: &mut egui::Ui, tc: &ThemeColors) {
        // 表头
        ui.horizontal(|ui| {
            ui.add_sized(
                [ui.available_width() - 50.0, 22.0],
                egui::Label::new(egui::RichText::new("描述").strong().color(tc.text_secondary).size(11.0)),
            );
            ui.add(egui::Label::new(egui::RichText::new("条目").strong().color(tc.text_secondary).size(11.0)));
        });
        ui.separator();

        egui::ScrollArea::vertical().id_source("rule_list_scroll").show(ui, |ui| {
            for (id, packed) in &self.rules {
                let (desc, count) = packed.split_once('|').unwrap_or((packed.as_str(), ""));
                let row = ListRow {
                    id,
                    title: desc,
                    badge: if count.is_empty() { None } else { Some(count) },
                };
                let selected = self.selected_rule.as_deref() == Some(id.as_str());
                if selectable_row(ui, tc, &row, selected, 50.0) {
                    self.selected_rule = Some(id.clone());
                }
            }
        });
    }
}
