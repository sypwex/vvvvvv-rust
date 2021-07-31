use crate::{entity, game::{self, MenuName}, key_poll, map, music, scenes::RenderResult, script, utility_class};

use self::graphics::graphics_util;
pub mod graphics;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BackGround {
    Title,
    Tower,
}

// static int inline FLIP(int ypos)
macro_rules! FLIP {
    ($ypos:tt) => {
        match graphics.flipmode {
            true => 220 - ypos,
            false => ypos,
        }
    };
}

pub struct Render {
    pub graphics: graphics::Graphics,
    tr: i32,
    tg: i32,
    tb: i32,
}

impl Render {
    pub fn new(pf: sdl2::pixels::PixelFormatEnum) -> Render {
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
                self.graphics.drawsprite_rgb((160 - 96) + 0 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.drawsprite_rgb((160 - 96) + 1 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.drawsprite_rgb((160 - 96) + 2 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.drawsprite_rgb((160 - 96) + 3 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.drawsprite_rgb((160 - 96) + 4 * 32, temp, vsprite, tr, tg, tb);
                self.graphics.drawsprite_rgb((160 - 96) + 5 * 32, temp, vsprite, tr, tg, tb);

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
                self.graphics.print(-1, 65, "ERROR: This level has", tr, tg, tb, Some(true));
                self.graphics.print(-1, 75, "no start point!", tr, tg, tb, Some(true));
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
                        self.graphics.bigprint(-1, 30, "Toggle Fullscreen", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Change to fullscreen/windowed mode.", tr, tg, tb, Some(true));

                        if screen_params.isWindowed {
                            self.graphics.print(-1, 85, "Current mode: WINDOWED", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 85, "Current mode: FULLSCREEN", tr, tg, tb, Some(true));
                        }
                    },
                    1 => {
                        self.graphics.bigprint(-1, 30, "Scaling Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Choose letterbox/stretch/integer mode.", tr, tg, tb, Some(true));

                        match screen_params.stretchMode {
                            2 => self.graphics.print(-1, 85, "Current mode: INTEGER", tr, tg, tb, Some(true)),
                            1 => self.graphics.print(-1, 85, "Current mode: STRETCH", tr, tg, tb, Some(true)),
                            _ => self.graphics.print(-1, 85, "Current mode: LETTERBOX", tr, tg, tb, Some(true)),
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
                        self.graphics.bigprint(-1, 30, "Toggle Filter", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Change to nearest/linear filter.", tr, tg, tb, Some(true));

                        if screen_params.isFiltered {
                            self.graphics.print(-1, 85, "Current mode: LINEAR", tr, tg, tb, Some(true));
                        } else {
                            self.graphics.print(-1, 85, "Current mode: NEAREST", tr, tg, tb, Some(true));
                        }
                    },
                    4 => {
                        self.graphics.bigprint(-1, 30, "Analogue Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "There is nothing wrong with your", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "television set. Do not attempt to", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 85, "adjust the picture.", tr, tg, tb, Some(true));
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
                self.graphics.print(-1, 50, "VVVVVV is a game by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 65, "Terry Cavanagh", tr, tg, tb, Some(true), Some(2));

                self.graphics.drawimagecol(7, -1, 86, Some((tr as f32 * 0.75) as i32), Some((tg as f32 * 0.75) as i32), Some((tb as f32 * 0.75) as i32), Some(true));

                self.graphics.print(-1, 120, "and features music by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 135, "Magnus P~lsson", tr, tg, tb, Some(true), Some(2));
                self.graphics.drawimagecol(8, -1, 156, Some((tr as f32 * 0.75) as i32), Some((tg as f32 * 0.75) as i32), Some((tb as f32 * 0.75) as i32), Some(true));
            },
            MenuName::credits2 => {
                self.graphics.print(-1, 50, "Roomnames are by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 65, "Bennett Foddy", tr, tg, tb, Some(true), None);
                self.graphics.drawimagecol(9, -1, 86, Some((tr as f32 * 0.75) as i32), Some((tg as f32 * 0.75) as i32), Some((tb as f32 * 0.75) as i32), Some(true));
                self.graphics.print(-1, 110, "C++ version by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 125, "Simon Roth", tr, tg, tb, Some(true), None);
                self.graphics.bigprint( 40, 145, "Ethan Lee", tr, tg, tb, Some(true), None);
            },
            MenuName::credits25 => {
                self.graphics.print(-1, 40, "Beta Testing by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 55, "Sam Kaplan", tr, tg, tb, Some(true), None);
                self.graphics.bigprint( 40, 75, "Pauli Kohberger", tr, tg, tb, Some(true), None);
                self.graphics.print(-1, 130, "Ending Picture by", tr, tg, tb, Some(true));
                self.graphics.bigprint( 40, 145, "Pauli Kohberger", tr, tg, tb, Some(true), None);
            },
            MenuName::credits3 => {
                self.graphics.print(-1, 20, "VVVVVV is supported by", tr, tg, tb, Some(true));
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
                self.graphics.print(-1, 20, "and also by", tr, tg, tb, Some(true));

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
                self.graphics.print(-1, 20, "With contributions on", tr, tg, tb, Some(true));
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
                self.graphics.print(-1, 20, "and thanks also to:", tr, tg, tb, Some(true));

                self.graphics.bigprint(80, 60, "You!", tr, tg, tb, Some(true), None);

                self.graphics.print( 80, 100, "Your support makes it possible", tr, tg, tb, Some(true));
                self.graphics.print( 80, 110,"for me to continue making the", tr, tg, tb, Some(true));
                self.graphics.print( 80, 120,"games I want to make, now", tr, tg, tb, Some(true));
                self.graphics.print( 80, 130, "and into the future.", tr, tg, tb, Some(true));

                self.graphics.print( 80, 150,"Thank you!", tr, tg, tb, Some(true));
            },
            MenuName::setinvincibility => {
                self.graphics.print(-1, 100, "Are you sure you want to ", tr, tg, tb, Some(true));
                self.graphics.print(-1, 110, "enable invincibility?", tr, tg, tb, Some(true));
            },
            MenuName::setslowdown => {
                self.graphics.bigprint(-1, 40, "Game Speed", tr, tg, tb, Some(true), None);
                self.graphics.print(-1, 75, "Select a new game speed below.", tr, tg, tb, Some(true));
                self.drawslowdowntext();
            },
            MenuName::newgamewarning => {
                self.graphics.print(-1, 100, "Are you sure? This will", tr, tg, tb, Some(true));
                self.graphics.print(-1, 110, "delete your current saves...", tr, tg, tb, Some(true));
            },
            MenuName::cleardatamenu => {
                self.graphics.print(-1, 100, "Are you sure you want to", tr, tg, tb, Some(true));
                self.graphics.print(-1, 110, "delete all your saved data?", tr, tg, tb, Some(true));
            },
            MenuName::startnodeathmode => {
                self.graphics.print(-1, 45, "Good luck!", tr, tg, tb, Some(true));
                self.graphics.print(-1, 80, "You cannot save in this mode.", tr, tg, tb, Some(true));
                self.graphics.print(-1, 100, "Would you like to disable the", tr, tg, tb, Some(true));
                self.graphics.print(-1, 112, "cutscenes during the game?", tr, tg, tb, Some(true));
            },
            MenuName::controller => {
                self.graphics.bigprint(-1, 30, "Game Pad", tr, tg, tb, Some(true), None);
                self.graphics.print(-1, 55, "Change controller options.", tr, tg, tb, Some(true));
                match game.currentmenuoption {
                    0 => {
                        match key.sensitivity {
                            0 => {
                                self.graphics.print(-1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print(-1, 95, "[]..................", tr, tg, tb, Some(true));
                            },
                            1 => {
                                self.graphics.print(-1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print(-1, 95, ".....[].............", tr, tg, tb, Some(true));
                            },
                            2 => {
                                self.graphics.print(-1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print(-1, 95, ".........[].........", tr, tg, tb, Some(true));
                            },
                            3 => {
                                self.graphics.print(-1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print(-1, 95, ".............[].....", tr, tg, tb, Some(true));
                            },
                            4 => {
                                self.graphics.print(-1, 85, " Low     Medium     High", tr, tg, tb, Some(true));
                                self.graphics.print(-1, 95, "..................[]", tr, tg, tb, Some(true));
                            },
                            _ => println!("incorrect sensitivity"),
                        }
                    },
                    1 | 2 | 3 | 4 => {
                        let s = format!("Flip is bound to: {}", help.GCString(&game.controllerButton_flip));
                        self.graphics.print(-1, 85, &s, tr, tg, tb, Some(true));
                        let s = format!("Enter is bound to: {}",  help.GCString(&game.controllerButton_map));
                        self.graphics.print(-1, 95, &s, tr, tg, tb, Some(true));
                        let s = format!("Menu is bound to: {}", help.GCString(&game.controllerButton_esc));
                        self.graphics.print(-1, 105, &s, tr, tg, tb, Some(true));
                        let s = format!("Restart is bound to: {}", help.GCString(&game.controllerButton_restart));
                        self.graphics.print(-1, 115, &s, tr, tg, tb, Some(true));
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
                        self.graphics.bigprint(-1, 30, "Unfocus Pause", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Toggle if the game will pause", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "when the window is unfocused.", tr, tg, tb, Some(true));
                        if game.disablepause {
                            self.graphics.print(-1, 95, "Unfocus pause is OFF", tr/2, tg/2, tb/2, Some(true));
                        } else {
                            self.graphics.print(-1, 95, "Unfocus pause is ON", tr, tg, tb, Some(true));
                        }
                    },
                    1 => {
                        self.graphics.bigprint(-1, 30, "Room Name BG", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Lets you see through what is behind", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "the name at the bottom of the screen.", tr, tg, tb, Some(true));
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
                self.graphics.print(-1, 65, "Who do you want to play", tr, tg, tb, Some(true));
                self.graphics.print(-1, 75, "the level with?", tr, tg, tb, Some(true));
            },
            MenuName::playmodes => {
                match game.currentmenuoption {
                    0 => {
                        self.graphics.bigprint(-1, 30, "Time Trials", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Replay any level in the game in", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "a competitive time trial mode.", tr, tg, tb, Some(true));

                        if game.slowdown < 30 || map.invincibility {
                            self.graphics.print(-1, 105, "Time Trials are not available", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 115, "with slowdown or invincibility.", tr, tg, tb, Some(true));
                        }
                    },
                    1 => {
                        self.graphics.bigprint(-1, 30, "Intermissions", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Replay the intermission levels.", tr, tg, tb, Some(true));

                        if !game.unlock[15] && !game.unlock[16] {
                            self.graphics.print(-1, 95, "TO UNLOCK: Complete the", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 105, "intermission levels in-game.", tr, tg, tb, Some(true));
                        }
                    },
                    2 => {
                        self.graphics.bigprint(-1, 30, "No Death Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Play the entire game", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "without dying once.", tr, tg, tb, Some(true));

                        if game.slowdown < 30 || map.invincibility {
                            self.graphics.print(-1, 105, "No Death Mode is not available", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 115, "with slowdown or invincibility.", tr, tg, tb, Some(true));
                        }
                        else if !game.unlock[17] {
                            self.graphics.print(-1, 105, "TO UNLOCK: Achieve an S-rank or", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 115, "above in at least 4 time trials.", tr, tg, tb, Some(true));
                        }
                    },
                    3 => {
                        // WARNING: Partially duplicated in MenuName::options
                        self.graphics.bigprint(-1, 30, "Flip Mode", tr, tg, tb, Some(true), None);
                        self.graphics.print(-1, 65, "Flip the entire game vertically.", tr, tg, tb, Some(true));
                        self.graphics.print(-1, 75, "Compatible with other game modes.", tr, tg, tb, Some(true));

                        if game.unlock[18] {
                            if self.graphics.setflipmode {
                                self.graphics.print(-1, 105, "Currently ENABLED!", tr, tg, tb, Some(true));
                            } else {
                                self.graphics.print(-1, 105, "Currently Disabled.", tr/2, tg/2, tb/2, Some(true));
                            }
                        } else {
                            self.graphics.print(-1, 105, "TO UNLOCK: Complete the game.", tr, tg, tb, Some(true));
                        }
                    },
                    _ => println!("incorrect play mode"),
                };
            },
            MenuName::youwannaquit => {
                self.graphics.print(-1, 75, "Are you sure you want to quit?", tr, tg, tb, Some(true));
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
                        self.graphics.print(262-graphics::Graphics::len(&trinketcount), 132-20, &trinketcount, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);

                        self.graphics.drawsprite_clru32(34, 126-20, 50, self.graphics.col_clock.colour);
                        self.graphics.drawsprite_clru32(270, 126-20, 22, self.graphics.col_trinket.colour);
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
                        self.graphics.print(262-graphics::Graphics::len(&trinketcount), 132-20, &trinketcount, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);

                        self.graphics.drawsprite_clru32(34, 126-20, 50, self.graphics.col_clock.colour);
                        self.graphics.drawsprite_clru32(270, 126-20, 22, self.graphics.col_trinket.colour);
                    },
                    _ => println!("incorrect continue menu option"),
                };
            },
            MenuName::gameover | MenuName::gameover2 => {
                self.graphics.bigprint(-1, 25, "GAME OVER", tr, tg, tb, Some(true), Some(3));

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
                self.graphics.bigprint(-1, 8, "WOW", tr, tg, tb, Some(true), Some(4));

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
                self.graphics.bigprint(-1, 20, "Results", tr, tg, tb, Some(true), Some(3));

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
                self.graphics.bigprint(-1, 30, "Unlock Time Trials", tr, tg, tb, Some(true), None);
                self.graphics.print(-1, 65, "You can unlock each time", tr, tg, tb, Some(true));
                self.graphics.print(-1, 75, "trial separately.", tr, tg, tb, Some(true));
            },
            MenuName::timetrials => {
                match game.currentmenuoption {
                    0 => {
                        if game.unlock[9] {
                            self.graphics.bigprint(-1, 30, "Space Station 1", tr, tg, tb, Some(true), None);
                            if game.besttimes[0] == -1 {
                                self.graphics.print(-1, 75, "Not yet attempted", tr, tg, tb, Some(true));
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
                            self.graphics.bigprint(-1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print(-1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 75, "Rescue Violet", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 85, "Find three trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    1 => {
                        if game.unlock[10] {
                            self.graphics.bigprint(-1, 30, "The Laboratory", tr, tg, tb, Some(true), None);
                            if game.besttimes[1] == -1 {
                                self.graphics.print(-1, 75, "Not yet attempted", tr, tg, tb, Some(true));
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
                            self.graphics.bigprint(-1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print(-1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 75, "Rescue Victoria", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 85, "Find six trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    2 => {
                        if game.unlock[11] {
                            self.graphics.bigprint(-1, 30, "The Tower", tr, tg, tb, Some(true), None);
                            if game.besttimes[2] == -1 {
                                self.graphics.print(-1, 75, "Not yet attempted", tr, tg, tb, Some(true));
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
                            self.graphics.bigprint(-1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print(-1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 75, "Rescue Vermilion", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 85, "Find nine trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    3 => {
                        if game.unlock[12] {
                            self.graphics.bigprint(-1, 30, "Space Station 2", tr, tg, tb, Some(true), None);
                            if game.besttimes[3] == -1 {
                                self.graphics.print(-1, 75, "Not yet attempted", tr, tg, tb, Some(true));
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
                            self.graphics.bigprint(-1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print(-1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 75, "Rescue Vitellary", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 85, "Find twelve trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    4 => {
                        if game.unlock[13] {
                            self.graphics.bigprint(-1, 30, "The Warp Zone", tr, tg, tb, Some(true), None);
                            if game.besttimes[4] == -1 {
                                self.graphics.print(-1, 75, "Not yet attempted", tr, tg, tb, Some(true));
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
                            self.graphics.bigprint(-1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print(-1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 75, "Rescue Verdigris", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 85, "Find fifteen trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    5 => {
                        if game.unlock[14] {
                            self.graphics.bigprint(-1, 30, "The Final Level", tr, tg, tb, Some(true), None);
                            if game.besttimes[5] == -1 {
                                self.graphics.print(-1, 75, "Not yet attempted", tr, tg, tb, Some(true));
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
                            self.graphics.bigprint(-1, 30, "???", tr, tg, tb, Some(true), None);
                            self.graphics.print(-1, 60, "TO UNLOCK:", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 75, "Complete the game", tr, tg, tb, Some(true));
                            self.graphics.print(-1, 85, "Find eighteen trinkets", tr, tg, tb, Some(true));
                        }
                    },
                    _ => println!("incorrect time trials option"),
                }
            },
            MenuName::gamecompletecontinue => {
                self.graphics.bigprint(-1, 25, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print(-1, 45, "Your save files have been updated.", tr, tg, tb, Some(true));

                self.graphics.print(-1, 110, "If you want to keep exploring", tr, tg, tb, Some(true));
                self.graphics.print(-1, 120, "the game, select CONTINUE", tr, tg, tb, Some(true));
                self.graphics.print(-1, 130, "from the play menu.", tr, tg, tb, Some(true));
            },
            MenuName::unlockmenu => {
                self.graphics.bigprint(-1, 25, "Unlock Play Modes", tr, tg, tb, Some(true), Some(2));

                self.graphics.print(-1, 55, "From here, you may unlock parts", tr, tg, tb, Some(true));
                self.graphics.print(-1, 65, "of the game that are normally", tr, tg, tb, Some(true));
                self.graphics.print(-1, 75, "unlocked as you play.", tr, tg, tb, Some(true));
            },
            MenuName::unlocktimetrial => {
                self.graphics.bigprint(-1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print(-1, 125, "You have unlocked", tr, tg, tb, Some(true));
                self.graphics.print(-1, 135, "a new Time Trial.", tr, tg, tb, Some(true));
            },
            MenuName::unlocktimetrials => {
                self.graphics.bigprint(-1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print(-1, 125, "You have unlocked some", tr, tg, tb, Some(true));
                self.graphics.print(-1, 135, "new Time Trials.", tr, tg, tb, Some(true));
            },
            MenuName::unlocknodeathmode => {
                self.graphics.bigprint(-1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print(-1, 125, "You have unlocked", tr, tg, tb, Some(true));
                self.graphics.print(-1, 135, "No Death Mode.", tr, tg, tb, Some(true));
            },
            MenuName::unlockflipmode => {
                self.graphics.bigprint(-1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print(-1, 125, "You have unlocked", tr, tg, tb, Some(true));
                self.graphics.print(-1, 135, "Flip Mode.", tr, tg, tb, Some(true));
            },
            MenuName::unlockintermission => {
                self.graphics.bigprint(-1, 45, "Congratulations!", tr, tg, tb, Some(true), Some(2));

                self.graphics.print(-1, 125, "You have unlocked", tr, tg, tb, Some(true));
                self.graphics.print(-1, 135, "the intermission levels.", tr, tg, tb, Some(true));
            },
            // MenuName::playerworlds => {
            //     let tempstring = FILESYSTEM_getUserLevelDirectory();
            //     if tempstring.len() > 80 {
            //         self.graphics.print(-1, 160, "To install new player levels, copy", tr, tg, tb, Some(true));
            //         self.graphics.print(-1, 170, "the .vvvvvv files to this folder:", tr, tg, tb, Some(true));
            //         self.graphics.print( 320-((tempstring.len()-80)*8), 190, tempstring.substr(0,tempstring.len()-80), tr, tg, tb, None);
            //         self.graphics.print( 0, 200, tempstring.substr(tempstring.len()-80,40), tr, tg, tb, None);
            //         self.graphics.print( 0, 210, tempstring.substr(tempstring.len()-40,40), tr, tg, tb, None);
            //     } else if tempstring.len() > 40 {
            //         self.graphics.print(-1, 170, "To install new player levels, copy", tr, tg, tb, Some(true));
            //         self.graphics.print(-1, 180, "the .vvvvvv files to this folder:", tr, tg, tb, Some(true));
            //         self.graphics.print( 320-((tempstring.len()-40)*8), 200, tempstring.substr(0,tempstring.len()-40), tr, tg, tb, None);
            //         self.graphics.print( 0, 210, tempstring.substr(tempstring.len()-40,40), tr, tg, tb, None);
            //     } else {
            //         self.graphics.print(-1, 180, "To install new player levels, copy", tr, tg, tb, Some(true));
            //         self.graphics.print(-1, 190, "the .vvvvvv files to this folder:", tr, tg, tb, Some(true));
            //         self.graphics.print( 320-(tempstring.len()*8), 210, tempstring, tr, tg, tb, None);
            //     }
            // },
            MenuName::errorsavingsettings => {
                self.graphics.print(-1, 95, "ERROR: Could not save settings file!", tr, tg, tb, Some(true));
            },
            _ => println!("{:?} menuen not implemented yet", game.currentmenuname),
        }
    }

    // void titlerender(void)
    pub fn titlerender(&mut self, game: &mut game::Game, music: &music::Music, map: &map::Map, help: &utility_class::UtilityClass, key: &key_poll::KeyPoll, screen_params: crate::screen::ScreenParams) -> Result<Option<RenderResult>, i32> {
        self.graphics.buffers.fill_back_buffer_with_color(sdl2::pixels::Color::BLACK);

        if !game.menustart {
            self.tr = self.graphics.col_tr;
            self.tg = self.graphics.col_tg;
            self.tb = self.graphics.col_tb;

            let temp = 50;
            let vsprite = 27; // 23
            self.graphics.drawsprite_rgb((160 - 96) + 0 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.drawsprite_rgb((160 - 96) + 1 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.drawsprite_rgb((160 - 96) + 2 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.drawsprite_rgb((160 - 96) + 3 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.drawsprite_rgb((160 - 96) + 4 * 32, temp, vsprite, self.tr, self.tg, self.tb);
            self.graphics.drawsprite_rgb((160 - 96) + 5 * 32, temp, vsprite, self.tr, self.tg, self.tb);
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

        Ok(Some(RenderResult::WithScreenEffects))
    }

    // void gamecompleterender(void)
    // void gamecompleterender2(void)

    // void gamerender(void)
    pub fn gamerender(&mut self, game: &mut game::Game, map: &mut map::Map, help: &mut utility_class::UtilityClass, obj: &mut entity::EntityClass) -> Result<Option<RenderResult>, i32> {
        if !game.blackout {
            if map.towermode {
                if !game.colourblindmode {
                    self.graphics.drawtowerbackground(BackGround::Tower);
                } else {
                    graphics_util::ClearSurface(&mut self.graphics.buffers.backBuffer);
                }
                self.graphics.drawtowermap();
            } else {
                if !game.colourblindmode {
                    self.graphics.drawbackground(map);
                } else {
                    graphics_util::ClearSurface(&mut self.graphics.buffers.backBuffer);
                }
                if map.final_colormode {
                    self.graphics.drawfinalmap(map);
                } else {
                    self.graphics.drawmap(map);
                }
            }

            self.graphics.drawentities(game, obj, map);
            if map.towermode {
                self.graphics.drawtowerspikes();
            }
        }

        if map.extrarow == 0 || (map.custommode && map.roomname != "") {
            self.graphics.footerrect.y = 230;
            if self.graphics.translucentroomname {
                self.graphics.buffers.footerbuffer.blit(None, &mut self.graphics.buffers.backBuffer, self.graphics.footerrect);
            } else {
                graphics_util::FillRect_rect(&mut self.graphics.buffers.backBuffer, self.graphics.footerrect, sdl2::pixels::Color::BLACK);
            }

            if map.finalmode {
                self.graphics.bprint(5, 231, &map.glitchname, 196, 196, 255 - help.glow, Some(true));
            } else {
                self.graphics.bprint(5, 231, &map.roomname, 196, 196, 255 - help.glow, Some(true));
            }
        }

        if map.roomtexton {
            //Draw room text!
            for char in map.roomtext.iter() {
                self.graphics.print(char.x*8, char.y*8, &char.text, 196, 196, 255 - help.glow, None);
            }
        }

        // #if !defined(NO_CUSTOM_LEVELS)
        // if map.custommode && !map.custommodeforreal && !game.advancetext {
        //     //Return to level editor
        //     let alpha = self.graphics.lerp(ed.oldreturneditoralpha, ed.returneditoralpha);
        //     self.graphics.bprintalpha(5, 5, "[Press ENTER to return to editor]", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), alpha, false);
        // }
        // #endif

        self.graphics.cutscenebars();
        self.graphics.drawfade();
        if let Err(s) = self.graphics.buffers.backBuffer.blit(None, &mut self.graphics.buffers.tempBuffer, None) {
            error!("unable to render to screen buffer: {}", s);
        };

        self.graphics.drawgui(help);
        if self.graphics.flipmode {
            if game.advancetext {
                self.graphics.bprint(5, 228, "- Press ACTION to advance text -", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
            }
        } else {
            if game.advancetext {
                self.graphics.bprint(5, 5, "- Press ACTION to advance text -", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
            }
        }

        if game.readytotele > 100 || game.oldreadytotele > 100 {
            let alpha = self.graphics.lerp(game.oldreadytotele as f32, game.readytotele as f32) as i32;
            if self.graphics.flipmode {
                self.graphics.bprint(5, 20, "- Press ENTER to Teleport -", alpha - 20 - (help.glow / 2), alpha - 20 - (help.glow / 2), alpha, Some(true));
            } else {
                self.graphics.bprint(5, 210, "- Press ENTER to Teleport -", alpha - 20 - (help.glow / 2), alpha - 20 - (help.glow / 2), alpha, Some(true));
            }
        }

        if game.swnmode {
            if game.swngame == 0 {
                let tempstring = help.timestring(game.swntimer);
                self.graphics.bigbprint(-1, 20, &tempstring, 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), true, 2);
            } else if game.swngame == 1 {
                if game.swnmessage == 0 {
                    let tempstring = help.timestring(game.swntimer);
                    self.graphics.bprint( 10, 10, "Current Time", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(false));
                    self.graphics.bigbprint( 25, 24, &tempstring, 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), false, 2);

                    let tempstring = help.timestring(game.swnrecord);
                    self.graphics.bprint( 240, 10, "Best Time", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(false));
                    self.graphics.bigbrprint( 300, 24, &tempstring, 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), false, 2f32);

                    match game.swnbestrank {
                        0 => {
                            self.graphics.bprint(-1, 204, "Next Trophy at 5 seconds", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
                        },
                        1 => {
                            self.graphics.bprint(-1, 204, "Next Trophy at 10 seconds", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
                        },
                        2 => {
                            self.graphics.bprint(-1, 204, "Next Trophy at 15 seconds", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
                        },
                        3 => {
                            self.graphics.bprint(-1, 204, "Next Trophy at 20 seconds", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
                        },
                        4 => {
                            self.graphics.bprint(-1, 204, "Next Trophy at 30 seconds", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
                        },
                        5 => {
                            self.graphics.bprint(-1, 204, "Next Trophy at 1 minute", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
                        },
                        6 => {
                            self.graphics.bprint(-1, 204, "All Trophies collected!", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
                        },
                        _ => println!("unknown swnbestrank value"),
                    };
                } else if game.swnmessage == 1 {
                    let tempstring = help.timestring(game.swntimer);
                    self.graphics.bprint( 10, 10, "Current Time", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(false));
                    self.graphics.bigbprint( 25, 24, &tempstring, 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), false, 2);

                    let tempstring = help.timestring(game.swnrecord);
                    if (game.deathseq / 5) % 2 == 1 {
                        self.graphics.bprint( 240, 10, "Best Time", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(false));
                        self.graphics.bigbrprint( 300, 24, &tempstring, 128 - help.glow, 220 - help.glow, 128 - (help.glow / 2), false, 2f32);

                        self.graphics.bigbprint(-1, 200, "New Record!", 128 - help.glow, 220 - help.glow, 128 - (help.glow / 2), true, 2);
                    }
                } else if game.swnmessage >= 2 {
                    game.swnmessage -= 1;
                    if game.swnmessage == 2 { game.swnmessage = 0; }
                    let tempstring = help.timestring(game.swntimer);
                    self.graphics.bprint( 10, 10, "Current Time", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(false));
                    self.graphics.bigbprint( 25, 24, &tempstring, 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), false, 2);

                    let tempstring = help.timestring(game.swnrecord);
                    self.graphics.bprint( 240, 10, "Best Time", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(false));
                    self.graphics.bigbrprint( 300, 24, &tempstring, 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), false, 2f32);

                    if (game.swnmessage / 5) % 2 == 1 {
                        self.graphics.bigbprint(-1, 200, "New Trophy!", 220 - help.glow, 128 - help.glow, 128 - (help.glow / 2), true, 2);
                    }
                }

                self.graphics.bprint(20, 228, "[Press ENTER to stop]", 160 - (help.glow/2), 160 - (help.glow/2), 160 - (help.glow/2), Some(true));
            } else if game.swngame == 2 {
                if (game.swndelay / 15) % 2 == 1 || game.swndelay >= 120 {
                    let y1;
                    let y2;
                    if self.graphics.flipmode {
                        y1 = 30;
                        y2 = 10;
                    } else {
                        y1 = 10;
                        y2 = 30;
                    }
                    self.graphics.bigbprint(-1, y1, "Survive for", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), true, 2);
                    self.graphics.bigbprint(-1, y2, "60 seconds!", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), true, 2);
                }
            } else if game.swngame == 7 {
                if game.swndelay >= 60 {
                    self.graphics.bigbprint(-1, 20, "SUPER GRAVITRON", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), true, 2);

                    let tempstring = help.timestring(game.swnrecord);
                    self.graphics.bprint(240, 190, "Best Time", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true));
                    self.graphics.bigbrprint(300, 205, &tempstring, 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), true, 2f32);
                } else if (game.swndelay / 10) % 2 == 1 {
                    self.graphics.bigbprint(-1, 20, "SUPER GRAVITRON", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), true, 2);
                    self.graphics.bigbprint(-1, 200, "GO!", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), true, 3);
                }
            }
        }

        if game.intimetrial && self.graphics.fademode == 0 {
            //Draw countdown!
            if game.timetrialcountdown > 0 {
                if game.timetrialcountdown < 30 {
                    if (game.timetrialcountdown / 4) % 2 == 0 {
                        self.graphics.bigprint(-1, 100, "Go!", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true), Some(4));
                    }
                } else if game.timetrialcountdown < 60 {
                    self.graphics.bigprint(-1, 100, "1", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true), Some(4));
                } else if game.timetrialcountdown < 90 {
                    self.graphics.bigprint(-1, 100, "2", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true), Some(4));
                } else if game.timetrialcountdown < 120 {
                    self.graphics.bigprint(-1, 100, "3", 220 - help.glow, 220 - help.glow, 255 - (help.glow / 2), Some(true), Some(4));
                }
            } else {
                //Draw OSD stuff
                self.graphics.bprint(6, 18, "TIME :", 255, 255, 255, None);
                self.graphics.bprint(6, 30, "DEATH:", 255, 255, 255, None);
                self.graphics.bprint(6, 42, "SHINY:", 255, 255, 255, None);

                if game.timetrialparlost {
                    self.graphics.bprint(56, 18, &game.timestring(help), 196, 80, 80, None);
                } else {
                    self.graphics.bprint(56, 18, &game.timestring(help), 196, 196, 196, None);
                }

                if game.deathcounts > 0 {
                    self.graphics.bprint(56, 30,&help.String(game.deathcounts), 196, 80, 80, None);
                } else {
                    self.graphics.bprint(56, 30,&help.String(game.deathcounts), 196, 196, 196, None);
                }

                if game.trinkets(obj) < game.timetrialshinytarget {
                    self.graphics.bprint(56, 42,&format!("{}{}{}", help.String(game.trinkets(obj)), " of ", help.String(game.timetrialshinytarget)), 196, 80, 80, None);
                } else {
                    self.graphics.bprint(56, 42,&format!("{}{}{}", help.String(game.trinkets(obj)), " of ", help.String(game.timetrialshinytarget)), 196, 196, 196, None);
                }

                if game.timetrialparlost {
                    self.graphics.bprint(195, 214, "PAR TIME:", 80, 80, 80, None);
                    self.graphics.bprint(275, 214, game.partimestring(), 80, 80, 80, None);
                } else {
                    self.graphics.bprint(195, 214, "PAR TIME:", 255, 255, 255, None);
                    self.graphics.bprint(275, 214, game.partimestring(), 196, 196, 196, None);
                }
            }
        }

        let act_alpha = (self.graphics.lerp(game.prev_act_fade as f32, game.act_fade as f32) / 10.0f32) as i32;
        if game.act_fade > 5 || game.prev_act_fade > 5 {
            self.graphics.drawtextbox(16, 4, 36, 3, game.activity_r*act_alpha, game.activity_g*act_alpha, game.activity_b*act_alpha);
            self.graphics.print(5, 12, &game.activity_lastprompt, game.activity_r*act_alpha, game.activity_g*act_alpha, game.activity_b*act_alpha, Some(true));
        }

        if obj.trophytext > 0 || obj.oldtrophytext > 0 {
            self.graphics.drawtrophytext(obj, help);
        }

        Ok(Some(RenderResult::WithScreenEffects))
    }

    // void maprender(void)
    pub fn maprender(&mut self, map: &mut map::Map, help: &mut utility_class::UtilityClass, game: &mut game::Game, script: &mut script::ScriptClass, obj: &mut entity::EntityClass) -> Result<Option<RenderResult>, i32> {
        graphics_util::ClearSurface(&mut self.graphics.buffers.backBuffer);

        //draw screen alliteration
        //Roomname:
        if map.hiddenname != "" {
            self.graphics.print(5, 2, &map.hiddenname, 196, 196, 255 - help.glow, Some(true));
        } else {
            if map.finalmode {
                self.graphics.print(5, 2, &map.glitchname, 196, 196, 255 - help.glow, Some(true));
            } else {
                self.graphics.print(5, 2, &map.roomname, 196, 196, 255 - help.glow, Some(true));
            }
        }

        //Background color
        graphics_util::FillRect_xywh_rgb(&mut self.graphics.buffers.backBuffer, 0, 12, 320, 240, 10, 24, 26 );

        //Menubar:
        self.graphics.drawtextbox( -10, 212, 42, 3, 65, 185, 207);

        // Draw the selected page name at the bottom
        // menupage 0 - 3 is the pause screen
        if script.running && game.menupage == 3 {
            // While in a cutscene, you can only save
            self.graphics.print(-1, 220, "[SAVE]", 196, 196, 255 - help.glow, Some(true));
        } else if game.menupage <= 3 {
            let tab1 = match (game.insecretlab, obj.flags[67] && !map.custommode) {
                (true, _) => "GRAV",
                (_, true) => "SHIP",
                        _ => "CREW",
            };

            macro_rules! TAB {
                ($opt:expr, $text:expr) => {
                    self.graphics.map_tab($opt, $text, Some(game.menupage == $opt), help)
                }
            }

            TAB!(0, "MAP");
            TAB!(1, tab1);
            TAB!(2, "STATS");
            TAB!(3, "SAVE");
        }

        // Draw menu header
        match game.menupage {
            30 | 31 | 32 | 33 => self.graphics.print(-1, 220, "[ PAUSE ]", 196, 196, 255 - help.glow, Some(true)),
            _ => (),
        };

        // Draw menu options
        if game.menupage >= 30 && game.menupage <= 33 {
            macro_rules! OPTION {
                ($opt:expr, $text:expr) => {
                    self.graphics.map_option($opt, 4, $text, Some(game.menupage - 30 == $opt), help)
                }
            }
            OPTION!(0, "return to game");
            OPTION!(1, "options");
            OPTION!(2, "quit to menu");
        }

        // Draw the actual menu
        match game.menupage {
            0 => {
                if map.finalmode || (map.custommode && !map.customshowmm) {
                    //draw the map image
                    self.graphics.drawpixeltextbox(35, 16, 250, 190, 32,24, 65, 185, 207,4,0);
                    self.graphics.drawimage(1, 40, 21, Some(false));
                    for j in 0..20 {
                        for i in 0..20 {
                            self.graphics.drawimage(2, 40 + (i * 12), 21 + (j * 9), Some(false));
                        }
                    }
                    self.graphics.print(-1, 105, "NO SIGNAL", 245, 245, 245, Some(true));
                }
                // #ifndef NO_CUSTOM_LEVELS
                else if map.custommode {
                    //draw the map image
                    self.graphics.drawcustompixeltextbox(35 + map.custommmxoff, 16 + map.custommmyoff, map.custommmxsize + 10, map.custommmysize + 10, (map.custommmxsize+10)/8, (map.custommmysize + 10)/8, 65, 185, 207, 4, 0);
                    if self.graphics.minimap_mounted {
                        self.graphics.drawpartimage(1, 40+map.custommmxoff, 21+map.custommmyoff, map.custommmxsize, map.custommmysize);
                    } else {
                        self.graphics.drawpartimage(12, 40+map.custommmxoff, 21+map.custommmyoff, map.custommmxsize,map.custommmysize);
                    }

                    //Black out here
                    if map.customzoom == 4 {
                        for j in 0..map.customheight {
                            for i in 0..map.customwidth {
                                if !map.isexplored(i, j) {
                                    //Draw the fog of war on the map
                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 48), map.custommmyoff+21 + (j * 36), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 48), map.custommmyoff+21 + (j * 36), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 48), map.custommmyoff+21 + 9 + (j * 36), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 48), map.custommmyoff+21 + 9+ (j * 36), Some(false));

                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 48) + 24, map.custommmyoff+21 + (j * 36), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 48) + 24, map.custommmyoff+21 + (j * 36), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 48) + 24, map.custommmyoff+ 21 + 9 + (j * 36), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 48) + 24, map.custommmyoff+21 + 9+ (j * 36), Some(false));

                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 48), map.custommmyoff+21 + (j * 36)+18, Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 48), map.custommmyoff+21 + (j * 36)+18, Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 48), map.custommmyoff+21 + 9 + (j * 36)+18, Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 48), map.custommmyoff+21 + 9+ (j * 36)+18, Some(false));

                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 48) + 24, map.custommmyoff+21 + (j * 36)+18, Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 48) + 24, map.custommmyoff+21 + (j * 36)+18, Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 48) + 24, map.custommmyoff+21 + 9 + (j * 36)+18, Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 48) + 24, map.custommmyoff+21 + 9+ (j * 36)+18, Some(false));
                                }
                            }
                        }
                    } else if map.customzoom == 2 {
                        for j in 0..map.customheight {
                            for i in 0..map.customwidth {
                                if !map.isexplored(i, j) {
                                    //Draw the fog of war on the map
                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 24), map.custommmyoff+21 + (j * 18), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 24), map.custommmyoff+21 + (j * 18), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 24), map.custommmyoff+21 + 9 + (j * 18), Some(false));
                                    self.graphics.drawimage(2, map.custommmxoff+40 + 12 + (i * 24), map.custommmyoff+21 + 9+ (j * 18), Some(false));
                                }
                            }
                        }
                    } else {
                        for j in 0..map.customheight {
                            for i in 0..map.customwidth {
                                if !map.isexplored(i, j) {
                                    //Draw the fog of war on the map
                                    self.graphics.drawimage(2, map.custommmxoff+40 + (i * 12), map.custommmyoff+21 + (j * 9), Some(false));
                                }
                            }
                        }
                    }

                    //normal size maps
                    if map.customzoom == 4 {
                        if map.cursorstate == 1 {
                            if (map.cursordelay / 4) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 48) +map.custommmxoff, 21 + ((game.roomy - 100) * 36)+map.custommmyoff , 48 , 36 , 255,255,255);
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 48) + 2+map.custommmxoff, 21 + ((game.roomy - 100) * 36) + 2+map.custommmyoff, 48 - 4, 36 - 4, 255,255,255);
                            }
                        } else if map.cursorstate == 2 {
                            if (map.cursordelay / 15) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 48) + 2+map.custommmxoff, 21 + ((game.roomy - 100) * 36) + 2+map.custommmyoff, 48 - 4, 36 - 4, 16, 245 - (help.glow), 245 - (help.glow));
                            }
                        }
                    } else if map.customzoom == 2 {
                        if map.cursorstate == 1 {
                            if (map.cursordelay / 4) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 24)+map.custommmxoff , 21 + ((game.roomy - 100) * 18)+map.custommmyoff , 24 , 18 , 255,255,255);
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 24) + 2+map.custommmxoff, 21 + ((game.roomy - 100) * 18) + 2+map.custommmyoff, 24 - 4, 18 - 4, 255,255,255);
                            }
                        } else if map.cursorstate == 2 {
                            if (map.cursordelay / 15) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 24) + 2+map.custommmxoff, 21 + ((game.roomy - 100) * 18) + 2+map.custommmyoff, 24 - 4, 18 - 4, 16, 245 - (help.glow), 245 - (help.glow));
                            }
                        }
                    } else {
                        if map.cursorstate == 1 {
                            if (map.cursordelay / 4) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12)+map.custommmxoff , 21 + ((game.roomy - 100) * 9)+map.custommmyoff , 12 , 9 , 255,255,255);
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12) + 2+map.custommmxoff, 21 + ((game.roomy - 100) * 9) + 2+map.custommmyoff, 12 - 4, 9 - 4, 255,255,255);
                            }
                        } else if map.cursorstate == 2 {
                            if (map.cursordelay / 15) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12) + 2+map.custommmxoff, 21 + ((game.roomy - 100) * 9) + 2+map.custommmyoff, 12 - 4, 9 - 4, 16, 245 - (help.glow), 245 - (help.glow));
                            }
                        }
                    }

                    if map.showtrinkets {
                        for i in 0..map.shinytrinkets.len() {
                            if !obj.collect[i] {
                                let mut temp = 1086;
                                if self.graphics.flipmode { temp += 3; }
                                if map.customzoom == 4 {
                                    self.graphics.drawtile(40 + (map.shinytrinkets[i].x * 48) + 20 + map.custommmxoff, 21 + (map.shinytrinkets[i].y * 36) + 14 + map.custommmyoff, temp);
                                } else if map.customzoom == 2 {
                                    self.graphics.drawtile(40 + (map.shinytrinkets[i].x * 24) + 8 + map.custommmxoff, 21 + (map.shinytrinkets[i].y * 18) + 5 + map.custommmyoff, temp);
                                } else {
                                    self.graphics.drawtile(40 + 3 + (map.shinytrinkets[i].x * 12) + map.custommmxoff, 22 + (map.shinytrinkets[i].y * 9) + map.custommmyoff, temp);
                                }
                            }
                        }
                    }
                }
                // #endif /* NO_CUSTOM_LEVELS */
                else {
                    //draw the map image
                    self.graphics.drawpixeltextbox(35, 16, 250, 190, 32,24, 65, 185, 207,4,0);
                    self.graphics.drawimage(1, 40, 21, Some(false));

                    //black out areas we can't see yet
                    for j in 0..20 {
                        for i in 0..20 {
                            if !map.isexplored(i, j) {
                                //Draw the fog of war on the map
                                self.graphics.drawimage(2, 40 + (i * 12), 21 + (j * 9), Some(false));
                            }
                        }
                    }

                    //draw the coordinates
                    if game.roomx == 109 {
                        //tower!instead of room y, scale map.ypos
                        if map.cursorstate == 1 {
                            if (map.cursordelay / 4) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12) , 21 , 12, 180, 255,255,255);
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12) + 2 , 21  + 2, 12 - 4, 180 - 4, 255,255,255);
                            }
                            if map.cursordelay > 30 { map.cursorstate = 2; }
                        } else if map.cursorstate == 2 {
                            if (map.cursordelay / 15) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12) + 2 , 21  + 2, 12 - 4, 180 - 4,16, 245 - (help.glow), 245 - (help.glow));
                            }
                        }
                    } else {
                        if map.cursorstate == 1 {
                            if (map.cursordelay / 4) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12) , 21 + ((game.roomy - 100) * 9) , 12 , 9 , 255,255,255);
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12) + 2, 21 + ((game.roomy - 100) * 9) + 2, 12 - 4, 9 - 4, 255,255,255);
                            }
                        } else if map.cursorstate == 2 {
                            if (map.cursordelay / 15) % 2 == 0 {
                                self.graphics.drawrect(40 + ((game.roomx - 100) * 12) + 2, 21 + ((game.roomy - 100) * 9) + 2, 12 - 4, 9 - 4, 16, 245 - (help.glow), 245 - (help.glow));
                            }
                        }
                    }

                    //draw legend details
                    for i in 0..map.teleporters.len() {
                        if map.showteleporters && map.isexplored(map.teleporters[i].x, map.teleporters[i].y) {
                            let mut temp = 1126 + (if map.isexplored(map.teleporters[i].x, map.teleporters[i].y) { 1 } else { 0 });
                            if self.graphics.flipmode { temp += 3; }
                            self.graphics.drawtile(40 + 3 + (map.teleporters[i].x * 12), 22 + (map.teleporters[i].y * 9), temp);
                        } else if map.showtargets && !map.isexplored(map.teleporters[i].x, map.teleporters[i].y) {
                            let mut temp = 1126 + (if map.isexplored(map.teleporters[i].x, map.teleporters[i].y) { 1 } else { 0 });
                            if self.graphics.flipmode { temp += 3; }
                            self.graphics.drawtile(40 + 3 + (map.teleporters[i].x * 12), 22 + (map.teleporters[i].y * 9), temp);
                        }
                    }

                    if map.showtrinkets {
                        for i in 0..map.shinytrinkets.len() {
                            if !obj.collect[i] {
                                let mut temp = 1086;
                                if self.graphics.flipmode {
                                    temp += 3;
                                }

                                self.graphics.drawtile(40 + 3 + (map.shinytrinkets[i].x * 12), 22 + (map.shinytrinkets[i].y * 9),temp);
                            }
                        }
                    }
                }
            },
            1 => {
                if game.insecretlab {
                    if self.graphics.flipmode {
                        self.graphics.print(0, 174, "SUPER GRAVITRON HIGHSCORE", 196, 196, 255 - help.glow, Some(true));

                        let tempstring = help.timestring(game.swnrecord);
                        self.graphics.print( 240, 124, "Best Time", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.bigrprint( 300, 94, &tempstring, 196, 196, 255 - help.glow, true, 2.0);

                        match game.swnbestrank {
                            0 => {
                                self.graphics.print( -1, 40, "Next Trophy at 5 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            1 => {
                                self.graphics.print( -1, 40, "Next Trophy at 10 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            2 => {
                                self.graphics.print( -1, 40, "Next Trophy at 15 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            3 => {
                                self.graphics.print( -1, 40, "Next Trophy at 20 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            4 => {
                                self.graphics.print( -1, 40, "Next Trophy at 30 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            5 => {
                                self.graphics.print( -1, 40, "Next Trophy at 1 minute", 196, 196, 255 - help.glow, Some(true));
                            },
                            6 => {
                                self.graphics.print( -1, 40, "All Trophies collected!", 196, 196, 255 - help.glow, Some(true));
                            },
                            _ => (),
                        }
                    } else {
                        self.graphics.print(0, 40, "SUPER GRAVITRON HIGHSCORE", 196, 196, 255 - help.glow, Some(true));

                        let tempstring = help.timestring(game.swnrecord);
                        self.graphics.print( 240, 90, "Best Time", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.bigrprint( 300, 104, &tempstring, 196, 196, 255 - help.glow, true, 2.0);

                        match game.swnbestrank {
                            0 => {
                                self.graphics.print( -1, 174, "Next Trophy at 5 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            1 => {
                                self.graphics.print( -1, 174, "Next Trophy at 10 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            2 => {
                                self.graphics.print( -1, 174, "Next Trophy at 15 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            3 => {
                                self.graphics.print( -1, 174, "Next Trophy at 20 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            4 => {
                                self.graphics.print( -1, 174, "Next Trophy at 30 seconds", 196, 196, 255 - help.glow, Some(true));
                            },
                            5 => {
                                self.graphics.print( -1, 174, "Next Trophy at 1 minute", 196, 196, 255 - help.glow, Some(true));
                            },
                            6 => {
                                self.graphics.print( -1, 174, "All Trophies collected!", 196, 196, 255 - help.glow, Some(true));
                            },
                            _ => (),
                        }
                    }
                } else if obj.flags[67] && !map.custommode {
                    self.graphics.print(0, 105, "Press ACTION to warp to the ship.", 196, 196, 255 - help.glow, Some(true));
                }
                // // #if !defined(NO_CUSTOM_LEVELS)
                // else if map.custommode {
                //     LevelMetaData& meta = ed.ListOfMetaData[game.playcustomlevel];
                //
                //     self.graphics.bigprint( -1, FLIP!(45), meta.title, 196, 196, 255 - help.glow, Some(true));
                //     self.graphics.print( -1, FLIP!(70), "by " + meta.creator, 196, 196, 255 - help.glow, Some(true));
                //     self.graphics.print( -1, FLIP!(80), meta.website, 196, 196, 255 - help.glow, Some(true));
                //     self.graphics.print( -1, FLIP!(100), meta.Desc1, 196, 196, 255 - help.glow, Some(true));
                //     self.graphics.print( -1, FLIP!(110), meta.Desc2, 196, 196, 255 - help.glow, Some(true));
                //     self.graphics.print( -1, FLIP!(120), meta.Desc3, 196, 196, 255 - help.glow, Some(true));
                //
                //     let remaining = ed.numcrewmates() - game.crewmates();
                //
                //     if remaining == 1 {
                //         self.graphics.print(1,FLIP!(165), help.number(remaining)+ " crewmate remains", 196, 196, 255 - help.glow, Some(true));
                //     } else if remaining > 0 {
                //         self.graphics.print(1,FLIP!(165), help.number(remaining)+ " crewmates remain", 196, 196, 255 - help.glow, Some(true));
                //     }
                // }
                // // #endif
                else {
                    if self.graphics.flipmode {
                        for i in 0..3 {
                            self.graphics.drawcrewman(16, 32 + (i * 64), 2-i, game.crewstats[2-i as usize], None);
                            if game.crewstats[2-i as usize] {
                                self.graphics.printcrewname(44, 32 + (i * 64)+4+10, 2-i);
                                self.graphics.printcrewnamestatus(44, 32 + (i * 64)+4, 2-i);
                            } else {
                                self.graphics.printcrewnamedark(44, 32 + (i * 64)+4+10, 2-i);
                                self.graphics.print(44, 32 + (i * 64) + 4, "Missing...", 64,64,64, None);
                            }

                            self.graphics.drawcrewman(16+160, 32 + (i * 64), (2-i)+3, game.crewstats[(2-i as usize)+3], None);
                            if game.crewstats[(2-i as usize)+3] {
                                self.graphics.printcrewname(44+160, 32 + (i * 64)+4+10, (2-i)+3);
                                self.graphics.printcrewnamestatus(44+160, 32 + (i * 64)+4, (2-i)+3);
                            } else {
                                self.graphics.printcrewnamedark(44+160, 32 + (i * 64)+4+10, (2-i)+3);
                                self.graphics.print(44+160, 32 + (i * 64) + 4, "Missing...", 64,64,64, None);
                            }
                        }
                    } else {
                        for i in 0..3 {
                            self.graphics.drawcrewman(16, 32 + (i * 64), i, game.crewstats[i as usize], None);
                            if game.crewstats[i as usize] {
                                self.graphics.printcrewname(44, 32 + (i * 64)+4, i);
                                self.graphics.printcrewnamestatus(44, 32 + (i * 64)+4+10, i);
                            } else {
                                self.graphics.printcrewnamedark(44, 32 + (i * 64)+4, i);
                                self.graphics.print(44, 32 + (i * 64) + 4 + 10, "Missing...", 64,64,64, None);
                            }

                            self.graphics.drawcrewman(16+160, 32 + (i * 64), i+3, game.crewstats[i as usize + 3], None);
                            if game.crewstats[i as usize + 3] {
                                self.graphics.printcrewname(44+160, 32 + (i * 64)+4, i+3);
                                self.graphics.printcrewnamestatus(44+160, 32 + (i * 64)+4+10, i+3);
                            } else {
                                self.graphics.printcrewnamedark(44+160, 32 + (i * 64)+4, i+3);
                                self.graphics.print(44+160, 32 + (i * 64) + 4 + 10, "Missing...", 64,64,64, None);
                            }
                        }
                    }
                }
            },
            2 => {
                // #if !defined(NO_CUSTOM_LEVELS)
                // if map.custommode {
                //     if self.graphics.flipmode {
                //         self.graphics.print(0, 164, "[Trinkets found]", 196, 196, 255 - help.glow, Some(true));
                //         self.graphics.print(0, 152, help.number(game.trinkets()) + " out of " + help.number(ed.numtrinkets()), 96,96,96, Some(true));

                //         self.graphics.print(0, 114, "[Number of Deaths]", 196, 196, 255 - help.glow, Some(true));
                //         self.graphics.print(0, 102,help.String(game.deathcounts),  96,96,96, Some(true));

                //         self.graphics.print(0, 64, "[Time Taken]", 196, 196, 255 - help.glow, Some(true));
                //         self.graphics.print(0, 52, game.timestring(),  96, 96, 96, Some(true));
                //     } else {
                //         self.graphics.print(0, 52, "[Trinkets found]", 196, 196, 255 - help.glow, Some(true));
                //         self.graphics.print(0, 64, help.number(game.trinkets()) + " out of "+help.number(ed.numtrinkets()), 96,96,96, Some(true));

                //         self.graphics.print(0, 102, "[Number of Deaths]", 196, 196, 255 - help.glow, Some(true));
                //         self.graphics.print(0, 114,help.String(game.deathcounts),  96,96,96, Some(true));

                //         self.graphics.print(0, 152, "[Time Taken]", 196, 196, 255 - help.glow, Some(true));
                //         self.graphics.print(0, 164, game.timestring(),  96, 96, 96, Some(true));
                //     }
                // } else {
                // #endif
                {
                    if self.graphics.flipmode {
                        self.graphics.print(0, 164, "[Trinkets found]", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 152, &(help.number(game.trinkets(obj)).to_owned() + " out of Twenty"), 96,96,96, Some(true));

                        self.graphics.print(0, 114, "[Number of Deaths]", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 102,&game.deathcounts.to_string(),  96,96,96, Some(true));

                        self.graphics.print(0, 64, "[Time Taken]", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 52, &game.timestring(help).to_string(),  96, 96, 96, Some(true));
                    } else {
                        self.graphics.print(0, 52, "[Trinkets found]", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 64, &(help.number(game.trinkets(obj)).to_owned() + " out of Twenty"), 96,96,96, Some(true));

                        self.graphics.print(0, 102, "[Number of Deaths]", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 114,&game.deathcounts.to_string(),  96,96,96, Some(true));

                        self.graphics.print(0, 152, "[Time Taken]", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 164, &game.timestring(help).to_string(),  96, 96, 96, Some(true));
                    }
                }
            },
            3 => {
                if game.inintermission {
                    self.graphics.print(0, 115, "Cannot Save in Level Replay", 146, 146, 180, Some(true));
                } else if game.nodeathmode {
                    self.graphics.print(0, 115, "Cannot Save in No Death Mode", 146, 146, 180, Some(true));
                } else if game.intimetrial {
                    self.graphics.print(0, 115, "Cannot Save in Time Trial", 146, 146, 180, Some(true));
                } else if game.insecretlab {
                    self.graphics.print(0, 115, "Cannot Save in Secret Lab", 146, 146, 180, Some(true));
                } else if game.gamesavefailed {
                    self.graphics.print(0, 115, "ERROR: Could not save game!", 146, 146, 180, Some(true));
                } else if map.custommode {
                    if game.gamesaved {
                        self.graphics.print(0, 36, "Game saved ok!", 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), Some(true));
                        self.graphics.drawpixeltextbox(17, 65, 286, 90, 36,12, 65, 185, 207,0,4);

                        if self.graphics.flipmode {
                            self.graphics.print(0, 122, &game.customleveltitle, 25, 255 - (help.glow / 2), 255 - (help.glow / 2), Some(true));
                            self.graphics.print(59, 78, &game.savetime, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);
                            let trinketcount = help.number(game.savetrinkets);
                            self.graphics.print(262 - graphics::Graphics::len(&trinketcount), 78,&trinketcount, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);

                            self.graphics.drawsprite_clru32(34, 74, 50, self.graphics.col_clock.colour);
                            self.graphics.drawsprite_clru32(270, 74, 22, self.graphics.col_trinket.colour);
                        } else {
                            self.graphics.print(0, 90, &game.customleveltitle, 25, 255 - (help.glow / 2), 255 - (help.glow / 2), Some(true));
                            self.graphics.print(59, 132, &game.savetime, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);
                            let trinketcount = help.number(game.savetrinkets);
                            self.graphics.print(262 - graphics::Graphics::len(&trinketcount), 132, &trinketcount, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);

                            self.graphics.drawsprite_clru32(34, 126, 50, self.graphics.col_clock.colour);
                            self.graphics.drawsprite_clru32(270, 126, 22, self.graphics.col_trinket.colour);
                        }
                    } else {
                        self.graphics.print(0, 80, "[Press ACTION to save your game]", 255 - (help.glow * 2), 255 - (help.glow * 2), 255 - help.glow, Some(true));
                    }
                } else {
                    if self.graphics.flipmode {
                        self.graphics.print(0, 186, "(Note: The game is autosaved", 146, 146, 180, Some(true));
                        self.graphics.print(0, 174, "at every teleporter.)", 146, 146, 180, Some(true));
                    } else {
                        self.graphics.print(0, 174, "(Note: The game is autosaved", 146, 146, 180, Some(true));
                        self.graphics.print(0, 186, "at every teleporter.)", 146, 146, 180, Some(true));
                    }

                    if game.gamesaved {
                        self.graphics.print(0, 36, "Game saved ok!", 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), Some(true));

                        self.graphics.drawpixeltextbox(17, 65, 286, 90, 36,12, 65, 185, 207,0,4);

                        if self.graphics.flipmode {
                            self.graphics.print(0, 132, &game.savearea, 25, 255 - (help.glow / 2), 255 - (help.glow / 2), Some(true));
                            for i in 0..6 {
                                self.graphics.drawcrewman(169-(3*42)+(i*42), 98, i, game.crewstats[i as usize], Some(true));
                            }
                            self.graphics.print(59, 78, &game.savetime, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);
                            let trinketcount = help.number(game.savetrinkets);
                            self.graphics.print(262 - graphics::Graphics::len(&trinketcount), 78, &trinketcount, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);

                            self.graphics.drawsprite_clru32(34, 74, 50, self.graphics.col_clock.colour);
                            self.graphics.drawsprite_clru32(270, 74, 22, self.graphics.col_trinket.colour);
                        } else {
                            self.graphics.print(0, 80, &game.savearea, 25, 255 - (help.glow / 2), 255 - (help.glow / 2), Some(true));
                            for i in 0..6 {
                                self.graphics.drawcrewman(169-(3*42)+(i*42), 95, i, game.crewstats[i as usize], Some(true));
                            }
                            self.graphics.print(59, 132, &game.savetime, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);
                            let trinketcount = help.number(game.savetrinkets);
                            self.graphics.print(262 - graphics::Graphics::len(&trinketcount), 132, &trinketcount, 255 - (help.glow / 2), 255 - (help.glow / 2), 255 - (help.glow / 2), None);

                            self.graphics.drawsprite_clru32(34, 126, 50, self.graphics.col_clock.colour);
                            self.graphics.drawsprite_clru32(270, 126, 22, self.graphics.col_trinket.colour);
                        }
                    } else {
                        self.graphics.print(0, 80, "[Press ACTION to save your game]", 255 - (help.glow * 2), 255 - (help.glow * 2), 255 - help.glow, Some(true));

                        if game.quicksummary != "" {
                            if self.graphics.flipmode {
                                self.graphics.print(0, 110, "Last Save:", 164 - (help.glow / 4), 164 - (help.glow / 4), 164, Some(true));
                                self.graphics.print(0, 100, &game.quicksummary, 164  - (help.glow / 4), 164 - (help.glow / 4), 164, Some(true));
                            } else {
                                self.graphics.print(0, 100, "Last Save:", 164 - (help.glow / 4), 164 - (help.glow / 4), 164, Some(true));
                                self.graphics.print(0, 110, &game.quicksummary, 164  - (help.glow / 4), 164 - (help.glow / 4), 164, Some(true));
                            }
                        }
                    }
                }
            },
            10 => {
                self.graphics.print(128, 220, "[ QUIT ]", 196, 196, 255 - help.glow, None);

                if self.graphics.flipmode {
                    if game.inspecial() {
                        self.graphics.print(0, 135, "Return to main menu?", 196, 196, 255 - help.glow, Some(true));
                    } else {
                        self.graphics.print(0, 142, "Do you want to quit? You will", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 130, "lose any unsaved progress.", 196, 196, 255 - help.glow, Some(true));
                    }

                    self.graphics.print(80-16, 88, "[ NO, KEEP PLAYING ]", 196, 196, 255 - help.glow, None);
                    self.graphics.print(80 + 32, 76, "yes, quit to menu",  96, 96, 96, None);
                } else {
                    if game.inspecial() {
                        self.graphics.print(0, 80, "Return to main menu?", 196, 196, 255 - help.glow, Some(true));
                    } else {
                        self.graphics.print(0, 76, "Do you want to quit? You will", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 88, "lose any unsaved progress.", 196, 196, 255 - help.glow, Some(true));
                    }

                    self.graphics.print(80-16, 130, "[ NO, KEEP PLAYING ]", 196, 196, 255 - help.glow, None);
                    self.graphics.print(80 + 32, 142, "yes, quit to menu",  96, 96, 96, None);

                }
            },
            11 => {
                self.graphics.print(128, 220, "[ QUIT ]", 196, 196, 255 - help.glow, None);

                if self.graphics.flipmode {
                    if game.inspecial() {
                        self.graphics.print(0, 135, "Return to main menu?", 196, 196, 255 - help.glow, Some(true));
                    } else {
                        self.graphics.print(0, 142, "Do you want to quit? You will", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 130, "lose any unsaved progress.", 196, 196, 255 - help.glow, Some(true));
                    }

                    self.graphics.print(80, 88, "no, keep playing", 96,96,96, None);
                    self.graphics.print(80+32-16, 76, "[ YES, QUIT TO MENU ]",  196, 196, 255 - help.glow, None);
                } else {
                    if game.inspecial() {
                        self.graphics.print(0, 80, "Return to main menu?", 196, 196, 255 - help.glow, Some(true));
                    } else {
                        self.graphics.print(0, 76, "Do you want to quit? You will", 196, 196, 255 - help.glow, Some(true));
                        self.graphics.print(0, 88, "lose any unsaved progress.", 196, 196, 255 - help.glow, Some(true));
                    }

                    self.graphics.print(80, 130, "no, keep playing", 96,96,96, None);
                    self.graphics.print(80+32-16, 142, "[ YES, QUIT TO MENU ]", 196, 196, 255 - help.glow, None);
                }
            },
            20 => {
                self.graphics.print(128, 220, "[ GRAVITRON ]", 196, 196, 255 - help.glow, Some(true));

                if self.graphics.flipmode {
                    self.graphics.print(0, 76, "the secret laboratory?", 196, 196, 255 - help.glow, Some(true));
                    self.graphics.print(0, 88, "Do you want to return to", 196, 196, 255 - help.glow, Some(true));
                    self.graphics.print(80-16, 142, "[ NO, KEEP PLAYING ]", 196, 196, 255 - help.glow, None);
                    self.graphics.print(80 + 32, 130, "yes, return",  96, 96, 96, None);
                } else {
                    self.graphics.print(0, 76, "Do you want to return to", 196, 196, 255 - help.glow, Some(true));
                    self.graphics.print(0, 88, "the secret laboratory?", 196, 196, 255 - help.glow, Some(true));
                    self.graphics.print(80-16, 130, "[ NO, KEEP PLAYING ]", 196, 196, 255 - help.glow, None);
                    self.graphics.print(80 + 32, 142, "yes, return",  96, 96, 96, None);
                }

            },
            21 => {
                self.graphics.print(128, 220, "[ GRAVITRON ]", 196, 196, 255 - help.glow, Some(true));

                if self.graphics.flipmode {
                    self.graphics.print(0, 76, "the secret laboratory?", 196, 196, 255 - help.glow, Some(true));
                    self.graphics.print(0, 88, "Do you want to return to", 196, 196, 255 - help.glow, Some(true));
                    self.graphics.print(80, 142, "no, keep playing", 96, 96, 96, None);
                    self.graphics.print(80 + 32-16, 130, "[ YES, RETURN ]",  196, 196, 255 - help.glow, None);
                } else {
                    self.graphics.print(0, 76, "Do you want to return to", 196, 196, 255 - help.glow, Some(true));
                    self.graphics.print(0, 88, "the secret laboratory?", 196, 196, 255 - help.glow, Some(true));
                    self.graphics.print(80, 130, "no, keep playing", 96, 96, 96, None);
                    self.graphics.print(80 + 32-16, 142, "[ YES, RETURN ]",  196, 196, 255 - help.glow, None);
                }
            },
            _ => eprintln!("unknown menu page {}", game.menupage),
        }

        // We need to draw the black screen above the menu in order to disguise it
        // being jankily brought down in glitchrunner mode when exiting to the title
        // Otherwise, there's no reason to obscure the menu
        if game.glitchrunnermode || self.graphics.fademode == 3 || self.graphics.fademode == 5 {
            self.graphics.drawfade();
        }

        Ok(Some(if self.graphics.resumegamemode || self.graphics.menuoffset > 0 || self.graphics.oldmenuoffset > 0 {
            RenderResult::MenuOffRender
        } else {
            RenderResult::WithScreenEffects
        }))
    }

    // void teleporterrender(void)
    pub fn teleporterrender(&mut self, game: &mut game::Game, map: &mut map::Map, help: &mut utility_class::UtilityClass, obj: &mut entity::EntityClass) -> Result<Option<RenderResult>, i32> {
        graphics_util::ClearSurface(&mut self.graphics.buffers.backBuffer);

        //draw screen alliteration
        //Roomname:
        let temp = map.area(game.roomx, game.roomy);
        if temp < 2 && !map.custommode && self.graphics.fademode == 0 {
            self.graphics.print(5, 2, &map.hiddenname, 196, 196, 255 - help.glow, Some(true));
        } else {
            self.graphics.print(5, 2, &map.roomname, 196, 196, 255 - help.glow, Some(true));
        }

        //Background color
        graphics_util::FillRect_xywh_rgb(&mut self.graphics.buffers.backBuffer, 0, 12, 320, 240, 10, 24, 26);

        //draw the map image
        self.graphics.drawpixeltextbox(35, 16, 250, 190, 32,24, 65, 185, 207,4,0);
        self.graphics.drawimage(1, 40, 21, Some(false));
        //black out areas we can't see yet
        for j in 0..20 {
            for i in 0..20 {
                if !map.isexplored(i, j) {
                    self.graphics.drawimage(2, 40 + (i * 12), 21 + (j * 9), Some(false));
                }
            }
        }

        //draw the coordinates //current
        if game.roomx == 109 {
            //tower!instead of room y, scale map.ypos
            self.graphics.drawrect(40 + ((game.roomx - 100) * 12) + 2, 21  + 2, 12 - 4, 180 - 4, 16, 245 - (help.glow * 2), 245 - (help.glow * 2));
        } else {
            self.graphics.drawrect(40 + ((game.roomx - 100) * 12) + 2, 21 + ((game.roomy - 100) * 9) + 2, 12 - 4, 9 - 4, 16, 245 - (help.glow * 2), 245 - (help.glow * 2));
        }

        if game.useteleporter {
            //Draw the chosen destination coordinate!
            //TODO
            //draw the coordinates //destination
            let tempx_ = map.teleporters[game.teleport_to_teleporter as usize].x;
            let tempy_ = map.teleporters[game.teleport_to_teleporter as usize].y;
            self.graphics.drawrect(40 + (tempx_ * 12) + 1, 21 + (tempy_ * 9) + 1, 12 - 2, 9 - 2, 245 - (help.glow * 2), 16, 16);
            self.graphics.drawrect(40 + (tempx_ * 12) + 3, 21 + (tempy_ * 9) + 3, 12 - 6, 9 - 6, 245 - (help.glow * 2), 16, 16);
        }

        //draw legend details
        // for (size_t i = 0; i < map.teleporters.size(); i++)
        for i in 0..map.teleporters.len() {
            if map.showteleporters && map.isexplored(map.teleporters[i].x, map.teleporters[i].y) {
                let mut temp = 1126 + if map.isexplored(map.teleporters[i].x, map.teleporters[i].y) { 1 } else { 0 };
                if self.graphics.flipmode { temp += 3; }
                self.graphics.drawtile(40 + 3 + (map.teleporters[i].x * 12), 22 + (map.teleporters[i].y * 9), temp);
            } else if map.showtargets && !map.isexplored(map.teleporters[i].x, map.teleporters[i].y) {
                let mut temp = 1126 + if map.isexplored(map.teleporters[i].x, map.teleporters[i].y) { 1 } else { 0 };
                if self.graphics.flipmode { temp += 3; }
                self.graphics.drawtile(40 + 3 + (map.teleporters[i].x * 12), 22 + (map.teleporters[i].y * 9), temp);
            }
        }

        if map.showtrinkets {
            for i in 0..map.shinytrinkets.len() {
                if !obj.collect[i] {
                    let mut temp = 1086;
                    if self.graphics.flipmode { temp += 3; }
                    self.graphics.drawtile(40 + 3 + (map.shinytrinkets[i].x * 12), 22 + (map.shinytrinkets[i].y * 9),	temp);
                }
            }
        }

        let tempx = map.teleporters[game.teleport_to_teleporter as usize].x;
        let tempy = map.teleporters[game.teleport_to_teleporter as usize].y;
        if game.useteleporter && (help.slowsine % 16 > 8) {
            //colour in the legend
            let mut temp = 1128;
            if self.graphics.flipmode { temp += 3; }
            self.graphics.drawtile(40 + 3 + (tempx * 12), 22 + (tempy * 9), temp);
        }

        self.graphics.cutscenebars();

        if game.useteleporter {
            //Instructions!
            self.graphics.print(5, 210, "Press Left/Right to choose a Teleporter", 220 - (help.glow), 220 - (help.glow), 255 - (help.glow / 2), Some(true));
            self.graphics.print(5, 225, "Press ENTER to Teleport", 220 - (help.glow), 220 - (help.glow), 255 - (help.glow / 2), Some(true));
        }

        self.graphics.drawgui(help);

        if self.graphics.flipmode {
            if game.advancetext {
                self.graphics.print(5, 228, "- Press ACTION to advance text -", 220 - (help.glow), 220 - (help.glow), 255 - (help.glow / 2), Some(true));
            }
        } else {
            if game.advancetext {
                self.graphics.print(5, 5, "- Press ACTION to advance text -", 220 - (help.glow), 220 - (help.glow), 255 - (help.glow / 2), Some(true));
            }
        }

        Ok(Some(if self.graphics.resumegamemode || self.graphics.menuoffset > 0 || self.graphics.oldmenuoffset > 0 {
            RenderResult::MenuOffRender
        } else {
            // self.graphics.render();
            RenderResult::Plain
        }))
    }
}
