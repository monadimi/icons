mod build;
mod icon;

use icon::*;

fn main() {
    let icon_info = IconInfo::new(1024.0, "#27e2a4".to_string(), "#121212".to_string());
    let dir = format!("result/{}", icon_info);
    icon::Icon::new(&icon_info).save(dir);
}
