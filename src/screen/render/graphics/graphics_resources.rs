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
    pub fn new () -> GraphicsResources {
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
                surfaces: Image::LoadImage("teleporter", true, false, 8, 8),
                rect: sdl2::rect::Rect::new(0, 0, 8, 8),
            },
            images: Image {
                name: "images".to_string(),
                surfaces: vec![
                    Image::LoadOneImage("levelcomplete", false, false, 320, 48),
                    Image::LoadOneImage("minimap", true, true, 240, 180),
                    Image::LoadOneImage("covered", true, true, 12, 9),
                    Image::LoadOneImage("elephant", true, false, 464, 320),
                    Image::LoadOneImage("gamecomplete", false, false, 320, 48),
                    Image::LoadOneImage("fliplevelcomplete", false, false, 320, 48),
                    Image::LoadOneImage("flipgamecomplete", false, false, 320, 48),
                    Image::LoadOneImage("site", false, true, 121, 5),
                    Image::LoadOneImage("site2", true, false, 64, 5),
                    Image::LoadOneImage("site3", true, false, 81, 5),
                    Image::LoadOneImage("ending", true, false, 320, 240),
                    Image::LoadOneImage("site4", true, false, 121, 5),
                    Image::LoadOneImage("minimap", true, false, 64, 5),
                ],
                rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            },
        }
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

        let pf = match no_alpha {
            true => sdl2::pixels::PixelFormatEnum::RGBA8888,
            // false => sdl2::pixels::PixelFormatEnum::RGBX8888,
            false => sdl2::pixels::PixelFormatEnum::ABGR8888,
        };
        let surface: sdl2::surface::Surface = match sdl2::surface::Surface::from_data(data.as_mut_slice(), info.width, info.height, info.width*4, pf) {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        };

        let mut surfaces = vec![];
        let mut i = 0;
        while i*h < info.width {
            let mut j = 0;
            while j*w < info.height {
                let mut src_destt = sdl2::surface::Surface::new(w, h, pf).unwrap();
                let src_rect = sdl2::rect::Rect::new((j * w) as i32, (i * h) as i32, w, w);
                surface.blit(src_rect, &mut src_destt, None).unwrap();
                surfaces.push(src_destt);

                j += 1;
            }

            i += 1;
        }

        // //Temporary storage for the image that's loaded
        // SDL_Surface* loadedImage = NULL;
        // //The optimized image that will be used
        // SDL_Surface* optimizedImage = NULL;

        // unsigned char *data;
        // unsigned int width, height;

        // unsigned char *fileIn = NULL;
        // size_t length = 0;
        // FILESYSTEM_loadFileToMemory(filename, &fileIn, &length);
        // lodepng_decode32(&data, &width, &height, fileIn, length);

        // FILESYSTEM_freeMemory(&fileIn);

        // loadedImage = SDL_CreateRGBSurfaceFrom(
        // 	data,
        // 	width,
        // 	height,
        // 	noAlpha ? 24 : 32,
        // 	width * (noAlpha ? 3 : 4),
        // 	0x000000FF,
        // 	0x0000FF00,
        // 	0x00FF0000,
        // 	noAlpha ? 0x00000000 : 0xFF000000
        // );

        // if (loadedImage != NULL)
        // {
        // 	optimizedImage = SDL_ConvertSurfaceFormat(
        // 		loadedImage,
        // 		SDL_PIXELFORMAT_ABGR8888, // FIXME: Format? -flibit
        // 		0
        // 	);
        // 	SDL_FreeSurface( loadedImage );
        // 	free(data);
        // 	if (noBlend) SDL_SetSurfaceBlendMode(optimizedImage, SDL_BLENDMODE_BLEND);
        // 	return optimizedImage;
        // } else {
        // 	fprintf(stderr,"Image not found: %s\n", filename);
        // 	SDL_assert(0 && "Image not found! See stderr.");
        // 	return NULL;
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

        let pf = match no_alpha {
            true => sdl2::pixels::PixelFormatEnum::RGBA8888,
            // false => sdl2::pixels::PixelFormatEnum::RGBX8888,
            false => sdl2::pixels::PixelFormatEnum::ABGR8888,
        };
        let surface: sdl2::surface::Surface = match sdl2::surface::Surface::from_data(data.as_mut_slice(), info.width, info.height, info.width * 4, pf) {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        };

        let mut src_destt = sdl2::surface::Surface::new(0, 0, pf).unwrap();
        surface.blit(None, &mut src_destt, None).unwrap();

        return src_destt
    }
}
