//! 通用原子组件层 —— 可独立复用在任何 egui 项目中。
//!
//! - `flatlaf_tab`  自绘 FlatLaf 风格 tab 按钮（不依赖 egui 默认 tab）
//! - `path_row`     标签 + 自适应输入框 + 浏览按钮（避免 UI 溢出的关键实现）
//! - `log_view`     只读滚动日志区（吃满父容器剩余高度）
//! - `file_dialog`  异步 rfd 文件/目录选择 + recv_pick 结果收集
//! - `dialog`       简易错误弹窗

pub mod flatlaf_tab;
pub mod path_row;
pub mod log_view;
pub mod file_dialog;
pub mod dialog;

pub use flatlaf_tab::flatlaf_tab;
pub use path_row::{BROWSE_BTN_W, TE_V_MARGIN, path_row, text_edit_height};
pub use log_view::LogView;
pub use file_dialog::{PickRx, pick_file_async, pick_folder_async, recv_pick};
pub use dialog::show_error_window;
