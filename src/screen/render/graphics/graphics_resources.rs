use std::fs::File;

extern crate png;
extern crate sdl2_sys;

pub struct GraphicsResources {
    pub tiles: Image,
    pub tiles2: Image,
    pub tiles3: Image,
    pub entcolours: Image,
    pub flipsprites: Image,
    pub sprites: Image,
    pub bfont: Image,
    pub teleporter: Image,
    pub images: Image,
}

impl GraphicsResources {
    pub fn new() -> GraphicsResources {
        GraphicsResources {
            tiles: Image {
                name: "tiles".to_string(),
                surfaces: Image::LoadImage("tiles", true, false, 8, 8),
                rect: sdl2::rect::Rect::new(0, 0, 8, 8),
            },
            tiles2: Image {
                name: "tiles2".to_string(),
                surfaces: Image::LoadImage("tiles2", true, false, 8, 8),
                rect: sdl2::rect::Rect::new(0, 0, 8, 8),
            },
            tiles3: Image {
                name: "tiles3".to_string(),
                surfaces: Image::LoadImage("tiles3", true, false, 8, 8),
                rect: sdl2::rect::Rect::new(0, 0, 8, 8),
            },
            entcolours: Image {
                name: "entcolours".to_string(),
                surfaces: Image::LoadImage("entcolours", true, false, 8, 8),
                rect: sdl2::rect::Rect::new(0, 0, 8, 8),
            },
            flipsprites: Image {
                name: "flipsprites".to_string(),
                surfaces: Image::LoadImage("flipsprites", true, false, 32, 32),
                rect: sdl2::rect::Rect::new(0, 0, 32, 32),
            },
            sprites: Image {
                name: "sprites".to_string(),
                surfaces: Image::LoadImage("sprites", true, false, 32, 32),
                rect: sdl2::rect::Rect::new(0, 0, 32, 32),
            },
            bfont: Image {
                name: "font".to_string(),
                surfaces: Image::LoadImage("font", true, false, 8, 8),
                rect: sdl2::rect::Rect::new(0, 0, 8, 8),
            },
            teleporter: Image {
                name: "teleporter".to_string(),
                surfaces: Image::LoadImage("teleporter", true, false, 96, 96),
                rect: sdl2::rect::Rect::new(0, 0, 96, 96),
            },
            images: Image {
                name: "images".to_string(),
                surfaces: vec![
                    Image::LoadOneImage("levelcomplete",     false, false, 320, 48),
                    Image::LoadOneImage("minimap",           true,  true,  240, 180),
                    Image::LoadOneImage("covered",           true,  true,  12, 9),
                    Image::LoadOneImage("elephant",          true,  false, 464, 320),
                    Image::LoadOneImage("gamecomplete",      false, false, 320, 48),
                    Image::LoadOneImage("fliplevelcomplete", false, false, 320, 48),
                    Image::LoadOneImage("flipgamecomplete",  false, false, 320, 48),
                    Image::LoadOneImage("site",              false, false, 121, 5),
                    Image::LoadOneImage("site2",             true,  false, 64, 5),
                    Image::LoadOneImage("site3",             true,  false, 81, 5),
                    Image::LoadOneImage("ending",            true,  false, 320, 240),
                    Image::LoadOneImage("site4",             true,  false, 121, 5),
                    Image::LoadOneImage("minimap",           true,  false, 240, 180),
                ],
                rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            },
        }
    }
}

// @sx currently used for tests only
impl IntoIterator for GraphicsResources {
    type Item = Image;
    type IntoIter = std::array::IntoIter<Image, 9>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([
            self.tiles,
            self.tiles2,
            self.tiles3,
            self.entcolours,
            self.flipsprites,
            self.sprites,
            self.bfont,
            self.teleporter,
            self.images,
        ])
    }
}

pub struct Image {
    pub name: String,
    pub surfaces: Vec<sdl2::surface::Surface<'static>>,
    pub rect: sdl2::rect::Rect,
}

impl Image {
    pub fn new(name: String, surfaces: Vec<sdl2::surface::Surface<'static>>, rect: sdl2::rect::Rect) -> Self {
        Self {
            name,
            surfaces,
            rect,
        }
    }

    // static SDL_Surface* LoadImage(const char *filename, bool noBlend = true, bool noAlpha = false)
    pub fn LoadImage(file: &str, no_blend: bool, no_alpha: bool, w: u32, h: u32) -> Vec<sdl2::surface::Surface<'static>> {
        let file_path = ["assets/graphics/", file, ".png"].concat();
        let decoder = match File::open(file_path.to_owned()) {
            Ok(x) => png::Decoder::new(x),
            Err(e) => panic!("{}: {}", e, file_path),
        };
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut data: Vec<u8> = vec![0; info.buffer_size()];
        reader.next_frame(&mut data).unwrap();

        let masks = sdl2::pixels::PixelMasks {
            bpp: if no_alpha { 24 } else { 32 },
            rmask: 0x000000FF,
            gmask: 0x0000FF00,
            bmask: 0x00FF0000,
            amask: if no_alpha { 0x00000000 } else { 0xFF000000 },
        };
        let surface = sdl2::surface::Surface::from_data_pixelmasks(data.as_mut_slice(), info.width, info.height, info.line_size as u32, &masks).unwrap();
        let mut surface = surface.convert_format(sdl2::pixels::PixelFormatEnum::ABGR8888).unwrap();
        if no_blend {
            surface.set_blend_mode(sdl2::render::BlendMode::Blend);
        }
        let pf = surface.pixel_format_enum();
        let mut surfaces = vec![];

        match file {
            // TODO: @sx: incorrect loading process, need rewrite
            "teleporter" => {
                let mut i = 0;
                while i*w < info.width {
                    let mut j = 0;
                    while j*h < info.height {
                        let mut s = sdl2::surface::Surface::new(w, h, pf).unwrap();
                        let src_rect = sdl2::rect::Rect::new((i * w) as i32, (j * h) as i32, w, w);
                        surface.blit(src_rect, &mut s, None).unwrap();
                        // println!("teleporter blit from ({}:{}) {}x{}", j*h, i*w, w, w);
                        if no_blend {
                            s.set_blend_mode(sdl2::render::BlendMode::Blend);
                        }

                        // // crate::rustutil::dump_surface_pixels(&s, "image");
                        // crate::rustutil::dump_surface_pixels_slice(&s, info.width as usize, info.height as usize, info.line_size);
                        // crate::rustutil::dump_surface(&s, "image", file);

                        surfaces.push(s);
                        j += 1;
                    }

                    i += 1;
                }
            },
            _ => {
                let mut i = 0;
                while i*h < info.height {
                    let mut j = 0;
                    while j*w < info.width {
                        let mut s = sdl2::surface::Surface::new(w, h, pf).unwrap();
                        let src_rect = sdl2::rect::Rect::new((j * w) as i32, (i * h) as i32, w, w);
                        surface.blit(src_rect, &mut s, None).unwrap();
                        // println!("teleporter blit from ({}:{}) {}x{}", j*h, i*w, w, w);
                        if no_blend {
                            s.set_blend_mode(sdl2::render::BlendMode::Blend);
                        }

                        // // crate::rustutil::dump_surface_pixels(&s, "image");
                        // crate::rustutil::dump_surface_pixels_slice(&s, info.width as usize, info.height as usize, info.line_size);
                        // crate::rustutil::dump_surface(&s, "image", file);

                        surfaces.push(s);
                        j += 1;
                    }

                    i += 1;
                }
            },
        }

        // if file == "teleporter" {
        //     crate::rustutil::dump_surface(&surface, "image", file);
        //     println!("{} --- {:?} --- {:?}", file, info, data[0..info.line_size].to_vec());
        // }
        surfaces
    }

    // static SDL_Surface* LoadImage(const char *filename, bool noBlend = true, bool noAlpha = false)
    pub fn LoadOneImage(file: &str, no_blend: bool, no_alpha: bool, w: u32, h: u32) -> sdl2::surface::Surface<'static> {
        let file_path = ["assets/graphics/", file, ".png"].concat();
        let decoder = match File::open(file_path.to_owned()) {
            Ok(x) => png::Decoder::new(x),
            Err(e) => panic!("{}: {}", e, file_path),
        };
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut data: Vec<u8> = vec![0; info.buffer_size()];
        reader.next_frame(&mut data).unwrap();

        let masks = sdl2::pixels::PixelMasks {
            bpp: if no_alpha { 24 } else { 32 },
            rmask: 0x000000FF,
            gmask: 0x0000FF00,
            bmask: 0x00FF0000,
            amask: if no_alpha { 0x00000000 } else { 0xFF000000 },
        };
        let surface = sdl2::surface::Surface::from_data_pixelmasks(data.as_mut_slice(), info.width, info.height, 0u32, &masks).unwrap();
        let mut surface = surface.convert_format(sdl2::pixels::PixelFormatEnum::ABGR8888).unwrap();
        if no_blend {
            surface.set_blend_mode(sdl2::render::BlendMode::Blend);
        }

        // println!("{} --- {:?} --- {:?}", file, info, data[0..20].to_vec());
        // crate::rustutil::dump_surface(&surface, "image", file);
        return surface
    }
}

#[cfg(test)]
mod tests {
    use crate::screen::render::graphics::graphics_util;

    use super::*;

    #[test]
    fn surfaces_are_created() {
        for image in GraphicsResources::new() {
            assert_ne!(image.surfaces.len(), 0);
            println!("{}", image.name);
        }
    }

    #[test]
    fn surfaces_have_not_zero_size() {
        for image in GraphicsResources::new() {
            for (i, surface) in image.surfaces.iter().enumerate() {
                println!("{}[{}]: (w:h) - {}:{}", image.name, i, surface.width(), surface.height());
                assert_ne!(surface.width(), 0);
                assert_ne!(surface.height(), 0);
            }
        }
    }

    // @sx not implemented yet
    #[test]
    fn surfaces_should_not_be_empty() {
        let mut empty = true;
        for image in GraphicsResources::new() {
            for (i, surface) in image.surfaces.iter().enumerate() {
                println!("checking image: {} @ surface {}", image.name, i);
                for x in 0..surface.width() as i32 {
                    for y in 0..surface.height() as i32 {
                        let pixel = graphics_util::ReadPixel(surface, x, y);
                        if pixel != 0 { empty = false; }
                    }
                }

                assert_eq!(empty, true);
            }
        }

    }
}
