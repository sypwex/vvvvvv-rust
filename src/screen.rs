extern crate sdl2;
use sdl2::pixels::Color;

use crate::{game, scenes::RenderResult};

use self::render::graphics::graphics_util;
pub mod render;
pub mod renderfixed;

pub struct ScreenSettings {
    pub windowWidth: i32,
	pub windowHeight: i32,
	pub fullscreen: bool,
	pub useVsync: bool,
	pub stretch: i32,
	pub linearFilter: bool,
	pub badSignal: bool,
}

impl ScreenSettings {
    pub fn new() -> Self {
        Self {
            windowWidth: 320,
            windowHeight: 240,
            fullscreen: false,
            useVsync: false,
            stretch: 0,
            linearFilter: false,
            badSignal: false,
        }
    }
}

// class that known as screenbuffer in graphics.cpp
pub struct Screen {
    pub render: Box<render::Render>,
    pub renderfixed: Box<renderfixed::RenderFixed>,
    canvas: Box<sdl2::render::Canvas<sdl2::video::Window>>,

    /* Screen.h */

	pub isWindowed: bool,
	isFiltered: bool,
    pub badSignalEffect: bool,
	stretchMode: i32,
	vsync: bool,

	m_window: Box<sdl2::video::Window>,
	// m_renderer: &SDL_Renderer,
	// m_screenTexture: &sdl2::render::Texture,
    m_screen: Box<sdl2::surface::Surface<'static>>,

	filterSubrect: sdl2::rect::Rect,
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Screen {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("VVVVVV on Rust", 640, 480)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        /* @sx: moved here from Screen::init */

        // SDL_SetHintWithPriority(
        //     SDL_HINT_RENDER_SCALE_QUALITY,
        //     isFiltered ? "linear" : "nearest",
        //     SDL_HINT_OVERRIDE
        // );
        // SDL_SetHintWithPriority(
        //     SDL_HINT_RENDER_VSYNC,
        //     vsync ? "1" : "0",
        //     SDL_HINT_OVERRIDE
        // );

        // // Uncomment this next line when you need to debug -flibit
        // // SDL_SetHintWithPriority(SDL_HINT_RENDER_DRIVER, "software", SDL_HINT_OVERRIDE);
        // // FIXME: m_renderer is also created in Graphics::processVsync()!
        // SDL_CreateWindowAndRenderer(
        //     640,
        //     480,
        //     SDL_WINDOW_HIDDEN | SDL_WINDOW_RESIZABLE,
        //     &m_window,
        //     &m_renderer
        // );
        // SDL_SetWindowTitle(m_window, "VVVVVV");
        let m_window = Box::new(window);

        // LoadIcon();

        // // ALSO FIXME: This SDL_CreateTexture() is duplicated in Graphics::processVsync()!
        // m_screenTexture = SDL_CreateTexture(
        //     m_renderer,
        //     SDL_PIXELFORMAT_ARGB8888,
        //     SDL_TEXTUREACCESS_STREAMING,
        //     320,
        //     240
        // );
        // let m_screenTexture = sdl2::render::Texture::(320, 240, sdl2::pixels::PixelFormatEnum::RGBX8888).unwrap();

        //FIXME: This surface should be the actual backbuffer! -flibit
        let m_screen = sdl2::surface::Surface::new(320, 240, sdl2::pixels::PixelFormatEnum::RGBX8888).unwrap();

        Screen {
            render: Box::new(render::Render::new(m_screen.pixel_format_enum())),
            renderfixed: Box::new(renderfixed::RenderFixed::new()),
            canvas: Box::new(canvas),

            /* Screen.h */

            m_window: Box::new(window),
            // m_renderer: &SDL_Renderer,
            // m_screenTexture: &sdl2::render::Texture,
            m_screen: Box::new(m_screen),

            isWindowed: false,
            stretchMode: 0,
            isFiltered: false,
            badSignalEffect: false,
            vsync: false,
            filterSubrect: sdl2::rect::Rect::new(1, 1, 318, 238),
        }
    }

    // void Screen::init(const ScreenSettings& settings)
    pub fn init (&self, settings: &ScreenSettings) {
        self.isWindowed = !settings.fullscreen;
        self.stretchMode = settings.stretch;
        self.isFiltered = settings.linearFilter;
        self.vsync = settings.useVsync;
        self.badSignalEffect = settings.badSignal;
        self.ResizeScreen(settings.windowWidth, settings.windowHeight);
    }

    // void Screen::destroy(void)
    // void Screen::GetSettings(ScreenSettings* settings)
    // void Screen::LoadIcon(void)
    // void Screen::ResizeScreen(int x, int y)
    fn ResizeScreen(&mut self, x: i32, y: i32) {
        let (resX, resY) = match x != -1 && y != -1 {
            True => (x, y), // This is a user resize!
            False => (320, 240),
        };

        println!("STUB CODE START HERE");

        unsafe {
            if !self.isWindowed {
                if sdl2_sys::SDL_SetWindowFullscreen(self.m_window, SDL_WINDOW_FULLSCREEN_DESKTOP) != 0 {
                    println!("Error: could not set the game to fullscreen mode: {:?}", sdl2_sys::SDL_GetError());
                    return;
                }
            } else {
                if sdl2_sys::SDL_SetWindowFullscreen(self.m_window, 0) != 0 {
                    println!("Error: could not set the game to windowed mode: {:?}", sdl2_sys::SDL_GetError());
                    return;
                }
                if x != -1 && y != -1 {
                    sdl2_sys::SDL_SetWindowSize(self.m_window, resX, resY);
                    sdl2_sys::SDL_SetWindowPosition(self.m_window, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED);
                }
            }
            if self.stretchMode == 1 {
                let mut winX: i32;
                let mut winY: i32;
                sdl2_sys::SDL_GetWindowSize(&winX, &winY);
                if result = sdl2_sys::SDL_RenderSetLogicalSize(m_renderer, winX, winY) != 0 {
                    println!("Error: could not set logical size: {:?}", sdl2_sys::SDL_GetError());
                    return;
                }
                if sdl2_sys::SDL_RenderSetIntegerScale(m_renderer, SDL_FALSE) != 0 {
                    println!("Error: could not set scale: {:?}", sdl2_sys::SDL_GetError());
                    return;
                }
            } else {
                sdl2_sys::SDL_RenderSetLogicalSize(m_renderer, 320, 240);
                int result = SDL_RenderSetIntegerScale(m_renderer, (SDL_bool) (stretchMode == 2));
                if result != 0 {
                    println!("Error: could not set scale: {:?}", SDL_GetError());
                    return;
                }
            }

            sdl2_sys::SDL_ShowWindow(m_window);
        }
    }

    // void Screen::ResizeToNearestMultiple(void)
    // void Screen::GetWindowSize(int* x, int* y)
    // void Screen::UpdateScreen(SDL_Surface* buffer, SDL_Rect* rect )
    fn update_screen(&mut self, rect: sdl2::rect::Rect) {
        // TODO: refactor
        let buffer;
        let flip_buffer = &graphics_util::FlipSurfaceVerticle(&self.render.graphics.buffers.backBuffer);
        if !self.render.graphics.flipmode {
            buffer = &self.render.graphics.buffers.backBuffer;
        } else {
            buffer = flip_buffer;
        }

        // if self.badSignalEffect {
        //     buffer = &self.render.graphics.ApplyFilter(buffer);
        // }

        // ClearSurface(m_screen);
        // FillRect(m_screen, 0x000);
        let rect_dst = sdl2::rect::Rect::new(0, 0, self.m_screen.width(), self.m_screen.height());
        match self.m_screen.fill_rect(rect_dst, sdl2::pixels::Color::BLACK) {
            Ok(_x) => (),
            Err(s) => panic!("{}", s),
        };

        // BlitSurfaceStandard(buffer, NULL, m_screen, rect);
        match buffer.blit(rect, &mut self.m_screen, rect_dst) {
            Ok(_x) => (),
            Err(s) => panic!("{}", s),
        };

        let texture_creator = self.canvas.texture_creator();
        match self.m_screen.as_texture(&texture_creator) {
            Ok(texture) => {
                self.canvas.copy(&texture, None, None);
            },
            Err(s) => panic!("{}", s),
        }
        self.canvas.present();

        if self.badSignalEffect {
            // SDL_FreeSurface(buffer);
            drop(buffer);
        }
    }

    // const SDL_PixelFormat* Screen::GetFormat(void)
    // void Screen::FlipScreen(void)
    // void Screen::toggleFullScreen(void)
    // void Screen::toggleStretchMode(void)
    // void Screen::toggleLinearFilter(void)
    // void Screen::resetRendererWorkaround(void)

    /* moved here from Graohics.cpp */

    pub fn do_screen_render(&mut self, render_result: RenderResult, game: &mut game::Game) {
        match render_result {
            RenderResult::MenuOffRender => {
                0;
            }
            RenderResult::Screenshake => {
                self.screenshake();
            }
            RenderResult::Plain => {
                self.render();
            }
            RenderResult::WithScreenEffects => {
                self.render_with_screen_effects(game);
            }
            _ => {}
        }
    }

    // void Graphics::flashlight(void)
    fn flashlight(&mut self) {
        // FillRect(backBuffer, 0xBBBBBBBB);
        let color = sdl2::pixels::Color::from_u32(&self.render.graphics.buffers.backBuffer.pixel_format(), 0xBBBBBBBB);
        self.render.graphics.buffers.fill_back_buffer_with_color(color);
    }

    // void Graphics::screenshake(void)
    fn screenshake(&mut self) {
        let screenshake_x = self.render.graphics.screenshake_x;
        let screenshake_y = self.render.graphics.screenshake_y;
        let width = self.render.graphics.buffers.backBuffer.width();
        let height = self.render.graphics.buffers.backBuffer.height();

        // SDL_Rect shakeRect;
        // setRect(shakeRect,screenshake_x, screenshake_y, backBuffer->w, backBuffer->h);
        let shakeRect = sdl2::rect::Rect::new(screenshake_x, screenshake_y, width, height);

        if self.render.graphics.flipmode {
            // TODO: @sx
            // SDL_Surface* flipBackBuffer = FlipSurfaceVerticle(backBuffer);
            // screenbuffer->UpdateScreen( flipBackBuffer, &shakeRect);
            // SDL_FreeSurface(flipBackBuffer);
        } else {
            // SDL_Rect shakeRect;
            // setRect(shakeRect,screenshake_x, screenshake_y, backBuffer->w, backBuffer->h);
            // screenbuffer->UpdateScreen( backBuffer, &shakeRect);
            self.update_screen(shakeRect);
            // TODO: @sx really needed?
            graphics_util::ClearSurface(self.render.graphics.buffers.backBuffer.as_mut());
        }
    }

    // void Graphics::render(void)
    fn render(&mut self) {
        let rect = sdl2::rect::Rect::new(0, 0, self.render.graphics.buffers.backBuffer.width(), self.render.graphics.buffers.backBuffer.height());
        self.update_screen(rect);
    }

    // void Graphics::renderwithscreeneffects(void)
    fn render_with_screen_effects(&mut self, game: &mut game::Game) {
        if game.flashlight > 0 && !game.noflashingmode {
            self.flashlight();
        }

        if game.screenshake > 0 && !game.noflashingmode {
            self.screenshake();
        } else {
            self.render();
        }
    }

    /* */

    pub fn init_canvas (&mut self) {
        self.canvas.set_draw_color(Color::RGB(128, 128, 128));
        self.canvas.clear();
        self.canvas.present();
    }
}
