//! 后台线程弹出 rfd 原生对话框，避免阻塞 egui UI 线程。

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;

/// rfd 选择结果接收器：None 表示用户取消。
pub type PickRx = Receiver<Option<PathBuf>>;

/// 后台线程弹出"选择目录"对话框。
pub fn pick_folder_async() -> PickRx {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let picked = rfd::FileDialog::new().pick_folder();
        let _ = tx.send(picked);
    });
    rx
}

/// 后台线程弹出"选择文件"对话框。
///
/// `filter` 为 `(描述, 扩展名列表)`，例如 `("M3U8文件", &["m3u8"])`。
pub fn pick_file_async(filter: Option<(&'static str, &'static [&'static str])>) -> PickRx {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut dialog = rfd::FileDialog::new();
        if let Some((desc, exts)) = filter {
            dialog = dialog.add_filter(desc, exts);
        }
        let _ = tx.send(dialog.pick_file());
    });
    rx
}

/// 接收一次文件选择结果：收到后自动注销接收器；取消则不变更字段。
pub fn recv_pick(rx: &mut Option<PickRx>) -> Option<PathBuf> {
    if let Some(receiver) = rx {
        match receiver.try_recv() {
            Ok(Some(path)) => {
                *rx = None;
                return Some(path);
            }
            Ok(None) => {
                *rx = None;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                *rx = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
        }
    }
    None
}
