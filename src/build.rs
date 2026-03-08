use crate::icon::*;
use std::{
    f64::consts::TAU,
    time::{SystemTime, UNIX_EPOCH},
};
use svg::{
    Document,
    node::{Comment, element::*},
};

#[derive(Clone, Copy)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn from_polar(theta: f64, radius: f64) -> Coord {
        let x: f64 = theta.cos() * radius;
        let y: f64 = theta.sin() * radius;
        Coord { x, y }
    }

    fn from_tricoord(ic: f64, jc: f64) -> Coord {
        let i = (ic - 4.5) * (3f64.sqrt() / 9.0);
        let j = (jc - 4.5) * (3f64.sqrt() / 9.0);

        Coord {
            x: i,
            y: (-i + 2.0 * j) * (3f64.sqrt() / 3.0),
        }
    }
}

impl Icon {
    pub fn new(info: &IconInfo) -> Self {
        print!(
            "({},{})\n",
            Coord::from_tricoord(6.0, 6.0).x,
            Coord::from_tricoord(6.0, 6.0).y
        );
        let r = 3f64.sqrt() / 9.0; // tmp

        let icon_size = 1024.0;
        let stroke_color = info.stroke_color.clone();
        let background_color = info.background_color.clone();
        let bg = {
            let bg_color = background_color;
            Rectangle::new()
                .set("x", 0.0)
                .set("y", 0.0)
                .set("width", icon_size)
                .set("height", icon_size)
                .set("fill", bg_color)
        };

        let fi = (3f64.sqrt() / 6f64).asin();

        let orbs = (0..3)
            .map(|x| 90f64 + (120f64 * (x as f64)))
            .map(|x| TAU * x / 360f64)
            .map(|x| Coord::from_polar(x, 1f64))
            .map(|v| Polyline::new().set("points", format!("{},{} {},{}", v.x, v.y, v.x, v.y)))
            .fold(Group::new(), |g, x| g.add(x));

        let body = {
            let body_p: Vec<Coord> = (0..3)
                .map(|x| 30f64 + (120f64 * (x as f64)))
                .map(|x| TAU * x / 360f64)
                .flat_map(|x| [x - fi, x + fi])
                .map(|x| Coord::from_polar(x, 1f64))
                .collect();
            let center_j = vec![
                Coord::from_tricoord(3f64, 3f64),
                Coord::from_tricoord(6f64, 6f64),
            ];
            let edge_j = vec![
                Coord::from_tricoord(9f64, 7.5f64),
                Coord::from_tricoord(7.5f64, 9f64),
            ];
            let ret = vec![
                vec![body_p[0], edge_j[0]],
                vec![body_p[1], edge_j[1]],
                vec![body_p[2], center_j[1], body_p[5]],
                vec![body_p[3], center_j[0], body_p[4]],
            ]
            .iter()
            .map(|arr| {
                Polyline::new().set(
                    "points",
                    arr.iter()
                        .map(|v| format!("{},{}", v.x, v.y))
                        .collect::<Vec<_>>()
                        .join(" "),
                )
            })
            .fold(Group::new(), |g, x| g.add(x));
            ret
        };

        let icon = Group::new()
            .set(
                "transform",
                format!(
                    "translate({} {}) scale({} -{})",
                    icon_size / 2.0,
                    icon_size / 2.0,
                    icon_size / 2.0 / (1.0 + r + 1.0 / 64.0),
                    icon_size / 2.0 / (1.0 + r + 1.0 / 64.0)
                ),
            )
            .set("fill", "none")
            .set("stroke", stroke_color)
            .set("stroke-width", r * 2.0)
            .set("stroke-linecap", "round")
            .set("stroke-linejoin", "round")
            .add(orbs)
            .add(body);

        let comment = {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            Group::new()
                .add(Comment::new("The Monad Club icon"))
                .add(Comment::new("Author: Sua Tso <zhaoxiuya13@gmail.com>"))
                .add(Comment::new(
                    "Repository: https://github.com/monadimi/icons",
                ))
                .add(Comment::new(format!("Generated at Unix timestamp {}", now)))
        };

        let data = Document::new()
            .set("viewBox", (0.0, 0.0, icon_size, icon_size))
            .add(comment)
            .add(bg)
            .add(icon);

        Icon {
            data,
            info: info.clone(),
        }
    }
}
