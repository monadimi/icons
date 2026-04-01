use std::fmt;
use svg::Document;

mod save;

#[derive(Clone)]
pub struct IconInfo {
    pub icon_size: f64,
    pub background_color: String,
    pub stroke_color: String,
}

impl IconInfo {
    pub fn new(icon_size: f64, background_color: String, stroke_color: String) -> Self {
        Self {
            icon_size,
            background_color,
            stroke_color,
        }
    }
}

impl fmt::Display for IconInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "sz{}-bc{}-sc{}",
            self.icon_size,
            &self.background_color[1..],
            &self.stroke_color[1..]
        )
    }
}

pub struct Icon {
    pub data: Document,
    pub info: IconInfo,
}
