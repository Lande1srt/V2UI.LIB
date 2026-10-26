/// 启动时加载中文字体（微软雅黑/黑体/宋体），加入 proportional 与 monospace 回退链。
///
/// egui 默认字体不含 CJK 字形，不加载会显示为方块。
/// 找不到任何字体时仅向 stderr 记录日志，不影响启动（中文将显示为缺字方框）。
pub fn setup_chinese_fonts(ctx: &eframe::egui::Context) {
    // 优先宋体（SimSun），其次微软雅黑，最后黑体。
    // ttc 集合默认取 index=0（常规体）。
    const CANDIDATES: &[&str] = &[
        r"C:\Windows\Fonts\simsun.ttc",    // 宋体 常规
        r"C:\Windows\Fonts\simhei.ttf",    // 黑体
        r"C:\Windows\Fonts\msyh.ttc",       // 微软雅黑
        r"C:\Windows\Fonts\msyhbd.ttc",     // 微软雅黑 Bold
    ];

    for path in CANDIDATES {
        match std::fs::read(path) {
            Ok(bytes) => {
                let mut fonts = eframe::egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "chinese".to_owned(),
                    std::sync::Arc::new(eframe::egui::FontData::from_owned(bytes)),
                );
                fonts
                    .families
                    .entry(eframe::egui::FontFamily::Proportional)
                    .or_default()
                    .push("chinese".to_owned());
                fonts
                    .families
                    .entry(eframe::egui::FontFamily::Monospace)
                    .or_default()
                    .push("chinese".to_owned());
                ctx.set_fonts(fonts);
                eprintln!("已加载中文字体: {path}");
                return;
            }
            Err(e) => eprintln!("加载中文字体失败 {path}: {e}"),
        }
    }
    eprintln!("警告：未找到任何中文字体，中文界面可能无法正常显示");
}
