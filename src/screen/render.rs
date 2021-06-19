use crate::{game::{self, MenuName}, music, scenes::RenderResult};
pub mod graphics;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BackGround {
    Title,
    Tower,
}

pub struct Render {
    pub graphics: Box<graphics::Graphics>,
    tr: i32,
    tg: i32,
    tb: i32,
}

impl Render {
    pub fn new (pf: sdl2::pixels::PixelFormatEnum) -> Render {
        let graphics = Box::new(graphics::Graphics::new(pf));

        Render {
            graphics,
            tr: 0,
            tg: 0,
            tb: 0,
        }
    }

    pub fn get_render_rect (&mut self) -> sdl2::rect::Rect {
        self.graphics.buffers.get_back_buffer_rect()
    }

    /* actual Render.cpp goes below */

    // static int inline FLIP(int ypos)
    // static inline void drawslowdowntext(void)
    // static void volumesliderrender(void)

    // static void menurender(void)
    pub fn menu_render (&mut self, game: &game::Game, music: &music::Music) {
        let temp = 50;
        let tr = self.tr;
        let tg = self.tg;
        let tb = self.tb;

        match game.currentmenuname {
            MenuName::mainmenu => {
                let vsprite = 27; // 23
                self.graphics.draw_sprite((160 - 96) + 0 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.draw_sprite((160 - 96) + 1 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.draw_sprite((160 - 96) + 2 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.draw_sprite((160 - 96) + 3 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.draw_sprite((160 - 96) + 4 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.draw_sprite((160 - 96) + 5 * 32, temp, vsprite, tr, tg, tb);

                // #if defined(MAKEANDPLAY)
                // graphics.Print(-1,temp+35,"     MAKE AND PLAY EDITION",tr, tg, tb, true);
                // #endif
                // #ifdef COMMIT_DATE
                // graphics.Print( 310 - (10*8), 210, COMMIT_DATE, tr/2, tg/2, tb/2);
                // #endif
                // #ifdef INTERIM_COMMIT
                // graphics.Print( 310 - (SDL_arraysize(INTERIM_COMMIT) - 1) * 8, 220, INTERIM_COMMIT, tr/2, tg/2, tb/2);
                // #endif
                // graphics.Print( 310 - (4*8), 230, "v2.3", tr/2, tg/2, tb/2);
                self.graphics.print( 310 - (6*8), 230, "v2.3-rust".to_string(), tr/2, tg/2, tb/2, false);

                if music.mmmmmm {
                    self.graphics.print( 10, 230, "[MMMMMM Mod Installed]".to_string(), tr/2, tg/2, tb/2, false);
                }
            },
            MenuName::errornostart => {
                self.graphics.print( -1, 65, "ERROR: This level has".to_string(), tr, tg, tb, true);
                self.graphics.print( -1, 75, "no start point!".to_string(), tr, tg, tb, true);
            },
            MenuName::gameplayoptions => {
                let mut gameplayoptionsoffset = 0;
                // #if !defined(MAKEANDPLAY)
                if game.ingame_titlemode && game.unlock[18] {
                // #endif
                    gameplayoptionsoffset = 1;
                    if game.currentmenuoption == 0 {
                        self.graphics.bigprint(-1, 30, "Flip Mode", tr, tg, tb, true, 0);
                        self.graphics.print(-1, 65, "Flip the entire game vertically.".to_string(), tr, tg, tb, true);
                        if self.graphics.setflipmode {
                            self.graphics.print(-1, 85, "Currently ENABLED!".to_string(), tr, tg, tb, true);
                        } else {
                            self.graphics.print(-1, 85, "Currently Disabled.".to_string(), tr / 2, tg / 2, tb / 2, true);
                        }
                    }
                }

                if game.currentmenuoption == gameplayoptionsoffset + 0 {
                    //Toggle FPS
                    self.graphics.bigprint(-1, 30, "Toggle 30+ FPS", tr, tg, tb, true, 0);
                    self.graphics.print(-1, 65, "Change whether the game".to_string(), tr, tg, tb, true);
                    self.graphics.print(-1, 75, "runs at 30 or over 30 FPS.".to_string(), tr, tg, tb, true);

                    if !game.over30mode {
                        self.graphics.print(-1, 95, "Current mode: 30 FPS".to_string(), tr / 2, tg / 2, tb / 2, true);
                    } else {
                        self.graphics.print(-1, 95, "Current mode: Over 30 FPS".to_string(), tr, tg, tb, true);
                    }
                } else if game.currentmenuoption == gameplayoptionsoffset + 1 {
                    //Speedrunner options
                    self.graphics.bigprint(-1, 30, "Speedrunner Options", tr, tg, tb, true, 0);
                    self.graphics.print(-1, 65, "Access some advanced settings that".to_string(), tr, tg, tb, true);
                    self.graphics.print(-1, 75, "might be of interest to speedrunners".to_string(), tr, tg, tb, true);
                } else if game.currentmenuoption == gameplayoptionsoffset + 2 {
                    //Advanced options
                    self.graphics.bigprint(-1, 30, "Advanced Options", tr, tg, tb, true, 0);
                    self.graphics.print(-1, 65, "All other settings".to_string(), tr, tg, tb, true);
                } else if game.currentmenuoption == gameplayoptionsoffset + 3 {
                    //Clear Data
                    self.graphics.bigprint(-1, 30, "Clear Data", tr, tg, tb, true, 0);
                    self.graphics.print(-1, 65, "Delete your save data".to_string(), tr, tg, tb, true);
                    self.graphics.print(-1, 75, "and unlocked play modes".to_string(), tr, tg, tb, true);
                }
            },
            MenuName::options => {
                match game.currentmenuoption {
                    0 => {
                        self.graphics.bigprint(-1, 30, "Gameplay Options", tr, tg, tb, true, 0);
                        self.graphics.print(-1, 65, "Adjust various gameplay options".to_string(), tr, tg, tb, true);
                    },
                    1 => {
                        self.graphics.bigprint(-1, 30, "Graphics Options", tr, tg, tb, true, 0);
                        self.graphics.print(-1, 65, "Adjust screen settings".to_string(), tr, tg, tb, true);
                    },
                    2 => {
                        self.graphics.bigprint(-1, 30, "Audio Options", tr, tg, tb, true, 0);
                        self.graphics.print(-1, 65, "Adjust volume settings".to_string(), tr, tg, tb, true);
                        if music.mmmmmm {
                            self.graphics.print(-1, 75, "and soundtrack".to_string(), tr, tg, tb, true);
                        }
                    },
                    3 => {
                        self.graphics.bigprint(-1, 30, "Game Pad Options", tr, tg, tb, true, 0);
                        self.graphics.print(-1, 65, "Rebind your controller's buttons".to_string(), tr, tg, tb, true);
                        self.graphics.print(-1, 75, "and adjust sensitivity".to_string(), tr, tg, tb, true);
                    },
                    4 => {
                        self.graphics.bigprint(-1, 30, "Accessibility", tr, tg, tb, true, 0);
                        self.graphics.print(-1, 65, "Disable screen effects, enable".to_string(), tr, tg, tb, true);
                        self.graphics.print(-1, 75, "slowdown modes or invincibility".to_string(), tr, tg, tb, true);
                    },
                    _ => panic!("{:?} unknown menu option", game.currentmenuoption),
                }
            },
            _ => panic!("{:?} menuen not implemented yet", game.currentmenuname),
        }
    }

    // void titlerender(void)
    pub fn title_render(&mut self, game: &mut game::Game, music: &music::Music) -> Option<RenderResult> {
        self.graphics.buffers.fill_back_buffer_with_color(sdl2::pixels::Color::BLACK);

        if !game.menustart {
            self.tr = self.graphics.col_tr;
            self.tg = self.graphics.col_tg;
            self.tb = self.graphics.col_tb;

            let temp = 50;
            let vsprite = 27; // 23
            self.graphics.draw_sprite((160 - 96) + 0 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.draw_sprite((160 - 96) + 1 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.draw_sprite((160 - 96) + 2 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.draw_sprite((160 - 96) + 3 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.draw_sprite((160 - 96) + 4 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.draw_sprite((160 - 96) + 5 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            // #if defined(MAKEANDPLAY)
            // graphics.Print(-1,temp+35,"     MAKE AND PLAY EDITION", self.tr, self.tg, self.tb, true);
            // #endif

            self.graphics.print(5, 175, "[ Press ACTION to Start ]".to_string(), self.tr, self.tg, self.tb, true);
            self.graphics.print(5, 195, "ACTION = Space, Z, or V".to_string(), self.tr/2, self.tg/2, self.tb/2, true);
        } else {
            if !game.colourblindmode {
                self.graphics.drawtowerbackground(BackGround::Title);
            }

            self.menu_render(game, music);

            self.tr = ((self.graphics.col_tr as f32) * 0.8) as i32;
            self.tg = ((self.graphics.col_tg as f32) * 0.8) as i32;
            self.tb = ((self.graphics.col_tb as f32) * 0.8) as i32;

            if self.tr <   0 { self.tr = 0   };
            if self.tr > 255 { self.tr = 255 };
            if self.tg <   0 { self.tg = 0   };
            if self.tg > 255 { self.tg = 255 };
            if self.tb <   0 { self.tb = 0   };
            if self.tb > 255 { self.tb = 255 };

            self.graphics.drawmenu(self.tr, self.tg, self.tb, Some(game.currentmenuname == MenuName::levellist), game);
        }

        self.graphics.drawfade();

        Some(RenderResult::WithScreenEffects)
    }

    // void gamecompleterender(void)
    // void gamecompleterender2(void)
    // void gamerender(void)
    // void maprender(void)
    // void teleporterrender(void)
}
