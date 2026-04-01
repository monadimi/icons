mod build;
mod icon;
use std::path::PathBuf;

use icon::*;

fn main() {
    let icon_info = IconInfo::new(1024.0, "none".to_string(), "#27e2a4".to_string());
    let dir = PathBuf::from("result").join(icon_info.to_string());
    match icon::Icon::new(&icon_info).save(&dir) {
        Ok(_) => println!("success"),
        Err(e) => eprintln!("error: {}", e),
    }
}
