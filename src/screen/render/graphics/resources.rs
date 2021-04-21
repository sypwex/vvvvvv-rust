use std::fs::File;
extern crate png;

pub struct Resources {
    pub sprites: Image,
    pub font: Image,
}

impl Resources {
    pub fn new () -> Resources {
        Resources {
            // im_tiles = LoadImage("graphics/tiles.png");
            // im_tiles2 = LoadImage("graphics/tiles2.png");
            // im_tiles3 = LoadImage("graphics/tiles3.png");
            // im_entcolours = LoadImage("graphics/entcolours.png");
            // im_flipsprites = LoadImage("graphics/flipsprites.png");
            sprites: Image::new("sprites.png", true, false),
            font: Image::new("font.png", true, false),
            // im_teleporter = LoadImage("graphics/teleporter.png");

            // im_image0 = LoadImage("graphics/levelcomplete.png", false);
            // im_image1 = LoadImage("graphics/minimap.png", true, true);
            // im_image2 = LoadImage("graphics/covered.png", true, true);
            // im_image3 = LoadImage("graphics/elephant.png");
            // im_image4 = LoadImage("graphics/gamecomplete.png", false);
            // im_image5 = LoadImage("graphics/fliplevelcomplete.png", false);
            // im_image6 = LoadImage("graphics/flipgamecomplete.png", false);
            // im_image7 = LoadImage("graphics/site.png", false);
            // im_image8 = LoadImage("graphics/site2.png");
            // im_image9 = LoadImage("graphics/site3.png");
            // im_image10 = LoadImage("graphics/ending.png");
            // im_image11 = LoadImage("graphics/site4.png");
            // im_image12 = LoadImage("graphics/minimap.png");
        }
    }

}

pub struct Image {
    pub surface: sdl2::surface::Surface<'static>,
    pub rect: sdl2::rect::Rect,
}

impl Image {
    pub fn new (file_name: &str, no_blend: bool, no_alpha: bool) -> Image {
        let file_path = ["/home/sypwex/prj/vvvvvvonrust/assets/graphics/", file_name].concat();
        let decoder = match File::open(file_path.to_owned()) {
            Ok(x) => png::Decoder::new(x),
            Err(e) => panic!("{}: {}", e, file_path),
        };
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut data: Vec<u8> = vec![0; info.buffer_size()];
        reader.next_frame(&mut data).unwrap();

        let surface = match sdl2::surface::Surface::from_data(data.as_mut_slice(), info.width, info.height, info.width*4, sdl2::pixels::PixelFormatEnum::RGBX8888) {
            Ok(x) => x,
            Err(e) => panic!(),
        };
        let mut src_dest = sdl2::surface::Surface::new(32, 32, sdl2::pixels::PixelFormatEnum::RGBX8888).unwrap();
        let src_rect = sdl2::rect::Rect::new(32*11, 32, 32, 32);
        surface.blit(src_rect, &mut src_dest, None);

        match src_dest.save_bmp("sprites.bmp") {
            Ok(_x) => (),
            Err(s) => panic!(s),
        };

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

        Image {
            surface: src_dest,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
        }
    }
}
