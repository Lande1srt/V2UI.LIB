//! R2UI.egui - 可复用 egui UI 组件库。
//!
//! 模块划分：
//! - [`theme`]   : 深/浅色主题、全局 Style、语义化颜色板 [`ThemeColors`]
//! - [`fonts`]   : Windows 中文字体自动检测与注册
//! - [`widgets`] : 基础组件（徽标、卡片、状态文字、搜索框等）
//! - [`layout`]  : 布局框架（顶栏、可拖拽分割的双区侧栏、自适应卡片网格）
//! - [`dialogs`] : 对话框（居中视口工具、确认对话框）

pub mod dialogs;
pub mod fonts;
pub mod layout;
pub mod theme;
pub mod widgets;

pub use theme::{apply_theme, apply_theme_kind, ThemeColors, ThemeKind};
