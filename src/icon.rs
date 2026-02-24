use resvg::tiny_skia::{self, Pixmap};
use resvg::usvg::{Options, Tree};
use std::fmt;
use std::fs;
use svg::Document;

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

impl Icon {
    pub fn save(self: &Self, icon_dir: String) {
        println!("{}", self.info);
        fs::create_dir_all(&icon_dir).unwrap();

        let svg_path = format!("{}/icon.svg", icon_dir);
        svg::save(svg_path, &self.data).unwrap();

        let tree = Tree::from_str(&self.data.to_string(), &Options::default()).unwrap();
        let mut pixmap =
            Pixmap::new(tree.size().width() as u32, tree.size().height() as u32).unwrap();
        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        let png_path = format!("{}/icon.png", icon_dir);
        pixmap.save_png(&png_path).unwrap();
    }
}
