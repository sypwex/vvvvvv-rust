use std::{collections::HashMap, ptr};

extern crate sdl2;
extern crate sdl2_sys;
use sdl2::render::BlendMode;

use crate::{INBOUNDS_VEC, WHINE_ONCE, entity, filesystem, game::{self, SLIDERMODE}, map, maths, music, utility_class};
use super::BackGround;
use self::{graphics_util::ColourTransform, textbox::TextBoxClass};

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

    pub ct: graphics_util::ColourTransform,

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
    pub footerrect: sdl2::rect::Rect,
    warprect: sdl2::rect::Rect,
    // images_rect: sdl2::rect::Rect,
    // foot_rect: sdl2::rect::Rect,

    pub linestate: i32,
    pub linedelay: i32,
    backoffset: i32,
    pub backgrounddrawn: bool,
    pub foregrounddrawn: bool,

    pub trinketcolset: bool,
    trinketr: i32,
    trinketg: i32,
    trinketb: i32,

    pub menuoffset: i32,
    pub oldmenuoffset: i32,
    pub resumegamemode: bool,

    pub crewframe: i32,
    pub crewframedelay: i32,

    pub fademode: i32,
    fadeamount: i32,
    oldfadeamount: i32,
    fadebars: [i32; 15],
    pub ingame_fademode: i32,

    pub translucentroomname: bool,

    pub alpha: f32,

    pub screenshake_x: i32,
    pub screenshake_y: i32,

    pub textbox: Vec<textbox::TextBoxClass>,

    pub showcutscenebars: bool,
    pub cutscenebarspos: i32,
    oldcutscenebarspos: i32,

    stars: [sdl2::rect::Rect; numstars],
    starsspeed: [i32; numstars],

    spcol: i32,
    spcoldel: i32,
    pub rcol: i32,
    m: usize,
    backboxes: [sdl2::rect::Rect; numbackboxes],
    backboxvx: [i32; numbackboxes],
    backboxvy: [i32; numbackboxes],
    backboxint: [f32; numbackboxes],

    warpskip: i32,
    warpfcol: sdl2::pixels::Color,
    warpbcol: sdl2::pixels::Color,

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

    pub kludgeswnlinewidth: bool,

    // #ifndef NO_CUSTOM_LEVELS
    tiles1_mounted: bool,
    tiles2_mounted: bool,
    pub minimap_mounted: bool,
    // #endif

    /* @sx: moved here from graphic_util since required storage for variables */
    oldscrollamount: i32,
    scrollamount: i32,
    isscrolling: bool,
}

impl Graphics {
    // void Graphics::init(void)
    pub fn new(pf: sdl2::pixels::PixelFormatEnum) -> Graphics {
        //Background inits
        let mut stars = [sdl2::rect::Rect::new(0, 0, 0, 0); numstars];
        let mut starsspeed = [0; numstars];
        for i in 0..numstars {
            stars[i].x = (maths::fRandom() * 320.0) as i32;
            stars[i].y = (maths::fRandom() * 240.0) as i32;
            stars[i].w = 2;
            stars[i].h = 2;
            starsspeed[i] = 4 + (maths::fRandom() * 4.0) as i32;
        }

        let mut backboxes = [sdl2::rect::Rect::new(0, 0, 0, 0); numbackboxes];
        let mut backboxvx = [0; numbackboxes];
        let mut backboxvy = [0; numbackboxes];
        let mut backboxint = [0.0; numbackboxes];
        for i in 0..numbackboxes {
            let mut bvx = 0;
            let mut bvy = 0;
            backboxes[i] = if maths::fRandom() * 100.0 > 50.0 {
                bvx = 9 - (maths::fRandom() * 19.0) as i32;
                if bvx > -6 && bvx < 6 { bvx = 6; }
                bvx = bvx * 3 / 2;
                sdl2::rect::Rect::new((maths::fRandom() * 320.0) as i32, (maths::fRandom() * 240.0) as i32, 32, 12)
            } else {
                bvy = 9 - (maths::fRandom() * 19.0) as i32;
                if bvy > -6 && bvy < 6 { bvy = 6; }
                bvy = bvy * 3 / 2;
                sdl2::rect::Rect::new((maths::fRandom() * 320.0) as i32, (maths::fRandom() * 240.0) as i32, 12, 32)
            };
            backboxvx[i] = bvx;
            backboxvy[i] = bvy;
            backboxint[i] = 0.5 + (maths::fRandom() * 100.0 / 200.0);
        }

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
            fadebars: [0; 15],
            ingame_fademode: 0,

            translucentroomname: false,

            alpha: 1.0f32,

            screenshake_x: 0,
            screenshake_y: 0,

            textbox: vec![],

            showcutscenebars: false,
            cutscenebarspos: 0,
            oldcutscenebarspos: 0,

            stars,
            starsspeed,

            spcol: 0,
            spcoldel: 0,
            rcol: 0,
            m: 0,
            backboxes,
            backboxvx,
            backboxvy,
            backboxint,

            warpskip: 0,
            warpfcol: sdl2::pixels::Color::BLACK,
            warpbcol: sdl2::pixels::Color::BLACK,

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
        // if font_positions.len() > 0 {
        //     std::map<int, int>::iterator iter = font_positions.find(ch);
        //     if iter == font_positions.end()) {
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
        warn!("DEADBEEF: Graphics::Makebfont not implemented yet");

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
    fn bfontlen(ch: char) -> i32 {
        if (ch as u32) < 32 {
            6
        } else {
            8
        }
    }

    // void Graphics::MakeTileArray(void)
    pub fn MakeTileArray(&mut self) {
        println!("DEADBEEF: Graphics::MakeTileArray() method not implemented yet");
    }

    // void Graphics::maketelearray(void)
    pub fn maketelearray(&mut self) {
        println!("DEADBEEF: Graphics::maketelearray() method not implemented yet");
    }

    // void Graphics::MakeSpriteArray(void)
    pub fn MakeSpriteArray(&mut self) {
        println!("DEADBEEF: Graphics::MakeSpriteArray() method not implemented yet");
    }

    // void Graphics::map_tab(int opt, const std::string& text, bool selected /*= false*/)
    pub fn map_tab(&mut self, opt: i32, text: &str, selected: Option<bool>, help: &mut utility_class::UtilityClass) {
        let selected = selected.unwrap_or(false);

        let x = opt * 80 + 40 - text.len() as i32 / 2;
        if selected {
            self.print(x-8, 220, &["[", text, "]"].concat(), 196, 196, 255 - help.glow, None);
        } else {
            self.print(x, 220, text, 64, 64, 64, None);
        }
    }

    // void Graphics::map_option(int opt, int num_opts, const std::string& text, bool selected /*= false*/)
    pub fn map_option(&mut self, opt: i32, num_opts: i32, text: &str, selected: Option<bool>, help: &mut utility_class::UtilityClass) {
        let selected = selected.unwrap_or(false);
        let x = 80 + opt * 32;
        let mut y = 136; // start from middle of menu
        let mut yoff = -(num_opts * 12) / 2; // could be simplified to -num_opts * 6, this conveys my intent better though
        yoff += opt * 12;

        if self.flipmode {
            y -= yoff; // going down, which in Flip Mode means going up
            y -= 40;
        } else {
            y += yoff; // going up
        }

        if selected {
            let text_upper = text.to_ascii_uppercase();
            self.print(x - 16, y, &["[ ", &text_upper, " ]"].concat(), 196, 196, 255 - help.glow, None);
        } else {
            self.print(x, y, text, 96, 96, 96, None);
        }
    }

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

            bfontpos += Graphics::bfontlen(curr);
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

            // if INBOUNDS_VEC!(idx, font) {
                // SDL_Surface* tempPrint = ScaleSurface(font[idx], font[idx]->w *sc,font[idx]->h *sc);
                // SDL_Rect printrect = {_x + bfontpos, _y, bfont_rect.w*sc + 1, bfont_rect.h*sc + 1};
                // BlitSurfaceColoured(tempPrint, NULL, self.buffers.backBuffer, &printrect, self.ct.colour);
                // SDL_FreeSurface(tempPrint);
            // }
            let tempPrint = graphics_util::ScaleSurface(font_surface, font_surface.width() * sc as u32, font_surface.height() * sc as u32);
            let printrect = graphics_util::setRect(_x + bfontpos, _y, (self.grphx.bfont.rect.w * sc + 1) as u32, (self.grphx.bfont.rect.h * sc + 1) as u32);
            graphics_util::BlitSurfaceColoured(&tempPrint, None, &mut self.buffers.backBuffer, printrect, self.ct.colour);

            bfontpos += Graphics::bfontlen(curr) * sc;
        }
    }

    // void Graphics::bigbprint(int x, int y, std::string s, int r, int g, int b, bool cen, int sc)
    pub fn bigbprint(&mut self, x: i32, y: i32, s: &str, r: i32, g: i32, b: i32, cen: bool, sc: i32) {
        if !self.notextoutline {
            self.bigprint(x, y - sc, s, 0, 0, 0, Some(cen), Some(sc));
            if cen {
                let x_cen = maths::VVV_max(160 - (Graphics::len(s) / 2) * sc, 0);
                self.bigprint(x_cen - sc, y, s, 0, 0, 0, Some(false), Some(sc));
                self.bigprint(x_cen + sc, y, s, 0, 0, 0, Some(false), Some(sc));
            } else {
                self.bigprint(x - sc, y, s, 0, 0, 0, Some(cen), Some(sc));
                self.bigprint(x + sc, y, s, 0, 0, 0, Some(cen), Some(sc));
            }
            self.bigprint(x, y + sc, s, 0, 0, 0, Some(cen), Some(sc));
        }

        self.bigprint(x, y, s, r, g, b, Some(cen), Some(sc));
    }

    // int Graphics::len(std::string t)
    pub fn len(_s: &str) -> i32 {
        _s.len() as i32 * 8
    }

    // void Graphics::PrintOffAlpha( int _x, int _y, std::string _s, int r, int g, int b, int a, bool cen /*= false*/ )
    fn PrintOffAlpha(&mut self, _x: i32, _y: i32, _s: &str, r: i32, g: i32, b: i32, a: i32, cen: Option<bool>) {
        let cen = cen.unwrap_or(false);
        // std::vector<SDL_Surface*>& font = flipmode ? flipbfont : bfont;

        let r = maths::clamp(r, 0, 255);
        let g = maths::clamp(g, 0, 255);
        let b = maths::clamp(b, 0, 255);
        let a = maths::clamp(a, 0, 255);

        self.ct.colour = self.getRGB(r, g, b);

        let _x = if cen {
            (160 - (_s.len() / 2) as i32) + _x
        } else { _x };

        let mut bfontpos = 0;
        for curr in _s.chars() {
            self.bfont_rect.x = _x + bfontpos;
            self.bfont_rect.y = _y;

            let idx = Graphics::font_idx(curr as i32);
            if INBOUNDS_VEC!(idx, self.grphx.bfont.surfaces) {
                let font_surface = &self.grphx.bfont.surfaces[idx];
                // BlitSurfaceColoured(font_surface[idx], None, self.buffers.backBuffer, &bfont_rect, self.ct.colour);
                graphics_util::BlitSurfaceColoured(font_surface, None, &mut self.buffers.backBuffer, self.bfont_rect, self.ct.colour);
            }
            bfontpos += Graphics::bfontlen(curr);
        }
    }

    // void Graphics::bprint( int x, int y, std::string t, int r, int g, int b, bool cen /*= false*/ )
    pub fn bprint(&mut self, x: i32, y: i32, t: &str, r: i32, g: i32, b: i32, cen: Option<bool>) {
        self.bprintalpha(x, y, t, r, g, b, 255, cen);
    }

    // void Graphics::bprintalpha( int x, int y, std::string t, int r, int g, int b, int a, bool cen /*= false*/ )
    fn bprintalpha(&mut self, x: i32, y: i32, t: &str, r: i32, g: i32, b: i32, a: i32, cen: Option<bool>) {
        let cen = cen.unwrap_or(false);

        if !self.notextoutline {
            self.print_alpha(x, y - 1, t, 0, 0, 0, a, Some(cen));
            match cen {
                true => {
                    self.PrintOffAlpha(-1, y, t, 0, 0, 0, a, Some(cen));
                    self.PrintOffAlpha(1, y, t, 0, 0, 0, a, Some(cen));
                },
                false => {
                    self.print_alpha(x  -1, y, t, 0, 0, 0, a, Some(cen));
                    self.print_alpha(x  +1, y, t, 0, 0, 0, a, Some(cen));
                },
            };
            self.print_alpha(x, y+1, t, 0, 0, 0, a, Some(cen));
        }

        self.print_alpha(x, y, t, r, g, b, a, Some(cen));
    }

    // void Graphics::printcrewname( int x, int y, int t )
    pub fn printcrewname(&mut self, x: i32, y: i32, t: i32) {
        //print the name of crew member t in the right colour
        match t {
            0 => self.print(x, y, "viridian", 16, 240, 240, Some(false)),
            1 => self.print(x, y, "violet", 240, 16, 240, Some(false)),
            2 => self.print(x, y, "vitellary", 240, 240, 16, Some(false)),
            3 => self.print(x, y, "vermilion", 240, 16, 16, Some(false)),
            4 => self.print(x, y, "verdigris", 16, 240, 16, Some(false)),
            5 => self.print(x, y, "victoria", 16, 16, 240, Some(false)),
            _ => (),
        }
    }

    // void Graphics::printcrewnamedark( int x, int y, int t )
    pub fn printcrewnamedark(&mut self, x: i32, y: i32, t: i32) {
        //Print the name of crew member t in the right colour
        match t {
            0 => self.print(x, y, "Viridian", 128, 128, 128, Some(false)),
            1 => self.print(x, y, "Violet", 128, 128, 128, Some(false)),
            2 => self.print(x, y, "Vitellary", 128, 128, 128, Some(false)),
            3 => self.print(x, y, "Vermilion", 128, 128, 128, Some(false)),
            4 => self.print(x, y, "Verdigris", 128, 128, 128, Some(false)),
            5 => self.print(x, y, "Victoria", 128, 128, 128, Some(false)),
            _ => (),
        }
    }

    // void Graphics::printcrewnamestatus( int x, int y, int t )
    pub fn printcrewnamestatus(&mut self, x: i32, y: i32, t: i32) {
        //print the name of crew member t in the right colour
        match t {
            0 => self.print(x, y, "(that's you!)", 16, 240, 240, Some(false)),
            1 => self.print(x, y, "Rescued!", 240, 16, 240, Some(false)),
            2 => self.print(x, y, "Rescued!", 240, 240, 16, Some(false)),
            3 => self.print(x, y, "Rescued!", 240, 16, 16, Some(false)),
            4 => self.print(x, y, "Rescued!", 16, 240, 16, Some(false)),
            5 => self.print(x, y, "Rescued!", 16, 16, 240, Some(false)),
            _ => (),
        }
    }

    // void Graphics::drawsprite( int x, int y, int t, int r, int g,  int b )
    pub fn drawsprite_rgb(&mut self, x: i32, y: i32, t: i32, r: i32, g: i32, b: i32) {
        let c = self.getRGB(r,g,b);
        self.drawsprite_clru32(x, y, t, c);
    }

    // void Graphics::drawsprite(int x, int y, int t, Uint32 c)
    pub fn drawsprite_clru32(&mut self, x: i32, y: i32, t: i32, c: u32) {
        let sprites_rect = self.grphx.sprites.rect;
        let rect_dst = sdl2::rect::Rect::new(x, y, sprites_rect.w as u32, sprites_rect.h as u32);
        self.setcolreal(c);
        let sprite = &self.grphx.sprites.surfaces[t as usize];
        graphics_util::BlitSurfaceColoured(
            sprite,
            None,
            self.buffers.backBuffer.as_mut(),
            rect_dst,
            c
        );
    }

    // bool Graphics::shouldrecoloroneway(const int tilenum, const bool mounted)

    // void Graphics::drawtile( int x, int y, int t )
    pub fn drawtile(&mut self, x: i32, y: i32, t: usize) {
        if !INBOUNDS_VEC!(t, self.grphx.tiles.surfaces) {
            WHINE_ONCE!("drawtile() out-of-bounds!");
            return;
        }

        let rect = sdl2::rect::Rect::new(x, y, self.grphx.tiles.rect.w as u32, self.grphx.tiles.rect.h as u32);
        // #if !defined(NO_CUSTOM_LEVELS)
        // if self.shouldrecoloroneway(t, tiles1_mounted) {
        //     colourTransform thect = {ed.getonewaycol()};
        //     BlitSurfaceTinted(tiles[t], NULL, backBuffer, &rect, thect);
        // } else {
        // #endif
        {
            self.grphx.tiles.surfaces[t].blit(None, &mut self.buffers.backBuffer, rect);
        }
    }

    // void Graphics::drawtile2( int x, int y, int t )
    fn drawtile2(&mut self, x: i32, y: i32, t: usize) {
        if !INBOUNDS_VEC!(t, self.grphx.tiles2.surfaces) {
            WHINE_ONCE!("drawtile2() out-of-bounds!");
            return;
        }

        let rect = sdl2::rect::Rect::new(x, y, self.grphx.tiles2.rect.w as u32, self.grphx.tiles2.rect.h as u32);
        // #if !defined(NO_CUSTOM_LEVELS)
        // if self.shouldrecoloroneway(t, tiles2_mounted) {
        //     colourTransform thect = {ed.getonewaycol()};
        //     BlitSurfaceTinted(tiles2[t], NULL, backBuffer, &rect, thect);
        // } else {
        // #endif
        {
            self.grphx.tiles2.surfaces[t].blit(None, &mut self.buffers.backBuffer, rect);
        }
    }

    // void Graphics::drawtile3( int x, int y, int t, int off, int height_subtract /*= 0*/ )
    fn drawtile3(&mut self, x: i32, y: i32, t: usize, off: i32, height_subtract: Option<i32>) {
        let tiles_rect_width = self.tiles_rect.width();
        let tiles_rect_height = self.tiles_rect.height();

        let height_subtract = height_subtract.unwrap_or(0) as u32;
        let t = t + off as usize * 30;

        if !INBOUNDS_VEC!(t, self.grphx.tiles3.surfaces) {
            WHINE_ONCE!("drawtile3() out-of-bounds!");
            return;
        }

        let src_rect = sdl2::rect::Rect::new(0, 0, tiles_rect_width, tiles_rect_height - height_subtract);
        let rect = sdl2::rect::Rect::new(x, y, tiles_rect_width, tiles_rect_height);

        WHINE_ONCE!("drawtile3() render not implemented!");
        // BlitSurfaceStandard(self.grphx.tiles3[t], &src_rect, self.buffers.backBuffer, &rect);
        self.grphx.tiles3.surfaces[t].blit(src_rect, &mut self.buffers.backBuffer, rect);
    }

    // void Graphics::drawtowertile( int x, int y, int t )
    pub fn drawtowertile(&mut self, x: i32, y: i32, t: i32) {
        let t = t as usize;
        if !INBOUNDS_VEC!(t, self.grphx.tiles2.surfaces) {
            WHINE_ONCE!("drawtowertile() out-of-bounds!");
            return;
        }

        let rect = sdl2::rect::Rect::new(x+8, y+8, self.tiles_rect.w as u32, self.tiles_rect.h as u32);
        self.grphx.tiles2.surfaces[t].blit(None, &mut self.buffers.warpbuffer, rect);
    }

    // void Graphics::drawtowertile3( int x, int y, int t, TowerBG& bg_obj )
    fn drawtowertile3(&mut self, x: i32, y: i32, t: usize, colstate: i32, tiles_rect_width: u32, tiles_rect_height: u32) -> Option<(usize, sdl2::rect::Rect)> {
        let t = t + colstate as usize * 30;
        // if !INBOUNDS_VEC!(t, self.grphx.tiles3) {
        if !INBOUNDS_VEC!(t, self.grphx.tiles3.surfaces) {
            WHINE_ONCE!("drawtowertile3() out-of-bounds!");
            return None;
        }
        let x = x + 8;
        let y = y + 8;

        // SDL_Rect rect = {x, y, tiles_rect.w, tiles_rect.h};
        // BlitSurfaceStandard(tiles3[t], NULL, bg_obj.buffer, &rect);
        let rect = sdl2::rect::Rect::new(x, y, tiles_rect_width, tiles_rect_height);
        // self.grphx.tiles3.surfaces[t as usize].blit(None, buffer, rect);

        Some((t, rect))
    }

    // void Graphics::drawgui(void)
    pub fn drawgui(&mut self, help: &mut utility_class::UtilityClass) {
        let (text_sign, crew_yp, crew_sprite) = match self.flipmode {
            true => (-1 as i32, 64 + 48 + 4, 6),
            false => (1, 64 + 32 + 4, 0),
        };

        //Draw all the textboxes to the screen
        for i in 0..self.textbox.len() {
            let text_yoff: i32 = match self.flipmode {
                true => self.textbox[i].line.len() as i32 * 8,
                false => 8,
            };

            let mut yp = self.textbox[i].yp;
            if self.flipmode && self.textbox[i].flipme {
                yp += 2 * (120 - yp) - 8 * (self.textbox[i].line.len() + 2) as i32;
            }

            //This routine also updates self.textbox colors
            let lerp = self.lerp(self.textbox[i].prev_tl, self.textbox[i].tl);
            self.textbox[i].setcol_tl_lerp(lerp);

            if self.textbox[i].tr == 0 && self.textbox[i].tg == 0 && self.textbox[i].tb == 0 {
                for j in 0..self.textbox[i].line.len() {
                    let x = self.textbox[i].xp + 8;
                    let t = &self.textbox[i].line[j].to_owned();
                    self.bprint(x, yp + text_yoff + text_sign * (j * 8) as i32, t, 196, 196, 255 - help.glow, None);
                }
            } else {
                let textrect = sdl2::rect::Rect::new(self.textbox[i].xp, yp, self.textbox[i].w as u32, self.textbox[i].h as u32);

                graphics_util::FillRect_rect_rgb(&mut self.buffers.backBuffer, textrect, self.textbox[i].r / 6, self.textbox[i].g / 6, self.textbox[i].b / 6 );

                self.drawcoloredtile(self.textbox[i].xp, yp, 40, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b);
                self.drawcoloredtile(self.textbox[i].xp + self.textbox[i].w - 8, yp, 42, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b);
                self.drawcoloredtile(self.textbox[i].xp, yp + self.textbox[i].h - 8, 45, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b);
                self.drawcoloredtile(self.textbox[i].xp + self.textbox[i].w - 8, yp + self.textbox[i].h - 8, 47, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b);
                for k in 0..self.textbox[i].lw {
                    self.drawcoloredtile(self.textbox[i].xp + 8 + (k * 8), yp, 41, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b);
                    self.drawcoloredtile(self.textbox[i].xp + 8 + (k * 8), yp + self.textbox[i].h - 8, 46, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b);
                }
                for k in 0..self.textbox[i].line.len() as i32 {
                    self.drawcoloredtile(self.textbox[i].xp, yp + 8 + (k * 8), 43, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b);
                    self.drawcoloredtile(self.textbox[i].xp + self.textbox[i].w - 8, yp + 8 + (k * 8), 44, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b);
                }

                for j in 0..self.textbox[i].line.len() as i32 {
                    let s = &self.textbox[i].line[j as usize].to_owned();
                    self.print(self.textbox[i].xp + 8, yp + text_yoff + text_sign * (j * 8), s, self.textbox[i].r, self.textbox[i].g, self.textbox[i].b, None);
                }
            }

            let opaque = self.textbox[i].tl >= 1.0;
            if !opaque {
                continue;
            }

            if self.textbox[i].yp == 12 && self.textbox[i].tr == 165 {
                if self.flipmode {
                    self.drawimage(5, 0, 180, Some(true));
                } else {
                    self.drawimage(0, 0, 12, Some(true));
                }
            } else if self.textbox[i].yp == 12 && self.textbox[i].tg == 165 {
                if self.flipmode {
                    self.drawimage(6, 0, 180, Some(true));
                } else {
                    self.drawimage(4, 0, 12, Some(true));
                }
            }
            if self.textbox[i].tr == 175 && self.textbox[i].tg == 175 {
                //purple guy
                self.drawsprite_rgb(80 - 6, crew_yp, crew_sprite, 220 - help.glow / 4 - (maths::fRandom() * 20.0) as i32, 120 - help.glow / 4, 210 - help.glow / 4);
            } else if self.textbox[i].tr == 175 && self.textbox[i].tb == 175 {
                //red guy
                self.drawsprite_rgb(80 - 6, crew_yp, crew_sprite, 255 - help.glow / 8, 70 - help.glow / 4, 70 - help.glow / 4);
            } else if self.textbox[i].tr == 175 {
                //green guy
                self.drawsprite_rgb(80 - 6, crew_yp, crew_sprite, 120 - help.glow / 4 - (maths::fRandom() * 20.0) as i32, 220 - help.glow / 4, 120 - help.glow / 4);
            } else if self.textbox[i].tg == 175 {
                //yellow guy
                self.drawsprite_rgb(80 - 6, crew_yp, crew_sprite, 220 - help.glow / 4 - (maths::fRandom() * 20.0) as i32, 210 - help.glow / 4, 120 - help.glow / 4);
            } else if self.textbox[i].tb == 175 {
                //blue guy
                self.drawsprite_rgb(80 - 6, crew_yp, crew_sprite, 75, 75, 255 - help.glow / 4 - (maths::fRandom() * 20.0) as i32);
            }
        }
    }

    // void Graphics::updatetextboxes(void)
    pub fn updatetextboxes(&mut self) {
        for text in self.textbox.iter_mut() {
            text.update();
        };
        self.textbox.retain(|text| {
            !(text.tm == 2 && text.tl <= 0.5)
        });
    }

    // void Graphics::drawimagecol( int t, int xp, int yp, int r = 0, int g = 0, int b = 0, bool cent/*= false*/ )
    pub fn drawimagecol(&mut self, t: usize, xp: i32, yp: i32, r: Option<i32>, g: Option<i32>, b: Option<i32>, cent: Option<bool>) {
        let r = r.unwrap_or(0);
        let g = g.unwrap_or(0);
        let b = b.unwrap_or(0);
        let cent = cent.unwrap_or(false);

        if !INBOUNDS_VEC!(t, self.grphx.images.surfaces) {
            return;
        }
        if r + g + b != 0 {
            // TODO: @sx ??? unused return value
            self.RGBf(r, g, b);
        }

        if cent {
            let tpoint = maths::point { x: 160 - (self.grphx.images.surfaces[t].width() as i32 / 2), y: yp };
            let trect = sdl2::rect::Rect::new(tpoint.x, tpoint.y, self.grphx.images.surfaces[t].width(), self.grphx.images.surfaces[t].height());
            graphics_util::BlitSurfaceColoured(&self.grphx.images.surfaces[t], None, &mut self.buffers.backBuffer, trect, self.ct.colour);
        } else {
            let trect = sdl2::rect::Rect::new(xp, yp, self.grphx.images.surfaces[t].width(), self.grphx.images.surfaces[t].height());
            graphics_util::BlitSurfaceColoured(&self.grphx.images.surfaces[t], None, &mut self.buffers.backBuffer, trect, self.ct.colour);
        }

        // crate::rustutil::dump_surface(&self.grphx.images.surfaces[t], "drawimagecol", &t.to_string());
    }

    // void Graphics::drawimage( int t, int xp, int yp, bool cent/*=false*/ )
    pub fn drawimage(&mut self, t: usize, xp: i32, yp: i32, cent: Option<bool>) {
        let cent = cent.unwrap_or(true);
        if !INBOUNDS_VEC!(t, self.grphx.images.surfaces) {
            return;
        }
        let w = self.grphx.images.surfaces[t].width();
        let h = self.grphx.images.surfaces[t].height();

        let trect = if cent {
            sdl2::rect::Rect::new(160 - (w as i32 / 2), yp, w, h)
        } else {
            sdl2::rect::Rect::new(xp, yp, w, h)
        };

        // crate::rustutil::dump_surface(&self.grphx.images.surfaces[t], "drawimage", &t.to_string());
        self.grphx.images.surfaces[t].blit(None, &mut self.buffers.backBuffer, trect);
    }

    // void Graphics::drawpartimage( int t, int xp, int yp, int wp, int hp)
    pub fn drawpartimage(&mut self, t: usize, xp: i32, yp: i32, wp: i32, hp: i32) {
        if !INBOUNDS_VEC!(t, self.grphx.images.surfaces) {
            return;
        }

        let trect = sdl2::rect::Rect::new(xp, yp, wp as u32, hp as u32);
        let trect2 = sdl2::rect::Rect::new(0, 0, wp as u32, hp as u32);

        self.grphx.images.surfaces[t].blit(trect2, &mut self.buffers.backBuffer, trect);
    }

    // void Graphics::cutscenebars(void)
    pub fn cutscenebars(&mut self) {
        let usethispos = self.lerp(self.oldcutscenebarspos as f32, self.cutscenebarspos as f32) as u32;
        if self.showcutscenebars {
            graphics_util::FillRect(&mut self.buffers.backBuffer, 0, 0, usethispos, 16, sdl2::pixels::Color::BLACK);
            // TODO: @sx double check for buffer overflow
            let x: u32 = 360u32.checked_sub(usethispos).unwrap_or(0);
            graphics_util::FillRect(&mut self.buffers.backBuffer, x, 224, usethispos, 16, sdl2::pixels::Color::BLACK);
        } else if self.cutscenebarspos > 0 {
            //disappearing
            graphics_util::FillRect(&mut self.buffers.backBuffer, 0, 0, usethispos, 16, sdl2::pixels::Color::BLACK);
            graphics_util::FillRect(&mut self.buffers.backBuffer, 360 - usethispos, 224, usethispos, 16, sdl2::pixels::Color::BLACK);
        }
    }

    // void Graphics::cutscenebarstimer(void)
    pub fn cutscenebarstimer(&mut self) {
        self.oldcutscenebarspos = self.cutscenebarspos;
        if self.showcutscenebars {
            self.cutscenebarspos += 25;
            self.cutscenebarspos = maths::VVV_min(self.cutscenebarspos, 361);
        } else if self.cutscenebarspos > 0 {
            //disappearing
            self.cutscenebarspos -= 25;
            self.cutscenebarspos = maths::VVV_max(self.cutscenebarspos, 0);
        }
    }

    // void Graphics::setbars(const int position)
    pub fn setbars(&mut self, position: i32) {
        self.cutscenebarspos = position;
        self.oldcutscenebarspos = position;
    }

    // void Graphics::drawcrewman( int x, int y, int t, bool act, bool noshift /*=false*/ )
    pub fn drawcrewman(&mut self, x: i32, y: i32, t: i32, act: bool, noshift: Option<bool>) {
        let noshift = noshift.unwrap_or(false);

        if !act {
            if noshift {
                if self.flipmode {
                    self.drawsprite_clru32(x, y, 14, self.col_crewinactive.colour);
                } else {
                    self.drawsprite_clru32(x, y, 12, self.col_crewinactive.colour);
                }
            } else {
                if self.flipmode {
                    self.drawsprite_clru32(x - 8, y, 14, self.col_crewinactive.colour);
                } else {
                    self.drawsprite_clru32(x - 8, y, 12, self.col_crewinactive.colour);
                }
            }
        } else {
            if self.flipmode {
                self.crewframe += 6;
            }

            match t {
                0 => {
                    self.drawsprite_clru32(x, y, self.crewframe, self.col_crewcyan.colour);
                },
                1 => {
                    self.drawsprite_clru32(x, y, self.crewframe, self.col_crewpurple.colour);
                },
                2 => {
                    self.drawsprite_clru32(x, y, self.crewframe, self.col_crewyellow.colour);
                },
                3 => {
                    self.drawsprite_clru32(x, y, self.crewframe, self.col_crewred.colour);
                },
                4 => {
                    self.drawsprite_clru32(x, y, self.crewframe, self.col_crewgreen.colour);
                },
                5 => {
                    self.drawsprite_clru32(x, y, self.crewframe, self.col_crewblue.colour);
                },
                _ => (),
            };

            if self.flipmode {
                self.crewframe -= 6;
            }
        }
    }

    // void Graphics::drawpixeltextbox( int x, int y, int w, int h, int w2, int h2, int r, int g, int b, int xo, int yo )
    pub fn drawpixeltextbox(&mut self, x: i32, y: i32, w: i32, h: i32, w2: i32, h2: i32, r: i32, g: i32, b: i32, xo: i32, yo: i32) {
        //given these parameters, draw a textbox with a pixel width

        //madrect.x = x; madrect.y = y; madrect.w = w; madrect.h = h;
        graphics_util::FillRect_xywh_rgb(&mut self.buffers.backBuffer, x, y, w, h, r/6, g/6, b/6);

        // for (int k = 0; k < w2-2; k++)
        for k in 0..w2-2 {
            self.drawcoloredtile(x + 8-xo + (k * 8), y, 41, r, g, b);
            self.drawcoloredtile(x + 8-xo + (k * 8), y + (h) - 8, 46, r, g, b);
        }

        // for (int k = 0; k < h2-2; k++)
        for k in 0..h2-2 {
            self.drawcoloredtile(x, y + 8-yo + (k * 8), 43, r, g, b);
            self.drawcoloredtile(x + (w) - 8, y + 8-yo + (k * 8), 44, r, g, b);
        }

        self.drawcoloredtile(x, y, 40, r, g, b);
        self.drawcoloredtile(x + (w) - 8, y, 42, r, g, b);
        self.drawcoloredtile(x, y + (h) - 8, 45, r, g, b);
        self.drawcoloredtile(x + (w) - 8, y + (h) - 8, 47, r, g, b);
    }

    // void Graphics::drawcustompixeltextbox( int x, int y, int w, int h, int w2, int h2, int r, int g, int b, int xo, int yo )
    pub fn drawcustompixeltextbox(&mut self, x: i32, y: i32, w: i32, h: i32, w2: i32, h2: i32, r: i32, g: i32, b: i32, xo: i32, yo: i32) {
        //given these parameters, draw a textbox with a pixel width

        graphics_util::FillRect_xywh_rgb(&mut self.buffers.backBuffer, x, y, w, h, r/6, g/6, b/6);

        for k in 0..w2-2 {
            self.drawcoloredtile(x + 8-xo + (k * 8), y, 41, r, g, b);
            self.drawcoloredtile(x + 8-xo + (k * 8), y + (h) - 8, 46, r, g, b);
        }

        self.drawcoloredtile(x + (w) - 16, y, 41, r, g, b);
        self.drawcoloredtile(x + (w) - 16, y + (h) - 8, 46, r, g, b);
        self.drawcoloredtile(x + (w) - 24, y, 41, r, g, b);
        self.drawcoloredtile(x + (w) - 24, y + (h) - 8, 46, r, g, b);

        for k in 0..h2-2 {
            self.drawcoloredtile(x, y + 8-yo + (k * 8), 43, r, g, b);
            self.drawcoloredtile(x + (w) - 8, y + 8-yo + (k * 8), 44, r, g, b);
        }

        self.drawcoloredtile(x, y + (h) - 16, 43, r, g, b);
        self.drawcoloredtile(x + (w) - 8, y + (h) - 16, 44, r, g, b);
        self.drawcoloredtile(x, y + (h) - 24, 43, r, g, b);
        self.drawcoloredtile(x + (w) - 8, y + (h) - 24, 44, r, g, b);

        self.drawcoloredtile(x, y, 40, r, g, b);
        self.drawcoloredtile(x + (w) - 8, y, 42, r, g, b);
        self.drawcoloredtile(x, y + (h) - 8, 45, r, g, b);
        self.drawcoloredtile(x + (w) - 8, y + (h) - 8, 47, r, g, b);
    }

    // void Graphics::drawtextbox( int x, int y, int w, int h, int r, int g, int b )
    pub fn drawtextbox(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32) {
        //given these parameters, draw a textbox
        graphics_util::FillRect_xywh_rgb(&mut self.buffers.backBuffer, x, y, w*8, h*8, r/6, g/6, b/6);

        self.drawcoloredtile(x, y, 40, r, g, b);
        self.drawcoloredtile(x + (w*8) - 8, y, 42, r, g, b);
        self.drawcoloredtile(x, y + (h*8) - 8, 45, r, g, b);
        self.drawcoloredtile(x + (w*8) - 8, y + (h*8) - 8, 47, r, g, b);

        for k in 0..(w-2) {
            self.drawcoloredtile(x + 8 + (k * 8), y, 41, r, g, b);
            self.drawcoloredtile(x + 8 + (k * 8), y + (h * 8) - 8, 46, r, g, b);
        }

        for k in 0..(h-2) {
            self.drawcoloredtile(x, y + 8 + (k * 8), 43, r, g, b);
            self.drawcoloredtile(x + (w * 8) - 8, y + 8 + (k * 8), 44, r, g, b);
        }
    }

    // void Graphics::textboxactive(void)
    pub fn textboxactive(&mut self) {
        //Remove all but the most recent textbox
        // for (int i = 0; i < (int) textbox.size(); i++) {
        for i in 0..self.textbox.len() {
            if self.m != i {
                self.textbox[i].remove();
            }
        }
    }

    // void Graphics::textboxremovefast(void)
    pub fn textboxremovefast(&mut self) {
        //Remove all textboxes
        // for (size_t i = 0; i < textbox.size(); i++) {
        for i in 0..self.textbox.len() {
            self.textbox[i].removefast();
        }
    }

    // void Graphics::textboxremove(void)
    pub fn textboxremove(&mut self) {
        //Remove all textboxes
        // for (size_t i = 0; i < textbox.size(); i++)
        for i in 0..self.textbox.len() {
            self.textbox[i].remove();
        }
    }

    // void Graphics::textboxtimer( int t )
    pub fn textboxtimer(&mut self, t: i32) {
        if !INBOUNDS_VEC!(self.m, self.textbox) {
            println!("textboxtimer() out-of-bounds!");
            return;
        }

        self.textbox[self.m].timer = t;
    }

    // void Graphics::addline( std::string t )
    pub fn addline(&mut self, t: &str) {
        if !INBOUNDS_VEC!(self.m, self.textbox) {
            println!("addline() out-of-bounds!");
            return;
        }

        self.textbox[self.m].addline(t);
    }

    // void Graphics::textboxadjust(void)
    pub fn textboxadjust(&mut self) {
        if !INBOUNDS_VEC!(self.m, self.textbox) {
            println!("textboxadjust() out-of-bounds!");
            return;
        }

        self.textbox[self.m].adjust();
    }

    // void Graphics::createtextboxreal(std::string t, int xp, int yp, int r, int g, int b, bool flipme)
    pub fn createtextboxreal(&mut self, t: &str, xp: i32, yp: i32, r: i32, g: i32, b: i32, flipme: bool) {
        let m = self.textbox.len();

        if m < 20 {
            let mut text = TextBoxClass::new();
            text.line.push(t.to_string());
            text.xp = xp;
            // let length = utf8::unchecked::distance(t.begin(), t.end());
            let length = t.len() as i32;

            if xp == -1 {
                text.xp = 160 - (((length / 2) + 1) * 8);
            }
            text.yp = yp;
            text.initcol(r, g, b);
            text.flipme = flipme;
            text.resize();

            self.textbox.push(text);
        }
    }

    // void Graphics::createtextbox(std::string t, int xp, int yp, int r, int g, int b)
    pub fn createtextbox(&mut self, t: &str, xp: i32, yp: i32, r: i32, g: i32, b: i32) {
        self.createtextboxreal(t, xp, yp, r, g, b, false);
    }

    // void Graphics::createtextboxflipme(std::string t, int xp, int yp, int r, int g, int b)
    pub fn createtextboxflipme(&mut self, t: &str, xp: i32, yp: i32, r: i32, g: i32, b: i32) {
        self.createtextboxreal(t, xp, yp, r, g, b, true);
    }

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
                    let x = fadebar.checked_sub(usethisamount).unwrap_or(0);
                    graphics_util::FillRect(self.buffers.backBuffer.as_mut(), x, i * 16, 500, 16, sdl2::pixels::Color::BLACK);
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
                //prepare fade out
                for i in 0..self.fadebars.len() {
                    self.fadebars[i] = -(maths::fRandom() * 12.0) as i32 * 8;
                }
                self.setfade(0);
                self.fademode = 3;
            } else if self.fademode == 3 {
                self.fadeamount += 24;
                if self.fadeamount > 416 {
                    self.fademode = 1; //faded
                }
            } else if self.fademode == 4 {
                //prepare fade in
                for i in 0..self.fadebars.len() {
                    self.fadebars[i] = 320 + (maths::fRandom() * 12.0) as i32 * 8;
                }
                println!("{:?} fadebars", self.fadebars);
                self.setfade(416);
                self.fademode = 5;
            } else if self.fademode == 5 {
                self.fadeamount -= 24;
                if self.fadeamount <= 0 {
                    self.fademode = 0; //normal
                }
            }
        }
    }

    // void Graphics::setfade(const int amount)
    pub fn setfade (&mut self, amount: i32) {
        self.fadeamount = amount;
        self.oldfadeamount = amount;
    }

    // void Graphics::drawmenu( int cr, int cg, int cb, bool levelmenu /*= false*/ )
    pub fn drawmenu (&mut self, cr: i32, cg: i32, cb: i32, _levelmenu: Option<bool>, game: &mut game::Game) {
        let levelmenu = match _levelmenu {
            Some(v) => v,
            None => false,
        };

        // for (size_t i = 0; i < game.menuoptions.len(); i++) {
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
    pub fn drawcoloredtile(&mut self, x: i32, y: i32, t: usize, r: i32, g: i32, b: i32) {
        if !INBOUNDS_VEC!(t, self.grphx.tiles.surfaces) {
            return;
        }
        self.setcolreal(self.getRGB(r, g, b));

        let rect = sdl2::rect::Rect::new(x, y, self.tiles_rect.w as u32, self.tiles_rect.h as u32);
        // self.grphx.tiles.surfaces[t].blit(None, self.buffers.backBuffer, &rect, self.ct);
        graphics_util::BlitSurfaceColoured(&self.grphx.tiles.surfaces[t], None, &mut self.buffers.backBuffer, rect, self.ct.colour);
    }

    // bool Graphics::Hitest(SDL_Surface* surface1, point p1, SDL_Surface* surface2, point p2)
    pub fn Hitest(&mut self, drawframe1: usize, p1: maths::point, drawframe2: usize, p2: maths::point, help: &mut utility_class::UtilityClass) -> bool {
        let (surface1, surface2) = if self.flipmode {
            if INBOUNDS_VEC!(drawframe1, self.grphx.flipsprites.surfaces) && INBOUNDS_VEC!(drawframe2, self.grphx.flipsprites.surfaces) {
                return false
            }
            (&self.grphx.flipsprites.surfaces[drawframe1], &self.grphx.flipsprites.surfaces[drawframe2])
        } else {
            if INBOUNDS_VEC!(drawframe1, self.grphx.sprites.surfaces) && INBOUNDS_VEC!(drawframe2, self.grphx.sprites.surfaces) {
                return false
            }
            (&self.grphx.sprites.surfaces[drawframe1], &self.grphx.sprites.surfaces[drawframe2])
        };

        //find rectangle where they intersect:
        let r1_left = p1.x;
        let r1_right = r1_left + surface1.width() as i32;
        let r2_left = p2.x;
        let r2_right = r2_left + surface2.width() as i32;

        let r1_bottom = p1.y;
        let r1_top = p1.y + surface1.height() as i32;
        let r2_bottom  = p2.y;
        let r2_top = p2.y + surface2.height() as i32;

        let rect1 = sdl2::rect::Rect::new(p1.x, p1.y, surface1.width(), surface1.height());
        let rect2 = sdl2::rect::Rect::new(p2.x, p2.y, surface2.width(), surface2.height());
        let intersection = help.intersects(rect1, rect2);

        if intersection {
            let r3_left = maths::VVV_max(r1_left, r2_left);
            let r3_top = maths::VVV_min(r1_top, r2_top);
            let r3_right = maths::VVV_min(r1_right, r2_right);
            let r3_bottom = maths::VVV_max(r1_bottom, r2_bottom);

            //for every pixel inside rectangle
            for x in r3_left..r3_right {
                for y in r3_bottom..r3_top {
                    let pixel1 = graphics_util::ReadPixel(surface1, x - p1.x, y - p1.y);
                    let pixel2 = graphics_util::ReadPixel(surface2, x - p2.x, y - p2.y);
                    // TODO: @sx recheck conditions
                    if ((pixel1 & 0x000000FF) > 0) && ((pixel2 & 0x000000FF) > 0) {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    // void Graphics::drawgravityline( int t )
    pub fn drawgravityline(&mut self, t: usize) {
        println!("DEADBEEF: Graphics::drawgravityline method not implemented yet");
    }

    // void Graphics::drawtrophytext(void)
    pub fn drawtrophytext(&mut self, obj: &mut entity::EntityClass, help: &mut utility_class::UtilityClass) {
        let mut temp: i32;
        let mut temp2: i32;
        let mut temp3: i32;

        if obj.trophytext < 15 {
            let usethismult = self.lerp(obj.oldtrophytext as f32, obj.trophytext as f32) as i32;
            temp = (196 * usethismult) / 15;
            temp2 = (196 * usethismult) / 15;
            temp3 = ((255 - help.glow) * usethismult) / 15;
        } else {
            temp = 196;
            temp2 = 196;
            temp3 = 255 - help.glow;
        }

        match obj.trophytype {
            1 => {
                self.print( -1, 6, "SPACE STATION 1 MASTERED", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Obtain a V Rank in this Time Trial", temp, temp2, temp3, Some(true));
            },
            2 => {
                self.print( -1, 6, "LABORATORY MASTERED", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Obtain a V Rank in this Time Trial", temp, temp2, temp3, Some(true));
            },
            3 => {
                self.print( -1, 6, "THE TOWER MASTERED", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Obtain a V Rank in this Time Trial", temp, temp2, temp3, Some(true));
            },
            4 => {
                self.print( -1, 6, "SPACE STATION 2 MASTERED", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Obtain a V Rank in this Time Trial", temp, temp2, temp3, Some(true));
            },
            5 => {
                self.print( -1, 6, "WARP ZONE MASTERED", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Obtain a V Rank in this Time Trial", temp, temp2, temp3, Some(true));
            },
            6 => {
                self.print( -1, 6, "FINAL LEVEL MASTERED", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Obtain a V Rank in this Time Trial", temp, temp2, temp3, Some(true));
            },
            7 => {
                self.print( -1, 6, "GAME COMPLETE", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Complete the game", temp, temp2, temp3, Some(true));
            },
            8 => {
                self.print( -1, 6, "FLIP MODE COMPLETE", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Complete the game in flip mode", temp, temp2, temp3, Some(true));
            },
            9 => {
                self.print( -1, 11, "Win with less than 50 deaths", temp, temp2, temp3, Some(true));
            },
            10 => {
                self.print( -1, 11, "Win with less than 100 deaths", temp, temp2, temp3, Some(true));
            },
            11 => {
                self.print( -1, 11, "Win with less than 250 deaths", temp, temp2, temp3, Some(true));
            },
            12 => {
                self.print( -1, 11, "Win with less than 500 deaths", temp, temp2, temp3, Some(true));
            },
            13 => {
                self.print( -1, 11, "Last 5 seconds on the Super Gravitron", temp, temp2, temp3, Some(true));
            },
            14 => {
                self.print( -1, 11, "Last 10 seconds on the Super Gravitron", temp, temp2, temp3, Some(true));
            },
            15 => {
                self.print( -1, 11, "Last 15 seconds on the Super Gravitron", temp, temp2, temp3, Some(true));
            },
            16 => {
                self.print( -1, 11, "Last 20 seconds on the Super Gravitron", temp, temp2, temp3, Some(true));
            },
            17 => {
                self.print( -1, 11, "Last 30 seconds on the Super Gravitron", temp, temp2, temp3, Some(true));
            },
            18 => {
                self.print( -1, 11, "Last 1 minute on the Super Gravitron", temp, temp2, temp3, Some(true));
            },
            20 => {
                self.print( -1, 6, "MASTER OF THE UNIVERSE", temp, temp2, temp3, Some(true));
                self.print( -1, 16, "Complete the game in no death mode", temp, temp2, temp3, Some(true));
            },
            _ => println!("unmatched trophytype value {}", obj.trophytype),
        };
    }

    // void Graphics::drawentities(void)
    pub fn drawentities(&mut self, game: &mut game::Game, obj: &mut entity::EntityClass, map: &mut map::Map) {
        trace!("=================          DRAWING ENTITIES            ==========================");
        let yoff = if map.towermode {
            self.lerp(map.oldypos as f32, map.ypos as f32) as i32
        } else {
            0
        };

        if !map.custommode {
            for i in (0..obj.entities.len()).rev() {
                if !obj.entities[i].ishumanoid() {
                    self.drawentity(i, yoff, game, obj, map);
                }
            }

            for i in (0..obj.entities.len()).rev() {
                if obj.entities[i].ishumanoid() {
                    self.drawentity(i, yoff, game, obj, map);
                }
            }
        } else {
            for i in (0..obj.entities.len()).rev() {
                self.drawentity(i, yoff, game, obj, map);
            }
        }
    }

    // void Graphics::drawentity(const int i, const int yoff)
    fn drawentity(&mut self, i: usize, yoff: i32, game: &mut game::Game, obj: &mut entity::EntityClass, map: &mut map::Map) {
        if !INBOUNDS_VEC!(i, obj.entities) {
            WHINE_ONCE!("drawentity() out-of-bounds!");
            return;
        }

        if obj.entities[i].invis {
            trace!("{:?} - entity is invisible!", i);
            return;
        }

        let mut tpoint = maths::point { x: 0, y: 0 };
        let mut drawRect = sdl2::rect::Rect::new(0, 0, 0, 0);

        // // #if !defined(NO_CUSTOM_LEVELS)
        // // Special case for gray Warp Zone tileset!
        // const edlevelclass* const room = ed.getroomprop(game.roomx - 100, game.roomy - 100);
        // const bool custom_gray = room->tileset == 3 && room->tilecol == 6;
        // // #else
        const custom_gray: bool = false;
        // #endif

        let tilesvec = if map.custommode && !map.finalmode {
            &mut self.grphx.entcolours.surfaces
        } else {
            &mut self.grphx.tiles.surfaces
        };
        let spritesvec = if self.flipmode {
            &mut self.grphx.flipsprites.surfaces
        } else {
            &mut self.grphx.sprites.surfaces
        };

        fn lerp(alpha: f32, v0: f32, v1: f32) -> f32 {
            v0 + alpha * (v1 - v0)
        }
        let xp: i32 = lerp(self.alpha, obj.entities[i].lerpoldxp as f32, obj.entities[i].xp as f32) as i32;
        let yp: i32 = lerp(self.alpha, obj.entities[i].lerpoldyp as f32, obj.entities[i].yp as f32) as i32;

        trace!("{:?} - drawing {:?}", i, obj.entities[i]);
        match obj.entities[i].size {
            0 => {
                // Sprites
                if !INBOUNDS_VEC!(obj.entities[i].drawframe, spritesvec) {
                    return;
                }
                tpoint.x = xp;
                tpoint.y = yp - yoff;
                // self.setcolreal(obj.entities[i].realcol);
                self.ct.colour = obj.entities[i].realcol;

                drawRect = self.sprites_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;

                graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);

                //screenwrapping!
                let mut wrappedPoint = maths::point { x: 0, y: 0 };
                let mut wrapX = false;
                let mut wrapY = false;

                wrappedPoint.x = tpoint.x;
                if tpoint.x < 0 {
                    wrapX = true;
                    wrappedPoint.x += 320;
                } else if tpoint.x > 300 {
                    wrapX = true;
                    wrappedPoint.x -= 320;
                }

                wrappedPoint.y = tpoint.y;
                if tpoint.y < 8 {
                    wrapY = true;
                    wrappedPoint.y += 232;
                } else if tpoint.y > 210 {
                    wrapY = true;
                    wrappedPoint.y -= 232;
                }

                let isInWrappingAreaOfTower = map.towermode && !map.minitowermode && map.ypos >= 500 && map.ypos <= 5000;
                if wrapX && (map.warpx || isInWrappingAreaOfTower) {
                    drawRect = self.sprites_rect;
                    drawRect.x += wrappedPoint.x;
                    drawRect.y += tpoint.y;
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }
                if wrapY && map.warpy {
                    drawRect = self.sprites_rect;
                    drawRect.x += tpoint.x;
                    drawRect.y += wrappedPoint.y;
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }
                if wrapX && wrapY && map.warpx && map.warpy {
                    drawRect = self.sprites_rect;
                    drawRect.x += wrappedPoint.x;
                    drawRect.y += wrappedPoint.y;
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }
            },
            1 => {
                // Tiles
                if !INBOUNDS_VEC!(obj.entities[i].drawframe, self.grphx.tiles.surfaces) {
                    return;
                }
                tpoint.x = xp;
                tpoint.y = yp - yoff;
                drawRect = self.tiles_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;
                self.grphx.tiles.surfaces[obj.entities[i].drawframe].blit(None, &mut self.buffers.backBuffer, drawRect);
            },
            2 | 8 => {
                // Special: Moving platform, 4 tiles or 8 tiles
                if !INBOUNDS_VEC!(obj.entities[i].drawframe, tilesvec) {
                    return;
                }
                tpoint.x = xp;
                tpoint.y = yp - yoff;
                let mut thiswidth = 4;
                if obj.entities[i].size == 8 {
                    thiswidth = 8;
                }
                // for (int ii = 0; ii < thiswidth; ii++ {
                for ii in 0..thiswidth {
                    drawRect = self.tiles_rect;
                    drawRect.x += tpoint.x;
                    drawRect.y += tpoint.y;
                    drawRect.x += 8 * ii;
                    if custom_gray {
                        graphics_util::BlitSurfaceTinted(&tilesvec[obj.entities[i].drawframe], None, &mut self.buffers.backBuffer, drawRect, 0xFFFFFFFF)
                    } else {
                        tilesvec[obj.entities[i].drawframe].blit(None, &mut self.buffers.backBuffer, drawRect);
                    }
                }
            },
            3 => {
                // Big chunky pixels!
                self.prect.x = xp;
                self.prect.y = yp - yoff;
                graphics_util::FillRect_rect_coloru32(&mut self.buffers.backBuffer, self.prect, obj.entities[i].realcol);
            },
            4 => {
                // Small pickups
                self.setcolreal(obj.entities[i].realcol);
                self.drawhuetile(xp, yp - yoff, obj.entities[i].tile as usize);
            },
            5 => {
                //Horizontal Line
                let mut oldw = obj.entities[i].w;
                if (game.swngame == 3 || self.kludgeswnlinewidth) && obj.getlineat(84 - 32) == i {
                    oldw -= 24;
                }
                self.line_rect.x = xp;
                self.line_rect.y = yp - yoff;
                self.line_rect.w = self.lerp(oldw as f32, obj.entities[i].w as f32) as i32;
                self.line_rect.h = 1;
                self.drawgravityline(i);
            },
            6 => {
                //Vertical Line
                self.line_rect.x = xp;
                self.line_rect.y = yp - yoff;
                self.line_rect.w = 1;
                self.line_rect.h = obj.entities[i].h;
                self.drawgravityline(i);
            },
            7 => {
                //Teleporter
                self.drawtele(xp, yp - yoff, obj.entities[i].drawframe, obj.entities[i].realcol);
            },
            9 => {
                // Really Big Sprite! (2x2)
                // self.setcolreal(obj.entities[i].realcol);
                self.ct.colour = obj.entities[i].realcol;

                tpoint.x = xp;
                tpoint.y = yp - yoff;

                drawRect = self.sprites_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;
                if INBOUNDS_VEC!(obj.entities[i].drawframe, spritesvec) {
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe], None,&mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }

                tpoint.x = xp+32;
                tpoint.y = yp - yoff;
                //
                drawRect = self.sprites_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;
                if INBOUNDS_VEC!(obj.entities[i].drawframe+1, spritesvec) {
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe+1], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }

                tpoint.x = xp;
                tpoint.y = yp+32 - yoff;
                //
                drawRect = self.sprites_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;
                if INBOUNDS_VEC!(obj.entities[i].drawframe+12, spritesvec) {
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe+12], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }

                tpoint.x = xp+32;
                tpoint.y = yp+32 - yoff;
                //
                drawRect = self.sprites_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;
                if INBOUNDS_VEC!(obj.entities[i].drawframe+13, spritesvec) {
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe+13], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }
            },
            10 => {
                // 2x1 Sprite
                // self.setcolreal(obj.entities[i].realcol);
                self.ct.colour = obj.entities[i].realcol;

                tpoint.x = xp;
                tpoint.y = yp - yoff;
                //
                drawRect = self.sprites_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;
                if INBOUNDS_VEC!(obj.entities[i].drawframe, spritesvec) {
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }

                tpoint.x = xp+32;
                tpoint.y = yp - yoff;
                //
                drawRect = self.sprites_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;
                if INBOUNDS_VEC!(obj.entities[i].drawframe+1, spritesvec) {
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe+1], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }
            },
            11 => {
                //The fucking elephant
                self.setcolreal(obj.entities[i].realcol);
                self.drawimagecol(3, xp, yp - yoff, None, None, None, None);
            },
            12 => {
                // Regular sprites that don't wrap
                tpoint.x = xp;
                tpoint.y = yp - yoff;
                // self.setcolreal(obj.entities[i].realcol);
                self.ct.colour = obj.entities[i].realcol;

                //
                drawRect = self.sprites_rect;
                drawRect.x += tpoint.x;
                drawRect.y += tpoint.y;
                if INBOUNDS_VEC!(obj.entities[i].drawframe, spritesvec) {
                    graphics_util::BlitSurfaceColoured(&spritesvec[obj.entities[i].drawframe], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                }

                //if we're outside the screen, we need to draw indicators

                if obj.entities[i].xp < -20 && obj.entities[i].vx > 0.0 {
                    if obj.entities[i].xp < -100 {
                        tpoint.x = -5 + (( -obj.entities[i].xp) / 10);
                    } else {
                        tpoint.x = 5;
                    }

                    tpoint.y = tpoint.y+4;

                    drawRect = self.tiles_rect;
                    drawRect.x += tpoint.x;
                    drawRect.y += tpoint.y;
                    if INBOUNDS_VEC!(1167, self.grphx.tiles.surfaces) {
                        graphics_util::BlitSurfaceColoured(&self.grphx.tiles.surfaces[1167], None, &mut self.buffers.backBuffer, drawRect, self.ct.colour);
                    }
                } else if obj.entities[i].xp > 340 && obj.entities[i].vx < 0.0 {
                    if obj.entities[i].xp > 420 {
                        tpoint.x = 320 - (( obj.entities[i].xp-320) / 10);
                    } else {
                        tpoint.x = 310;
                    }

                    tpoint.y = tpoint.y+4;
                    //

                    drawRect = self.tiles_rect;
                    drawRect.x += tpoint.x;
                    drawRect.y += tpoint.y;
                    if INBOUNDS_VEC!(1166, self.grphx.tiles.surfaces) {
                        graphics_util::BlitSurfaceColoured(&self.grphx.tiles.surfaces[1166], None,&mut self.buffers.backBuffer, drawRect, self.ct.colour);
                    }
                }
            },
            13 => {
                //Special for epilogue: huge hero!
                if !INBOUNDS_VEC!(obj.entities[i].drawframe, spritesvec) {
                    return;
                }

                tpoint.x = xp; tpoint.y = yp - yoff;
                // self.setcolreal(obj.entities[i].realcol);
                self.ct.colour = obj.entities[i].realcol;

                let drawRect = sdl2::rect::Rect::new(xp, yp - yoff, self.sprites_rect.x as u32 * 6, self.sprites_rect.y as u32 * 6);
                let TempSurface = graphics_util::ScaleSurface( &spritesvec[obj.entities[i].drawframe], 6 * self.sprites_rect.w as u32, 6* self.sprites_rect.h as u32);
                graphics_util::BlitSurfaceColoured(&TempSurface, None, &mut self.buffers.backBuffer,  drawRect, self.ct.colour);
            },
            _ => println!("warning! unknown entity size, ({})", obj.entities[i].size),
        };
    }

    fn blit_dynamically() {

    }

    // void Graphics::drawbackground( int t )
    pub fn drawbackground(&mut self, map: &mut map::Map) {
        match map.background {
            1 => {
                //Starfield
                graphics_util::ClearSurface(&mut self.buffers.backBuffer);
                for i in 0..numstars {
                    self.stars[i].w = 2;
                    self.stars[i].h = 2;

                    self.stars[i].x = self.lerp((self.stars[i].x + self.starsspeed[i]) as f32, self.stars[i].x as f32) as i32;
                    if self.starsspeed[i] <= 6 {
                        let rgba = self.getRGB_AsPixelColor(0x22, 0x22, 0x22);
                        graphics_util::FillRect_rect(&mut self.buffers.backBuffer, self.stars[i], rgba);
                    } else {
                        let rgba = self.getRGB_AsPixelColor(0x55, 0x55, 0x55);
                        graphics_util::FillRect_rect(&mut self.buffers.backBuffer, self.stars[i], rgba);
                    }
                }
            },
            2 => {
                let mut bcol = sdl2::pixels::Color::BLACK;
                let mut bcol2 = sdl2::pixels::Color::BLACK;

                //Lab
                match self.rcol {
                    //Akward ordering to match tileset
                    0 => {
                        //Cyan
                        bcol2 = self.RGBflip_AsPixelColor(0, 16 * self.backboxint[0] as i32, 16 * self.backboxint[0] as i32);
                    },
                    1 => {
                        //Red
                        bcol2 = self.RGBflip_AsPixelColor(16 * self.backboxint[0] as i32, 0, 0);
                    },
                    2 => {
                        //Purple
                        bcol2 = self.RGBflip_AsPixelColor(16 * self.backboxint[0] as i32, 0, 16 * self.backboxint[0] as i32);
                    },
                    3 => {
                        //Blue
                        bcol2 = self.RGBflip_AsPixelColor(0, 0, 16 * self.backboxint[0] as i32);
                    },
                    4 => {
                        //Yellow
                        bcol2 = self.RGBflip_AsPixelColor(16 * self.backboxint[0] as i32, 16 * self.backboxint[0] as i32, 0);
                    },
                    5 => {
                        //Green
                        bcol2 = self.RGBflip_AsPixelColor(0, 16 * self.backboxint[0] as i32, 0);
                    },
                    6 => {
                        //crazy case
                        match self.spcol {
                            0 => {
                                bcol2 = self.RGBflip_AsPixelColor(0, 16 * self.backboxint[0] as i32, 16 * self.backboxint[0] as i32);
                                //Cyan
                            },
                            1 => {
                                bcol2 = self.RGBflip_AsPixelColor(0, (self.spcoldel+1) * self.backboxint[0] as i32, 16 * self.backboxint[0] as i32);
                                //Cyan
                            },
                            2 => {
                                bcol2 = self.RGBflip_AsPixelColor(0, 0, 16 * self.backboxint[0] as i32);
                                //Blue
                            },
                            3 => {
                                bcol2 = self.RGBflip_AsPixelColor((16-self.spcoldel) * self.backboxint[0] as i32, 0, 16 * self.backboxint[0] as i32);
                                //Blue
                            },
                            4 => {
                                bcol2 = self.RGBflip_AsPixelColor(16 * self.backboxint[0] as i32, 0, 16 * self.backboxint[0] as i32);
                                //Purple
                            },
                            5 => {
                                bcol2 = self.RGBflip_AsPixelColor(16 * self.backboxint[0] as i32, 0, (self.spcoldel+1) * self.backboxint[0] as i32);
                                //Purple
                            },
                            6 => {
                                bcol2 = self.RGBflip_AsPixelColor(16 * self.backboxint[0] as i32, 0, 0);
                                //Red
                            },
                            7 => {
                                bcol2 = self.RGBflip_AsPixelColor(16 * self.backboxint[0] as i32, (16-self.spcoldel) * self.backboxint[0] as i32, 0);
                                //Red
                            },
                            8 => {
                                bcol2 = self.RGBflip_AsPixelColor(16 * self.backboxint[0] as i32, 16 * self.backboxint[0] as i32, 0);
                                //Yellow
                            },
                            9 => {
                                bcol2 = self.RGBflip_AsPixelColor((self.spcoldel+1) * self.backboxint[0] as i32, 16 * self.backboxint[0] as i32, 0);
                                //Yellow
                            },
                            10 => {
                                bcol2 = self.RGBflip_AsPixelColor(0, 16 * self.backboxint[0] as i32, 0);
                                //Green
                            },
                            11 => {
                                bcol2 = self.RGBflip_AsPixelColor(0, 16 * self.backboxint[0] as i32, (16-self.spcoldel) * self.backboxint[0] as i32);
                                //Green
                            },
                            _ => println!("unkown case {}", self.spcol),
                        };
                    },
                    _ => println!("unkown rcol case {}", self.rcol),
                };

                graphics_util::FillRectWithColor(&mut self.buffers.backBuffer, bcol2);

                for i in 0..numbackboxes {
                    match self.rcol {
                        //Akward ordering to match tileset
                        0 => {
                            //Cyan
                            bcol = self.RGBflip_AsPixelColor(16, 128 * self.backboxint[0] as i32, 128 * self.backboxint[0] as i32);
                        },
                        1 => {
                            //Red
                            bcol = self.RGBflip_AsPixelColor(128 * self.backboxint[0] as i32, 16, 16);
                        },
                        2 => {
                            //Purple
                            bcol = self.RGBflip_AsPixelColor(128 * self.backboxint[0] as i32, 16, 128 * self.backboxint[0] as i32);
                        },
                        3 => {
                            //Blue
                            bcol = self.RGBflip_AsPixelColor(16, 16, 128 * self.backboxint[0] as i32);
                        },
                        4 => {
                            //Yellow
                            bcol = self.RGBflip_AsPixelColor(128 * self.backboxint[0] as i32, 128 * self.backboxint[0] as i32, 16);
                        },
                        5 => {
                            //Green
                            bcol = self.RGBflip_AsPixelColor(16, 128 * self.backboxint[0] as i32, 16);
                        },
                        6 => {
                            //crazy case
                            match self.spcol {
                                0 => {
                                    //Cyan
                                    bcol = self.RGBflip_AsPixelColor(16, 128 * self.backboxint[0] as i32, 128 * self.backboxint[0] as i32);
                                },
                                1 => {
                                    //Cyan
                                    bcol = self.RGBflip_AsPixelColor(16, ((self.spcoldel+1)*8) * self.backboxint[0] as i32, 128 * self.backboxint[0] as i32);
                                },
                                2 => {
                                    //Blue
                                    bcol = self.RGBflip_AsPixelColor(16, 16, 128 * self.backboxint[0] as i32);
                                },
                                3 => {
                                    //Blue
                                    bcol = self.RGBflip_AsPixelColor((128-(self.spcoldel*8)) * self.backboxint[0] as i32, 16, 128 * self.backboxint[0] as i32);
                                },
                                4 => {
                                    //Purple
                                    bcol = self.RGBflip_AsPixelColor(128 * self.backboxint[0] as i32, 16, 128 * self.backboxint[0] as i32);
                                },
                                5 => {
                                    //Purple
                                    bcol = self.RGBflip_AsPixelColor(128 * self.backboxint[0] as i32, 16, ((self.spcoldel+1)*8) * self.backboxint[0] as i32);
                                },
                                6 => {
                                    //Red
                                    bcol = self.RGBflip_AsPixelColor(128 * self.backboxint[0] as i32, 16, 16);
                                },
                                7 => {
                                    //Red
                                    bcol = self.RGBflip_AsPixelColor(128 * self.backboxint[0] as i32, (128-(self.spcoldel*8)) * self.backboxint[0] as i32, 16);
                                },
                                8 => {
                                    //Yellow
                                    bcol = self.RGBflip_AsPixelColor(128 * self.backboxint[0] as i32, 128 * self.backboxint[0] as i32, 16);
                                },
                                9 => {
                                    //Yellow
                                    bcol = self.RGBflip_AsPixelColor(((self.spcoldel+1)*8) * self.backboxint[0] as i32, 128 * self.backboxint[0] as i32, 16);
                                },
                                10 => {
                                    //Green
                                    bcol = self.RGBflip_AsPixelColor(16, 128 * self.backboxint[0] as i32, 16);
                                },
                                11 => {
                                    //Green
                                    bcol = self.RGBflip_AsPixelColor(16, 128 * self.backboxint[0] as i32, (128-(self.spcoldel*8)) * self.backboxint[0] as i32);
                                },
                                _ => println!("unknown crazy case {}", self.spcol),
                            };
                        }
                        _ => println!("unknown rcol case {}", self.rcol),
                    };

                    self.backboxes[i].x = self.lerp((self.backboxes[i].x - self.backboxvx[i]) as f32, self.backboxes[i].x as f32) as i32;
                    self.backboxes[i].y = self.lerp((self.backboxes[i].y - self.backboxvy[i]) as f32, self.backboxes[i].y as f32) as i32;

                    graphics_util::FillRect_rect(&mut self.buffers.backBuffer, self.backboxes[i], bcol);
                    self.backboxes[i].x += 1;
                    self.backboxes[i].y += 1;
                    self.backboxes[i].w -= 2;
                    self.backboxes[i].h -= 2;
                    graphics_util::FillRect_rect(&mut self.buffers.backBuffer, self.backboxes[i], bcol2);
                };
            },
            3 => {
                //Warp zone (horizontal)
                graphics_util::ClearSurface(&mut self.buffers.backBuffer);
                self.buffers.warpbuffer.blit(None, &mut self.buffers.warpbuffer_lerp, None);
                let pX = self.lerp(0.0, -3.0) as i32;
                graphics_util::ScrollSurface(&mut self.buffers.warpbuffer_lerp, pX, 0);
                self.buffers.warpbuffer_lerp.blit(self.towerbuffer_rect, &mut self.buffers.backBuffer, None);
            },
            4 => {
                //Warp zone (vertical)
                graphics_util::ClearSurface(&mut self.buffers.backBuffer);
                self.buffers.warpbuffer.blit(None, &mut self.buffers.warpbuffer_lerp, None);
                let pY = self.lerp(0.0, -3.0) as i32;
                graphics_util::ScrollSurface(&mut self.buffers.warpbuffer_lerp, 0, pY);
                self.buffers.warpbuffer_lerp.blit(self.towerbuffer_rect, &mut self.buffers.backBuffer, None);
            },
            5 => {
                //Warp zone, central
                match self.rcol {
                    //Akward ordering to match tileset
                    0 => {
                        //Cyan
                        self.warpbcol = self.RGBflip_AsPixelColor(0x0Ai32, 0x10i32, 0x0Ei32);
                        self.warpfcol = self.RGBflip_AsPixelColor(0x10i32, 0x22i32, 0x21i32);
                    },
                    1 => {
                        //Red
                        self.warpbcol = self.RGBflip_AsPixelColor(0x11i32, 0x09i32, 0x0Bi32);
                        self.warpfcol = self.RGBflip_AsPixelColor(0x22i32, 0x10i32, 0x11i32);
                    },
                    2 => {
                        //Purple
                        self.warpbcol = self.RGBflip_AsPixelColor(0x0Fi32, 0x0Ai32, 0x10i32);
                        self.warpfcol = self.RGBflip_AsPixelColor(0x22i32, 0x10i32, 0x22i32);
                    },
                    3 => {
                        //Blue
                        self.warpbcol = self.RGBflip_AsPixelColor(0x0Ai32, 0x0Bi32, 0x10i32);
                        self.warpfcol = self.RGBflip_AsPixelColor(0x10i32, 0x10i32, 0x22i32);
                    },
                    4 => {
                        //Yellow
                        self.warpbcol = self.RGBflip_AsPixelColor(0x10i32, 0x0Di32, 0x0Ai32);
                        self.warpfcol = self.RGBflip_AsPixelColor(0x22i32, 0x1Ei32, 0x10i32);
                    },
                    5 => {
                        //Green
                        self.warpbcol = self.RGBflip_AsPixelColor(0x0Di32, 0x10i32, 0x0Ai32);
                        self.warpfcol = self.RGBflip_AsPixelColor(0x14i32, 0x22i32, 0x10i32);
                    },
                    6 => {
                        //Gray
                        self.warpbcol = self.RGBflip_AsPixelColor(0x0Ai32, 0x0Ai32, 0x0Ai32);
                        self.warpfcol = self.RGBflip_AsPixelColor(0x12i32, 0x12i32, 0x12i32);
                    },
                    _ => {
                        self.warpbcol = self.RGBflip_AsPixelColor(0xFFi32, 0xFFi32, 0xFFi32);
                        self.warpfcol = self.RGBflip_AsPixelColor(0xFFi32, 0xFFi32, 0xFFi32);
                    },
                };

                for i in (0..10).rev() {
                    let temp = (i << 4) + self.lerp((self.backoffset - 1) as f32, self.backoffset as f32) as i32;
                    self.setwarprect(160 - temp, 120 - temp, temp * 2, temp * 2);
                    if i % 2 == self.warpskip {
                        graphics_util::FillRect_rect(&mut self.buffers.backBuffer, self.warprect, self.warpbcol);
                    } else {
                        graphics_util::FillRect_rect(&mut self.buffers.backBuffer, self.warprect, self.warpfcol);
                    }
                }
            },
            6 => {
                //Final Starfield
                graphics_util::ClearSurface(&mut self.buffers.backBuffer);
                // for int i = 0; i < numstars; i++ {
                for i in 0..numstars {
                    self.stars[i].w = 2;
                    self.stars[i].h = 2;
                    let mut star_rect = self.stars[i];
                    star_rect.y = self.lerp((star_rect.y + self.starsspeed[i]) as f32, star_rect.y as f32) as i32;
                    if self.starsspeed[i] <= 8 {
                        let rgba = self.getRGB_AsPixelColor(0x22, 0x22, 0x22);
                        graphics_util::FillRect_rect(&mut self.buffers.backBuffer, star_rect, rgba);
                    } else {
                        let rgba = self.getRGB_AsPixelColor(0x55, 0x55, 0x55);
                        graphics_util::FillRect_rect(&mut self.buffers.backBuffer, star_rect, rgba);
                    }
                }
            },
            7 => {
                //Static, unscrolling section of the tower
                for j in 0..30 {
                    for i in 0..40 {
                        let t = map.tower.backat(i, j, 200);
                        self.drawtile3(i * 8, j * 8, t, 15, None);
                    }
                }
            },
            8 => {
                //Static, unscrolling section of the tower
                for j in 0..30 {
                    for i in 0..40 {
                        let t = map.tower.backat(i, j, 200);
                        self.drawtile3(i * 8, j * 8, t, 10, None);
                    }
                }
            },
            9 => {
                //Static, unscrolling section of the tower
                for j in 0..30 {
                    for i in 0..40 {
                        let t = map.tower.backat(i, j, 600);
                        self.drawtile3(i * 8, j * 8, t, 0, None);
                    }
                }
            },
            _ => graphics_util::ClearSurface(&mut self.buffers.backBuffer),
        };
    }

    // void Graphics::updatebackground(int t)
    pub fn updatebackground(&mut self, t: i32) {
        match t {
            1 => {
                //Starfield
                for i in 0..numstars {
                    self.stars[i].w = 2;
                    self.stars[i].h = 2;
                    self.stars[i].x -= self.starsspeed[i];
                    if self.stars[i].x < -10 {
                        self.stars[i].x += 340;
                        self.stars[i].y = (maths::fRandom() * 240.0) as i32;
                        self.stars[i].w = 2;
                        self.starsspeed[i] = 4 + (maths::fRandom() * 4.0) as i32;
                    }
                }
            },
            2 => {
                //Lab
                if self.rcol == 6 {
                    //crazy caze
                    self.spcoldel -= 1;
                    if self.spcoldel <= 0 {
                        self.spcoldel = 15;
                        self.spcol += 1;
                        if self.spcol >= 12 {
                            self.spcol = 0;
                        }
                    }
                }

                for i in 0..numbackboxes {
                    self.backboxes[i].x += self.backboxvx[i];
                    self.backboxes[i].y += self.backboxvy[i];
                    if self.backboxes[i].x < -40 {
                        self.backboxes[i].x = 320;
                        self.backboxes[i].y = maths::fRandom() as i32 * 240;
                    }
                    if self.backboxes[i].x > 320 {
                        self.backboxes[i].x = -32;
                        self.backboxes[i].y = maths::fRandom() as i32 * 240;
                    }
                    if self.backboxes[i].y < -40 {
                        self.backboxes[i].y = 240;
                        self.backboxes[i].x = maths::fRandom() as i32 * 320;
                    }
                    if self.backboxes[i].y > 260 {
                        self.backboxes[i].y = -32;
                        self.backboxes[i].x = maths::fRandom() as i32 * 320;
                    }
                }
            },
            3 => {
                //Warp zone (horizontal)
                let temp = 680 + (self.rcol * 3);
                self.backoffset += 3;
                if self.backoffset >= 16 {
                    self.backoffset -= 16;
                }

                if self.backgrounddrawn {
                    graphics_util::ScrollSurface(&mut self.buffers.warpbuffer, -3, 0);

                    for j in 0..15 {
                        for i in 0..2 {
                            self.drawtowertile(317 - self.backoffset + (i * 16), j * 16, temp + 40);  //20*16 = 320
                            self.drawtowertile(317 - self.backoffset + (i * 16) + 8, j * 16, temp + 41);
                            self.drawtowertile(317 - self.backoffset + (i * 16), (j * 16) + 8, temp + 80);
                            self.drawtowertile(317 - self.backoffset + (i * 16) + 8, (j * 16) + 8, temp + 81);
                        }
                    }
                } else {
                    //draw the whole thing for the first time!
                    self.backoffset = 0;
                    graphics_util::ClearSurface(&mut self.buffers.warpbuffer);

                    for j in 0..15 {
                        for i in 0..21 {
                            self.drawtowertile((i * 16) - self.backoffset - 3, j * 16, temp + 40);
                            self.drawtowertile((i * 16) - self.backoffset + 8 - 3, j * 16, temp + 41);
                            self.drawtowertile((i * 16) - self.backoffset - 3, (j * 16) + 8, temp + 80);
                            self.drawtowertile((i * 16) - self.backoffset + 8 - 3, (j * 16) + 8, temp + 81);
                        }
                    }
                    self.backgrounddrawn = true;
                }
            },
            4 => {
                //Warp zone (vertical)
                let temp = 760 + (self.rcol * 3);
                self.backoffset += 3;
                if self.backoffset >= 16 {
                    self.backoffset -= 16;
                }

                if self.backgrounddrawn {
                    graphics_util::ScrollSurface(&mut self.buffers.warpbuffer, 0, -3);

                    for j in 0..2 {
                        for i in 0..21 {
                            self.drawtowertile((i * 16), 237 - self.backoffset + j * 16, temp + 40); //14*17=240 - 3
                            self.drawtowertile((i * 16) + 8, 237 - self.backoffset + j * 16, temp + 41);
                            self.drawtowertile((i * 16), 237 - self.backoffset + (j * 16) + 8, temp + 80);
                            self.drawtowertile((i * 16) + 8, 237 - self.backoffset + (j * 16) + 8, temp + 81);
                        }
                    }
                } else {
                    //draw the whole thing for the first time!
                    self.backoffset = 0;
                    graphics_util::ClearSurface(&mut self.buffers.warpbuffer);

                    for j in 0..16 {
                        for i in 0..21 {
                            self.drawtowertile((i * 16), (j * 16) - self.backoffset - 3, temp + 40);
                            self.drawtowertile((i * 16) + 8, (j * 16) - self.backoffset - 3, temp + 41);
                            self.drawtowertile((i * 16), (j * 16) - self.backoffset + 8 - 3, temp + 80);
                            self.drawtowertile((i * 16) + 8, (j * 16) - self.backoffset + 8 - 3, temp + 81);
                        }
                    }
                    self.backgrounddrawn = true;
                }
            },
            5 => {
                //Warp zone, central
                self.backoffset += 1;
                if self.backoffset >= 16 {
                    self.backoffset -= 16;
                    self.warpskip = (self.warpskip + 1) % 2;
                }
            },
            6 => {
                //Final Starfield
                for i in 0..numstars {
                    self.stars[i].w = 2;
                    self.stars[i].h = 2;
                    self.stars[i].y -= self.starsspeed[i];
                    if self.stars[i].y < -10 {
                        self.stars[i].y += 260;
                        self.stars[i].x = maths::fRandom() as i32 * 320;
                        self.starsspeed[i] = 5 + (maths::fRandom() as i32 * 5);
                    }
                }
            },
            _ => (),
        };
    }

    // void Graphics::drawmap(void)
    pub fn drawmap(&mut self, map: &mut map::Map) {
        if !self.foregrounddrawn {
            graphics_util::ClearSurface(&mut self.buffers.foregroundBuffer);

            // println!("drawing map with tileset ({}) and some contents of ", map.tileset);

            if map.tileset == 0 {
                for j in 0..30 {
                    for i in 0..40 {
                        if map.contents[i as usize + map.vmult[j as usize] as usize] as usize > 0 {
                            self.drawforetile(i * 8, j * 8, map.contents[i as usize + map.vmult[j as usize] as usize] as i32);
                        }
                    }
                }
            } else if map.tileset == 1 {
                for jt in 0..30 {
                    for it in 0..40 {
                        if map.contents[it + map.vmult[jt] as usize] > 0 {
                            self.drawforetile2(it as i32 * 8, jt as i32 * 8, map.contents[it as usize + map.vmult[jt as usize] as usize] as i32);
                        }
                    }
                }
            } else if map.tileset == 2 {
                for j in 0..30 {
                    for i in 0..40 {
                        if map.contents[i as usize + map.vmult[j as usize] as usize] > 0 {
                            self.drawforetile3(i * 8, j * 8, map.contents[i as usize + map.vmult[j as usize] as usize] as i32, map.rcol);
                        }
                    }
                }
            }
            self.foregrounddrawn = true;
        }

        self.buffers.foregroundBuffer.blit(None, &mut self.buffers.backBuffer, None);
    }

    // void Graphics::drawfinalmap(void)
    pub fn drawfinalmap(&mut self, map: &mut map::Map) {
        if !self.foregrounddrawn {
            graphics_util::ClearSurface(&mut self.buffers.foregroundBuffer);

            if map.tileset == 0 {
                for j in 0..30 {
                    for i in 0..40 {
                        if (map.contents[i as usize + map.vmult[j as usize] as usize]) > 0 {
                            self.drawforetile(i*8, j*8, map.finalat(i, j));
                        }
                    }
                }
            } else if map.tileset == 1 {
                for j in 0..30 {
                    for i in 0..40 {
                        if (map.contents[i as usize + map.vmult[j as usize] as usize]) > 0 {
                            self.drawforetile2(i*8, j*8, map.finalat(i, j));
                        }
                    }
                }
            }
            self.foregrounddrawn = true;
        }

        self.buffers.foregroundBuffer.blit(None, &mut self.buffers.backBuffer, None);
    }

    // void Graphics::drawtowermap(void)
    pub fn drawtowermap(&mut self) {
        println!("DEADBEEF: Graphics::drawtowermap method not implemented yet");
        // int temp;
        // int yoff = lerp(map.oldypos, map.ypos);
        // for (int j = 0; j < 31; j++) {
        //     for (int i = 0; i < 40; i++) {
        //         temp = map.tower.at(i, j, yoff);
        //         if (temp > 0) drawtile3(i * 8, (j * 8) - (yoff % 8), temp, towerbg.colstate);
        //     }
        // }
    }

    // void Graphics::drawtowerspikes(void)
    pub fn drawtowerspikes(&mut self) {
        println!("DEADBEEF: Graphics::drawtowerspikes method not implemented yet");
    }

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
                    draw_acc.push(self.drawtowertile3(i * 8, (j * 8) - (bypos % 8) - off, back_char, colstate, tiles_rect_width, tiles_rect_height));
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
                    draw_acc.push(self.drawtowertile3(i * 8, 0 - (bypos % 8), back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, 0, bypos);
                    draw_acc.push(self.drawtowertile3(i * 8, -1*8 - (bypos % 8), back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, -1, bypos);
                }
            } else {
                for i in 0..40 {
                    back_char = map.tower.backat(i, 29, bypos);
                    draw_acc.push(self.drawtowertile3(i * 8, 29*8 - (bypos % 8) - bscroll, back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, 30, bypos);
                    draw_acc.push(self.drawtowertile3(i * 8, 30*8 - (bypos % 8) - bscroll, back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, 31, bypos);
                    draw_acc.push(self.drawtowertile3(i * 8, 31*8 - (bypos % 8) - bscroll, back_char, colstate, tiles_rect_width, tiles_rect_height));
                    back_char = map.tower.backat(i, 32, bypos);
                    draw_acc.push(self.drawtowertile3(i * 8, 32*8 - (bypos % 8) - bscroll, back_char, colstate, tiles_rect_width, tiles_rect_height));
                }
            }
        }

        let buffer = match bg {
            BackGround::Title => &mut self.buffers.titlebg_buffer,
            BackGround::Tower => &mut self.buffers.towerbg_buffer,
        };
        for task in draw_acc.iter().rev() {
            if let Some((t, rect)) = task {
                self.grphx.tiles3.surfaces[*t].blit(None, buffer, *rect)
                    .expect("error while rendering towertile3");
            }
        }
    }

    // void Graphics::setcol( int t )
    pub fn setcol(&mut self, t: i32, glow: i32) {
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
    fn menuoffrender(&mut self) {
        println!("DEADBEEF: menuoffrender method not implemented yet");
    }

    // void Graphics::drawhuetile( int x, int y, int t )
    fn drawhuetile(&mut self, x: i32, y: i32, t: usize) {
        if !INBOUNDS_VEC!(t, self.grphx.tiles.surfaces) {
            return;
        }

        let tpoint = maths::point {x, y};
        let rect = sdl2::rect::Rect::new(tpoint.x, tpoint.y, self.tiles_rect.w as u32, self.tiles_rect.h as u32);
        graphics_util::BlitSurfaceColoured(&self.grphx.tiles.surfaces[t], None, &mut self.buffers.backBuffer, rect, self.ct.colour);
    }

    // void Graphics::huetilesetcol(int t)
    pub fn huetilesetcol(&mut self, t: i32) {
        match t {
            0 => self.setcolreal(self.getRGB(250-(maths::fRandom()*32.0) as i32, 250-(maths::fRandom()*32.0) as i32, 10)),
            1 => self.setcolreal(self.getRGB(250-(maths::fRandom()*32.0) as i32, 250-(maths::fRandom()*32.0) as i32, 10)),
            _ => self.setcolreal(self.getRGB(250-(maths::fRandom()*32.0) as i32, 250-(maths::fRandom()*32.0) as i32, 10)),
        };
    }

    // Uint32 Graphics::bigchunkygetcol(int t)
    pub fn bigchunkygetcol(&mut self, t: i32) -> u32 {
        println!("DEADBEEF: bigchunkygetcol method not implemented yet");
        0
    }

    // void Graphics::setwarprect( int a, int b, int c, int d )
    fn setwarprect(&mut self, a: i32, b: i32, c: i32, d: i32) {
        self.warprect.x = a;
        self.warprect.y = b;
        self.warprect.w = c;
        self.warprect.h = d;
    }

    // void Graphics::textboxcenterx(void)
    pub fn textboxcenterx(&mut self) {
        if !INBOUNDS_VEC!(self.m, self.textbox) {
            warn!("textboxcenterx() out-of-bounds!");
            return;
        }

        self.textbox[self.m].centerx();
    }

    // int Graphics::textboxwidth(void)
    pub fn textboxwidth(&mut self) -> i32 {
        if !INBOUNDS_VEC!(self.m, self.textbox) {
            println!("textboxwidth() out-of-bounds!");
            return 0;
        }

        return self.textbox[self.m].w;
    }

    // void Graphics::textboxmoveto(int xo)
    pub fn textboxmoveto(&mut self, xo: i32) {
        if !INBOUNDS_VEC!(self.m, self.textbox) {
            println!("textboxmoveto() out-of-bounds!");
            return;
        }

        self.textbox[self.m].xp = xo;
    }

    // void Graphics::textboxcentery(void)
    pub fn textboxcentery(&mut self) {
        if !INBOUNDS_VEC!(self.m, self.textbox) {
            println!("textboxcentery() out-of-bounds!");
            return;
        }

        self.textbox[self.m].centery();
    }

    // int Graphics::crewcolour(const int t)
    pub fn crewcolour(&mut self, t: i32) -> i32 {
        //given crewmate t, return colour in setcol
        match t {
            0 => 0,
            1 => 20,
            2 => 14,
            3 => 15,
            4 => 13,
            5 => 16,
            _ => 0,
        }
    }

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
    pub fn bigrprint(&mut self, x: i32, y: i32, t: &str, r: i32, g: i32, b: i32, cen: bool, sc: f32) {
        // std::vector<SDL_Surface*>& font = flipmode ? flipbfont : bfont;

        let r = maths::clamp(r, 0, 255);
        let g = maths::clamp(g, 0, 255);
        let b = maths::clamp(b, 0, 255);
        self.ct.colour = self.getRGB(r, g, b);

        let mut x = (x / sc as i32) - Graphics::len(t) as i32;
        x = match cen {
            true => maths::VVV_max(160 - ((t.len() as f32 / 2.0) * sc) as i32, 0 ),
            false => x * sc as i32,
        };

        let mut bfontpos = 0;
        let mut printrect = sdl2::rect::Rect::new(x + bfontpos, y, self.grphx.bfont.rect.w as u32 * sc as u32, self.grphx.bfont.rect.h as u32 * sc as u32);
        for (_, curr) in t.chars().enumerate() {
            let idx = Graphics::font_idx(curr as i32);

            match &self.grphx.bfont.surfaces.get(idx) {
                Some(char_surface) => {
                    let tempPrint = graphics_util::ScaleSurface(char_surface, char_surface.width() * sc as u32, char_surface.height() * sc as u32);
                    printrect.x = x + bfontpos;
                    graphics_util::BlitSurfaceColoured(
                        &tempPrint,
                        None,
                        &mut self.buffers.backBuffer,
                        printrect,
                        self.ct.colour
                    );
                },
                None => (),
            }

            bfontpos += Graphics::bfontlen(curr) * sc as i32;
        }

    }

    // void Graphics::bigbrprint(int x, int y, std::string& s, int r, int g, int b, bool cen, float sc)
    pub fn bigbrprint(&mut self, x: i32, y: i32, s: &str, r: i32, g: i32, b: i32, cen: bool, sc: f32) {
        let sc_i32 = sc as i32;

        if !self.notextoutline {
            self.bigrprint(x, y - sc_i32, s, 0, 0, 0, cen, sc);
            if cen {
                let x_o = maths::VVV_max(160 - (Graphics::len(s) / 2) * sc_i32, 0);
                self.bigprint(x_o - sc_i32, y, s, 0, 0, 0, Some(false), Some(sc_i32));
                self.bigprint(x_o + sc_i32, y, s, 0, 0, 0, Some(false), Some(sc_i32));
            } else {
                let x_o = x / sc_i32 - Graphics::len(s) * sc_i32;
                self.bigprint(x_o - sc_i32, y, s, 0, 0, 0, Some(false), Some(sc_i32));
                self.bigprint(x_o + sc_i32, y, s, 0, 0, 0, Some(false), Some(sc_i32));
            }
            self.bigrprint(x, y + sc_i32, s, 0, 0, 0, cen, sc);
        }

        self.bigrprint(x, y, s, r, g, b, cen, sc);
    }

    // void Graphics::drawtele(int x, int y, int t, Uint32 c)
    fn drawtele(&mut self, x: i32, y: i32, t: usize, c: u32) {
        trace!("drawtele({},{},{},{})", x, y, t, c);

        self.setcolreal(self.getRGB(16, 16, 16));

        let telerect = sdl2::rect::Rect::new(x, y, self.tele_rect.w as u32, self.tele_rect.h as u32);
        if INBOUNDS_VEC!(0, self.grphx.teleporter.surfaces) {
            graphics_util::BlitSurfaceColoured(&self.grphx.teleporter.surfaces[0], None, &mut self.buffers.backBuffer, telerect, self.ct.colour);
        }

        self.setcolreal(c);
        let t = match t {
            t if t > 9 => 8,
            t if t < 1 => 1,
            _ => t,
        };

        let telerect = sdl2::rect::Rect::new(x, y, self.tele_rect.w as u32, self.tele_rect.h as u32);
        if INBOUNDS_VEC!(t, self.grphx.teleporter.surfaces) {
            graphics_util::BlitSurfaceColoured(&self.grphx.teleporter.surfaces[t], None, &mut self.buffers.backBuffer, telerect, self.ct.colour);
        }
    }

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
    pub fn getRGB_AsPixelColor(&self, r: i32, g: i32, b: i32) -> sdl2::pixels::Color {
        unsafe {
            let color = sdl2_sys::SDL_MapRGB(self.buffers.backBuffer.pixel_format().raw(), b as u8, g as u8, r as u8);
            let color = sdl2::pixels::Color::from_u32(&self.buffers.backBuffer.pixel_format(), color);
            color
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
        let r = ( (r+128) / 3) as u8;
        let g = ( (g+128) / 3) as u8;
        let b = ( (b+128) / 3) as u8;

        unsafe {
            // TODO: @sx does this work?
            sdl2_sys::SDL_MapRGB(self.buffers.backBuffer.pixel_format().raw(), r, g, b)
        }
    }

    // void Graphics::setcolreal(Uint32 t)
    fn setcolreal(&mut self, t: u32) {
        self.ct.colour = t;
    }

    // void Graphics::drawforetile(int x, int y, int t)
    fn drawforetile(&mut self, x: i32, y: i32, t: i32) {
        if !INBOUNDS_VEC!(t, self.grphx.tiles.surfaces) {
            WHINE_ONCE!("drawforetile() out-of-bounds!");
            return;
        }

        let rect = sdl2::rect::Rect::new(x, y, self.tiles_rect.w as u32, self.tiles_rect.h as u32);

        // #if !defined(NO_CUSTOM_LEVELS)
        // if shouldrecoloroneway(t, tiles1_mounted))
        // {
        //     colourTransform thect = {ed.getonewaycol()};
        //     BlitSurfaceTinted(tiles[t], NULL, foregroundBuffer, &rect, thect);
        // }
        // else
        // #endif
        self.grphx.tiles.surfaces[t as usize].blit(None, &mut self.buffers.foregroundBuffer, rect);
    }

    // void Graphics::drawforetile2(int x, int y, int t)
    fn drawforetile2(&mut self, x: i32, y: i32, t: i32) {
        if !INBOUNDS_VEC!(t, self.grphx.tiles2.surfaces) {
            WHINE_ONCE!("drawforetile2() out-of-bounds!");
            return;
        }

        let rect = sdl2::rect::Rect::new(x, y, self.tiles_rect.w as u32, self.tiles_rect.h as u32);

        // #if !defined(NO_CUSTOM_LEVELS)
        // if shouldrecoloroneway(t, tiles2_mounted) {
        //     colourTransform thect = {ed.getonewaycol()};
        //     BlitSurfaceTinted(self.grphx.tiles2.surfaces[t as usize], None, self.buffers.foregroundBuffer, &rect, thect);
        // }
        // else
        // #endif
        self.grphx.tiles2.surfaces[t as usize].blit(None, &mut self.buffers.foregroundBuffer, rect);
    }

    // void Graphics::drawforetile3(int x, int y, int t, int off)
    fn drawforetile3(&mut self, x: i32, y: i32, t: i32, off: i32) {
        let mut t = t + off * 30;
        if !INBOUNDS_VEC!(t, self.grphx.tiles3.surfaces) {
            WHINE_ONCE!("drawforetile3() out-of-bounds!");
            return;
        }
        let rect = sdl2::rect::Rect::new(x, y, self.tiles_rect.w as u32, self.tiles_rect.h as u32);
        self.grphx.tiles3.surfaces[t as usize].blit(None, &mut self.buffers.foregroundBuffer, rect);
    }

    // void Graphics::drawrect(int x, int y, int w, int h, int r, int g, int b)
    pub fn drawrect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, g: i32, b: i32) {
        let mut madrect = sdl2::rect::Rect::new(x, y, w as u32, 1);

        //Draw the retangle indicated by that object
        let rgba = self.getRGB(b, g, r);
        graphics_util::FillRect_rect_coloru32(&mut self.buffers.backBuffer, madrect, rgba);

        madrect.w = 1;
        madrect.h = h;
        let rgba = self.getRGB(b, g, r);
        graphics_util::FillRect_rect_coloru32(&mut self.buffers.backBuffer, madrect, rgba);

        madrect.x = x + w - 1;
        madrect.w = 1;
        madrect.h = h;
        let rgba = self.getRGB(b, g, r);
        graphics_util::FillRect_rect_coloru32(&mut self.buffers.backBuffer, madrect, rgba);

        madrect.x = x;
        madrect.y = y + h - 1;
        madrect.w = w;
        madrect.h = 1;
        let rgba = self.getRGB(b, g, r);
        graphics_util::FillRect_rect_coloru32(&mut self.buffers.backBuffer, madrect, rgba);
    }

    // bool Graphics::onscreen(int t)

    // void Graphics::reloadresources(void)
    pub fn reload_resources(&mut self, music: &mut music::Music, fs: &mut filesystem::FileSystem) {
        warn!("DEADBEEF: Graphics::reloadresources not fully implemented yet");

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
        music.init(fs);

        // // #ifndef NO_CUSTOM_LEVELS
        // tiles1_mounted = FILESYSTEM_isAssetMounted("graphics/tiles.png");
        // tiles2_mounted = FILESYSTEM_isAssetMounted("graphics/tiles2.png");
        // minimap_mounted = FILESYSTEM_isAssetMounted("graphics/minimap.png");
        // // #endif
    }

    // Uint32 Graphics::crewcolourreal(int t)

    /* graphics.cpp inline methods */

    // float inline lerp(const float v0, const float v1)
    pub fn lerp(&self, v0: f32, v1: f32) -> f32 {
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
