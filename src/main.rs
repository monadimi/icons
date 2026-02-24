use resvg::tiny_skia::{self, Pixmap};
use resvg::usvg::{Options, Tree};
use std::fs;
use svg::{Document, node::element::*};

fn tricoord(a: f64, b: f64, c: f64) -> (f64, f64) {
    let unit: f64 = 1.0 / 26.0;
    (
        ((b - c) * (3.0f64).sqrt() / 2.0f64) * unit,
        (-a + (b + c) / 2.0f64) * unit,
    )
}

macro_rules! tripolyline {
    ( ( $a0:expr, $b0:expr, $c0:expr ), $( ( $a:expr, $b:expr, $c:expr ) ),* $(,)? ) => {{
        let mut s = String::new();
        let (x, y) = tricoord($a0 as f64, $b0 as f64, $c0 as f64);
        s.push_str(&format!("M{},{} ",x,y));
        $(
            let (x, y) = tricoord($a as f64, $b as f64, $c as f64);
            s.push_str(&format!("L{},{} ",x,y));
        )*
        s.push_str("Z");
        s
    }}
}

fn main() {
    let icon_size = 1024.0;
    let icon_dir = "result/tmp";
    let stroke_color = "#27e2a4";
    let unit: f64 = 1.0 / 26.0;
    let body = vec![
        tripolyline![
            (13, 0, 8),
            (11, 0, 10),
            (0, 0, -1),
            (-11, 0, -1),
            (-13, 0, -5),
            (0, 0, -5)
        ],
        tripolyline![
            (13, 0, 8),
            (11, 0, 10),
            (0, 0, -1),
            (-11, 0, -1),
            (-13, 0, -5),
            (0, 0, -5)
        ],
        tripolyline![
            (10, 0, 11),
            (8, 0, 13),
            (0, 0, 5),
            (-8, 0, 5),
            (-10, 0, 1),
            (0, 0, 1)
        ],
        tripolyline![(5, 0, -2), (5, 0, -8), (1, 0, -10), (1, 0, -6)],
        tripolyline![(-5, 0, -7), (-5, 0, -13), (-1, 0, -11), (-1, 0, -7)],
    ]
    .iter()
    .map(|x| Path::new().set("d", x.to_string()))
    .fold(Group::new(), |g, x| g.add(x));
    let orbs = [
        tricoord(9.0, 0.0, 0.0),
        tricoord(0.0, 9.0, 0.0),
        tricoord(0.0, 0.0, 9.0),
    ]
    .map(|(x, y)| Circle::new().set("cx", x).set("cy", y).set("r", unit * 2.0))
    .iter()
    .fold(Group::new(), |g, x| g.add(x.clone()));
    let icon = Group::new()
        .set(
            "transform",
            format!(
                "translate({} {}) scale({})",
                icon_size / 2.0,
                icon_size / 2.0,
                icon_size
            ),
        )
        .set("fill", "none")
        .set("stroke", stroke_color)
        .set("stroke-width", icon_size / 32768.0)
        .set("stroke-linecap", "round")
        .set("stroke-linejoin", "round")
        .add(body)
        .add(orbs);

    let data = Document::new()
        .set("viewBox", (0.0, 0.0, icon_size, icon_size))
        .add(icon);

    fs::create_dir_all(&icon_dir).unwrap();
    let svg_path = format!("{}/icon.svg", icon_dir);
    svg::save(svg_path, &data).unwrap();
    let tree = Tree::from_str(&data.to_string(), &Options::default()).unwrap();
    let mut pixmap = Pixmap::new(tree.size().width() as u32, tree.size().height() as u32).unwrap();
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let png_path = format!("{}/icon.png", icon_dir);
    pixmap.save_png(&png_path).unwrap();
}
