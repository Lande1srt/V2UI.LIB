//! 字体加载：自动检测 Windows 系统中文字体并注册到 egui。

use egui;
use std::path::PathBuf;

/// 候选中文字体文件名（按优先级）。
pub const DEFAULT_CJK_FONTS: &[&str] = &["msyh.ttc", "msyhbd.ttc", "simhei.ttf", "simsun.ttc"];

/// 在 `%WINDIR%\\Fonts` 下查找存在的中文字体路径。
pub fn find_system_font_paths() -> Vec<PathBuf> {
    find_named_font_paths(DEFAULT_CJK_FONTS)
}

/// 在系统字体目录下查找给定文件名列表中实际存在的字体。
pub fn find_named_font_paths(names: &[&str]) -> Vec<PathBuf> {
    let mut fonts = Vec::new();

    let font_dir = if let Ok(windir) = std::env::var("WINDIR") {
        PathBuf::from(windir).join("Fonts")
    } else if let Ok(home) = std::env::var("HOME") {
        // 非 Windows 兜底：常见 Linux 字体目录
        PathBuf::from(home).join(".fonts")
    } else {
        return fonts;
    };

    for candidate in names {
        let path = font_dir.join(candidate);
        if path.exists() {
            fonts.push(path);
        }
    }

    fonts
}

/// 加载系统中文字体并注入 egui 上下文（Proportional 优先，Monospace 追加）。
pub fn load_chinese_fonts(ctx: &egui::Context) {
    load_fonts_from_paths(ctx, find_system_font_paths());
}

/// 使用指定字体文件路径加载并注入 egui。
pub fn load_fonts_from_paths(ctx: &egui::Context, font_paths: Vec<PathBuf>) {
    if font_paths.is_empty() {
        return;
    }

    let mut fonts = egui::FontDefinitions::default();

    for (i, font_path) in font_paths.iter().enumerate() {
        if let Ok(font_data) = std::fs::read(font_path) {
            let font_name = format!("ChineseFont_{}", i);
            fonts
                .font_data
                .insert(font_name.clone(), egui::FontData::from_owned(font_data));

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, font_name.clone());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push(font_name);
        }
    }

    ctx.set_fonts(fonts);
}
