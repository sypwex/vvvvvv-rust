extern crate sdl2;
use sdl2_sys::{SDL_WindowFlags, SDL_bool};

use crate::{game, scenes::RenderResult};
use self::render::graphics::graphics_util;

pub mod render;
pub mod renderfixed;

const SCREEN_PIXEL_FORMAT: sdl2::pixels::PixelFormatEnum = sdl2::pixels::PixelFormatEnum::RGBX8888;
const TEXTURE_PIXEL_FORMAT: sdl2::pixels::PixelFormatEnum = sdl2::pixels::PixelFormatEnum::RGBX8888;

#[derive(Debug, Clone, Copy)]
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
    pub screen_settings: ScreenSettings,
    pub render: Box<render::Render>,
    pub renderfixed: Box<renderfixed::RenderFixed>,
    // canvas: Box<sdl2::render::Canvas<sdl2::video::Window>>,
    // texture_creator: Box<TextureCreator<WindowContext>>,

    /* Screen.h */

    pub isWindowed: bool,
    pub isFiltered: bool,
    pub badSignalEffect: bool,
    pub stretchMode: i32,
    pub vsync: bool,

    m_window: *mut sdl2_sys::SDL_Window,
    m_renderer: *mut sdl2_sys::SDL_Renderer,
    m_screenTexture: *mut sdl2_sys::SDL_Texture,
    m_screen: sdl2::surface::Surface<'static>,

    filterSubrect: sdl2::rect::Rect,
}

#[derive(Debug,Copy,Clone)]
pub struct ScreenParams {
    pub isWindowed: bool,
    pub isFiltered: bool,
    pub badSignalEffect: bool,
    pub stretchMode: i32,
    pub vsync: bool,
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Screen {
        // @sx stub code for using SDL video subsystem
        // let video_subsystem = sdl_context.video().unwrap();
        // let m_window = video_subsystem.window("VVVVVV/Rust", 640, 480)
        //     .position_centered()
        //     .build()
        //     .unwrap();
        // let canvas = m_window.into_canvas().build().unwrap();
        // let texture_creator = Box::new(canvas.texture_creator());
        // let m_screenTexture = texture_creator.create_texture_streaming(TEXTURE_PIXEL_FORMAT, 320, 240).unwrap();

        /* */

        let m_window = unsafe { sdl2_sys::SDL_CreateWindow("".as_ptr() as *const libc::c_char, 0, 0, 100, 100, 0) };
        let m_renderer = unsafe { sdl2_sys::SDL_CreateRenderer(m_window, 0, 0) };
        let m_screen = sdl2::surface::Surface::new(1, 1, SCREEN_PIXEL_FORMAT).unwrap();
        let m_screenTexture = unsafe { sdl2_sys::SDL_CreateTextureFromSurface(m_renderer, m_screen.raw()) };

        Screen {
            screen_settings: ScreenSettings::new(),
            render: Box::new(render::Render::new(SCREEN_PIXEL_FORMAT)),
            renderfixed: Box::new(renderfixed::RenderFixed::new()),
            // canvas: Box::new(canvas),
            // texture_creator,

            /* Screen.h */

            m_window,
            m_renderer,
            m_screenTexture,
            m_screen,

            isWindowed: false,
            stretchMode: 0,
            isFiltered: false,
            badSignalEffect: false,
            vsync: false,
            filterSubrect: sdl2::rect::Rect::new(1, 1, 318, 238),
        }
    }

    pub fn get_screen_params(&self) -> ScreenParams {
        ScreenParams {
            isWindowed: self.isWindowed,
            isFiltered: self.isFiltered,
            badSignalEffect: self.badSignalEffect,
            stretchMode: self.stretchMode,
            vsync: self.vsync,
        }
    }

    // void Screen::init(const ScreenSettings& settings)
    pub fn init (&mut self, settings: ScreenSettings) {
        self.isWindowed = !settings.fullscreen;
        self.stretchMode = settings.stretch;
        self.isFiltered = settings.linearFilter;
        self.vsync = settings.useVsync;

        unsafe {
            sdl2_sys::SDL_SetHintWithPriority(
                sdl2_sys::SDL_HINT_RENDER_SCALE_QUALITY.as_ptr() as *const libc::c_char,
                (if self.isFiltered { "linear" } else { "nearest" }).as_ptr() as *const libc::c_char,
                sdl2_sys::SDL_HintPriority::SDL_HINT_OVERRIDE
            );
            sdl2_sys::SDL_SetHintWithPriority(
                sdl2_sys::SDL_HINT_RENDER_VSYNC.as_ptr() as *const libc::c_char,
                (if self.vsync { "1" } else { "0" }).as_ptr() as *const libc::c_char,
                sdl2_sys::SDL_HintPriority::SDL_HINT_OVERRIDE
            );

            //Uncomment this next line when you need to debug -flibit
            // SDL_SetHintWithPriority(SDL_HINT_RENDER_DRIVER, "software", SDL_HINT_OVERRIDE);
            //FIXME: m_renderer is also created in Graphics::processVsync()!
            sdl2_sys::SDL_CreateWindowAndRenderer(
                640,
                480,
                SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32 | SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32 | SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32,
                as_mut_ptr!(self.m_window),
                as_mut_ptr!(self.m_renderer),
            );
            sdl2_sys::SDL_SetWindowTitle(self.m_window, "VVVVVV/Rust".as_ptr() as *const libc::c_char);

            self.LoadIcon();

            //FIXME: This surface should be the actual backbuffer! -flibit
            self.m_screen = sdl2::surface::Surface::new(320, 240, SCREEN_PIXEL_FORMAT).unwrap();

            //ALSO FIXME: This SDL_CreateTexture() is duplicated in Graphics::processVsync()!
            // m_screenTexture = SDL_CreateTexture(m_renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STREAMING, 320, 240);
            self.m_screenTexture = sdl2_sys::SDL_CreateTexture(self.m_renderer, TEXTURE_PIXEL_FORMAT as u32, sdl2::render::TextureAccess::Streaming as i32, 320, 240);

        }

        self.badSignalEffect = settings.badSignal;

        self.ResizeScreen(settings.windowWidth, settings.windowHeight);
    }

    // void Screen::destroy(void)
    // void Screen::GetSettings(ScreenSettings* settings)

    // void Screen::LoadIcon(void)
    fn LoadIcon(&mut self) {
        // #ifndef __APPLE__
        // unsigned char *fileIn;
        // size_t length;
        // unsigned char *data;
        // unsigned int width, height;
        // FILESYSTEM_loadAssetToMemory("VVVVVV.png", &fileIn, &length, false);
        // lodepng_decode24(&data, &width, &height, fileIn, length);
        // FILESYSTEM_freeMemory(&fileIn);
        // SDL_Surface *icon = SDL_CreateRGBSurfaceFrom(
        //     data,
        //     width,
        //     height,
        //     24,
        //     width * 3,
        //     0x000000FF,
        //     0x0000FF00,
        //     0x00FF0000,
        //     0x00000000
        // );
        // SDL_SetWindowIcon(m_window, icon);
        // SDL_FreeSurface(icon);
        // SDL_free(data);
        // #endif /* __APPLE__ */
    }

    // void Screen::ResizeScreen(int x, int y)
    pub fn ResizeScreen(&mut self, x: i32, y: i32) {
        trace!("ResizeScreen({}, {})", x, y);

        let (resX, resY) = match x != -1 && y != -1 {
            true => (x, y), //This is a user resize!
            false => (320, 240),
        };

        unsafe {
            if !self.isWindowed {
                if sdl2_sys::SDL_SetWindowFullscreen(self.m_window, SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32) != 0 {
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
                    sdl2_sys::SDL_SetWindowPosition(self.m_window, sdl2_sys::SDL_WINDOWPOS_CENTERED_MASK as i32, sdl2_sys::SDL_WINDOWPOS_CENTERED_MASK as i32);
                }
            }

            if self.stretchMode == 1 {
                let mut winX: libc::c_int = 0;
                let mut winY: libc::c_int = 0;
                sdl2_sys::SDL_GetWindowSize(self.m_window, &mut winX, &mut winY);
                if sdl2_sys::SDL_RenderSetLogicalSize(self.m_renderer, winX, winY) != 0 {
                    println!("Error: could not set logical size: {:?}", sdl2_sys::SDL_GetError());
                    return;
                }
                if sdl2_sys::SDL_RenderSetIntegerScale(self.m_renderer, SDL_bool::SDL_FALSE) != 0 {
                    println!("Error: could not set scale: {:?}", sdl2_sys::SDL_GetError());
                    return;
                }
            } else {
                let enable = match self.stretchMode == 2 {
                    True => SDL_bool::SDL_TRUE,
                    False => SDL_bool::SDL_FALSE,
                };

                sdl2_sys::SDL_RenderSetLogicalSize(self.m_renderer, 320, 240);
                if sdl2_sys::SDL_RenderSetIntegerScale(self.m_renderer, enable) != 0 {
                    println!("Error: could not set scale: {:?}", sdl2_sys::SDL_GetError());
                    return;
                }
            }

            sdl2_sys::SDL_ShowWindow(self.m_window);
        }
    }

    // void Screen::ResizeToNearestMultiple(void)
    pub fn ResizeToNearestMultiple(&mut self) {
        println!("DEADBEEF: Screen::ResizeToNearestMultiple method not implemented yet");
    }

    // void Screen::GetWindowSize(int* x, int* y)
    // void Screen::UpdateScreen(SDL_Surface* buffer, SDL_Rect* rect )
    fn update_screen(&mut self, rect: sdl2::rect::Rect) {
        // TODO: refactor
        let flip_buffer = &graphics_util::FlipSurfaceVerticle(&self.render.graphics.buffers.backBuffer);
        let buffer = if !self.render.graphics.flipmode {
            &self.render.graphics.buffers.backBuffer
        } else {
            flip_buffer
        };

        // if self.badSignalEffect {
        //     buffer = &self.render.graphics.ApplyFilter(buffer);
        // }

        // FillRect(m_screen, 0x000);
        graphics_util::ClearSurface(&mut self.m_screen);

        // BlitSurfaceStandard(buffer, NULL, m_screen, rect);
        let rect_dst = sdl2::rect::Rect::new(0, 0, self.m_screen.width(), self.m_screen.height());
        if let Err(s) = buffer.blit(rect, &mut self.m_screen, rect_dst) {
            error!("unable to render to screen buffer: {}", s);
        };

        if self.badSignalEffect {
            // SDL_FreeSurface(buffer);
            drop(buffer);
        }
    }

    // const SDL_PixelFormat* Screen::GetFormat(void)
    // void Screen::FlipScreen(void)
    pub fn FlipScreen(&mut self) {
        // @sx canvas/texture_creator stub
        // let texture_creator = self.canvas.texture_creator();
        // match self.m_screen.as_texture(&texture_creator) {
        //     Ok(texture) => {
        //         self.canvas.copy(&texture, None, None);
        //     },
        //     Err(s) => panic!("{}", s),
        // }

        unsafe {
            // SDL_UpdateTexture(
            //     m_screenTexture,
            //     NULL,
            //     m_screen->pixels,
            //     m_screen->pitch
            // );
            // SDL_RenderCopy(
            //     m_renderer,
            //     m_screenTexture,
            //     isFiltered ? &filterSubrect : NULL,
            //     NULL
            // );
            sdl2_sys::SDL_UpdateTexture(
                self.m_screenTexture,
                std::ptr::null(),
                (*self.m_screen.raw()).pixels,
                self.m_screen.pitch() as i32
            );
            // let texture = sdl2_sys::SDL_CreateTextureFromSurface(self.m_renderer, self.m_screen.raw());
            if self.isFiltered {
                sdl2_sys::SDL_RenderCopy(self.m_renderer, self.m_screenTexture, self.filterSubrect.raw(), std::ptr::null());
            } else {
                sdl2_sys::SDL_RenderCopy(self.m_renderer, self.m_screenTexture, std::ptr::null(), std::ptr::null());
            }

            // SDL_RenderPresent(m_renderer);
            // SDL_RenderClear(m_renderer);
            // ClearSurface(m_screen);
            sdl2_sys::SDL_RenderPresent(self.m_renderer);
            sdl2_sys::SDL_RenderClear(self.m_renderer);
            graphics_util::ClearSurface(&mut self.m_screen);
        }
    }

    // void Screen::toggleFullScreen(void)
    pub fn toggleFullScreen(&mut self) {
        self.isWindowed = !self.isWindowed;
        self.ResizeScreen(-1, -1);
    }

    // void Screen::toggleStretchMode(void)
    pub fn toggleStretchMode(&mut self) {
        self.stretchMode = (self.stretchMode + 1) % 3;
        self.ResizeScreen(-1, -1);
    }

    // void Screen::toggleLinearFilter(void)
    pub fn toggleLinearFilter(&mut self) {
        println!("DEADBEEF: Screen::toggleLinearFilter method not implemented yet");
    }

    // void Screen::resetRendererWorkaround(void)
    pub fn resetRendererWorkaround(&mut self) {
        println!("DEADBEEF: Screen::resetRendererWorkaround method not implemented yet");
    }

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
}
