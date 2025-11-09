use std::path::PathBuf;

/// 返回一个 Iterator，按优先级从高到低产出所有 XDG 视角下的字体目录。
/// 用户级目录在前，系统级目录在后，已自动去重。
pub fn xdg_font_dirs() -> impl Iterator<Item = PathBuf> {
    // 1. 用户级：$XDG_DATA_HOME/fonts 或 ~/.local/share/fonts
    let user = xdg::BaseDirectories::new()
        .ok()
        .and_then(|xdg| xdg.find_data_file("fonts"))
        .or_else(|| dirs::data_dir().map(|d| d.join("fonts")));

    // 2. 系统级：$XDG_DATA_DIRS/fonts 或 /usr/local/share/fonts + /usr/share/fonts
    let system: Vec<_> = xdg::BaseDirectories::new()
        .ok()
        .map(|xdg| {
            xdg.get_data_dirs()
                .into_iter()
                .map(|mut p| {
                    p.push("fonts");
                    p
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            // 环境变量没设置时的 fallback
            ["/usr/local/share", "/usr/share"]
                .iter()
                .map(|s| PathBuf::from(s).join("fonts"))
                .collect()
        });

    // 3. 把用户级插到最前面，整体去重（按路径字符串去重即可）
    user.into_iter()
        .chain(system)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
}

fn main() {
    for p in xdg_font_dirs() {
        println!("{}", p.display());
    }
}
