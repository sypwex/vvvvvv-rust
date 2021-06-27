use crate::{game::{self, MenuName}, key_poll, map, music, scenes::RenderResult, utility_class};

use super::Screen;
pub mod graphics;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BackGround {
    Title,
    Tower,
}

pub struct Render {
    pub graphics: graphics::Graphics,
    tr: i32,
    tg: i32,
    tb: i32,
}

impl Render {
    pub fn new (pf: sdl2::pixels::PixelFormatEnum) -> Render {
        let graphics = graphics::Graphics::new(pf);

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
    fn drawslowdowntext(&self) {
        println!("DEADBEEF(render.rs): drawslowdowntext not implemented yet");
    }

    // static void volumesliderrender(void)
    fn volumesliderrender(&self) {
        println!("DEADBEEF(render.rs): volumesliderrender not implemented yet");
    }

    // static void menurender(void)
    pub fn menurender(&mut self, game: &game::Game, music: &music::Music, map: &map::Map, help: &utility_class::UtilityClass, key: &key_poll::KeyPoll, screen_params: crate::screen::ScreenParams) {
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
                // self.graphics.print(-1,temp+35,"     MAKE AND PLAY EDITION",tr, tg, tb, Some(true));
                // #endif
                // #ifdef COMMIT_DATE
                // self.graphics.print( 310 - (10*8), 210, COMMIT_DATE, tr/2, tg/2, tb/2);
                // #endif
                // #ifdef INTERIM_COMMIT
                // self.graphics.print( 310 - (SDL_arraysize(INTERIM_COMMIT) - 1) * 8, 220, INTERIM_COMMIT, tr/2, tg/2, tb/2);
                // #endif
                // self.graphics.print( 310 - (4*8), 230, "v2.3", tr/2, tg/2, tb/2);
                self.graphics.print( 310 - (6*8), 230, "v2.3-rust", tr/2, tg/2, tb/2, None);

                if music.mmmmmm {
                    self.graphics.print( 10, 230, "[MMMMMM Mod Installed]", tr/2, tg/2, tb/2, None);
                }
            },
            MenuName::errornostart => {
                self.graphics.print( -1, 65, "ERROR: This level has", tr, tg, tb, Some(true));
                self.graphics.print( -1, 75, "no start point!", tr, tg, tb, Some(true));
            },
            MenuName::gameplayoptions => {
                let mut gameplayoptionsoffset = 0;
                // #if !defined(MAKEANDPLAY)
                if game.ingame_titlemode && game.unlock[18] {
                // #endif
                    gameplayoptionsoffset = 1;
                    if game.currentmenuoption == 0 {
                        self.graphics.bigprint(-1, 30, "Flip Mode", tr, tg, tb, Some(true), Some(2));
                        self.graphics.print(-1, 65, "Flip the entire game vertically.", tr, tg, tb, Some(true));
                        if self.graphics.setflipmode {
                            self.graphics.print(-1, 85, "Currently ENABLED!", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 85, "Currently Disabled.", tr / 2, tg / 2, tb / 2, Some(true));
                        }
                    }
                }

                if game.currentmenuoption == gameplayoptionsoffset + 0 {
                    //Toggle FPS
                    self.graphics.bigprint(-1, 30, "Toggle 30+ FPS", tr, tg, tb, Some(true), Some(2));
                    self.graphics.print(-1, 65, "Change whether the game", tr, tg, tb, Some(true));
                    self.graphics.print(-1, 75, "runs at 30 or over 30 FPS.", tr, tg, tb, Some(true));

                    if !game.over30mode {
                        self.graphics.print(-1, 95, "Current mode: 30 FPS", tr / 2, tg / 2, tb / 2, Some(true));
                    } else {
                        self.graphics.print(-1, 95, "Current mode: Over 30 FPS", tr, tg, tb, Some(true));
                    }
                } else if game.currentmenuoption == gameplayoptionsoffset + 1 {
                    //Speedrunner options
                    self.graphics.bigprint(-1, 30, "Speedrunner Options", tr, tg, tb, Some(true), Some(2));
                    self.graphics.print(-1, 65, "Access some advanced settings that", tr, tg, tb, Some(true));
                    self.graphics.print(-1, 75, "might be of interest to speedrunners", tr, tg, tb, Some(true));
                } else if game.currentmenuoption == gameplayoptionsoffset + 2 {
                    //Advanced options
                    self.graphics.bigprint(-1, 30, "Advanced Options", tr, tg, tb, Some(true), Some(2));
                    self.graphics.print(-1, 65, "All other settings", tr, tg, tb, Some(true));
                } else if game.currentmenuoption == gameplayoptionsoffset + 3 {
                    //Clear Data
                    self.graphics.bigprint(-1, 30, "Clear Data", tr, tg, tb, Some(true), Some(2));
                    self.graphics.print(-1, 65, "Delete your save data", tr, tg, tb, Some(true));
                    self.graphics.print(-1, 75, "and unlocked play modes", tr, tg, tb, Some(true));
                }
            },
            MenuName::options => {
                match game.currentmenuoption {
                    0 => {
                        self.graphics.bigprint(-1, 30, "Gameplay Options", tr, tg, tb, Some(true), Some(2));
                        self.graphics.print(-1, 65, "Adjust various gameplay options", tr, tg, tb, Some(true));
                    },
                    1 => {
                        self.graphics.bigprint(-1, 30, "Graphics Options", tr, tg, tb, Some(true), Some(2));
                        self.graphics.print(-1, 65, "Adjust screen settings", tr, tg, tb, Some(true));
                    },
                    2 => {
                        self.graphics.bigprint(-1, 30, "Audio Options", tr, tg, tb, Some(true), Some(2));
                        self.graphics.print(-1, 65, "Adjust volume settings", tr, tg, tb, Some(true));
                        if music.mmmmmm {
                            self.graphics.print(-1, 75, "and soundtrack", tr, tg, tb, Some(true));
                        }
                    },
                    3 => {
                        self.graphics.bigprint(-1, 30, "Game Pad Options", tr, tg, tb, Some(true), Some(2));
                        self.graphics.print(-1, 65, "Rebind your controller's buttons", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "and adjust sensitivity", tr, tg, tb, Some(true));
                    },
                    4 => {
                        self.graphics.bigprint(-1, 30, "Accessibility", tr, tg, tb, Some(true), Some(2));
                        self.graphics.print(-1, 65, "Disable screen effects, enable", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "slowdown modes or invincibility", tr, tg, tb, Some(true));
                    },
                    _ => println!("{:?} unknown menu option", game.currentmenuoption),
                }
            },
            MenuName::graphicoptions => {
                match game.currentmenuoption {
                    0 => {
                        self.graphics.bigprint( -1, 30, "Toggle Fullscreen", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Change to fullscreen/windowed mode.", tr, tg, tb, Some(true));

                        if screen_params.isWindowed {
                            self.graphics.print( -1, 85, "Current mode: WINDOWED", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print( -1, 85, "Current mode: FULLSCREEN", tr, tg, tb, Some(true));
                        }
                    },
                    1 => {
                        self.graphics.bigprint( -1, 30, "Scaling Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Choose letterbox/stretch/integer mode.", tr, tg, tb, Some(true));

                        match screen_params.stretchMode {
                            2 => self.graphics.print( -1, 85, "Current mode: INTEGER", tr, tg, tb, Some(true)),
                            1 => self.graphics.print( -1, 85, "Current mode: STRETCH", tr, tg, tb, Some(true)),
                            _ => self.graphics.print( -1, 85, "Current mode: LETTERBOX", tr, tg, tb, Some(true)),
                        }
                    },
                    2 => {
                        self.graphics.bigprint(-1, 30, "Resize to Nearest", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Resize to the nearest window size", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "that is of an integer multiple.", tr, tg, tb, Some(true));
                        if !screen_params.isWindowed {
                            self.graphics.print(-1, 95, "You must be in windowed mode", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 105, "to use this option.", tr, tg, tb, Some(true));
                        }
                    },
                    3 => {
                        self.graphics.bigprint( -1, 30, "Toggle Filter", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Change to nearest/linear filter.", tr, tg, tb, Some(true));

                        if screen_params.isFiltered {
                            self.graphics.print( -1, 85, "Current mode: LINEAR", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print( -1, 85, "Current mode: NEAREST", tr, tg, tb, Some(true));
                        }
                    },
                    4 => {
                        self.graphics.bigprint( -1, 30, "Analogue Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "There is nothing wrong with your", tr, tg, tb, Some(true));
                        self.graphics.print( -1, 75, "television set. Do not attempt to", tr, tg, tb, Some(true));
                        self.graphics.print( -1, 85, "adjust the picture.", tr, tg, tb, Some(true));
                    },
                    5 => {
                        self.graphics.bigprint(-1, 30, "Toggle VSync", tr, tg, tb, Some(true), None);
                        // #ifdef __HAIKU__ // FIXME: Remove after SDL VSync bug is fixed! -flibit
                        // self.graphics.print(-1, 65, "Edit the config file on Haiku!", tr, tg, tb, Some(true));
                        // #else
                        self.graphics.print(-1, 65, "Turn VSync on or off.", tr, tg, tb, Some(true));
                        // #endif

                        if !screen_params.vsync {
                            self.graphics.print(-1, 95, "Current mode: VSYNC OFF", tr/2, tg/2, tb/2, Some(true));
                        } else {
                            self.graphics.print(-1, 95, "Current mode: VSYNC ON", tr, tg, tb, Some(true));
                        }
                    },
                    _ => println!("unkown graphic option"),
                };
            },
            MenuName::audiooptions => {
                match game.currentmenuoption {
                    0 => {
                        self.graphics.bigprint(-1, 30, "Music Volume", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Change the volume of the music.", tr, tg, tb, Some(true));
                        self.volumesliderrender();
                    },
                    1 => {
                        self.graphics.bigprint(-1, 30, "Sound Volume", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Change the volume of sound effects.", tr, tg, tb, Some(true));
                        self.volumesliderrender();
                    },
                    // 2 => {
                    //     if music.mmmmmm {
                    //         /* Screen width 40 chars, 4 per char */
                    //         let buffer: [u8;160 + 1];
                    //         let soundtrack: [u8;6 + 1];
                    //         let letter = if music.usingmmmmmm {
                    //             'M'
                    //         } else {
                    //             'P'
                    //         };
                    //         VVV_fillstring(soundtrack, sizeof(soundtrack), letter);
                    //         SDL_snprintf(buffer, sizeof(buffer), "Current soundtrack: %s", soundtrack);

                    //         self.graphics.bigprint(-1, 30, "Soundtrack", tr, tg, tb, Some(true), None);
                    //         self.graphics.print(-1, 65, "Toggle between MMMMMM and PPPPPP", tr, tg, tb, Some(true));
                    //         self.graphics.print(-1, 85, buffer, tr, tg, tb, Some(true));
                    //     }
                    // },
                    _ => println!("menuen not implemented yet"),
                }
            },
            MenuName::credits => {
                self.graphics.print( -1, 50, "VVVVVV is a game by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 65, "Terry Cavanagh", tr, tg, tb, Some(true), Some(2));

                self.graphics.drawimagecol(7, -1, 86, (tr as f32 * 0.75) as i32, (tg as f32 * 0.75) as i32, (tb as f32 * 0.75) as i32, true);

                self.graphics.print( -1, 120, "and features music by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 135, "Magnus P~lsson", tr, tg, tb, Some(true), Some(2));
                self.graphics.drawimagecol(8, -1, 156, (tr as f32 * 0.75) as i32, (tg as f32 * 0.75) as i32, (tb as f32 * 0.75) as i32, true);
            },
            MenuName::credits2 => {
                self.graphics.print( -1, 50, "Roomnames are by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 65, "Bennett Foddy", tr, tg, tb, Some(true), None);
                self.graphics.drawimagecol(9, -1, 86, (tr as f32 * 0.75) as i32, (tg as f32 * 0.75) as i32, (tb as f32 * 0.75) as i32, true);
                self.graphics.print( -1, 110, "C++ version by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 125, "Simon Roth", tr, tg, tb, Some(true), None);
                self.graphics.bigprint( 40, 145, "Ethan Lee", tr, tg, tb, Some(true), None);
            },
            MenuName::credits25 => {
                self.graphics.print( -1, 40, "Beta Testing by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 55, "Sam Kaplan", tr, tg, tb, Some(true), None);
                self.graphics.bigprint( 40, 75, "Pauli Kohberger", tr, tg, tb, Some(true), None);
                self.graphics.print( -1, 130, "Ending Picture by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 145, "Pauli Kohberger", tr, tg, tb, Some(true), None);
            },
            MenuName::credits3 => {
                self.graphics.print( -1, 20, "VVVVVV is supported by", tr, tg, tb, Some(true));
                self.graphics.print( 40, 30, "the following patrons", tr, tg, tb, Some(true));

                let startidx = game.current_credits_list_index;
                // let endidx = VVV_min(startidx + 9, SDL_arraysize(Credits::superpatrons));

                let xofs = 80 - 16;
                let yofs = 40 + 20;

                // for i in startidx..endidx {
                //     self.graphics.print(xofs, yofs, Credits::superpatrons[i], tr, tg, tb, None);
                //     xofs += 4;
                //     yofs += 14;
                // }
                println!("DEADBEEF(render.rs): not implemented yet");
            },
            MenuName::credits4 => {
                self.graphics.print( -1, 20, "and also by", tr, tg, tb, Some(true));

                let startidx = game.current_credits_list_index;
                // let endidx = VVV_min(startidx + 14, SDL_arraysize(Credits::patrons));

                let maxheight = 10 * 14;
                // let totalheight = (endidx - startidx) * 10;
                // let emptyspace = maxheight - totalheight;

                // let yofs = 40 + (emptyspace / 2);

                // for i in startidx..endidx {
                //     self.graphics.print(80, yofs, Credits::patrons[i], tr, tg, tb, None);
                //     yofs += 10;
                // }
                println!("DEADBEEF(render.rs): not implemented yet");
            },
            MenuName::credits5 => {
                self.graphics.print( -1, 20, "With contributions on", tr, tg, tb, Some(true));
                self.graphics.print( 40, 30, "GitHub from", tr, tg, tb, Some(true));

                let startidx = game.current_credits_list_index;
                // let endidx = VVV_min(startidx + 9, SDL_arraysize(Credits::githubfriends));

                let maxheight = 14 * 9;
                // let totalheight = (endidx - startidx) * 14;
                // let emptyspace = maxheight - totalheight;

                // let xofs;
                // let yofs;

                // if startidx == 0 {
                //     self.graphics.print(-1, 60, Credits::githubfriends[0], tr, tg, tb, Some(true));
                //     self.graphics.print(-1, 80, Credits::githubfriends[2], tr, tg, tb, Some(true));
                //     startidx += 4; // Skip the superfriends now that we've drawn them...
                //     xofs = 80 - 28;
                //     yofs = 80 + 20 + (emptyspace / 2);
                // } else {
                //     xofs = 80 - 16;
                //     yofs = 40 + 20 + (emptyspace / 2);
                // }

                // for i in startidx..endidx {
                //     self.graphics.print(xofs, yofs, Credits::githubfriends[i], tr, tg, tb, None);
                //     xofs += 4;
                //     yofs += 14;
                // }
                println!("DEADBEEF(render.rs): not implemented yet");
            },
            MenuName::credits6 => {
                self.graphics.print( -1, 20, "and thanks also to:", tr, tg, tb, Some(true));

                self.graphics.bigprint(80, 60, "You!", tr, tg, tb, Some(true), None);

                self.graphics.print( 80, 100, "Your support makes it possible", tr, tg, tb, Some(true));
                self.graphics.print( 80, 110,"for me to continue making the", tr, tg, tb, Some(true));
                self.graphics.print( 80, 120,"games I want to make, now", tr, tg, tb, Some(true));
                self.graphics.print( 80, 130, "and into the future.", tr, tg, tb, Some(true));

                self.graphics.print( 80, 150,"Thank you!", tr, tg, tb, Some(true));
            },
            MenuName::setinvincibility => {
                self.graphics.print( -1, 100, "Are you sure you want to ", tr, tg, tb, Some(true));
                self.graphics.print( -1, 110, "enable invincibility?", tr, tg, tb, Some(true));
            },
            MenuName::setslowdown => {
                self.graphics.bigprint( -1, 40, "Game Speed", tr, tg, tb, Some(true), None);
                self.graphics.print( -1, 75, "Select a new game speed below.", tr, tg, tb, Some(true));
                self.drawslowdowntext();
            },
            MenuName::newgamewarning => {
                self.graphics.print( -1, 100, "Are you sure? This will", tr, tg, tb, Some(true));
                self.graphics.print( -1, 110, "delete your current saves...", tr, tg, tb, Some(true));
            },
            MenuName::cleardatamenu => {
                self.graphics.print( -1, 100, "Are you sure you want to", tr, tg, tb, Some(true));
                self.graphics.print( -1, 110, "delete all your saved data?", tr, tg, tb, Some(true));
            },
            MenuName::startnodeathmode => {
                self.graphics.print( -1, 45, "Good luck!", tr, tg, tb, Some(true));
                self.graphics.print( -1, 80, "You cannot save in this mode.", tr, tg, tb, Some(true));
                self.graphics.print( -1, 100, "Would you like to disable the", tr, tg, tb, Some(true));
                self.graphics.print( -1, 112, "cutscenes during the game?", tr, tg, tb, Some(true));
            },
            MenuName::controller => {
                self.graphics.bigprint( -1, 30, "Game Pad", tr, tg, tb, Some(true), None);
                self.graphics.print( -1, 55, "Change controller options.", tr, tg, tb, Some(true));
                match game.currentmenuoption {
                    0 => {
                        match key.sensitivity {
                            0 => {
                                self.graphics.print( -1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print( -1, 95, "[]..................", tr, tg, tb, Some(true));
                            },
                            1 => {
                                self.graphics.print( -1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print( -1, 95, ".....[].............", tr, tg, tb, Some(true));
                            },
                            2 => {
                                self.graphics.print( -1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print( -1, 95, ".........[].........", tr, tg, tb, Some(true));
                            },
                            3 => {
                                self.graphics.print( -1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print( -1, 95, ".............[].....", tr, tg, tb, Some(true));
                            },
                            4 => {
                                self.graphics.print( -1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print( -1, 95, "..................[]", tr, tg, tb, Some(true));
                            },
                            _ => println!("incorrect sensitivity"),
                        }
                    },
                    1 | 2 | 3 | 4 => {
                        let s = format!("Flip is bound to: {}", help.GCString(&game.controllerButton_flip));
                        self.graphics.print( -1, 85, &s, tr, tg, tb, Some(true));
                        let s = format!("Enter is bound to: {}",  help.GCString(&game.controllerButton_map));
                        self.graphics.print( -1, 95, &s, tr, tg, tb, Some(true));
                        let s = format!("Menu is bound to: {}", help.GCString(&game.controllerButton_esc));
                        self.graphics.print( -1, 105, &s, tr, tg, tb, Some(true));
                        let s = format!("Restart is bound to: {}", help.GCString(&game.controllerButton_restart));
                        self.graphics.print( -1, 115, &s, tr, tg, tb, Some(true));
                    },
                    _ => println!("incorrect controller menu option"),
                }
            },
            MenuName::speedrunneroptions => {
                match game.currentmenuoption {
                    0 => {
                        self.graphics.bigprint(-1, 30, "Glitchrunner Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Re-enable glitches that existed", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "in previous versions of the game.", tr, tg, tb, Some(true));
                        if game.glitchrunnermode {
                            self.graphics.print(-1, 95, "Glitchrunner mode is ON", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 95, "Glitchrunner mode is OFF", tr / 2, tg / 2, tb / 2, Some(true));
                        }
                    },
                    1 => {
                        self.graphics.bigprint(-1, 30, "Input Delay", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Re-enable the 1-frame input delay", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "from previous versions of the game.", tr, tg, tb, Some(true));
                        if game.inputdelay {
                            self.graphics.print(-1, 95, "Input delay is ON", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 95, "Input delay is OFF", tr / 2, tg / 2, tb / 2, Some(true));
                        }
                    },
                    2 => {
                        self.graphics.bigprint(-1, 30, "Fake Load Screen", tr, tg, tb, Some(true), None);
                        if game.skipfakeload {
                            self.graphics.print(-1, 65, "Fake loading screen is OFF", tr / 2, tg / 2, tb / 2, Some(true));
                        } else {
                            self.graphics.print(-1, 65, "Fake loading screen is ON", tr, tg, tb, Some(true));
                        }
                    },
                    _ => println!("incorrect speedrun option"),
                };
            },
            MenuName::advancedoptions => {
                match game.currentmenuoption {
                    0 => {
                        self.graphics.bigprint( -1, 30, "Unfocus Pause", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Toggle if the game will pause", tr, tg, tb, Some(true));
                        self.graphics.print( -1, 75, "when the window is unfocused.", tr, tg, tb, Some(true));
                        if game.disablepause {
                            self.graphics.print(-1, 95, "Unfocus pause is OFF", tr/2, tg/2, tb/2, Some(true));
                        } else {
                            self.graphics.print(-1, 95, "Unfocus pause is ON", tr, tg, tb, Some(true));
                        }
                    },
                    1 => {
                        self.graphics.bigprint(-1, 30, "Room Name BG", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Lets you see through what is behind", tr, tg, tb, Some(true));
                        self.graphics.print( -1, 75, "the name at the bottom of the screen.", tr, tg, tb, Some(true));
                        if self.graphics.translucentroomname {
                            self.graphics.print(-1, 95, "Room name background is TRANSLUCENT", tr/2, tg/2, tb/2, Some(true));
                        } else {
                            self.graphics.print(-1, 95, "Room name background is OPAQUE", tr, tg, tb, Some(true));
                        }
                    },
                    _ => println!("incorrect advanced option"),
                };
            },
            MenuName::accessibility => {
                // #ifdef MAKEANDPLAY
                // #define OFFSET 0
                // #else
                // #define OFFSET 1
                // #endif

                match game.currentmenuoption {
                    // #if !defined(MAKEANDPLAY)
                    0 => {
                        self.graphics.bigprint(-1, 30, "Unlock Play Modes", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Unlock parts of the game normally", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "unlocked as you progress", tr, tg, tb, Some(true));
                    },
                    // #endif
                    1 => { // OFFSET+0
                        self.graphics.bigprint(-1, 40, "Invincibility", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 75, "Explore the game freely without", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 85, "dying. (Can cause glitches.)", tr, tg, tb, Some(true));
                        if map.invincibility {
                            self.graphics.print(-1, 105, "Invincibility is ON.", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 105, "Invincibility is OFF.", tr / 2, tg / 2, tb / 2, Some(true));
                        }
                    },
                    2 => { // OFFSET+1
                        self.graphics.bigprint(-1, 40, "Slowdown", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 75, "Reduce the game speed.", tr, tg, tb, Some(true));
                        self.drawslowdowntext();
                    },
                    3 => { // OFFSET+2
                        self.graphics.bigprint(-1, 40, "Backgrounds", tr, tg, tb, Some(true), None);
                        if !game.colourblindmode {
                            self.graphics.print(-1, 75, "Backgrounds are ON.", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 75, "Backgrounds are OFF.", tr / 2, tg / 2, tb / 2, Some(true));
                        }
                    },
                    4 => { // OFFSET+3
                        self.graphics.bigprint(-1, 40, "Screen Effects", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 75, "Disables screen shakes and flashes.", tr, tg, tb, Some(true));
                        if !game.noflashingmode {
                            self.graphics.print(-1, 85, "Screen Effects are ON.", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 85, "Screen Effects are OFF.", tr / 2, tg / 2, tb / 2, Some(true));
                        }
                    },
                    5 => { // case OFFSET+4
                        self.graphics.bigprint(-1, 40, "Text Outline", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 75, "Disables outline on game text.", tr, tg, tb, Some(true));
                        // FIXME: Maybe do an outlined print instead? -flibit
                        if !self.graphics.notextoutline {
                            self.graphics.print(-1, 85, "Text outlines are ON.", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 85, "Text outlines are OFF.", tr / 2, tg / 2, tb / 2, Some(true));
                        }
                    },
                    _ => println!("incorrect accessibility option"),
                };

                // #undef OFFSET
            },
            MenuName::playint1 | MenuName::playint2 => {
                self.graphics.print( -1, 65, "Who do you want to play", tr, tg, tb, Some(true));
                self.graphics.print( -1, 75, "the level with?", tr, tg, tb, Some(true));
            },
            MenuName::playmodes => {
                match game.currentmenuoption {
                    0 => {
                        self.graphics.bigprint( -1, 30, "Time Trials", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Replay any level in the game in", tr, tg, tb, Some(true));
                        self.graphics.print( -1, 75, "a competitive time trial mode.", tr, tg, tb, Some(true));

                        if game.slowdown < 30 || map.invincibility {
                            self.graphics.print( -1, 105, "Time Trials are not available", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 115, "with slowdown or invincibility.", tr, tg, tb, Some(true));
                        }
                    },
                    1 => {
                        self.graphics.bigprint( -1, 30, "Intermissions", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Replay the intermission levels.", tr, tg, tb, Some(true));

                        if !game.unlock[15] && !game.unlock[16] {
                            self.graphics.print( -1, 95, "TO UNLOCK: Complete the", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 105, "intermission levels in-game.", tr, tg, tb, Some(true));
                        }
                    },
                    2 => {
                        self.graphics.bigprint( -1, 30, "No Death Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Play the entire game", tr, tg, tb, Some(true));
                        self.graphics.print( -1, 75, "without dying once.", tr, tg, tb, Some(true));

                        if game.slowdown < 30 || map.invincibility {
                            self.graphics.print( -1, 105, "No Death Mode is not available", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 115, "with slowdown or invincibility.", tr, tg, tb, Some(true));
                        }
                        else if !game.unlock[17] {
                            self.graphics.print( -1, 105, "TO UNLOCK: Achieve an S-rank or", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 115, "above in at least 4 time trials.", tr, tg, tb, Some(true));
                        }
                    },
                    3 => {
                        // WARNING: Partially duplicated in MenuName::options
                        self.graphics.bigprint( -1, 30, "Flip Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print( -1, 65, "Flip the entire game vertically.", tr, tg, tb, Some(true));
                        self.graphics.print( -1, 75, "Compatible with other game modes.", tr, tg, tb, Some(true));

                        if game.unlock[18] {
                            if self.graphics.setflipmode {
                                self.graphics.print( -1, 105, "Currently ENABLED!", tr, tg, tb, Some(true));
                            } else {
                                self.graphics.print( -1, 105, "Currently Disabled.", tr/2, tg/2, tb/2, Some(true));
                            }
                        } else {
                            self.graphics.print( -1, 105, "TO UNLOCK: Complete the game.", tr, tg, tb, Some(true));
                        }
                    },
                    _ => println!("incorrect play mode"),
                };
            },
            MenuName::youwannaquit => {
                self.graphics.print( -1, 75, "Are you sure you want to quit?", tr, tg, tb, Some(true));
            },
            MenuName::continuemenu => {
                match game.currentmenuoption {
                    0 => {
                        //Show teleporter save info
                        self.graphics.drawpixeltextbox(17, 65-20, 286, 90, 36,12, 65, 185, 207,0,4);

                        self.graphics.bigprint(-1, 20, "Tele Save", tr, tg, tb, Some(true), None);
                        self.graphics.print(0, 80-20, &game.tele_currentarea, 25, 255 - (help.glow / 2), 255 - (help.glow / 2), Some(true));
                        for i in 0..6 {
                            self.graphics.drawcrewman(169-(3*42)+(i*42), 95-20, i, game.tele_crewstats[i as usize], Some(true));
                        }
                        self.graphics.print(59, 132-20, &game.tele_gametime, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);
                        let trinketcount = help.number(game.tele_trinkets);
                        self.graphics.print(262-graphics::Graphics::len(trinketcount), 132-20, trinketcount, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);

                        self.graphics.draw_sprite_c(34, 126-20, 50, graphics::Color::Clock);
                        self.graphics.draw_sprite_c(270, 126-20, 22, graphics::Color::Trinket);
                    },
                    1 => {
                        //Show quick save info
                        self.graphics.drawpixeltextbox(17, 65-20, 286, 90, 36,12, 65, 185, 207,0,4);

                        self.graphics.bigprint(-1, 20, "Quick Save", tr, tg, tb, Some(true), None);
                        self.graphics.print(0, 80-20, &game.quick_currentarea, 25, 255 - (help.glow / 2), 255 - (help.glow / 2), Some(true));
                        for i in 0..6 {
                            self.graphics.drawcrewman(169-(3*42)+(i*42), 95-20, i, game.quick_crewstats[i as usize], Some(true));
                        }
                        self.graphics.print(59, 132-20, &game.quick_gametime, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);
                        let trinketcount = help.number(game.quick_trinkets);
                        self.graphics.print(262-graphics::Graphics::len(trinketcount), 132-20, trinketcount, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);

                        self.graphics.draw_sprite_c(34, 126-20, 50, graphics::Color::Clock);
                        self.graphics.draw_sprite_c(270, 126-20, 22, graphics::Color::Trinket);
                    },
                    _ => println!("incorrect continue menu option"),
                };
            },
            MenuName::gameover | MenuName::gameover2 => {
                self.graphics.bigprint( -1, 25, "GAME OVER", tr, tg, tb, Some(true), Some(3));

                for i in 0..game.ndmresultcrewstats.len() as i32 {
                    self.graphics.drawcrewman(169-(3*42)+(i*42), 68, i, game.ndmresultcrewstats[i as usize], Some(true));
                }
                let tempstring = format!("You rescued {} {}", help.number(game.ndmresultcrewrescued), if game.ndmresultcrewrescued == 1 { "crewmate" } else { "crewmates" });
                self.graphics.print(0, 100, &tempstring, tr, tg, tb, Some(true));

                let tempstring = format!("and found {} {}", help.number(game.ndmresulttrinkets), if game.ndmresulttrinkets == 1 { "trinket." } else { "trinkets."});
                self.graphics.print(0, 110, &tempstring, tr, tg, tb, Some(true));

                let tempstring = "You managed to reach:";
                self.graphics.print(0, 145, tempstring, tr, tg, tb, Some(true));
                self.graphics.print(0, 155, &game.ndmresulthardestroom, tr, tg, tb, Some(true));

                let tempstring = match game.ndmresultcrewrescued {
                    1 => "Keep trying! You'll get there!",
                    2 => "Nice one!",
                    3 => "Wow! Congratulations!",
                    4 => "Incredible!",
                    5 => "Unbelievable! Well done!",
                    6 => "Er, how did you do that?",
                    _ => {
                        println!("unkown ndmresultcrewrescued");
                        ""
                    },
                };

                self.graphics.print(0, 190, tempstring, tr, tg, tb, Some(true));
            },
            MenuName::nodeathmodecomplete | MenuName::nodeathmodecomplete2 => {
                self.graphics.bigprint( -1, 8, "WOW", tr, tg, tb, Some(true), Some(4));

                for i in 0..game.ndmresultcrewstats.len() as i32 {
                    self.graphics.drawcrewman(169-(3*42)+(i*42), 68, i, game.ndmresultcrewstats[i as usize], Some(true));
                }
                let tempstring = "You rescued all the crewmates!";
                self.graphics.print(0, 100, tempstring, tr, tg, tb, Some(true));

                let tempstring = format!("And you found {} trinkets.", help.number(game.ndmresulttrinkets));
                self.graphics.print(0, 110, &tempstring, tr, tg, tb, Some(true));

                self.graphics.print(0, 160, "A new trophy has been awarded and", tr, tg, tb, Some(true));
                self.graphics.print(0, 170, "placed in the secret lab to", tr, tg, tb, Some(true));
                self.graphics.print(0, 180, "acknowledge your achievement!", tr, tg, tb, Some(true));
            },
            MenuName::timetrialcomplete | MenuName::timetrialcomplete2 | MenuName::timetrialcomplete3 => {
                self.graphics.bigprint( -1, 20, "Results", tr, tg, tb, Some(true), Some(3));

                let tempstring = format!("{} / {}.99", game.resulttimestring(), game.timetstring(game.timetrialresultpar));

                self.graphics.drawspritesetcol(30, 80-15, 50, 22);
                self.graphics.print(65, 80-15, "TIME TAKEN:", 255, 255, 255, None);
                self.graphics.print(65, 90-15, &tempstring, tr, tg, tb, None);
                if game.timetrialresulttime <= game.timetrialresultpar {
                    self.graphics.print(220, 85-15, "+1 Rank!", 255, 255, 255, None);
                }

                let tempstring = game.timetrialresultdeaths.to_string();
                self.graphics.drawspritesetcol(30-4, 80+20-4, 12, 22);
                self.graphics.print(65, 80+20, "NUMBER OF DEATHS:", 255, 255, 255, None);
                self.graphics.print(65, 90+20, &tempstring, tr, tg, tb, None);
                if game.timetrialresultdeaths == 0 {
                    self.graphics.print(220, 85+20, "+1 Rank!", 255, 255, 255, None);
                }

                let tempstring = format!("{} of {}", game.timetrialresulttrinkets, game.timetrialresultshinytarget);
                self.graphics.drawspritesetcol(30, 80+55, 22, 22);
                self.graphics.print(65, 80+55, "SHINY TRINKETS:", 255, 255, 255, None);
                self.graphics.print(65, 90+55, &tempstring, tr, tg, tb, None);
                if game.timetrialresulttrinkets >= game.timetrialresultshinytarget {
                    self.graphics.print(220, 85+55, "+1 Rank!", 255, 255, 255, None);
                }

                if game.currentmenuname == MenuName::timetrialcomplete2 || game.currentmenuname == MenuName::timetrialcomplete3 {
                    self.graphics.bigprint( 100, 175, "Rank:", tr, tg, tb, Some(false), Some(2));
                }

                if game.currentmenuname == MenuName::timetrialcomplete3 {
                    match game.timetrialrank {
                        0 => self.graphics.bigprint( 195, 165, "B", 255, 255, 255, Some(false), Some(4)),
                        1 => self.graphics.bigprint( 195, 165, "A", 255, 255, 255, Some(false), Some(4)),
                        2 => self.graphics.bigprint( 195, 165, "S", 255, 255, 255, Some(false), Some(4)),
                        3 => self.graphics.bigprint( 195, 165, "V", 255, 255, 255, Some(false), Some(4)),
                        _ => println!("unknown timetrialrank"),
                    }
                }
            },
            MenuName::unlockmenutrials => {
                self.graphics.bigprint( -1, 30, "Unlock Time Trials", tr, tg, tb, Some(true), None);
                self.graphics.print( -1, 65, "You can unlock each time", tr, tg, tb, Some(true));
                self.graphics.print( -1, 75, "trial separately.", tr, tg, tb, Some(true));
            },
            MenuName::timetrials => {
                match game.currentmenuoption {
                    0 => {
                        if game.unlock[9] {
                            self.graphics.bigprint( -1, 30, "Space Station 1", tr, tg, tb, Some(true), None);
                            if game.besttimes[0] == -1 {
                                self.graphics.print( -1, 75, "Not yet attempted", tr, tg, tb, Some(true));
                            } else {
                                self.graphics.print( 16, 65, "BEST TIME  ", tr, tg, tb, None);
                                self.graphics.print( 16, 75, "BEST SHINY ", tr, tg, tb, None);
                                self.graphics.print( 16, 85, "BEST LIVES ", tr, tg, tb, None);
                                self.graphics.print( 110, 65, game.timetstring(game.besttimes[0]), tr, tg, tb, None);
                                self.graphics.print( 110, 75, &(game.besttrinkets[0].to_string() + "/2"), tr, tg, tb, None);
                                self.graphics.print( 110, 85, &game.bestlives[0].to_string(), tr, tg, tb, None);

                                self.graphics.print( 170, 65, "PAR TIME    1:15", tr, tg, tb, None);
                                self.graphics.print( 170, 85, "Best Rank", tr, tg, tb, None);
                                match game.bestrank[0] {
                                    0 => self.graphics.bigprint( 275, 82, "B", 225, 225, 225, None, None),
                                    1 => self.graphics.bigprint( 275, 82, "A", 225, 225, 225, None, None),
                                    2 => self.graphics.bigprint( 275, 82, "S", 225, 225, 225, None, None),
                                    3 => self.graphics.bigprint( 275, 82, "V", 225, 225, 225, None, None),
                                    _ => println!("unkown bestrank"),
                                };
                            }
                        } else {
                            self.graphics.bigprint( -1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print( -1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 75, "Rescue Violet", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 85, "Find three trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    1 => {
                        if game.unlock[10] {
                            self.graphics.bigprint( -1, 30, "The Laboratory", tr, tg, tb, Some(true), None);
                            if game.besttimes[1] == -1 {
                                self.graphics.print( -1, 75, "Not yet attempted", tr, tg, tb, Some(true));
                            } else {
                                self.graphics.print( 16, 65, "BEST TIME  ", tr, tg, tb, None);
                                self.graphics.print( 16, 75, "BEST SHINY ", tr, tg, tb, None);
                                self.graphics.print( 16, 85, "BEST LIVES ", tr, tg, tb, None);
                                self.graphics.print( 110, 65, game.timetstring(game.besttimes[1]), tr, tg, tb, None);
                                self.graphics.print( 110, 75, &(game.besttrinkets[1].to_string() + "/4"), tr, tg, tb, None);
                                self.graphics.print( 110, 85, &game.bestlives[1].to_string(), tr, tg, tb, None);


                                self.graphics.print( 170, 65, "PAR TIME    2:45", tr, tg, tb, None);
                                self.graphics.print( 170, 85, "Best Rank", tr, tg, tb, None);
                                match game.bestrank[1] {
                                    0 => self.graphics.bigprint( 275, 82, "B", 225, 225, 225, None, None),
                                    1 => self.graphics.bigprint( 275, 82, "A", 225, 225, 225, None, None),
                                    2 => self.graphics.bigprint( 275, 82, "S", 225, 225, 225, None, None),
                                    3 => self.graphics.bigprint( 275, 82, "V", 225, 225, 225, None, None),
                                    _ => println!("unkown bestrank"),
                                };
                            }
                        } else {
                            self.graphics.bigprint( -1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print( -1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 75, "Rescue Victoria", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 85, "Find six trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    2 => {
                        if game.unlock[11] {
                            self.graphics.bigprint( -1, 30, "The Tower", tr, tg, tb, Some(true), None);
                            if game.besttimes[2] == -1 {
                                self.graphics.print( -1, 75, "Not yet attempted", tr, tg, tb, Some(true));
                            } else {
                                self.graphics.print( 16, 65, "BEST TIME  ", tr, tg, tb, None);
                                self.graphics.print( 16, 75, "BEST SHINY ", tr, tg, tb, None);
                                self.graphics.print( 16, 85, "BEST LIVES ", tr, tg, tb, None);
                                self.graphics.print( 110, 65, game.timetstring(game.besttimes[2]), tr, tg, tb, None);
                                self.graphics.print( 110, 75, &(game.besttrinkets[2].to_string() + "/2"), tr, tg, tb, None);
                                self.graphics.print( 110, 85, &game.bestlives[2].to_string(), tr, tg, tb, None);


                                self.graphics.print( 170, 65, "PAR TIME    1:45", tr, tg, tb, None);
                                self.graphics.print( 170, 85, "Best Rank", tr, tg, tb, None);
                                match game.bestrank[2] {
                                    0 => self.graphics.bigprint( 275, 82, "B", 225, 225, 225, None, None),
                                    1 => self.graphics.bigprint( 275, 82, "A", 225, 225, 225, None, None),
                                    2 => self.graphics.bigprint( 275, 82, "S", 225, 225, 225, None, None),
                                    3 => self.graphics.bigprint( 275, 82, "V", 225, 225, 225, None, None),
                                    _ => println!("unkown bestrank"),
                                }
                            }

                        } else {
                            self.graphics.bigprint( -1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print( -1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 75, "Rescue Vermilion", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 85, "Find nine trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    3 => {
                        if game.unlock[12] {
                            self.graphics.bigprint( -1, 30, "Space Station 2", tr, tg, tb, Some(true), None);
                            if game.besttimes[3] == -1 {
                                self.graphics.print( -1, 75, "Not yet attempted", tr, tg, tb, Some(true));
                            } else {
                                self.graphics.print( 16, 65, "BEST TIME  ", tr, tg, tb, None);
                                self.graphics.print( 16, 75, "BEST SHINY ", tr, tg, tb, None);
                                self.graphics.print( 16, 85, "BEST LIVES ", tr, tg, tb, None);
                                self.graphics.print( 110, 65, game.timetstring(game.besttimes[3]), tr, tg, tb, None);
                                self.graphics.print( 110, 75, &(game.besttrinkets[3].to_string() + "/5"), tr, tg, tb, None);
                                self.graphics.print( 110, 85, &game.bestlives[3].to_string(), tr, tg, tb, None);


                                self.graphics.print( 170, 65, "PAR TIME    3:20", tr, tg, tb, None);
                                self.graphics.print( 170, 85, "Best Rank", tr, tg, tb, None);
                                match game.bestrank[3] {
                                    0 => self.graphics.bigprint( 275, 82, "B", 225, 225, 225, None, None),
                                    1 => self.graphics.bigprint( 275, 82, "A", 225, 225, 225, None, None),
                                    2 => self.graphics.bigprint( 275, 82, "S", 225, 225, 225, None, None),
                                    3 => self.graphics.bigprint( 275, 82, "V", 225, 225, 225, None, None),
                                    _ => println!("unkown bestrank"),
                                }
                            }

                        } else {
                            self.graphics.bigprint( -1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print( -1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 75, "Rescue Vitellary", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 85, "Find twelve trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    4 => {
                        if game.unlock[13] {
                            self.graphics.bigprint( -1, 30, "The Warp Zone", tr, tg, tb, Some(true), None);
                            if game.besttimes[4] == -1 {
                                self.graphics.print( -1, 75, "Not yet attempted", tr, tg, tb, Some(true));
                            } else {
                                self.graphics.print( 16, 65, "BEST TIME  ", tr, tg, tb, None);
                                self.graphics.print( 16, 75, "BEST SHINY ", tr, tg, tb, None);
                                self.graphics.print( 16, 85, "BEST LIVES ", tr, tg, tb, None);
                                self.graphics.print( 110, 65, game.timetstring(game.besttimes[4]), tr, tg, tb, None);
                                self.graphics.print( 110, 75, &(game.besttrinkets[4].to_string() + "/1"), tr, tg, tb, None);
                                self.graphics.print( 110, 85, &game.bestlives[4].to_string(), tr, tg, tb, None);


                                self.graphics.print( 170, 65, "PAR TIME    2:00", tr, tg, tb, None);
                                self.graphics.print( 170, 85, "Best Rank", tr, tg, tb, None);
                                match game.bestrank[4] {
                                    0 => self.graphics.bigprint( 275, 82, "B", 225, 225, 225, None, None),
                                    1 => self.graphics.bigprint( 275, 82, "A", 225, 225, 225, None, None),
                                    2 => self.graphics.bigprint( 275, 82, "S", 225, 225, 225, None, None),
                                    3 => self.graphics.bigprint( 275, 82, "V", 225, 225, 225, None, None),
                                    _ => println!("incorrect bestrank option"),
                                };
                            }
                        } else {
                            self.graphics.bigprint( -1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print( -1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 75, "Rescue Verdigris", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 85, "Find fifteen trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    5 => {
                        if game.unlock[14] {
                            self.graphics.bigprint( -1, 30, "The Final Level", tr, tg, tb, Some(true), None);
                            if game.besttimes[5] == -1 {
                                self.graphics.print( -1, 75, "Not yet attempted", tr, tg, tb, Some(true));
                            } else {
                                self.graphics.print( 16, 65, "BEST TIME  ", tr, tg, tb, None);
                                self.graphics.print( 16, 75, "BEST SHINY ", tr, tg, tb, None);
                                self.graphics.print( 16, 85, "BEST LIVES ", tr, tg, tb, None);
                                self.graphics.print( 110, 65, game.timetstring(game.besttimes[5]), tr, tg, tb, None);
                                self.graphics.print( 110, 75, &(game.besttrinkets[5].to_string() + "/1"), tr, tg, tb, None);
                                self.graphics.print( 110, 85, &game.bestlives[5].to_string(), tr, tg, tb, None);


                                self.graphics.print( 170, 65, "PAR TIME    2:15", tr, tg, tb, None);
                                self.graphics.print( 170, 85, "Best Rank", tr, tg, tb, None);
                                match game.bestrank[5] {
                                    0 => self.graphics.bigprint( 275, 82, "B", 225, 225, 225, None, None),
                                    1 => self.graphics.bigprint( 275, 82, "A", 225, 225, 225, None, None),
                                    2 => self.graphics.bigprint( 275, 82, "S", 225, 225, 225, None, None),
                                    3 => self.graphics.bigprint( 275, 82, "V", 225, 225, 225, None, None),
                                    _ => println!("unkown bestrank"),
                                }
                            }
                        } else {
                            self.graphics.bigprint( -1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print( -1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 75, "Complete the game", tr, tg, tb, Some(true));
                            self.graphics.print( -1, 85, "Find eighteen trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    _ => println!("incorrect time trials option"),
                }
            },
            MenuName::gamecompletecontinue => {
                self.graphics.bigprint( -1, 25, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print( -1, 45, "Your save files have been updated.", tr, tg, tb, Some(true));

                self.graphics.print( -1, 110, "If you want to keep exploring", tr, tg, tb, Some(true));
                self.graphics.print( -1, 120, "the game, select CONTINUE", tr, tg, tb, Some(true));
                self.graphics.print( -1, 130, "from the play menu.", tr, tg, tb, Some(true));
            },
            MenuName::unlockmenu => {
                self.graphics.bigprint( -1, 25, "Unlock Play Modes", tr, tg, tb, Some(true), Some(2));

                self.graphics.print( -1, 55, "From here, you may unlock parts", tr, tg, tb, Some(true));
                self.graphics.print( -1, 65, "of the game that are normally", tr, tg, tb, Some(true));
                self.graphics.print( -1, 75, "unlocked as you play.", tr, tg, tb, Some(true));
            },
            MenuName::unlocktimetrial => {
                self.graphics.bigprint( -1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print( -1, 125, "You have unlocked", tr, tg, tb, Some(true));
                self.graphics.print( -1, 135, "a new Time Trial.", tr, tg, tb, Some(true));
            },
            MenuName::unlocktimetrials => {
                self.graphics.bigprint( -1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print( -1, 125, "You have unlocked some", tr, tg, tb, Some(true));
                self.graphics.print( -1, 135, "new Time Trials.", tr, tg, tb, Some(true));
            },
            MenuName::unlocknodeathmode => {
                self.graphics.bigprint( -1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print( -1, 125, "You have unlocked", tr, tg, tb, Some(true));
                self.graphics.print( -1, 135, "No Death Mode.", tr, tg, tb, Some(true));
            },
            MenuName::unlockflipmode => {
                self.graphics.bigprint( -1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print( -1, 125, "You have unlocked", tr, tg, tb, Some(true));
                self.graphics.print( -1, 135, "Flip Mode.", tr, tg, tb, Some(true));
            },
            MenuName::unlockintermission => {
                self.graphics.bigprint( -1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print( -1, 125, "You have unlocked", tr, tg, tb, Some(true));
                self.graphics.print( -1, 135, "the intermission levels.", tr, tg, tb, Some(true));
            },
            // MenuName::playerworlds => {
            //     let tempstring = FILESYSTEM_getUserLevelDirectory();
            //     if tempstring.len() > 80 {
            //         self.graphics.print( -1, 160, "To install new player levels, copy", tr, tg, tb, Some(true));
            //         self.graphics.print( -1, 170, "the .vvvvvv files to this folder:", tr, tg, tb, Some(true));
            //         self.graphics.print( 320-((tempstring.len()-80)*8), 190, tempstring.substr(0,tempstring.len()-80), tr, tg, tb, None);
            //         self.graphics.print( 0, 200, tempstring.substr(tempstring.len()-80,40), tr, tg, tb, None);
            //         self.graphics.print( 0, 210, tempstring.substr(tempstring.len()-40,40), tr, tg, tb, None);
            //     } else if tempstring.len() > 40 {
            //         self.graphics.print( -1, 170, "To install new player levels, copy", tr, tg, tb, Some(true));
            //         self.graphics.print( -1, 180, "the .vvvvvv files to this folder:", tr, tg, tb, Some(true));
            //         self.graphics.print( 320-((tempstring.len()-40)*8), 200, tempstring.substr(0,tempstring.len()-40), tr, tg, tb, None);
            //         self.graphics.print( 0, 210, tempstring.substr(tempstring.len()-40,40), tr, tg, tb, None);
            //     } else {
            //         self.graphics.print( -1, 180, "To install new player levels, copy", tr, tg, tb, Some(true));
            //         self.graphics.print( -1, 190, "the .vvvvvv files to this folder:", tr, tg, tb, Some(true));
            //         self.graphics.print( 320-(tempstring.len()*8), 210, tempstring, tr, tg, tb, None);
            //     }
            // },
            MenuName::errorsavingsettings => {
                self.graphics.print( -1, 95, "ERROR: Could not save settings file!", tr, tg, tb, Some(true));
            },
            _ => println!("{:?} menuen not implemented yet", game.currentmenuname),
        }
    }

    // void titlerender(void)
    pub fn titlerender(&mut self, game: &mut game::Game, music: &music::Music, map: &map::Map, help: &utility_class::UtilityClass, key: &key_poll::KeyPoll, screen_params: crate::screen::ScreenParams) -> Option<RenderResult> {
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
            // self.graphics.print(-1,temp+35,"     MAKE AND PLAY EDITION", self.tr, self.tg, self.tb, Some(true));
            // #endif

            self.graphics.print(5, 175, "[ Press ACTION to Start ]", self.tr, self.tg, self.tb, Some(true));
            self.graphics.print(5, 195, "ACTION = Space, Z, or V", self.tr/2, self.tg/2, self.tb/2, Some(true));
        } else {
            if !game.colourblindmode {
                self.graphics.drawtowerbackground(BackGround::Title);
            }

            self.menurender(game, music, map, help, key, screen_params);

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
    pub fn gamerender(&mut self) -> Option<RenderResult> {
        println!("DEADBEEF: Render::gamerender() method not implemented yet");
        None
    }

    // void maprender(void)
    // void teleporterrender(void)
}
