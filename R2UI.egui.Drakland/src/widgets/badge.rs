//! 徽标（Badge）组件：标题旁的橙色小标签，如 `[新]`、`[推荐]`。

use egui::{Margin, RichText, Rounding, Ui};

use crate::theme::ThemeColors;

/// 单个徽标：以橙色填充圆角小框包裹文字。
pub struct Badge<'a> {
    pub text: &'a str,
}

impl<'a> Badge<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn show(self, ui: &mut Ui, tc: &ThemeColors) {
        egui::Frame::default()
            .fill(tc.badge_fill)
            .rounding(Rounding::same(4.0))
            .inner_margin(Margin {
                left: 6.0,
                right: 6.0,
                top: 3.0,
                bottom: 3.0,
            })
            .show(ui, |ui| {
                ui.label(RichText::new(self.text).color(tc.badge_text).size(11.0).strong());
            });
    }
}

/// 从形如 `"标题 [新] [推荐]"` 的字符串中解析出纯净标题与徽标列表。
///
/// 方括号内至少含一个字符才会被识别为徽标。
pub fn parse_title_badges(title: &str) -> (String, Vec<String>) {
    let mut badges = Vec::new();
    let mut clean = String::with_capacity(title.len());

    let bytes = title.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            if let Some(rel) = title[i + 1..].find(']') {
                let inner = &title[i + 1..i + 1 + rel];
                let close_abs = i + 1 + rel;
                if !inner.is_empty() {
                    badges.push(inner.to_string());
                    i = close_abs + 1;
                    continue;
                }
            }
        }
        // 普通字符：按 UTF-8 字符边界拷贝
        let ch_len = utf8_char_len(bytes[i]);
        clean.push_str(&title[i..i + ch_len]);
        i += ch_len;
    }

    (clean.trim().to_string(), badges)
}

fn utf8_char_len(first_byte: u8) -> usize {
    if first_byte < 0x80 {
        1
    } else if first_byte >> 5 == 0b110 {
        2
    } else if first_byte >> 4 == 0b1110 {
        3
    } else {
        4
    }
}
