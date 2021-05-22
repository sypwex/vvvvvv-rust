use std::ptr;

use sdl2::surface::Surface;

extern crate sdl2;
extern crate sdl2_sys;
mod maths;
mod graphics_util;
mod graphics_resources;

pub struct Graphics {
    pub grphx: graphics_resources::GraphicsResources,
    pub backBuffer: sdl2::surface::Surface<'static>,
    ct: graphics_util::ColourTransform,
}

impl Graphics {
    pub fn new () -> Graphics {
        let mut backBuffer = CREATE_SURFACE(320, 240).unwrap();
        backBuffer.set_blend_mode(sdl2::render::BlendMode::None);
        // SDL_SetSurfaceBlendMode(graphics.backBuffer, SDL_BLENDMODE_NONE);

        Graphics {
            backBuffer: backBuffer,
            ct: graphics_util::ColourTransform::new(),
            grphx: graphics_resources::GraphicsResources::new(),
        }
    }

    pub fn reload_resources (&mut self) {
        self.make_bfont();
    }

    pub fn fill_back_buffer_with_color (&mut self, color: sdl2::pixels::Color ) {
        let rect = sdl2::rect::Rect::new(0, 0, self.backBuffer.width(), self.backBuffer.height());
        self.backBuffer.fill_rect(rect, color);
    }

    // pub fn render (&self) {
    //     let rect = sdl2::rect::Rect::new(0, 0, self.backBuffer.width, self.backBuffer.height);
	// 	screenbuffer->UpdateScreen(backBuffer, &rect);
    // }

    pub fn get_back_buffer_rect (&mut self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(0, 0, self.backBuffer.width(), self.backBuffer.height())
    }

    /* */

    pub fn print(&mut self, x: i32, y: i32, s: String, r: i32, g: i32, b: i32, cen: bool) {
        self.print_alpha(x, y, s, r, g, b, 255, cen);
    }

    pub fn print_alpha(&mut self, x: i32, y: i32, s: String, r: i32, g: i32, b: i32, a: i32, cen: bool /*= false*/ ) {
        let mut xx = x;
        // std::vector<SDL_Surface*>& font = flipmode ? flipbfont : bfont;

        // println!("{:?} === {:?} {:?} {:?} {:?}", self.ct.colour, r, g, b, a);
        let r = maths::clamp(r, 0, 255) as u8;
        let g = maths::clamp(g, 0, 255) as u8;
        let b = maths::clamp(b, 0, 255) as u8;
        let a = maths::clamp(a, 0, 255) as u8;

        self.ct.colour = self.get_rgba(r, g, b, a);

        if cen {
            xx = 160 - (s.len() / 2) as i32;
        }

        let mut bfontpos = 0;

        for (curr, c) in s.chars().enumerate() {
            let ix = xx + bfontpos;
            let iy = y;

            self.grphx.bfont.rect.x = ix;
            self.grphx.bfont.rect.y = iy;

            let idx = Graphics::font_idx(c as i32);
            // if INBOUNDS_VEC(idx, font) {
            //     let rect = sdl2::rect::Rect::new(0, 0, 0, 0);
            //     graphics_util::BlitSurfaceColoured(font[idx], rect, self.backBuffer, font_rect, self.ct);
            // }

            let rect = sdl2::rect::Rect::new(0, 0, 8, 8);
            let char_surface = &self.grphx.bfont.surfaces[idx as usize];
            graphics_util::BlitSurfaceColoured(
                char_surface.raw(),
                char_surface.blend_mode(),
                rect,
                self.backBuffer.as_mut(),
                self.grphx.bfont.rect.as_mut(),
                self.ct.colour,
                c
            );
            bfontpos += Graphics::bfontlen(curr as i32);
        }
    }

    pub fn draw_sprite(&mut self, x: i32, y: i32, t: i32, r: i32, g: i32, b: i32) {
        // SDL_Rect rect = { Sint16(x), Sint16(y), sprites_rect.w, sprites_rect.h };
        let rect_src = sdl2::rect::Rect::new(0, 0, 50, 50);
        let sprites_rect = self.grphx.sprites.rect;
        let rect_dst = sdl2::rect::Rect::new(x, y, sprites_rect.w as u32, sprites_rect.h as u32);

        // setcolreal(getRGB(r,g,b));

        // BlitSurfaceColoured(sprites[t], NULL, self.backBuffer, &rect, ct);

        // SDL_BlitSurface(tempsurface, _srcRect, _dest, tempRect);
        let tempSurface = sdl2::surface::Surface::new(320, 240, sdl2::pixels::PixelFormatEnum::RGB24).unwrap();
        self.grphx.sprites.surfaces[t as usize].blit(rect_src, &mut (self.backBuffer), rect_dst).unwrap();

        crate::rustutil::dump_surface("draw_sprite", &self.backBuffer);
    }

    /* private */

    fn make_bfont(&mut self) {
        // PROCESS_TILESHEET(self.grphx.bfont, 8/*, {
        //     SDL_Surface* TempFlipped = FlipSurfaceVerticle(temp);
        //     flipbfont.push_back(TempFlipped);
        // }*/);

        // unsigned char* charmap;
        // size_t length;
        // FILESYSTEM_loadAssetToMemory("graphics/font.txt", &charmap, &length, false);
        //     unsigned char* current = charmap;
        //     unsigned char* end = charmap + length;
        //     let pos = 0;
        //     while current != end {
        //         let codepoint: i32 = utf8::unchecked::next(current);
        //         font_positions[codepoint] = pos;
        //         pos += 1;
        //     }
        //     FILESYSTEM_freeMemory(&charmap);
        // }
    }

    fn bfontlen(ch: i32) -> i32 {
        if ch < 32 {
            6
        } else {
            8
        }
    }

    fn font_idx(ch: i32) -> i32 {
        ch
        // if font_positions.size() > 0 {
        //     std::map<int, int>::iterator iter = font_positions.find(ch);
        //     if (iter == font_positions.end()) {
        //         iter = font_positions.find('?');
        //         if iter == font_positions.end() {
        //             panic!("font.txt missing fallback character!")
        //         }
        //     }

        //     iter->second
        // } else {
        //     ch
        // }
    }

	// void drawtele(int x, int y, int t, Uint32 c);

    fn get_rgba(&self, r: u8, g: u8, b: u8, a: u8) -> u32 {
        // SDL_MapRGBA(backBuffer->format, b, g, r, a);
        let sdl_color = sdl2::pixels::Color::RGBA(r, g, b, a);
        let pf = self.backBuffer.pixel_format();

        sdl_color.to_u32(&pf)
    }

	// Uint32 getRGBA(Uint8 r, Uint8 g, Uint8 b, Uint8 a);
	// Uint32 getRGB(Uint8 r, Uint8 g, Uint8 b);
	// Uint32 getBGR(Uint8 r, Uint8 g, Uint8 b);
	// Uint32 getRGB(Uint32 _col);
	// Uint32 RGBflip(Uint8  r, Uint8  g, Uint8  b);

}

/* from create_buffers */

fn CREATE_SURFACE(w: u32, h: u32) -> Result<sdl2::surface::Surface<'static>, String> {
  sdl2::surface::Surface::new(w, h, sdl2::pixels::PixelFormatEnum::RGB24)

  // SDL_CreateRGBSurface( \
  //     SDL_SWSURFACE, \
  //     w, h, \
  //     fmt->BitsPerPixel, \
  //     fmt->Rmask, fmt->Gmask, fmt->Bmask, fmt->Amask \
  // )
}

/* endfrom create_buffers */

fn PROCESS_TILESHEET(tilesheet: graphics_resources::Image, tile_square: u8/*, extra_code*/) {
    if !PROCESS_TILESHEET_CHECK_ERROR(tilesheet, tile_square) {
        // let _rect: sdl2_sys::SDL_Rect;
        // unsafe {
        //     // _rect = *(*tilesheet.rect);
        // }

        // let mut j = 1;
        // while j < rect.h / tile_square as i32 {
        //     let mut i = 1;
        //     while i < rect.w / tile_square as i32 {
        //         let temp = graphics_util::GetSubSurface(
        //             Box::new(tilesheet.surface.raw()),
        //             i * tile_square as i32,
        //             j * tile_square as i32,
        //             tile_square as i32,
        //             tile_square as i32
        //         );
        //         // tilesheet.push_back(temp);
        //         j += 1;

        //         // extra_code
        //     }

        //     j += 1;
        // }

        // unsafe {
        //     sdl2_sys::SDL_FreeSurface(tilesheet.surface);
        // }
        // drop(tilesheet);
    }
}

fn PROCESS_TILESHEET_CHECK_ERROR(tilesheet: graphics_resources::Image, tile_square: u8) -> bool {
    let rect = tilesheet.rect;

    if ((rect.w % tile_square as i32) != 0) || ((rect.h % tile_square as i32) != 0) {
        let error = "Error: %s.png dimensions not exact multiples of %i!";
        let message: [char; 128] = [' '; 128];
        let error_title = "Error with %s.png";
        let message_title: [char; 128] = [' '; 128];

        unsafe {
            sdl2_sys::SDL_snprintf(
                message.as_ptr() as *mut libc::c_char,
                message.len() as u64,
                error.as_ptr() as *const libc::c_char,
                tilesheet.name.as_str(),
                tile_square as libc::c_uint
            );
            sdl2_sys::SDL_snprintf(
                message_title.as_ptr() as *mut i8,
                message_title.len() as u64,
                error_title.as_ptr() as *const libc::c_char,
                tilesheet.name.as_str()
            );

            sdl2_sys::SDL_ShowSimpleMessageBox(
                sdl2_sys::SDL_MessageBoxFlags::SDL_MESSAGEBOX_ERROR as u32,
                message_title.as_ptr() as *const libc::c_char,
                message.as_ptr() as *const libc::c_char,
                ptr::null_mut()
            );
        }

        panic!("{:?}", message)
    } else {
        true
    }
}

/* Utility class .h */

// fn INBOUNDS_VEC(index: i32, vector: Vec<i32>) -> bool {
fn INBOUNDS_VEC(index: i32, vector: Surface) -> bool {
    true
    // index >= 0 && index < vector.size()
}
