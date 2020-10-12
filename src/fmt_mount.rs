use {
    crate::{
        error::*,
        mount::Mount,
    },
    crossterm::style::Color::*,
    file_size,
    minimad::{
        TextTemplate,
        OwningTemplateExpander,
    },
    termimad::{
        CompoundStyle,
        FmtText,
        MadSkin,
        terminal_size,
    },
};

static MD: &str = r#"
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:
|id|dev|filesystem|type|size|used|use%|avail|mount point
|-:|:-:|:-|:-:|:-:|-:|-:|:-
${mount-points
|${id}|*${dev-major}*:*${dev-minor}*|${fs}|${fs-type}|${size}|${used}|**${use-percents}%**|**${available}**|${mount-point}
}
|-:
"#;

pub fn print(mounts: &Vec<Mount>) -> Result<()> {
    let template = TextTemplate::from(MD);
    let mut expander = OwningTemplateExpander::new();
    let mut skin = MadSkin::default();
    skin.bold = CompoundStyle::with_fg(Yellow);
    skin.italic = CompoundStyle::with_fg(Magenta);
    expander.set("mounts_len", format!("{}", mounts.len()));
    for mount in mounts {
        expander.sub("mount-points")
            .set("id", format!("{}", mount.id))
            .set("dev-major", format!("{}", mount.dev.major))
            .set("dev-minor", format!("{}", mount.dev.minor))
            .set("fs", &mount.fs)
            .set("fs-type", &mount.fs_type)
            .set("mount-point", mount.mount_point.to_string_lossy())
            .set("size", file_size::fit_4(mount.size()))
            .set("used", file_size::fit_4(mount.used()))
            .set("use-percents", format!("{:.0}", 100.0*mount.use_share()))
            .set("available", file_size::fit_4(mount.available()));
    }
    let (width, _) = terminal_size();
    let text = expander.expand(&template);
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    print!("{}", fmt_text);
    Ok(())
}

