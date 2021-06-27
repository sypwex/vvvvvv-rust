use std::{collections::HashMap, ptr};
extern crate sdl2;
extern crate sdl2_sys;
use sdl2::render::BlendMode;

use crate::{game::{self, SLIDERMODE}, map, maths, screen::render::graphics::graphics_util::ColourTransform};

use super::BackGround;
pub mod graphics_util;
mod graphics_resources;
mod textbox;
pub mod towerbg;

pub enum Color {
    Clock,
    Trinket,
}

const numstars: usize = 50;
const numbackboxes: usize = 18;

pub struct Graphics {
    screen_pixelformat: sdl2::pixels::PixelFormatEnum,
    pub grphx: graphics_resources::GraphicsResources,
    pub buffers: GraphicBuffers,

    ct: graphics_util::ColourTransform,

    pub flipmode: bool,
	pub setflipmode: bool,
	pub notextoutline: bool,
	//buffer objects. //TODO refactor buffer objects
	// SDL_Surface* backBuffer;
	// Screen* screenbuffer;
	// SDL_Surface* menubuffer;
	// SDL_Surface* foregroundBuffer;
	// SDL_Surface* tempBuffer;
	// SDL_Surface* warpbuffer;
	// SDL_Surface* warpbuffer_lerp;
	// SDL_Surface* footerbuffer;

	bfont_rect: sdl2::rect::Rect,
	tiles_rect: sdl2::rect::Rect,
	sprites_rect: sdl2::rect::Rect,
	bg_rect: sdl2::rect::Rect,
	line_rect: sdl2::rect::Rect,
	tele_rect: sdl2::rect::Rect,
	towerbuffer_rect: sdl2::rect::Rect,
	prect: sdl2::rect::Rect,
	footerrect: sdl2::rect::Rect,
	warprect: sdl2::rect::Rect,
	// images_rect: sdl2::rect::Rect,
	// foot_rect: sdl2::rect::Rect,

	linestate: i32,
    linedelay: i32,
	backoffset: i32,
	pub backgrounddrawn: bool,
    foregrounddrawn: bool,

    trinketcolset: bool,
    trinketr: i32,
    trinketg: i32,
    trinketb: i32,

	menuoffset: i32,
	oldmenuoffset: i32,
	resumegamemode: bool,

	pub crewframe: i32,
	pub crewframedelay: i32,

	pub fademode: i32,
	fadeamount: i32,
	oldfadeamount: i32,
	fadebars: Vec<i32>,
	ingame_fademode: i32,

    pub translucentroomname: bool,

    pub alpha: f32,

    pub screenshake_x: i32,
    pub screenshake_y: i32,

	pub textbox: Vec<textbox::TextBoxClass>,

	pub showcutscenebars: bool,
	cutscenebarspos: i32,
	oldcutscenebarspos: i32,

	stars: [sdl2::rect::Rect; numstars],
	starsspeed: [i32; numstars],

	spcol: i32,
    spcoldel: i32,
	backboxes: [sdl2::rect::Rect; numbackboxes],
	backboxvx: [i32; numbackboxes],
	backboxvy: [i32; numbackboxes],
	backboxint: [f32; numbackboxes],

	warpskip: i32,
    warpfcol: i32,
    warpbcol: i32,

	font_positions: HashMap<i32, i32>,

    col_crewred: graphics_util::ColourTransform,
    col_crewyellow: graphics_util::ColourTransform,
    col_crewgreen: graphics_util::ColourTransform,
    col_crewcyan: graphics_util::ColourTransform,
    col_crewblue: graphics_util::ColourTransform,
    col_crewpurple: graphics_util::ColourTransform,
    col_crewinactive: graphics_util::ColourTransform,
    pub col_clock: graphics_util::ColourTransform,
    pub col_trinket: graphics_util::ColourTransform,
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
            // SDL_Surface* footerbuffer;

            bfont_rect: graphics_util::setRect(0,0,8,8),
            tiles_rect: graphics_util::setRect(0,0,8,8),
            sprites_rect: graphics_util::setRect(0,0,32,32),
            bg_rect: graphics_util::setRect(0,0,320,240),
            line_rect: graphics_util::setRect(0,0,0,0),
            tele_rect: graphics_util::setRect(0,0,96,96),
            towerbuffer_rect: graphics_util::setRect(8, 8, 320, 240),
            prect: graphics_util::setRect(0, 0, 4, 4),
            footerrect: graphics_util::setRect(0, 230, 320, 10),
            warprect: sdl2::rect::Rect::new(0, 0, 0, 0),
            // images_rect: sdl2::rect::Rect,
            // foot_rect: sdl2::rect::Rect,

            linestate: 0,
            linedelay: 0,
            backoffset: 0,
            backgrounddrawn: false,
            foregrounddrawn: false,

            trinketcolset: false,
            trinketr: 0,
            trinketg: 0,
            trinketb: 0,

            menuoffset: 0,
            oldmenuoffset: 0,
            resumegamemode: false,

            crewframe: 0,
            crewframedelay: 4,

            fademode: 0,
            fadeamount: 0, // TODO @sx set via mutator
            oldfadeamount: 0, // TODO @sx set via mutator
            fadebars: vec![],
            ingame_fademode: 0,

            translucentroomname: false,

            alpha: 1.0f32,

            screenshake_x: 0,
            screenshake_y: 0,

            textbox: vec![],

            showcutscenebars: false,
            cutscenebarspos: 0,
            oldcutscenebarspos: 0,

            stars: [sdl2::rect::Rect::new(0, 0, 0, 0); numstars],
            starsspeed: [0; numstars],

            spcol: 0,
            spcoldel: 0,
            backboxes: [sdl2::rect::Rect::new(0, 0, 0, 0); numbackboxes],
            backboxvx: [0; numbackboxes],
            backboxvy: [0; numbackboxes],
            backboxint: [0f32; numbackboxes],

            warpskip: 0,
            warpfcol: 0,
            warpbcol: 0,

            font_positions: HashMap::new(),

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
    pub fn drawspritesetcol(&self, x: i32, y: i32, t: i32, c: i32) {
        println!("DEADBEEF: drawspritesetcol not implemented yet");
    }

    // void Graphics::updatetitlecolours(void)
    pub fn updatetitlecolours (&mut self, glow: i32) {
        self.setcol(15, glow);
        self.col_crewred.set_colour(self.ct.colour);
        self.setcol(14, glow);
        self.col_crewyellow.set_colour(self.ct.colour);
        self.setcol(13, glow);
        self.col_crewgreen.set_colour(self.ct.colour);
        self.setcol(0, glow);
        self.col_crewcyan.set_colour(self.ct.colour);
        self.setcol(16, glow);
        self.col_crewblue.set_colour(self.ct.colour);
        self.setcol(20, glow);
        self.col_crewpurple.set_colour(self.ct.colour);
        self.setcol(19, glow);
        self.col_crewinactive.set_colour(self.ct.colour);

        self.setcol(18, glow);
        self.col_clock.set_colour(self.ct.colour);
        self.setcol(18, glow);
        self.col_trinket.set_colour(self.ct.colour);
    }

    // void Graphics::Makebfont(void)
    fn make_bfont(&mut self) {
        println!("DEADBEEF: Graphics::Makebfont not implemented yet");

        // PROCESS_TILESHEET(self.grphx.bfont, 8, {
        //     SDL_Surface* TempFlipped = FlipSurfaceVerticle(temp);
        //     flipbfont.push_back(TempFlipped);
        // });

        // @sx: actually this code is unused
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
    pub fn print(&mut self, x: i32, y: i32, s: &str, r: i32, g: i32, b: i32, cen: Option<bool>) {
        self.print_alpha(x, y, s, r, g, b, 255, cen);
    }

    // void Graphics::PrintAlpha( int _x, int _y, std::string _s, int r, int g, int b, int a, bool cen /*= false*/ )
    pub fn print_alpha(&mut self, x: i32, y: i32, s: &str, r: i32, g: i32, b: i32, a: i32, cen: Option<bool> ) {
        let cen = match cen {
            Some(v) => v,
            None => false,
        };
        let mut xx = x;
        // std::vector<SDL_Surface*>& font = flipmode ? flipbfont : bfont;

        let r = maths::clamp(r, 0, 255) as u8;
        let g = maths::clamp(g, 0, 255) as u8;
        let b = maths::clamp(b, 0, 255) as u8;
        let a = maths::clamp(a, 0, 255) as u8;

        self.ct.colour = self.get_rgba(r, g, b, a);

        if cen {
            xx = 160 - (Graphics::len(&s) / 2);
        }

        let mut bfontpos = 0;

        for (_, curr) in s.chars().enumerate() {
            let ix = xx + bfontpos;
            let iy = y;

            self.grphx.bfont.rect.x = ix;
            self.grphx.bfont.rect.y = iy;

            let idx = Graphics::font_idx(curr as i32);
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

    // void Graphics::bigprint(  int _x, int _y, std::string _s, int r, int g, int b, bool cen = false, int sc = 2)
    pub fn bigprint(&mut self, _x: i32, _y: i32, _s: &str, r: i32, g: i32, b: i32, cen: Option<bool>, sc: Option<i32>) {
        let cen = cen.unwrap_or(false);
        let sc = sc.unwrap_or(2);

        // std::vector<SDL_Surface*>& font = flipmode ? flipbfont : bfont;

        let r = maths::clamp(r, 0, 255);
        let g = maths::clamp(g, 0, 255);
        let b = maths::clamp(b, 0, 255);

        self.ct.colour = self.getRGB(r, g, b);

        let _x = if cen {
            maths::VVV_max(160 - (Graphics::len(_s) / 2 * sc), 0 )
        } else {
            _x
        };

        let mut bfontpos = 0;
        for curr in _s.to_string().chars() {
            let idx = Graphics::font_idx(curr as i32);
            let font_surface = &self.grphx.bfont.surfaces[idx];

            // if INBOUNDS_VEC(idx, font) {
                // SDL_Surface* tempPrint = ScaleSurface(font[idx], font[idx]->w *sc,font[idx]->h *sc);
                // SDL_Rect printrect = {_x + bfontpos, _y, bfont_rect.w*sc + 1, bfont_rect.h*sc + 1};
                // BlitSurfaceColoured(tempPrint, NULL, backBuffer, &printrect, ct);
                // SDL_FreeSurface(tempPrint);
            // }
            let tempPrint = graphics_util::ScaleSurface(font_surface, font_surface.width() * sc as u32, font_surface.height() * sc as u32);
            let printrect = graphics_util::setRect(_x + bfontpos, _y, (self.grphx.bfont.rect.w * sc + 1) as u32, (self.grphx.bfont.rect.h * sc + 1) as u32);
            graphics_util::BlitSurfaceColoured(&tempPrint, None, &mut self.buffers.backBuffer, printrect, self.ct.colour);

            bfontpos += Graphics::bfontlen(curr as i32) * sc;
        }
    }

    // void Graphics::bigbprint(int x, int y, std::string s, int r, int g, int b, bool cen, int sc)
    // int Graphics::len(std::string t)
    pub fn len(_s: &str) -> i32 {
        _s.len() as i32 * 8
    }

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
    pub fn draw_sprite_c(&mut self, x: i32, y: i32, t: i32, c: Color) {
        // self.graphics.draw_sprite_c(34, 126-20, 50, self.graphics.col_clock);
        // self.graphics.draw_sprite_c(270, 126-20, 22, self.graphics.col_trinket);
        println!("DEADBEEF: draw sprite c not implemented yet");
    }

    // bool Graphics::shouldrecoloroneway(const int tilenum, const bool mounted)
    // void Graphics::drawtile( int x, int y, int t )
    // void Graphics::drawtile2( int x, int y, int t )
    // void Graphics::drawtile3( int x, int y, int t, int off, int height_subtract /*= 0*/ )
    // void Graphics::drawtowertile( int x, int y, int t )
    // void Graphics::drawtowertile3( int x, int y, int t, TowerBG& bg_obj )
    fn drawtowertile3(x: i32, y: i32, t: u8, colstate: i32, tiles_rect_width: u32, tiles_rect_height: u32) -> (usize, sdl2::rect::Rect) {
        let t = (t as i32 + colstate * 30) as usize;
        // if !INBOUNDS_VEC(t, self.grphx.tiles3) {
        //     WHINE_ONCE("drawtowertile3() out-of-bounds!");
        //     return;
        // }
        let x = x + 8;
        let y = y + 8;

        // SDL_Rect rect = {x, y, tiles_rect.w, tiles_rect.h};
        // BlitSurfaceStandard(tiles3[t], NULL, bg_obj.buffer, &rect);
        let rect = sdl2::rect::Rect::new(x, y, tiles_rect_width, tiles_rect_height);
        // self.grphx.tiles3.surfaces[t as usize].blit(None, buffer, rect);

        (t, rect)
    }

    // void Graphics::drawgui(void)
    // void Graphics::updatetextboxes(void)
    // void Graphics::drawimagecol( int t, int xp, int yp, int r = 0, int g = 0, int b = 0, bool cent/*= false*/ )
    pub fn drawimagecol(&self, t: i32, xp: i32, yp: i32, r: i32, g: i32, b: i32, cent: bool) {
        println!("DEADBEEF: drawimagecol not implemented yet");
    }

    // void Graphics::drawimage( int t, int xp, int yp, bool cent/*=false*/ )
    // void Graphics::drawpartimage( int t, int xp, int yp, int wp, int hp)
    // void Graphics::cutscenebars(void)
    // void Graphics::cutscenebarstimer(void)
    // void Graphics::setbars(const int position)
    pub fn setbars(&mut self, position: i32) {
        self.cutscenebarspos = position;
        self.oldcutscenebarspos = position;
    }

    // void Graphics::drawcrewman( int x, int y, int t, bool act, bool noshift /*=false*/ )
    pub fn drawcrewman(&self, i: i32, y: i32, t: i32, act: bool, noshift: Option<bool>) {
        println!("DEADBEEF: drawcrewman is not implemented yet");
    }

    // void Graphics::drawpixeltextbox( int x, int y, int w, int h, int w2, int h2, int r, int g, int b, int xo, int yo )
    pub fn drawpixeltextbox(&self, x: i32, y: i32, w: i32, h: i32, w2: i32, h2: i32, r: i32, g: i32, b: i32, xo: i32, yo: i32) {
        println!("DEADBEEF: drawpixeltextbox not implemented yet");
    }

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
            2 => (), // 2 - fade out
            0 => (), // normal - no fade
            _ => {
                panic!("unknown drawfade value {}", self.fademode);
            }
        }
    }

    // void Graphics::processfade(void)
    pub fn processfade (&mut self) {
        self.oldfadeamount = self.fadeamount;
        if self.fademode > 1 {
            if self.fademode == 2 {
                // prepare fade out
                self.fadebars = vec![-(maths::fRandom() * 12.0f32) as i32 * 8; self.fadebars.len()];
                self.setfade(0);
                self.fademode = 3;
            } else if self.fademode == 3 {
                self.fadeamount += 24;
                if self.fadeamount > 416 {
                    self.fademode = 1; // faded
                }
            } else if self.fademode == 4 {
                // prepare fade in
                self.fadebars = vec![320 + (maths::fRandom() * 12.0f32) as i32 * 8; self.fadebars.len()];

                self.setfade(416);
                self.fademode = 5;
            } else if self.fademode == 5 {
                self.fadeamount -= 24;
                if self.fadeamount <= 0 {
                    self.fademode = 0; // normal
                }
            }
        }
    }

    // void Graphics::setfade(const int amount)
    fn setfade (&mut self, amount: i32) {
        self.fadeamount = amount;
        self.oldfadeamount = amount;
    }

    // void Graphics::drawmenu( int cr, int cg, int cb, bool levelmenu /*= false*/ )
    pub fn drawmenu (&mut self, cr: i32, cg: i32, cb: i32, _levelmenu: Option<bool>, game: &mut game::Game) {
        let levelmenu = match _levelmenu {
            Some(v) => v,
            None => false,
        };

        // for (size_t i = 0; i < game.menuoptions.size(); i++) {
        for (_i, opt) in game.menuoptions.iter().enumerate() {
            let i = _i as i32;
            let fr;
            let fg;
            let fb;

            if opt.active { // Color it normally
                fr = cr;
                fg = cg;
                fb = cb;
            } else { // Color it gray
                fr = 128;
                fg = 128;
                fb = 128;
            }

            let mut x = i*game.menuspacing + game.menuxoff;
            let mut y = 140 + i*12 + game.menuyoff;

            if levelmenu {
                if game.menuoptions.len() - _i <= 3 {
                    // We're on "next page", "previous page", or "return to menu". Draw them separated by a bit
                    y += 8;
                } else {
                    // Get out of the way of the level descriptions
                    y += 4;
                }
            }

            let mut tempstring = opt.text.clone();
            let buffer: String;

            if i == game.currentmenuoption && game.slidermode == SLIDERMODE::SLIDER_NONE {
                if opt.active {
                    // Uppercase the text
                    // FIXME: This isn't UTF-8 aware!
                    tempstring = tempstring.to_uppercase();
                }

                // Add brackets
                buffer = format!("[ {} ]", tempstring);

                // Account for brackets
                x -= 16;
            } else {
                buffer = tempstring.clone();
            }

            self.print(x, y, &buffer.to_owned(), fr, fg, fb, None);
        }
    }

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
    pub fn drawtowerbackground (&mut self, bg: BackGround) {
        // TODO: @sx refactor
        let (bg_buffer, bg_buffer_lerp, bscroll) = match bg {
            BackGround::Title => (
                &self.buffers.titlebg_buffer,
                &mut self.buffers.titlebg_buffer_lerp,
                self.buffers.titlebg.bscroll as f32,
            ),
            BackGround::Tower => (
                &self.buffers.towerbg_buffer,
                &mut self.buffers.towerbg_buffer_lerp,
                self.buffers.towerbg.bscroll as f32,
            ),
        };

        graphics_util::ClearSurface(&mut self.buffers.backBuffer);
        bg_buffer.blit(None, bg_buffer_lerp, None)
            .expect("error while blitting background buffer");
        graphics_util::ScrollSurface(bg_buffer_lerp, 0, Graphics::lerp_g(self.alpha, 0f32, -bscroll) as i32);
        bg_buffer_lerp.blit(self.towerbuffer_rect, &mut self.buffers.backBuffer, None)
            .expect("error while blitting scrolled background buffer");
    }

    // void Graphics::updatetowerbackground(TowerBG& bg_obj)
    // TODO: @sx refactor this mess
    pub fn updatetowerbackground(&mut self, bg: BackGround, map: &mut map::Map) {
        let mut back_char;
        let tiles_rect_width = self.tiles_rect.width();
        let tiles_rect_height = self.tiles_rect.height();

        // TODO: @sx use generics?
        match bg {
            BackGround::Title => {
                let bg_obj = &mut self.buffers.titlebg;
                if bg_obj.bypos < 0 {
                    bg_obj.bypos += 120 * 8;
                }
            },
            BackGround::Tower => {
                let bg_obj = &mut self.buffers.towerbg;
                if bg_obj.bypos < 0 {
                    bg_obj.bypos += 120 * 8;
                }
            },
        };

        let (tdrawback, bypos, bscroll, scrolldir, colstate) = match bg {
            BackGround::Title => (
                self.buffers.titlebg.tdrawback, self.buffers.titlebg.bypos, self.buffers.titlebg.bscroll, self.buffers.titlebg.scrolldir, self.buffers.titlebg.colstate
            ),
            BackGround::Tower => (
                self.buffers.towerbg.tdrawback, self.buffers.towerbg.bypos, self.buffers.towerbg.bscroll, self.buffers.towerbg.scrolldir, self.buffers.towerbg.colstate
            ),
        };

        let mut draw_acc = vec![];

        if tdrawback {
            let off = if scrolldir == 0 {
                0
            } else {
                bscroll
            };

            //Draw the whole thing; needed for every colour cycle!
            for j in -1..32 {
                for i in 0..40 {
                    back_char = map.tower.backat(i, j, bypos);
                    draw_acc.push(Graphics::drawtowertile3(i * 8, (j * 8) - (bypos % 8) - off, back_char, colstate, tiles_rect_width, tiles_rect_height));
                }
            }

            match bg {
                BackGround::Title => {
                    self.buffers.titlebg.tdrawback = false;
                },
                BackGround::Tower => {
                    self.buffers.towerbg.tdrawback = false;
                },
            };
        } else {
            // //just update the bottom
            match bg {
                BackGround::Title => {
                    graphics_util::ScrollSurface(&mut self.buffers.titlebg_buffer, 0, -bscroll);
                },
                BackGround::Tower => {
                    graphics_util::ScrollSurface(&mut self.buffers.towerbg_buffer, 0, -bscroll);
                },
            }

            if scrolldir == 0 {
                for i in 0..40 {
                    back_char = map.tower.backat(i, -1, bypos);
                    draw_acc.push(Graphics::drawtowertile3(i * 8, 0 - (bypos % 8), back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, 0, bypos);
                    draw_acc.push(Graphics::drawtowertile3(i * 8, -1*8 - (bypos % 8), back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, -1, bypos);
                }
            } else {
                for i in 0..40 {
                    back_char = map.tower.backat(i, 29, bypos);
                    draw_acc.push(Graphics::drawtowertile3(i * 8, 29*8 - (bypos % 8) - bscroll, back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, 30, bypos);
                    draw_acc.push(Graphics::drawtowertile3(i * 8, 30*8 - (bypos % 8) - bscroll, back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, 31, bypos);
                    draw_acc.push(Graphics::drawtowertile3(i * 8, 31*8 - (bypos % 8) - bscroll, back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, 32, bypos);
                    draw_acc.push(Graphics::drawtowertile3(i * 8, 32*8 - (bypos % 8) - bscroll, back_char, colstate, tiles_rect_width, tiles_rect_height));
                }
            }
        }

        let buffer = match bg {
            BackGround::Title => &mut self.buffers.titlebg_buffer,
            BackGround::Tower => &mut self.buffers.towerbg_buffer,
        };
        for (t, rect) in draw_acc.iter().rev() {
            self.grphx.tiles3.surfaces[*t].blit(None, buffer, *rect)
                .expect("error while rendering towertile3");
        }
    }

    // void Graphics::setcol( int t )
    fn setcol(&mut self, t: i32, glow: i32) {
        let temp;
        // let glow = se

        //Setup predefinied colours as per our zany palette
        match t {
            0 => {
                //Player Normal
                self.ct.colour = self.getRGB(160 - glow/2 - (maths::fRandom() * 20f32) as i32, 200 - glow/2, 220 - glow);
            },
            1 => {
                //Player Hurt
                self.ct.colour = self.getRGB(196 - (maths::fRandom() * 64f32) as i32, 10, 10);
            },
            2 => {
                //Enemies and stuff
                self.ct.colour = self.getRGB(225 - (glow/2), 75, 30);
            },
            3 => {
                //Trinket
                if !self.trinketcolset {
                    self.trinketr = 200 - (maths::fRandom() * 64f32) as i32;
                    self.trinketg = 200 - (maths::fRandom() * 128f32) as i32;
                    self.trinketb = 164 + (maths::fRandom() * 60f32) as i32;
                    self.trinketcolset = true;
                }
                self.ct.colour = self.getRGB(self.trinketr, self.trinketg, self.trinketb);
            },
            4 => {
                //Inactive savepoint
                temp = (glow/2) + (maths::fRandom() * 8f32) as i32;
                self.ct.colour = self.getRGB(80 + temp, 80 + temp, 80 + temp);
            },
            5 => {
                //Active savepoint
                self.ct.colour = self.getRGB(164 + (maths::fRandom()*64f32) as i32,164 + (maths::fRandom()*64f32) as i32, 255 - (maths::fRandom()*64f32) as i32);
            },
            6 => {
                //Enemy : Red
                self.ct.colour = self.getRGB(250 - glow/2, 60- glow/2, 60 - glow/2);
            },
            7 => {
                //Enemy : Green
                self.ct.colour = self.getRGB(100 - glow/2 - (maths::fRandom()*30f32) as i32, 250 - glow/2, 100 - glow/2 - (maths::fRandom()*30f32) as i32);
            },
            8 => {
                //Enemy : Purple
                self.ct.colour = self.getRGB(250 - glow/2, 20, 128 - glow/2 + (maths::fRandom()*30f32) as i32);
            },
            9 => {
                //Enemy : Yellow
                self.ct.colour = self.getRGB(250 - glow/2, 250 - glow/2, 20);
            },
            10 => {
                //Warp point (white)
                self.ct.colour = self.getRGB(255 - (maths::fRandom()*64f32) as i32, 255 - (maths::fRandom()*64f32) as i32, 255 - (maths::fRandom()*64f32) as i32);
            },
            11 => {
                //Enemy : Cyan
                self.ct.colour = self.getRGB(20, 250 - glow/2, 250 - glow/2);
            },
            12 => {
                //Enemy : Blue
                self.ct.colour = self.getRGB(90 - glow/2, 90 - glow/2, 250 - glow/2);
            },

            //Crew Members

            13 => {
                //green
                self.ct.colour = self.getRGB(120 - glow/4 - (maths::fRandom()*20f32) as i32, 220 - glow/4, 120 - glow/4);
            },
            14 => {
                //Yellow
                self.ct.colour = self.getRGB(220 - glow/4 - (maths::fRandom()*20f32) as i32, 210 - glow/4, 120 - glow/4);
            },
            15 => {
                //pink
                self.ct.colour = self.getRGB(255 - glow/8, 70 - glow/4, 70 - glow / 4);
            },
            16 => {
                //Blue
                self.ct.colour = self.getRGB(75, 75, 255 - glow/4 - (maths::fRandom()*20f32) as i32);
            },

            17 => {
                //Enemy : Orange
                self.ct.colour = self.getRGB(250 - glow/2, 130 - glow/2, 20);
            },
            18 => {
                //Enemy : Gray
                self.ct.colour = self.getRGB(130- glow/2, 130 - glow/2, 130 - glow/2);
            },
            19 => {
                //Enemy : Dark gray
                self.ct.colour = self.getRGB(60 - glow/8, 60 - glow/8, 60 - glow/8);
            },
            20 => {
                //Purple
                self.ct.colour = self.getRGB(220 - glow / 4 - (maths::fRandom()*20f32) as i32, 120 - glow / 4, 210 - glow / 4);
            },

            21 => {
                //Enemy : Light Gray
                self.ct.colour = self.getRGB(180 - glow/2, 180 - glow/2, 180 - glow/2);
            },
            22 => {
                //Enemy : Indicator Gray
                self.ct.colour = self.getRGB(230 - glow/2, 230 - glow/2, 230 - glow/2);
            },
            23 => {
                //Enemy : Indicator Gray
                self.ct.colour = self.getRGB(255 - glow/2 - (maths::fRandom()*40f32) as i32 , 255 - glow/2 - (maths::fRandom()*40f32) as i32, 255 - glow/2 - (maths::fRandom()*40f32) as i32);
            },

            //Trophies
            30 => {
                //Yellow
                self.ct.colour = self.RGBf(160, 200, 220);
            },
            31 => {
                //Purple
                self.ct.colour = self.RGBf(220, 120, 210);
            },
            32 => {
                //cyan
                self.ct.colour = self.RGBf(220, 210, 120);
            },
            33 => {
                //Blue
                self.ct.colour = self.RGBf(255, 70, 70);
            },
            34 => {
                //green
                self.ct.colour = self.RGBf(120, 220, 120);
            },
            35 => {
                //red
                self.ct.colour = self.RGBf(75, 75, 255);
            },
            36 => {
                //Gold
                self.ct.colour = self.getRGB(180, 120, 20);
            },
            37 => {
                //Trinket
                if !self.trinketcolset {
                    self.trinketr = 200 - (maths::fRandom()*64f32) as i32;
                    self.trinketg = 200 - (maths::fRandom()*128f32) as i32;
                    self.trinketb = 164 + (maths::fRandom()*60f32) as i32;
                    self.trinketcolset = true;
                }
                self.ct.colour = self.RGBf(self.trinketr, self.trinketg, self.trinketb);
            },
            38 => {
                //Silver
                self.ct.colour = self.RGBf(196, 196, 196);
            },
            39 => {
                //Bronze
                self.ct.colour = self.RGBf(128, 64, 10);
            },

            //Awesome

            40 => {
                //Teleporter in action!
                temp = (maths::fRandom() * 150f32) as i32;
                if temp < 33 {
                    self.ct.colour = self.RGBf(255 - (maths::fRandom()*64f32) as i32, 64 + (maths::fRandom()*64f32) as i32, 64 + (maths::fRandom()*64f32) as i32);
                } else if temp < 66 {
                    self.ct.colour = self.RGBf(64 + (maths::fRandom()*64f32) as i32, 255 - (maths::fRandom()*64f32) as i32, 64 + (maths::fRandom()*64f32) as i32);
                } else if temp < 100 {
                    self.ct.colour = self.RGBf(64 + (maths::fRandom()*64f32) as i32, 64 + (maths::fRandom()*64f32) as i32, 255 - (maths::fRandom()*64f32) as i32);
                } else {
                    self.ct.colour = self.RGBf(164+(maths::fRandom()*64f32) as i32,164+(maths::fRandom()*64f32) as i32, 255 -(maths::fRandom()*64f32) as i32);
                }
            },

            100 => {
                //Inactive Teleporter
                temp = (glow/2) + (maths::fRandom()*8f32) as i32;
                self.ct.colour = self.getRGB(42 + temp, 42 + temp, 42 + temp);
            },
            101 => {
                //Active Teleporter
                self.ct.colour = self.getRGB(164+(maths::fRandom()*64f32) as i32,164+(maths::fRandom()*64f32) as i32, 255 -(maths::fRandom()*64f32) as i32);
            },
            102 => {
                //Teleporter in action!
                temp = (maths::fRandom() * 150f32) as i32;
                if temp < 33 {
                    self.ct.colour = self.getRGB(255 - (maths::fRandom()*64f32) as i32, 64 + (maths::fRandom()*64f32) as i32, 64 + (maths::fRandom()*64f32) as i32);
                } else if temp < 66 {
                    self.ct.colour = self.getRGB(64 + (maths::fRandom()*64f32) as i32, 255 - (maths::fRandom()*64f32) as i32, 64 + (maths::fRandom()*64f32) as i32);
                } else if temp < 100 {
                    self.ct.colour = self.getRGB(64 + (maths::fRandom()*64f32) as i32, 64 + (maths::fRandom()*64f32) as i32, 255 - (maths::fRandom()*64f32) as i32);
                } else {
                    self.ct.colour = self.getRGB(164+(maths::fRandom()*64f32) as i32,164+(maths::fRandom()*64f32) as i32, 255 -(maths::fRandom()*64f32) as i32);
                }
            },

            _ => self.ct.colour = self.getRGB(255, 255, 255),
        }
    }

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
        if game.screenshake > 0 {
            self.updatescreenshake();
        }

        // if screenbuffer.badSignalEffect {
        if badSignalEffect {
            self.UpdateFilter();
        }
    }

    // void Graphics::renderfixedpost(void)
    pub fn renderfixedpost(&self, game: &mut game::Game) {
    	/* Screen effects timers */
        if game.flashlight > 0 {
            game.flashlight -= 1;
        }
        if game.screenshake > 0 {
            game.screenshake -= 1;
        }
    }

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
    fn RGBf (&self, r: i32, g: i32, b: i32) -> u32 {
        let r = ( (r+128) /  3) as u8;
        let g = ( (g+128) /  3) as u8;
        let b = ( (b+128) /  3) as u8;

        unsafe {
            sdl2_sys::SDL_MapRGB(self.buffers.backBuffer.pixel_format().raw(), r, g, b)
        }
    }

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
        // TODO @sx @impl
        println!("DEADBEEF: Graphics::reloadresources not fully implemented yet");

        // grphx.destroy();
        // grphx.init();

        // destroy();

        // MakeTileArray();
        // MakeSpriteArray();
        // maketelearray();
        self.make_bfont();

        // images.clear();

        // images.push_back(grphx.im_image0);
        // images.push_back(grphx.im_image1);
        // images.push_back(grphx.im_image2);
        // images.push_back(grphx.im_image3);
        // images.push_back(grphx.im_image4);
        // images.push_back(grphx.im_image5);
        // images.push_back(grphx.im_image6);

        // images.push_back(grphx.im_image7);
        // images.push_back(grphx.im_image8);
        // images.push_back(grphx.im_image9);
        // images.push_back(grphx.im_image10);
        // images.push_back(grphx.im_image11);
        // images.push_back(grphx.im_image12);

        // if screenbuffer != NULL {
        //     screenbuffer.LoadIcon();
        // }

        // music.destroy();
        // music.init();

        // // #ifndef NO_CUSTOM_LEVELS
        // tiles1_mounted = FILESYSTEM_isAssetMounted("graphics/tiles.png");
        // tiles2_mounted = FILESYSTEM_isAssetMounted("graphics/tiles2.png");
        // minimap_mounted = FILESYSTEM_isAssetMounted("graphics/minimap.png");
        // // #endif
    }

    // Uint32 Graphics::crewcolourreal(int t)

    /* graphics.cpp inline methods */

    // float inline lerp(const float v0, const float v1)
    fn lerp(&self, v0: f32, v1: f32) -> f32 {
        v0 + self.alpha * (v1 - v0)
    }
    fn lerp_g(alpha: f32, v0: f32, v1: f32) -> f32 {
        v0 + alpha * (v1 - v0)
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
    pub towerbg: towerbg::TowerBG,
    pub towerbg_buffer: sdl2::surface::Surface<'static>,
    pub towerbg_buffer_lerp: sdl2::surface::Surface<'static>,
    pub titlebg: towerbg::TowerBG,
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
            towerbg: towerbg::TowerBG::new(0),
            towerbg_buffer,
            towerbg_buffer_lerp,
            titlebg: towerbg::TowerBG::new(10),
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
