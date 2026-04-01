use anyhow::{Ok, Result};
use magick_rust::{MagickWand, magick_wand_genesis, magick_wand_terminus};
use resvg::tiny_skia::{self, Pixmap};
use std::fs;
use std::path::Path;

struct Format<'a> {
    name: &'a str,
    ext: &'a str,
    setup: fn(&mut MagickWand) -> Result<()>,
}

impl super::Icon {
    pub fn save(self: &Self, icon_dir: &Path) -> Result<()> {
        println!("{}", self.info);
        fs::create_dir_all(icon_dir).unwrap();

        let svg_path = icon_dir.join("icon.svg");
        svg::save(&svg_path, &self.data)?;
        let ai_path = icon_dir.join("icon.ai");
        fs::copy(&svg_path, &ai_path)?;

        let png_path = icon_dir.join("icon.png");
        {
            let svg_data = fs::read(&svg_path)?;
            let tree = resvg::usvg::Tree::from_data(&svg_data, &resvg::usvg::Options::default())?;
            let mut pixmap =
                Pixmap::new(tree.size().width() as u32, tree.size().height() as u32).unwrap();
            resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
            resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
            pixmap.save_png(&png_path)?;
        }

        magick_wand_genesis();
        {
            let base_wand = {
                let wand = MagickWand::new();
                wand.read_image(&png_path.to_str().unwrap())?;
                wand.resize_image(1024, 1024, magick_rust::bindings::FilterType::Lanczos)?;
                wand.set_image_alpha_channel(magick_rust::bindings::AlphaChannelOption::Activate)?;
                wand
            };

            let formats = [
                // Format {
                //     name: "png",
                //     ext: "png",
                //     setup: |_| Ok(()),
                // },
                Format {
                    name: "jpeg",
                    ext: "jpeg",
                    setup: |wand| {
                        wand.set_compression_quality(90)?;
                        Ok(())
                    },
                },
                Format {
                    name: "avif",
                    ext: "avif",
                    setup: |wand| {
                        wand.set_option("heic:chroma", "444")?;
                        wand.set_option("heic:lossless", "true")?;
                        Ok(())
                    },
                },
                Format {
                    name: "webp",
                    ext: "webp",
                    setup: |wand| {
                        wand.set_option("webp:quality", "90")?;
                        wand.set_option("webp:lossless", "true")?;
                        wand.set_option("webp:method", "6")?;
                        Ok(())
                    },
                },
                Format {
                    name: "bmp",
                    ext: "bmp",
                    setup: |_| Ok(()),
                },
                Format {
                    name: "tiff",
                    ext: "tiff",
                    setup: |wand| {
                        wand.set_option("tiff:compression", "lzw")?;
                        Ok(())
                    },
                },
                Format {
                    name: "gif",
                    ext: "gif",
                    setup: |_| Ok(()),
                },
                Format {
                    name: "heif",
                    ext: "heif",
                    setup: |wand| {
                        wand.set_option("heic:chroma", "444")?;
                        wand.set_option("heic:lossless", "true")?;
                        Ok(())
                    },
                },
                Format {
                    name: "dds",
                    ext: "dds",
                    setup: |_| Ok(()),
                },
                Format {
                    name: "tga",
                    ext: "tga",
                    setup: |_| Ok(()),
                },
                Format {
                    name: "ppm",
                    ext: "ppm",
                    setup: |_| Ok(()),
                },
            ];

            for fmt in formats {
                let mut wand = base_wand.clone();
                wand.set_image_format(fmt.name)?;
                (fmt.setup)(&mut wand)?;
                wand.set_image_alpha_channel(magick_rust::bindings::AlphaChannelOption::Activate)?;
                let icon_path = icon_dir.join(format!("icon.{}", fmt.ext));
                let icon_path = icon_path.to_str().ok_or_else(|| anyhow::anyhow!(""))?;
                wand.write_image(icon_path)?;
            }

            let ico_sizes = [16, 32, 48, 64, 128, 256];
            let mut ico = MagickWand::new();
            for size in ico_sizes {
                let wand = base_wand.clone();
                wand.resize_image(size, size, magick_rust::bindings::FilterType::Lanczos)?;
                ico.add_image(&wand)?;
            }
            ico.set_image_format("ico")?;
            let path = icon_dir.join("icon.ico");
            ico.write_image(path.to_str().unwrap())?;

            // let icns_sizes = [16, 32, 64, 128, 256, 512, 1024];
            // let mut icns = MagickWand::new();
            // for size in icns_sizes {
            //     let wand = base_wand.clone();
            //     wand.resize_image(size, size, magick_rust::bindings::FilterType::Lanczos)?;
            //     icns.add_image(&wand)?;
            // }
            // icns.set_image_format("icns")?;
            // let path = icon_dir.join("icon.icns");
            // icns.write_image(path.to_str().unwrap())?;
        }
        magick_wand_terminus();
        Ok(())
    }
}
