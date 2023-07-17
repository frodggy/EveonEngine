use std::{collections::HashMap, fs, path::Path};

use egui_glfw_gl::egui::{self as eglfw, ColorImage};
use egui_glfw_gl::egui::{Context, TextureHandle};
pub struct ImageLoader {
    textures: HashMap<String, TextureHandle>,
}

impl ImageLoader {
    pub fn load_from_dir(dir_path: &str, ctx: &Context) -> ImageLoader {
        let mut textures = HashMap::new();

        let data = fs::read_dir(dir_path.clone()).unwrap();

        for path in data {
            let path = path.unwrap();
            println!("{}", path.file_name().to_str().unwrap());
            if path.metadata().unwrap().is_file() {
                let name = path.file_name().to_str().unwrap().to_string();

                let image: ColorImage =
                    match load_image_from_path(&Path::new(&dir_path).join(&name)) {
                        Ok(image) => image,
                        Err(err) => panic!("{}: {}", err, name),
                    };

                let texture = ctx.load_texture(&name, image, Default::default());

                textures.insert(name, texture);
            }
        }

        Self { textures }
    }

    pub fn get_texture(&self, texture: &str) -> TextureHandle {
        self.textures.get(texture).unwrap().clone()
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<eglfw::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(eglfw::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
