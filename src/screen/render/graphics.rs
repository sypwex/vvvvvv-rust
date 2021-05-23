use std::ptr;

use sdl2::render::BlendMode;

use crate::{game, maths, screen::render::graphics::graphics_util::ColourTransform};

extern crate sdl2;
extern crate sdl2_sys;
pub mod graphics_util;
mod graphics_resources;
mod towerbg;

pub struct Graphics {
    screen_pixelformat: sdl2::pixels::PixelFormatEnum,
    pub grphx: graphics_resources::GraphicsResources,
    pub buffers: GraphicBuffers,

    ct: graphics_util::ColourTransform,

    pub flipmode: bool,
	pub setflipmode: bool,
	notextoutline: bool,
	//buffer objects. //TODO refactor buffer objects
	// SDL_Surface* backBuffer;
	// Screen* screenbuffer;
	// SDL_Surface* menubuffer;
	// SDL_Surface* foregroundBuffer;
	// SDL_Surface* tempBuffer;
	// SDL_Surface* warpbuffer;
	// SDL_Surface* warpbuffer_lerp;

	pub towerbg: towerbg::TowerBG,
	pub titlebg: towerbg::TowerBG,

	// SDL_Rect bfont_rect;
	// SDL_Rect tiles_rect;
	// SDL_Rect sprites_rect;
	// SDL_Rect images_rect;
	// SDL_Rect bg_rect;
	// SDL_Rect line_rect;
	// SDL_Rect tele_rect;
	// SDL_Rect towerbuffer_rect;

	// SDL_Rect foot_rect;
	// SDL_Rect prect;
	// SDL_Rect footerrect;
	// SDL_Surface* footerbuffer;

	linestate: i32,
    linedelay: i32,
	backoffset: i32,
	backgrounddrawn: bool,
    foregrounddrawn: bool,

	menuoffset: i32,
	oldmenuoffset: i32,
	resumegamemode: bool,

	// SDL_Rect warprect;

	pub crewframe: i32,
	pub crewframedelay: i32,

	pub fademode: i32,
	fadeamount: i32,
	oldfadeamount: i32,
	fadebars: Vec<i32>,
	ingame_fademode: i32,


    pub alpha: f32,

    pub screenshake_x: i32,
    pub screenshake_y: i32,

    col_crewred: graphics_util::ColourTransform,
    col_crewyellow: graphics_util::ColourTransform,
    col_crewgreen: graphics_util::ColourTransform,
    col_crewcyan: graphics_util::ColourTransform,
    col_crewblue: graphics_util::ColourTransform,
    col_crewpurple: graphics_util::ColourTransform,
    col_crewinactive: graphics_util::ColourTransform,
    col_clock: graphics_util::ColourTransform,
    col_trinket: graphics_util::ColourTransform,
    pub col_tr: i32,
    pub col_tg: i32,
    pub col_tb: i32,

    kludgeswnlinewidth: bool,

    // #ifndef NO_CUSTOM_LEVELS
    tiles1_mounted: bool,
    tiles2_mounted: bool,
    minimap_mounted: bool,
    // #endif

    /* @sx: moved here from graphic_util since required storage for variables */
    oldscrollamount: i32,
    scrollamount: i32,
    isscrolling: bool,
}

impl Graphics {
    // void Graphics::init(void)
    pub fn new(pf: sdl2::pixels::PixelFormatEnum) -> Graphics {
        Graphics {
            screen_pixelformat: pf,
            grphx: graphics_resources::GraphicsResources::new(),
            buffers: GraphicBuffers::create_buffers(pf),

            ct: graphics_util::ColourTransform::new(),

            flipmode: false,
            setflipmode: false,
            notextoutline: false,
            //buffer objects. //TODO refactor buffer objects
            // SDL_Surface* backBuffer;
            // Screen* screenbuffer;
            // SDL_Surface* menubuffer;
            // SDL_Surface* foregroundBuffer;
            // SDL_Surface* tempBuffer;
            // SDL_Surface* warpbuffer;
            // SDL_Surface* warpbuffer_lerp;

            towerbg: towerbg::TowerBG { tdrawback: false, bypos: 0, bscroll: 0, colstate: 0, scrolldir: 0, r: 0, g: 0, b: 0 },
            titlebg: towerbg::TowerBG { tdrawback: false, bypos: 0, bscroll: 0, colstate: 0, scrolldir: 0, r: 0, g: 0, b: 0 },

            // SDL_Rect bfont_rect;
            // SDL_Rect tiles_rect;
            // SDL_Rect sprites_rect;
            // SDL_Rect images_rect;
            // SDL_Rect bg_rect;
            // SDL_Rect line_rect;
            // SDL_Rect tele_rect;
            // SDL_Rect towerbuffer_rect;

            // SDL_Rect foot_rect;
            // SDL_Rect prect;
            // SDL_Rect footerrect;
            // SDL_Surface* footerbuffer;

            linestate: 0,
            linedelay: 0,
            backoffset: 0,
            backgrounddrawn: false,
            foregrounddrawn: false,

            menuoffset: 0,
            oldmenuoffset: 0,
            resumegamemode: false,

            // SDL_Rect warprect;

            crewframe: 0,
            crewframedelay: 4,

            fademode: 0,
            fadeamount: 0, // TODO @sx set via mutator
            oldfadeamount: 0, // TODO @sx set via mutator
            fadebars: vec![],
            ingame_fademode: 0,


            alpha: 1.0f32,

            screenshake_x: 0,
            screenshake_y: 0,

            col_crewred: ColourTransform::new(),
            col_crewyellow: ColourTransform::new(),
            col_crewgreen: ColourTransform::new(),
            col_crewcyan: ColourTransform::new(),
            col_crewblue: ColourTransform::new(),
            col_crewpurple: ColourTransform::new(),
            col_crewinactive: ColourTransform::new(),
            col_clock: ColourTransform::new(),
            col_trinket: ColourTransform::new(),
            col_tr: 0,
            col_tg: 0,
            col_tb: 0,

            kludgeswnlinewidth: false,

            // #ifndef NO_CUSTOM_LEVELS
            tiles1_mounted: false,
            tiles2_mounted: false,
            minimap_mounted: false,
            // #endif

            /* @sx: moved here from graphic_util since required storage for variables */
            oldscrollamount: 0,
            scrollamount: 0,
            isscrolling: false,
        }
    }

    // void Graphics::destroy(void)

    // int Graphics::font_idx(uint32_t ch)
    fn font_idx(ch: i32) -> usize {
        ch as usize
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

    // void Graphics::drawspritesetcol(int x, int y, int t, int c)
    // void Graphics::updatetitlecolours(void)

    // void Graphics::Makebfont(void)
    fn make_bfont(&mut self) {
        // PROCESS_TILESHEET(self.grphx.bfont, 8/*, {
        //     SDL_Surface* TempFlipped = FlipSurfaceVerticle(temp);
        //     flipbfont.push_back(TempFlipped);
        // }*/);

        // actually this code is unused
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

    // int Graphics::bfontlen(uint32_t ch)
    fn bfontlen(ch: i32) -> i32 {
        if ch < 32 {
            6
        } else {
            8
        }
    }

    // void Graphics::MakeTileArray(void)
    // void Graphics::maketelearray(void)
    // void Graphics::MakeSpriteArray(void)
    // void Graphics::map_tab(int opt, const std::string& text, bool selected /*= false*/)
    // void Graphics::map_option(int opt, int num_opts, const std::string& text, bool selected /*= false*/)

    // void Graphics::Print( int _x, int _y, std::string _s, int r, int g, int b, bool cen /*= false*/ )
    pub fn print(&mut self, x: i32, y: i32, s: String, r: i32, g: i32, b: i32, cen: bool) {
        self.print_alpha(x, y, s, r, g, b, 255, cen);
    }

    // void Graphics::PrintAlpha( int _x, int _y, std::string _s, int r, int g, int b, int a, bool cen /*= false*/ )
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
            match &self.grphx.bfont.surfaces.get(idx) {
                Some(char_surface) => {
                    graphics_util::BlitSurfaceColoured(
                        char_surface,
                        None,
                        self.buffers.backBuffer.as_mut(),
                        self.grphx.bfont.rect,
                        self.ct.colour
                    );
                },
                None => (),
            }

            bfontpos += Graphics::bfontlen(curr as i32);
        }
    }

    // void Graphics::bigprint(  int _x, int _y, std::string _s, int r, int g, int b, bool cen, int sc )
    // void Graphics::bigbprint(int x, int y, std::string s, int r, int g, int b, bool cen, int sc)
    // int Graphics::len(std::string t)
    // void Graphics::PrintOffAlpha( int _x, int _y, std::string _s, int r, int g, int b, int a, bool cen /*= false*/ )
    // void Graphics::bprint( int x, int y, std::string t, int r, int g, int b, bool cen /*= false*/ ) {
    // void Graphics::bprintalpha( int x, int y, std::string t, int r, int g, int b, int a, bool cen /*= false*/ )
    // void Graphics::printcrewname( int x, int y, int t )
    // void Graphics::printcrewnamedark( int x, int y, int t )
    // void Graphics::printcrewnamestatus( int x, int y, int t )

    // void Graphics::drawsprite( int x, int y, int t, int r, int g,  int b )
    pub fn draw_sprite(&mut self, x: i32, y: i32, t: i32, r: i32, g: i32, b: i32) {
        // SDL_Rect rect = { Sint16(x), Sint16(y), sprites_rect.w, sprites_rect.h };
        let sprites_rect = self.grphx.sprites.rect;
        let rect_dst = sdl2::rect::Rect::new(x, y, sprites_rect.w as u32, sprites_rect.h as u32);

        let ct = self.getRGB(r,g,b);
        self.setcolreal(ct);

        let sprite = &self.grphx.sprites.surfaces[t as usize];
        graphics_util::BlitSurfaceColoured(
            sprite,
            None,
            self.buffers.backBuffer.as_mut(),
            rect_dst,
            ct
        );
    }

    // void Graphics::drawsprite(int x, int y, int t, Uint32 c)
    // bool Graphics::shouldrecoloroneway(const int tilenum, const bool mounted)
    // void Graphics::drawtile( int x, int y, int t )
    // void Graphics::drawtile2( int x, int y, int t )
    // void Graphics::drawtile3( int x, int y, int t, int off, int height_subtract /*= 0*/ )
    // void Graphics::drawtowertile( int x, int y, int t )
    // void Graphics::drawtowertile3( int x, int y, int t, TowerBG& bg_obj )
    // void Graphics::drawgui(void)
    // void Graphics::updatetextboxes(void)
    // void Graphics::drawimagecol( int t, int xp, int yp, int r = 0, int g = 0, int b = 0, bool cent/*= false*/ )
    // void Graphics::drawimage( int t, int xp, int yp, bool cent/*=false*/ )
    // void Graphics::drawpartimage( int t, int xp, int yp, int wp, int hp)
    // void Graphics::cutscenebars(void)
    // void Graphics::cutscenebarstimer(void)
    // void Graphics::setbars(const int position)
    // void Graphics::drawcrewman( int x, int y, int t, bool act, bool noshift /*=false*/ )
    // void Graphics::drawpixeltextbox( int x, int y, int w, int h, int w2, int h2, int r, int g, int b, int xo, int yo )
    // void Graphics::drawcustompixeltextbox( int x, int y, int w, int h, int w2, int h2, int r, int g, int b, int xo, int yo )
    // void Graphics::drawtextbox( int x, int y, int w, int h, int r, int g, int b )
    // void Graphics::textboxactive(void)
    // void Graphics::textboxremovefast(void)
    // void Graphics::textboxremove(void)
    // void Graphics::textboxtimer( int t )
    // void Graphics::addline( std::string t )
    // void Graphics::textboxadjust(void)
    // void Graphics::createtextboxreal(std::string t, int xp, int yp, int r, int g, int b, bool flipme)
    // void Graphics::createtextbox(std::string t, int xp, int yp, int r, int g, int b)
    // void Graphics::createtextboxflipme(std::string t, int xp, int yp, int r, int g, int b)

    // void Graphics::drawfade(void)
    pub fn drawfade(&mut self) {
        let usethisamount = self.lerp(self.oldfadeamount as f32, self.fadeamount as f32) as u32;

        match self.fademode {
            // 1 -
            // 4 - fadein
            i if i == 1 || i == 4 => {
                graphics_util::ClearSurface(&mut self.buffers.backBuffer);
            },
            3 => {
                for (i, fadebar) in self.fadebars.iter().enumerate() {
                    let i = i as u32;
                    let fadebar = *fadebar as u32;
                    graphics_util::FillRect(self.buffers.backBuffer.as_mut(), fadebar, i * 16, usethisamount, 16, sdl2::pixels::Color::BLACK);
                }
            },
            5 => { // 5 - prepare fade in
                for (i, fadebar) in self.fadebars.iter().enumerate() {
                    let i = i as u32;
                    let fadebar = *fadebar as u32;
                    graphics_util::FillRect(self.buffers.backBuffer.as_mut(), fadebar - usethisamount, i * 16, 500, 16, sdl2::pixels::Color::BLACK);
                }
            },
            // 2 => (), // 2 - fade out
            0 => (), // normal - no fade
            _ => {
                panic!("unknown drawfade value {}", self.fademode);
            }
        }
    }

    // void Graphics::processfade(void)
    // void Graphics::setfade(const int amount)
    // void Graphics::drawmenu( int cr, int cg, int cb, bool levelmenu /*= false*/ )
    // void Graphics::drawcoloredtile( int x, int y, int t, int r, int g, int b )
    // bool Graphics::Hitest(SDL_Surface* surface1, point p1, SDL_Surface* surface2, point p2)
    // void Graphics::drawgravityline( int t )
    // void Graphics::drawtrophytext(void)
    // void Graphics::drawentities(void)
    // void Graphics::drawentity(const int i, const int yoff)
    // void Graphics::drawbackground( int t )
    // void Graphics::updatebackground(int t)
    // void Graphics::drawmap(void)
    // void Graphics::drawfinalmap(void)
    // void Graphics::drawtowermap(void)
    // void Graphics::drawtowerspikes(void)
    // void Graphics::drawtowerbackground(const TowerBG& bg_obj)
    // void Graphics::updatetowerbackground(TowerBG& bg_obj)
    // void Graphics::setcol( int t )
    // void Graphics::menuoffrender(void)
    // void Graphics::drawhuetile( int x, int y, int t )
    // void Graphics::huetilesetcol(int t)
    // Uint32 Graphics::bigchunkygetcol(int t)
    // void Graphics::setwarprect( int a, int b, int c, int d )
    // void Graphics::textboxcenterx(void)
    // int Graphics::textboxwidth(void)
    // void Graphics::textboxmoveto(int xo)
    // void Graphics::textboxcentery(void)
    // int Graphics::crewcolour(const int t)

    // void Graphics::updatescreenshake(void)
    fn updatescreenshake(&mut self) {
        self.screenshake_x = (maths::fRandom() as i32 * 7) - 4;
        self.screenshake_y = (maths::fRandom() as i32 * 7) - 4;
    }

    // void Graphics::renderfixedpre(void)
    pub fn renderfixedpre(&mut self, game: &mut game::Game, badSignalEffect: bool) {
        // TODO @sx @impl
        // println!("DEADBEEF: Graphics::renderfixedpre method is not implemented yet");

        if game.screenshake > 0 {
            self.updatescreenshake();
        }

        // if screenbuffer.badSignalEffect {
        if badSignalEffect {
            self.UpdateFilter();
        }
    }

    // void Graphics::renderfixedpost(void)
    // void Graphics::bigrprint(int x, int y, std::string& t, int r, int g, int b, bool cen, float sc)
    // void Graphics::bigbrprint(int x, int y, std::string& s, int r, int g, int b, bool cen, float sc)
    // void Graphics::drawtele(int x, int y, int t, Uint32 c)

    // Uint32 Graphics::getRGBA(Uint8 r, Uint8 g, Uint8 b, Uint8 a)
    fn get_rgba(&self, r: u8, g: u8, b: u8, a: u8) -> u32 {
        // SDL_MapRGBA(backBuffer->format, b, g, r, a);
        let sdl_color = sdl2::pixels::Color::RGBA(r, g, b, a);
        let pf = self.buffers.backBuffer.pixel_format();

        sdl_color.to_u32(&pf)
    }

    // Uint32 Graphics::getRGB(Uint8 r, Uint8 g, Uint8 b)
    fn getRGB(&self, r: i32, g: i32, b: i32) -> u32 {
        unsafe {
	        // SDL_MapRGB(backBuffer->format, b, g, r);
            sdl2_sys::SDL_MapRGB(self.buffers.backBuffer.pixel_format().raw(), b as u8, g as u8, r as u8)
        }
    }

    // Uint32 Graphics::getBGR(Uint8 r, Uint8 g, Uint8 b)
    pub fn getBGR(&self, r: i32, g: i32, b: i32) -> u32 {
        unsafe {
	        // SDL_MapRGB(backBuffer->format, b, g, r);
            sdl2_sys::SDL_MapRGB(self.buffers.backBuffer.pixel_format().raw(), r as u8, g as u8, b as u8)
        }
    }
    pub fn getBGR_AsPixelColor(&self, r: i32, g: i32, b: i32) -> sdl2::pixels::Color {
        unsafe {
	        // SDL_MapRGB(backBuffer->format, b, g, r);
            let color = sdl2_sys::SDL_MapRGB(self.buffers.backBuffer.pixel_format().raw(), r as u8, g as u8, b as u8);
            let color = sdl2::pixels::Color::from_u32(&self.buffers.backBuffer.pixel_format(), color);
            color
        }
    }

    // Uint32 Graphics::getRGB(Uint32 _col)
    // Uint32 Graphics::RGBflip(Uint8  r, Uint8  g, Uint8  b)
    pub fn RGBflip(&mut self, r: i32, g: i32, b: i32) -> u32 {
        unsafe {
	        // SDL_MapRGB(backBuffer->format, r, g, b);
            sdl2_sys::SDL_MapRGB(self.buffers.backBuffer.pixel_format().raw(), r as u8, g as u8, b as u8)
        }
    }

    pub fn RGBflip_AsPixelColor(&mut self, r: i32, g: i32, b: i32) -> sdl2::pixels::Color {
        unsafe {
	        // SDL_MapRGB(backBuffer->format, r, g, b);
            let color = sdl2_sys::SDL_MapRGB(self.buffers.backBuffer.pixel_format().raw(), r as u8, g as u8, b as u8);
            let color = sdl2::pixels::Color::from_u32(&self.buffers.backBuffer.pixel_format(), color);
            color
        }
    }

    // Uint32 Graphics::RGBf(int r, int g, int b)

    // void Graphics::setcolreal(Uint32 t)
    fn setcolreal(&mut self, t: u32) {
	    self.ct.colour = t;
    }

    // void Graphics::drawforetile(int x, int y, int t)
    // void Graphics::drawforetile2(int x, int y, int t)
    // void Graphics::drawforetile3(int x, int y, int t, int off)
    // void Graphics::drawrect(int x, int y, int w, int h, int r, int g, int b)
    // bool Graphics::onscreen(int t)

    // void Graphics::reloadresources(void)
    pub fn reload_resources(&mut self) {
        self.make_bfont();
    }

    // Uint32 Graphics::crewcolourreal(int t)

    /* graphics.cpp inline methods */

    // float inline lerp(const float v0, const float v1)
    fn lerp(&self, v0: f32, v1: f32) -> f32 {
        v0 + self.alpha * (v1 - v0)
    }

    /* @sx: moved here from graphic_util since required storage for variables */

    // void UpdateFilter(void)
    pub fn UpdateFilter(&mut self) {
        if maths::c_rand() % 4000 < 8 {
            self.isscrolling = true;
        }

        self.oldscrollamount = self.scrollamount;
        if self.isscrolling == true {
            self.scrollamount += 20;
            if self.scrollamount > 240 {
                self.scrollamount = 0;
                self.oldscrollamount = 0;
                self.isscrolling = false;
            }
        }
    }

    // SDL_Surface* ApplyFilter( SDL_Surface* _src )
    pub fn ApplyFilter(&mut self, src: &'static sdl2::surface::SurfaceRef) -> sdl2::surface::Surface<'static> {
        let mut _ret = graphics_util::RecreateSurface(src);
        let redOffset = maths::c_rand() % 4;

        let Gmask;
        let Bmask;
        let Rmask;
        let Amask;
        unsafe {
            let format = src.pixel_format();
            let pf = *format.raw();
            Gmask = pf.Gmask;
            Bmask = pf.Bmask;
            Rmask = pf.Rmask;
            Amask = pf.Amask;
        }

        for x in 0..src.width() as i32 {
            for y in 0..src.height() as i32 {
                let sampley = (y + (self.lerp(self.oldscrollamount as f32, self.scrollamount as f32) as i32) ) % 240;
                let pixel = graphics_util::ReadPixel(src, x, sampley);

                let mut green = ( (pixel & Gmask) >> 8) as u8;
                let mut blue = ( (pixel & Bmask) >> 0) as u8;

                let pixelOffset = graphics_util::ReadPixel(src, maths::VVV_min(x+redOffset, 319), sampley);
                let mut red = ( (pixelOffset & Rmask) >> 16) as u8;

                if self.isscrolling && sampley > 220 && ((maths::c_rand() % 10) < 4) {
                    red   = maths::VVV_min((red   as i32 + (maths::fRandom() * 0.6) as i32 * 254) as i32, 255) as u8;
                    green = maths::VVV_min((green as i32 + (maths::fRandom() * 0.6) as i32 * 254) as i32, 255) as u8;
                    blue  = maths::VVV_min((blue  as i32 + (maths::fRandom() * 0.6) as i32 * 254) as i32, 255) as u8;
                } else {
                    red   = maths::VVV_min((red   as i32 + (maths::fRandom() * 0.2) as i32 * 254) as i32, 255) as u8;
                    green = maths::VVV_min((green as i32 + (maths::fRandom() * 0.2) as i32 * 254) as i32, 255) as u8;
                    blue  = maths::VVV_min((blue  as i32 + (maths::fRandom() * 0.2) as i32 * 254) as i32, 255) as u8;
                }

                if y % 2 == 0 {
                    red = (red as f32 / 1.2f32) as u8;
                    green = (green as f32 / 1.2f32) as u8;
                    blue = (blue as f32 / 1.2f32) as u8;
                }

                let distX = (((160.0f32 - x as f32) / 160.0f32) * 16f32).abs() as i32;
                let distY = (((120.0f32 - y as f32) / 120.0f32) * 32f32).abs() as i32;

                let red   = maths::VVV_max(red   as i32 - (distX + distY), 0) as u32;
                let green = maths::VVV_max(green as i32 - (distX + distY), 0) as u32;
                let blue  = maths::VVV_max(blue  as i32 - (distX + distY), 0) as u32;

                let finalPixel = ((red << 16) + (green << 8) + (blue << 0)) | (pixel & Amask);
                graphics_util::DrawPixel(&mut _ret, x, y, finalPixel);

            }
        }

        _ret
    }

}

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

pub struct GraphicBuffers {
    pub backBuffer: sdl2::surface::Surface<'static>,
    pub footerbuffer: sdl2::surface::Surface<'static>,
    pub ghostbuffer: sdl2::surface::Surface<'static>,
    pub foregroundBuffer: sdl2::surface::Surface<'static>,
    pub menubuffer: sdl2::surface::Surface<'static>,
    pub warpbuffer: sdl2::surface::Surface<'static>,
    pub warpbuffer_lerp: sdl2::surface::Surface<'static>,
    pub towerbg_buffer: sdl2::surface::Surface<'static>,
    pub towerbg_buffer_lerp: sdl2::surface::Surface<'static>,
    pub titlebg_buffer: sdl2::surface::Surface<'static>,
    pub titlebg_buffer_lerp: sdl2::surface::Surface<'static>,
    pub tempBuffer: sdl2::surface::Surface<'static>,
}

impl GraphicBuffers {
    // void Graphics::create_buffers(const SDL_PixelFormat* fmt)
    pub fn create_buffers(pf: sdl2::pixels::PixelFormatEnum) -> GraphicBuffers {
        let mut backBuffer = CREATE_SURFACE(pf, 320, 240).unwrap();
        backBuffer.set_blend_mode(BlendMode::None).unwrap();

        let mut footerbuffer = CREATE_SURFACE(pf, 320, 10).unwrap();
        footerbuffer.set_blend_mode(BlendMode::Blend).unwrap();
        // SDL_SetSurfaceAlphaMod(footerbuffer, 127);
        // FillRect(footerbuffer, SDL_MapRGB(fmt, 0, 0, 0));

        let mut ghostbuffer = CREATE_SURFACE(pf, 320, 240).unwrap();
        ghostbuffer.set_blend_mode(BlendMode::Blend).unwrap();
        // SDL_SetSurfaceAlphaMod(ghostbuffer, 127);

        let mut foregroundBuffer = CREATE_SURFACE(pf, 320, 240).unwrap();
        foregroundBuffer.set_blend_mode(BlendMode::Blend).unwrap();

        let mut menubuffer = CREATE_SURFACE(pf, 320, 240).unwrap();
        menubuffer.set_blend_mode(BlendMode::None).unwrap();

        let mut warpbuffer = CREATE_SURFACE(pf, 320 + 16, 240 + 16).unwrap();
        warpbuffer.set_blend_mode(BlendMode::None).unwrap();

        let mut warpbuffer_lerp = CREATE_SURFACE(pf, 320 + 16, 240 + 16).unwrap();
        warpbuffer_lerp.set_blend_mode(BlendMode::None).unwrap();

        let mut towerbg_buffer = CREATE_SURFACE(pf, 320 + 16, 240 + 16).unwrap();
        towerbg_buffer.set_blend_mode(BlendMode::None).unwrap();

        let mut towerbg_buffer_lerp = CREATE_SURFACE(pf, 320 + 16, 240 + 16).unwrap();
        towerbg_buffer_lerp.set_blend_mode(BlendMode::None).unwrap();

        let mut titlebg_buffer = CREATE_SURFACE(pf, 320 + 16, 240 + 16).unwrap();
        titlebg_buffer.set_blend_mode(BlendMode::None).unwrap();

        let mut titlebg_buffer_lerp = CREATE_SURFACE(pf, 320 + 16, 240 + 16).unwrap();
        titlebg_buffer_lerp.set_blend_mode(BlendMode::None).unwrap();

        let mut tempBuffer = CREATE_SURFACE(pf, 320, 240).unwrap();
        tempBuffer.set_blend_mode(BlendMode::None).unwrap();

        GraphicBuffers {
            backBuffer,
            footerbuffer,
            ghostbuffer,
            foregroundBuffer,
            menubuffer,
            warpbuffer,
            warpbuffer_lerp,
            towerbg_buffer,
            towerbg_buffer_lerp,
            titlebg_buffer,
            titlebg_buffer_lerp,
            tempBuffer,
        }
    }

    // void Graphics::destroy_buffers(void)

    pub fn fill_back_buffer_with_color(&mut self, color: sdl2::pixels::Color) {
        let rect = sdl2::rect::Rect::new(0, 0, self.backBuffer.width(), self.backBuffer.height());
        self.backBuffer.fill_rect(rect, color);
    }

    pub fn fill_back_buffer_with_color_at_xy(&mut self, x: u32, y: u32, w: u32, h: u32, color: sdl2::pixels::Color) {
        // TODO @sx make sanity checks for width and height
        let rect = sdl2::rect::Rect::new(x as i32, y as i32, w, h);
        self.backBuffer.fill_rect(rect, color);
    }

    pub fn clear_back_buffer(&mut self) {
        graphics_util::ClearSurface(self.backBuffer.as_mut());
    }

    pub fn get_back_buffer_rect(&mut self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(0, 0, self.backBuffer.width(), self.backBuffer.height())
    }
}

pub fn CREATE_SURFACE(pf: sdl2::pixels::PixelFormatEnum, w: u32, h: u32) -> Result<sdl2::surface::Surface<'static>, String> {
    // SDL_CreateRGBSurface( \
    //     SDL_SWSURFACE, \
    //     w, h, \
    //     fmt->BitsPerPixel, \
    //     fmt->Rmask, fmt->Gmask, fmt->Bmask, fmt->Amask \
    // )
    sdl2::surface::Surface::new(w, h, pf)
}
