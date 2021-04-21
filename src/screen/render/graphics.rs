extern crate sdl2;
mod resources;
// use sdl2::pixels::Color;

pub struct Graphics {
    pub backBuffer: Box<sdl2::surface::Surface<'static>>,

    // ct:

    pub graphic_resources: resources::Resources,

}

impl Graphics {
    pub fn new () -> Graphics {
        let mut backBuffer = CREATE_SURFACE(320, 240);
        backBuffer.set_blend_mode(sdl2::render::BlendMode::None);
        // SDL_SetSurfaceBlendMode(graphics.backBuffer, SDL_BLENDMODE_NONE);

        Graphics {
            backBuffer: Box::new(backBuffer),
            graphic_resources: resources::Resources::new(),
        }
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

    pub fn make_font() {
        // PROCESS_TILESHEET(bfont, 8,
        // {
        //     SDL_Surface* TempFlipped = FlipSurfaceVerticle(temp);
        //     flipbfont.push_back(TempFlipped);
        // })

        // unsigned char* charmap = NULL;
        // size_t length;
        // FILESYSTEM_loadFileToMemory("graphics/font.txt", &charmap, &length);
        // if (charmap != NULL)
        // {
        //     unsigned char* current = charmap;
        //     unsigned char* end = charmap + length;
        //     int pos = 0;
        //     while (current != end)
        //     {
        //         int codepoint = utf8::unchecked::next(current);
        //         font_positions[codepoint] = pos;
        //         ++pos;
        //     }
        //     FILESYSTEM_freeMemory(&charmap);
        // }
        // else
        // {
        //     font_positions.clear();
        // }
    }

    pub fn print(&mut self, _x: i32, _y: i32, _s: String, r: i32, g: i32, b: i32, cen: bool) {
        self.print_alpha(_x, _y, _s, r, g, b, 255, cen);
    }

    pub fn print_alpha(&mut self, _x: i32, _y: i32, _s: String, r: i32, g: i32, b: i32, a: i32, cen:bool /*= false*/ ) {
        // std::vector<SDL_Surface*>& font = flipmode ? flipbfont : bfont;
        // let font: sdl2::surface::Surface<'static> = self.font;

        // r = clamp(r,0,255);
        // g = clamp(g,0,255);
        // b = clamp(b,0,255);
        // a = clamp(a,0,255);

        // ct.colour = getRGBA(r, g, b, a);

        // if (cen)
        //     _x = ((160 ) - ((len(_s)) / 2));
        // int bfontpos = 0;
        // int curr;
        // int idx;
        // std::string::iterator iter = _s.begin();
        // while (iter != _s.end()) {
        //     curr = utf8::unchecked::next(iter);
        //     point tpoint;
        //     tpoint.x = _x + bfontpos;
        //     tpoint.y = _y;

        //     SDL_Rect fontRect = bfont_rect;
        //     fontRect.x = tpoint.x ;
        //     fontRect.y = tpoint.y ;

        //     idx = font_idx(curr);
        //     if (INBOUNDS_VEC(idx, font))
        //     {
        //         BlitSurfaceColoured( font[idx], NULL, backBuffer, &fontRect , ct);
        //     }
        //     bfontpos+=bfontlen(curr) ;
        // }
    }

    pub fn draw_sprite(&mut self, x: i32, y: i32, t: i32, r: i32, g: i32, b: i32) {
        // SDL_Rect rect = { Sint16(x), Sint16(y), sprites_rect.w, sprites_rect.h };
        let rect_src = sdl2::rect::Rect::new(0, 0, 50, 50);
        let rect_dst = sdl2::rect::Rect::new(x, y, self.graphic_resources.sprites.rect.width(), self.graphic_resources.sprites.rect.height());

        // setcolreal(getRGB(r,g,b));

        // BlitSurfaceColoured(sprites[t], NULL, self.backBuffer, &rect, ct);

        // SDL_BlitSurface(tempsurface, _srcRect, _dest, tempRect);
        let tempSurface = sdl2::surface::Surface::new(320, 240, sdl2::pixels::PixelFormatEnum::RGB24).unwrap();
        self.graphic_resources.sprites.surface.blit(rect_src, &mut self.backBuffer, rect_dst);

        match self.graphic_resources.sprites.surface.save_bmp("drawsprite_from.bmp") {
            Ok(_x) => (),
            Err(s) => panic!(s),
        };
        match self.backBuffer.save_bmp("drawsprite_to.bmp") {
            Ok(_x) => (),
            Err(s) => panic!(s),
        };
    }

}

fn CREATE_SURFACE(w: u32, h: u32) -> sdl2::surface::Surface<'static> {
  sdl2::surface::Surface::new(w, h, sdl2::pixels::PixelFormatEnum::RGB24).unwrap()

  // SDL_CreateRGBSurface( \
  //     SDL_SWSURFACE, \
  //     w, h, \
  //     fmt->BitsPerPixel, \
  //     fmt->Rmask, fmt->Gmask, fmt->Bmask, fmt->Amask \
  // )
}
