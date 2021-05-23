use sdl2::keyboard::Keycode;

use crate::{game::{self, GameState}, key_poll};
use crate::screen::render::graphics;
use crate::scenes::{InputTrait, RenderFixedTrait, RenderTrait};

use super::RenderResult;

pub struct Preloader {
    pre_fakepercent: i32,
    pre_transition: i32,
    pre_startgame: bool,
    pre_darkcol: sdl2::pixels::Color,
    pre_lightcol: sdl2::pixels::Color,
    pre_curcol: i32,
    pre_coltimer: i32,
    pre_offset: i32,

    pre_frontrectx: i32,
    pre_frontrecty: i32,
    pre_frontrectw: i32,
    pre_frontrecth: i32,
    pre_temprectx: i32,
    pre_temprecty: i32,
    pre_temprectw: i32,
    pre_temprecth: i32,
}

impl Preloader {
    pub fn new() -> Preloader {
        Preloader {
            pre_fakepercent: 0,
            pre_transition: 30,
            pre_startgame: false,
            pre_darkcol: sdl2::pixels::Color::BLACK,
            pre_lightcol: sdl2::pixels::Color::BLACK,
            pre_curcol: 0,
            pre_coltimer: 0,
            pre_offset: 0,

            pre_frontrectx: 30,
            pre_frontrecty: 20,
            pre_frontrectw: 260,
            pre_frontrecth: 200,
            pre_temprectx: 0,
            pre_temprecty: 0,
            pre_temprectw: 320,
            pre_temprecth: 240,
        }
    }
}

impl InputTrait for Preloader {
    // void preloaderinput(void)
    fn input(&mut self, game: &mut game::Game, key_poll: &mut key_poll::KeyPoll) -> Option<RenderResult> {
        game.press_action = false;

        if key_poll.isDownKeycode(Keycode::Z) || key_poll.isDownKeycode(Keycode::Space) ||
           key_poll.isDownKeycode(Keycode::V) || key_poll.isDownVec(&game.controllerButton_flip) {
            game.press_action = true;
        }

        if game.press_action {
            // Skip to TITLEMODE immediately
            game.gamestate = GameState::TITLEMODE;
            game.jumpheld = true;
        }

        None
    }
}

impl RenderFixedTrait for Preloader {
    // void preloaderrenderfixed(void)
    fn render_fixed(&mut self, game: &mut game::Game) -> Option<RenderResult> {
        if self.pre_transition < 30 {
            self.pre_transition -= 1;
        }

        if self.pre_transition >= 30 {
            self.pre_fakepercent += 1;
            if self.pre_fakepercent >= 100 {
                self.pre_fakepercent = 100;
                self.pre_startgame = true;
            }

            // self.pre_offset = (self.pre_offset + 4 + int(fRandom() * 5.0f32))%32;
            self.pre_offset = (self.pre_offset + 4 + 5) % 32;
            self.pre_coltimer -= 1;
            if self.pre_coltimer <= 0 {
                // self.pre_curcol = (self.pre_curcol + int(fRandom() * 5.0f32)) % 6;
                self.pre_curcol = (self.pre_curcol + 5) % 6;
                self.pre_coltimer = 8;
            }
        }

        if self.pre_transition <= -10 {
            game.gamestate = GameState::TITLEMODE;
        }

        None
    }
}

impl RenderTrait for Preloader {
    fn render(&mut self, graphics: &mut graphics::Graphics) -> Option<RenderResult> {

        if self.pre_transition >= 30 {
            match self.pre_curcol {
                0 => {
                    self.pre_lightcol = graphics.RGBflip_AsPixelColor(0xBF,0x59,0x6F);
                    self.pre_darkcol = graphics.RGBflip_AsPixelColor(0x88,0x3E,0x53);
                },
                1 => {
                    self.pre_lightcol = graphics.RGBflip_AsPixelColor(0x6C,0xBC,0x5C);
                    self.pre_darkcol = graphics.RGBflip_AsPixelColor(0x50,0x86,0x40);
                },
                2 => {
                    self.pre_lightcol = graphics.RGBflip_AsPixelColor(0x5D,0x57,0xAA);
                    self.pre_darkcol = graphics.RGBflip_AsPixelColor(0x2F,0x2F,0x6C);
                },
                3 => {
                    self.pre_lightcol = graphics.RGBflip_AsPixelColor(0xB7,0xBA,0x5E);
                    self.pre_darkcol = graphics.RGBflip_AsPixelColor(0x84,0x83,0x42);
                },
                4 => {
                    self.pre_lightcol = graphics.RGBflip_AsPixelColor(0x57,0x90,0xAA);
                    self.pre_darkcol = graphics.RGBflip_AsPixelColor(0x2F,0x5B,0x6C);
                },
                5 => {
                    self.pre_lightcol = graphics.RGBflip_AsPixelColor(0x90,0x61,0xB1);
                    self.pre_darkcol = graphics.RGBflip_AsPixelColor(0x58,0x3D,0x71);
                },
                _ => {
                    self.pre_lightcol = graphics.RGBflip_AsPixelColor(0x00,0x00,0x00);
                    self.pre_darkcol = graphics.RGBflip_AsPixelColor(0x08,0x00,0x00);
                },
            }

            let mut i = 0;
            while i < 18 {
                self.pre_temprecty = (i * 16) - self.pre_offset;
                if i % 2 == 0 {
                    // FillRect(graphics.buffers.backBuffer, self.pre_temprectx, self.pre_temprecty, self.pre_temprectw, self.pre_temprecth, self.pre_lightcol);
                    graphics.buffers.fill_back_buffer_with_color_at_xy(self.pre_temprectx as u32, self.pre_temprecty as u32, self.pre_temprectw as u32, self.pre_temprecth as u32, self.pre_lightcol);
                } else {
                    // FillRect(graphics.buffers.backBuffer, self.pre_temprectx, self.pre_temprecty, self.pre_temprectw, self.pre_temprecth, self.pre_darkcol);
                    graphics.buffers.fill_back_buffer_with_color_at_xy(self.pre_temprectx as u32, self.pre_temprecty as u32, self.pre_temprectw as u32, self.pre_temprecth as u32, self.pre_darkcol);
                }
                i += 1;
            }

            graphics.buffers.fill_back_buffer_with_color_at_xy(self.pre_frontrectx as u32, self.pre_frontrecty as u32, self.pre_frontrectw as u32, self.pre_frontrecth as u32, graphics.getBGR_AsPixelColor(0x3E,0x31,0xA2));

            if self.pre_fakepercent == 100 {
                // graphics.Print(282-(15*8), 204, "LOADING... " + help.String(int(self.pre_fakepercent))+"%", 124, 112, 218, false);
                graphics.print(282-(15*8), 204, ["LOADING... ", &self.pre_fakepercent.to_string(), "%"].concat(), 124, 112, 218, false);
            } else {
                // graphics.Print(282-(14*8), 204, "LOADING... " + help.String(int(self.pre_fakepercent))+"%", 124, 112, 218, false);
                graphics.print(282-(14*8), 204, ["LOADING... ", &self.pre_fakepercent.to_string(), "%"].concat(), 124, 112, 218, false);
            }

            // Render
            if self.pre_startgame {
                self.pre_transition = 29;
            }
        } else if self.pre_transition <= -10 {
            // Switch to TITLEMODE (handled by preloaderrenderfixed)
        } else if self.pre_transition < 5 {
            graphics.buffers.clear_back_buffer();
        } else if self.pre_transition < 20 {
            self.pre_temprecty = 0;
            self.pre_temprecth = 240;

            // graphics.buffers.fill_back_buffer_with_color_at_xy(self.pre_temprectx as u32, self.pre_temprecty as u32, self.pre_temprectw as u32, self.pre_temprecth as u32, sdl2::pixels::Color::BLACK);
            // graphics.buffers.fill_back_buffer_with_color_at_xy(self.pre_frontrectx as u32, self.pre_frontrecty as u32, self.pre_frontrectw as u32, self.pre_frontrecth as u32, graphics.getBGR_AsPixelColor(0x3E,0x31,0xA2));

            graphics.print(282-(15*8), 204, "LOADING... 100%".to_string(), 124, 112, 218, false);
        }

        // graphics.drawfade();
        crate::rustutil::dump_surface(&graphics.buffers.backBuffer, "backBuffer", "");

        Some(RenderResult::Plain)
    }
}
