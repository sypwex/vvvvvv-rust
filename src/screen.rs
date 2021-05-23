extern crate sdl2;
use sdl2::pixels::Color;

use crate::{game, rustutil::dump_surface, scenes::RenderResult};
pub mod render;
pub mod renderfixed;

pub struct Screen {
    pub render: Box<render::Render>,
    pub renderfixed: Box<renderfixed::RenderFixed>,
    // window: Box<sdl2::video::Window>,
    canvas: Box<sdl2::render::Canvas<sdl2::video::Window>>,
    m_screen: Box<sdl2::surface::Surface<'static>>,

    badSignalEffect: bool,
}

impl Screen {
    pub fn new (sdl_context: &sdl2::Sdl) -> Screen {
        // m_window = NULL;
        // m_renderer = NULL;
        // m_screenTexture = NULL;
        // m_screen = NULL;
        // isWindowed = !fullscreen;
        // stretchMode = stretch;
        // isFiltered = linearFilter;
        // vsync = useVsync;
        // filterSubrect.x = 1;
        // filterSubrect.y = 1;
        // filterSubrect.w = 318;
        // filterSubrect.h = 238;

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

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("VVVVVV on Rust", 640, 480)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        // LoadIcon();

        // // FIXME: This surface should be the actual backbuffer! -flibit
        // m_screen = SDL_CreateRGBSurface(
        //     0,
        //     320,
        //     240,
        //     32,
        //     0x00FF0000,
        //     0x0000FF00,
        //     0x000000FF,
        //     0xFF000000
        // );

        let m_screen = sdl2::surface::Surface::new(320, 240, sdl2::pixels::PixelFormatEnum::RGBX8888).unwrap();

        // // ALSO FIXME: This SDL_CreateTexture() is duplicated in Graphics::processVsync()!
        // m_screenTexture = SDL_CreateTexture(
        //     m_renderer,
        //     SDL_PIXELFORMAT_ARGB8888,
        //     SDL_TEXTUREACCESS_STREAMING,
        //     320,
        //     240
        // );

        // badSignalEffect = badSignal;

        // ResizeScreen(windowWidth, windowHeight);

        Screen {
            render: Box::new(render::Render::new(m_screen.pixel_format_enum())),
            renderfixed: Box::new(renderfixed::RenderFixed::new()),
            // window: Box::new(window),
            canvas: Box::new(canvas),
            m_screen: Box::new(m_screen),

            badSignalEffect: false,
        }
    }

    pub fn init (
        // int windowWidth,
        // int windowHeight,
        // bool fullscreen,
        // bool useVsync,
        // int stretch,
        // bool linearFilter,
        // bool badSignal
    ) {

    }

    pub fn init_canvas (&mut self) {
        self.canvas.set_draw_color(Color::RGB(128, 128, 128));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn present_canvas (&mut self) {
        self.canvas.present();
    }

    // void Screen::UpdateScreen(SDL_Surface* buffer, SDL_Rect* rect )
    fn update_screen(&mut self) {
        let rect_src = self.render.get_render_rect();
        let surf_src = &self.render.graphics.buffers.backBuffer;

        // if (buffer == NULL) && (m_screen == NULL) {
        //     return;
        // }

        if self.badSignalEffect {
            // buffer = ApplyFilter(buffer);
        }

        // ClearSurface(m_screen);
        // FillRect(m_screen, 0x000);
        let rect_dst = sdl2::rect::Rect::new(0, 0, self.m_screen.width(), self.m_screen.height());
        match self.m_screen.fill_rect(rect_dst, sdl2::pixels::Color::BLACK) {
            Ok(_x) => (),
            Err(s) => panic!("{}", s),
        };

        // BlitSurfaceStandard(buffer, NULL, m_screen, rect);
        match (*surf_src).blit(rect_src, &mut self.m_screen, rect_dst) {
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
        self.present_canvas();

        if self.badSignalEffect {
            // SDL_FreeSurface(buffer);
        }

        // dump_surface(surf_src, "backbuffer", "");
    }

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
        // TODO @sx @impl
        println!("DEADBEEF: Graphics::screenshake method not implemented yet");

        // if self.flipmode {
        //     // SDL_Rect shakeRect;
        //     // setRect(shakeRect,screenshake_x, screenshake_y, backBuffer->w, backBuffer->h);
        //     // let flipBackBuffer = self.FlipSurfaceVerticle(backBuffer);
        //     // screenbuffer.UpdateScreen(flipBackBuffer, &shakeRect);
        //     // drop(flipBackBuffer);
        // } else {
        //     // SDL_Rect shakeRect;
        //     // setRect(shakeRect,screenshake_x, screenshake_y, backBuffer.w, backBuffer.h);
        //     // screenbuffer.UpdateScreen(backBuffer, &shakeRect);
        // }

        // graphics_util::ClearSurface(self.buffers.backBuffer.as_mut());

        self.update_screen(); // TODO @sx: use upper code
    }

    // void Graphics::updatescreenshake(void)

    // void Graphics::render(void)
    fn render(&mut self) {
        // let rect = sdl2::rect::Rect::new(0, 0, self.buffers.backBuffer.width(), self.buffers.backBuffer.height());
        // if self.flipmode {
        //     // let tempsurface = graphics_util::FlipSurfaceVerticle(buffers.backBuffer);
        //     // screenbuffer.update_screen(tempsurface, &rect);
        //     // drop(tempsurface);
        // } else {
        //     screenbuffer.update_screen(buffers.backBuffer, &rect);
        // }

        self.update_screen(); // TODO @sx: use upper code
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
