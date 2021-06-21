use sdl2::keyboard::Keycode;

use crate::game::MenuName;
use crate::screen::render::graphics;
use crate::{game, scenes::RenderResult, screen};
use crate::{key_poll, map, music};

pub struct Input {
    fadetomode: bool,
    fadetomodedelay: i32,
    gotomode: i32,
    user_changing_volume: Option<i32>,
    previous_volume: i32,
}

impl Input {
    pub fn new() -> Input {
        Input {
            fadetomode: false,
            fadetomodedelay: 0,
            gotomode: 0,
            user_changing_volume: None,
            previous_volume: 0,
        }
    }

    pub fn titleinput (&mut self, music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, screen: &mut screen::Screen, key: &mut key_poll::KeyPoll) -> Option<RenderResult> {
        let graphics = &mut screen.render.graphics;
        // @sx: disabled in original code
        // game.mx = (mouseX / 4);
        // game.my = (mouseY / 4);

        game.press_left = false;
        game.press_right = false;
        game.press_action = false;
        game.press_map = false;

        if graphics.flipmode {
            if key.isDownKeycode(Keycode::Left) || key.isDownKeycode(Keycode::Down) || key.isDownKeycode(Keycode::A) ||  key.isDownKeycode(Keycode::S)
            //    || key.controllerWantsRight(true)
            {
                game.press_left = true;
            }

            if key.isDownKeycode(Keycode::Right) || key.isDownKeycode(Keycode::Up)  || key.isDownKeycode(Keycode::D) ||  key.isDownKeycode(Keycode::W)
                // || key.controllerWantsLeft(true)
            {
                game.press_right = true;
            }
        } else {
            if key.isDownKeycode(Keycode::Left) || key.isDownKeycode(Keycode::Up) || key.isDownKeycode(Keycode::A) ||  key.isDownKeycode(Keycode::W)
                // || key.controllerWantsLeft(true)
            {
                game.press_left = true;
            }
            if key.isDownKeycode(Keycode::Right) || key.isDownKeycode(Keycode::Down)  || key.isDownKeycode(Keycode::D) ||  key.isDownKeycode(Keycode::S)
                // || key.controllerWantsRight(true)
            {
                game.press_right = true;
            }
        }
        if key.isDownKeycode(Keycode::Z) || key.isDownKeycode(Keycode::Space) || key.isDownKeycode(Keycode::V) || key.isDownVec(&game.controllerButton_flip) {
            game.press_action = true;
        }
        //|| key.isDownKeycode(Keycode::Up) || key.isDownKeycode(Keycode::Down)) game.press_action = true; //on menus, up and down don't work as action
        if key.isDownKeycode(Keycode::Return) {
            game.press_map = true;
        }

        // In the menu system, all keypresses are single taps rather than holds. Therefore this test has to be done for all presses
        if !game.press_action && !game.press_left && !game.press_right && !key.isDownKeycode(Keycode::Escape) && !key.isDownVec(&game.controllerButton_esc) {
            game.jumpheld = false;
        }
        if !game.press_map {
            game.mapheld = false;
        }

        if !game.jumpheld && graphics.fademode == 0 {
            if game.press_action || game.press_left || game.press_right || game.press_map || key.isDownKeycode(Keycode::Escape) || key.isDownVec(&game.controllerButton_esc) {
                game.jumpheld = true;
            }

            if game.menustart && game.menucountdown <= 0 && (key.isDownKeycode(Keycode::Escape) || key.isDownVec(&game.controllerButton_esc)) {
                music.playef(11);
                if game.currentmenuname == game::MenuName::mainmenu {
                    game.createmenu(game::MenuName::youwannaquit, false, graphics, music);
                    map.nexttowercolour(graphics);
                } else {
                    if game.slidermode != game::SLIDERMODE::SLIDER_NONE {
                        match game.slidermode {
                            /* Cancel volume change. */
                            game::SLIDERMODE::SLIDER_MUSICVOLUME | game::SLIDERMODE::SLIDER_SOUNDVOLUME => {
                                match self.user_changing_volume {
                                    None => {
                                        self.user_changing_volume = Some(self.previous_volume);
                                        deinitvolumeslider();
                                    },
                                    Some(v) => {
                                        // TODO @sx
                                        // SDL_assert(0 && "user_changing_volume is NULL!");
                                    }
                                }
                            },
                            _ => {},
                        }
                    } else if game.ingame_titlemode && game.currentmenuname == game::MenuName::options {
                        game.returntoingame(graphics);
                    } else {
                        game.return_menu(graphics, music);
                        map.nexttowercolour(graphics);
                    }
                }
            }

            if game.menustart {
                if game.slidermode == game::SLIDERMODE::SLIDER_NONE {
                    if game.press_left {
                        game.currentmenuoption -= 1;
                    } else if game.press_right {
                        game.currentmenuoption += 1;
                    }
                } else {
                    slidermodeinput();
                }
            }

            if game.currentmenuoption < 0 {
                game.currentmenuoption = game.menuoptions.len() as i32 - 1;
            }
            if game.currentmenuoption >= game.menuoptions.len() as i32 {
                game.currentmenuoption = 0;
            }

            if game.press_action {
                if !game.menustart {
                    game.menustart = true;
                    music.play(6);
                    music.playef(18);
                    game.screenshake = 10;
                    game.flashlight = 5;
                } else {
                    self.menuactionpress(music, map, game, graphics);
                }
            }
            if game.currentmenuname == game::MenuName::controller &&
               game.currentmenuoption > 0 &&
               game.currentmenuoption < 5 &&
               key.controllerButtonDown() {
                updatebuttonmappings(key, game, game.currentmenuoption);
            }

        }

        if self.fadetomode {
            if self.fadetomodedelay > 0 {
                self.fadetomodedelay -= 1;
            } else {
                self.fadetomode = false;
                // TODO @sx
                // script.startgamemode(self.gotomode);
                println!("READY TO STAERT THE GAME!!!");
            }
        }

        None
    }

    pub fn gameinput (&mut self) {
        // TODO @sx @impl
        println!("DEADBEEF: input::Input::toggleflipmode is not implemented yet");
    }

    pub fn mapinput (&mut self) {
        // TODO @sx @impl
        println!("DEADBEEF: input::Input::toggleflipmode is not implemented yet");
    }

    pub fn teleporterinput (&mut self) {
        // TODO @sx @impl
        println!("DEADBEEF: input::Input::toggleflipmode is not implemented yet");
    }

    pub fn gamecompleteinput (&mut self) {
        // TODO @sx @impl
        println!("DEADBEEF: input::Input::toggleflipmode is not implemented yet");
    }

    pub fn gamecompleteinput2 (&mut self) {
        // TODO @sx @impl
        println!("DEADBEEF: input::Input::toggleflipmode is not implemented yet");
    }

    // @sx: previously static methods

    // static void startmode(const int mode)
    fn startmode(&mut self, mode: i32, graphics: &mut graphics::Graphics) {
        self.gotomode = mode;
        graphics.fademode = 2; /* fading out */
        self.fadetomode = true;
        self.fadetomodedelay = 16;
    }

    // static void menuactionpress(void)
    fn menuactionpress(&mut self, music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, graphics: &mut graphics::Graphics) {
        match game.currentmenuname {
            MenuName::mainmenu => {
                // TODO @sx @define
                // #if defined(MAKEANDPLAY)
                //     #define MPOFFSET -1
                //     #else
                //     #define MPOFFSET 0
                // #endif

                // #if defined(NO_CUSTOM_LEVELS)
                //     #define NOCUSTOMSOFFSET -1
                //     #else
                //     #define NOCUSTOMSOFFSET 0
                // #endif

                // #define OFFSET (MPOFFSET+NOCUSTOMSOFFSET)
                match game.currentmenuoption {
                    // #if !defined(MAKEANDPLAY)
                    0 => {
                        // Play
                        if !game.save_exists() && !game.anything_unlocked() {
                            // No saves exist, just start a new game
                            music.playef(11);
                            self.startmode(0, graphics);
                        } else {
                            // Bring you to the normal playmenu
                            music.playef(11);
                            game.createmenu(MenuName::play, false, graphics, music);
                            map.nexttowercolour(graphics);
                        }
                    },
                    // #endif
                    // #if !defined(NO_CUSTOM_LEVELS)
                    1 => { // OFFSET+1
                        // Bring you to the normal playmenu
                        music.playef(11);
                        game.createmenu(MenuName::playerworlds, false, graphics, music);
                        map.nexttowercolour(graphics);
                    },
                    // #endif
                    2 => { // OFFSET+2
                        // Options
                        music.playef(11);
                        game.createmenu(MenuName::options, false, graphics, music);
                        map.nexttowercolour(graphics);
                    },
                    // #if !defined(MAKEANDPLAY)
                    3 => { // OFFSET+3
                        // Credits
                        music.playef(11);
                        game.createmenu(MenuName::credits, false, graphics, music);
                        map.nexttowercolour(graphics);
                    },
                    // #else
                    //     #undef MPOFFSET
                    //     #define MPOFFSET -2
                    // #endif
                    4 => { // OFFSET+4
                        music.playef(11);
                        game.createmenu(MenuName::youwannaquit, false, graphics, music);
                        map.nexttowercolour(graphics);
                    },
                    // #undef OFFSET
                    // #undef NOCUSTOMSOFFSET
                    // #undef MPOFFSET
                    _ => panic!("incorrect menuoption"),
                }
            },

            // #if !defined(NO_CUSTOM_LEVELS)
            MenuName::levellist => {
                println!("DEADBEEF: not fully implemented yet");

                if game.currentmenuoption == game.menuoptions.len() as i32 - 1 {
                    //go back to menu
                    music.playef(11);
                    game.return_menu(graphics, music);
                    map.nexttowercolour(graphics);
                } else if game.currentmenuoption == game.menuoptions.len() as i32 - 2 {
                    //previous page
                    music.playef(11);
                    // if game.levelpage == 0 {
                    //     game.levelpage = (ed.ListOfMetaData.size()-1)/8;
                    // } else {
                    //     game.levelpage -= 1;
                    // }
                    game.createmenu(MenuName::levellist, true, graphics, music);
                    game.currentmenuoption = game.menuoptions.len() as i32 - 2;
                    map.nexttowercolour(graphics);
                } else if game.currentmenuoption == game.menuoptions.len() as i32 - 3 {
                    //next page
                    music.playef(11);
                    // if (game.levelpage*8)+8 >= ed.ListOfMetaData.len() {
                    //     game.levelpage = 0;
                    // } else {
                    //     game.levelpage += 1;
                    // }
                    game.createmenu(MenuName::levellist, true, graphics, music);
                    game.currentmenuoption = game.menuoptions.len() as i32 - 3;
                    map.nexttowercolour(graphics);
                } else {
                    // // Ok, launch the level!
                    // // PLAY CUSTOM LEVEL HOOK
                    music.playef(11);
                    // game.playcustomlevel = (game.levelpage*8)+game.currentmenuoption;
                    // game.customleveltitle = ed.ListOfMetaData[game.playcustomlevel].title;
                    // game.customlevelfilename = ed.ListOfMetaData[game.playcustomlevel].filename;

                    // let name = "saves/" + ed.ListOfMetaData[game.playcustomlevel].filename.substr(7) + ".vvv";
                    // // tinyxml2::XMLDocument doc;
                    // let doc;
                    // if !FILESYSTEM_loadTiXml2Document(name.c_str(), doc) {
                    //     startmode(22);
                    // } else {
                    //     game.createmenu(Menu::quickloadlevel);
                    //     map.nexttowercolour(graphics);
                    // }
                }
            },
            // #endif
            MenuName::quickloadlevel => {
                match game.currentmenuoption {
                    0 => {
                        // continue save
                        music.playef(11);
                        self.startmode(23, graphics);
                    },
                    1 => {
                        music.playef(11);
                        self.startmode(22, graphics);
                    },
                    2 => {
                        music.playef(11);
                        game.return_menu(graphics, music);
                        map.nexttowercolour(graphics);
                    },
                    _ => panic!("incorrect menuoption"),
                };
            },
            // #if !defined(NO_CUSTOM_LEVELS)
            // MenuName::playerworlds => {
            //     // #if defined(NO_EDITOR)
            //     //     #define OFFSET -1
            //     // #else
            //     //     #define OFFSET 0
            //     // #endif
            //     match game.currentmenuoption {
            //         0 => {
            //             music.playef(11);
            //             game.levelpage = 0;
            //             ed.getDirectoryData();
            //             game.loadcustomlevelstats(); // Should only load a file if it's needed
            //             game.createmenu(MenuName::levellist, false);
            //             map.nexttowercolour(graphics);
            //         },
            //         // #if !defined(NO_EDITOR)
            //         1 => { // OFFSET+1
            //             // LEVEL EDITOR HOOK
            //             music.playef(11);
            //             self.startmode(20, graphics);
            //             ed.filename = "";
            //         },
            //         // #endif
            //         2 => { // OFFSET+2
            //             // "OPENFOLDERHOOK"
            //             if FILESYSTEM_openDirectoryEnabled() && FILESYSTEM_openDirectory(FILESYSTEM_getUserLevelDirectory()) {
            //                 music.playef(11);
            //                 SDL_MinimizeWindow(graphics.screenbuffer.m_window);
            //             } else {
            //                 music.playef(2);
            //             }
            //         },
            //         3 => { // OFFSET+3
            //             // back
            //             music.playef(11);
            //             game.return_menu();
            //             map.nexttowercolour(graphics);
            //         },
            //     };
            //     // #undef OFFSET
            //     // #endif
            // },
            _ => println!("DEADBEEF: input::menuactionpress({:?}) is not fully implemented yet", game.currentmenuname),
        }
    }
}

// static void updatebuttonmappings(int bind)
fn updatebuttonmappings(key: &mut key_poll::KeyPoll, game: &mut game::Game, bind: i32) {
    // TODO @sx @impl
    println!("DEADBEEF: input::Input::updatebuttonmappings is not implemented yet");

    // let i = sdl2_sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_A;
    // while i < sdl2_sys::SDL_GameControllerButton::SDL_CONTROLLER_BUTTON_DPAD_UP {
    //     if key.isDownKeycode(i) {
    //         let dupe = false;
    //         if bind == 1 {
    //             let j = 0;
    //             while j < game.controllerButton_flip.len() {
    //                 if i == game.controllerButton_flip[j] {
    //                     dupe = true;
    //                 }
    //                 j += 1;
    //             }
    //             if !dupe {
    //                 game.controllerButton_flip.push(i);
    //                 // music.playef(11);
    //             }
    //             let j = 0;
    //             while j < game.controllerButton_map.len() {
    //                 if i == game.controllerButton_map[j] {
    //                     game.controllerButton_map.erase(game.controllerButton_map.begin() + j);
    //                 }
    //                 j += 1;
    //             }
    //             let j = 0;
    //             while j < game.controllerButton_esc.len() {
    //                 if i == game.controllerButton_esc[j] {
    //                     game.controllerButton_esc.erase(game.controllerButton_esc.begin() + j);
    //                 }
    //                 j += 1;
    //             }
    //             let j = 0;
    //             while j < game.controllerButton_restart.len() {
    //                 if i == game.controllerButton_restart[j] {
    //                     game.controllerButton_restart.erase(game.controllerButton_restart.begin() + j);
    //                 }
    //                 j += 1;
    //             }
    //         }
    //         if bind == 2 {
    //             for (size_t j = 0; j < game.controllerButton_map.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_map[j] {
    //                     dupe = true;
    //                 }
    //             }
    //             if !dupe {
    //                 game.controllerButton_map.push(i);
    //                 // music.playef(11);
    //             }
    //             for (size_t j = 0; j < game.controllerButton_flip.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_flip[j] {
    //                     game.controllerButton_flip.erase(game.controllerButton_flip.begin() + j);
    //                 }
    //             }
    //             for (size_t j = 0; j < game.controllerButton_esc.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_esc[j] {
    //                     game.controllerButton_esc.erase(game.controllerButton_esc.begin() + j);
    //                 }
    //             }
    //             for (size_t j = 0; j < game.controllerButton_restart.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_restart[j] {
    //                     game.controllerButton_restart.erase(game.controllerButton_restart.begin() + j);
    //                 }
    //             }
    //         }
    //         if bind == 3 {
    //             for (size_t j = 0; j < game.controllerButton_esc.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_esc[j] {
    //                     dupe = true;
    //                 }
    //             }
    //             if !dupe {
    //                 game.controllerButton_esc.push(i);
    //                 // music.playef(11);
    //             }
    //             for (size_t j = 0; j < game.controllerButton_flip.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_flip[j] {
    //                     game.controllerButton_flip.erase(game.controllerButton_flip.begin() + j);
    //                 }
    //             }
    //             for (size_t j = 0; j < game.controllerButton_map.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_map[j] {
    //                     game.controllerButton_map.erase(game.controllerButton_map.begin() + j);
    //                 }
    //             }
    //             for (size_t j = 0; j < game.controllerButton_restart.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_restart[j] {
    //                     game.controllerButton_restart.erase(game.controllerButton_restart.begin() + j);
    //                 }
    //             }
    //         }
    //         if bind == 4 {
    //             for (size_t j = 0; j < game.controllerButton_restart.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_restart[j] {
    //                     dupe = true;
    //                 }
    //             }
    //             if !dupe {
    //                 game.controllerButton_restart.push(i);
    //                 music.playef(11);
    //             }
    //             for (size_t j = 0; j < game.controllerButton_flip.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_flip[j] {
    //                     game.controllerButton_flip.erase(game.controllerButton_flip.begin() + j);
    //                 }
    //             }
    //             for (size_t j = 0; j < game.controllerButton_map.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_map[j] {
    //                     game.controllerButton_map.erase(game.controllerButton_map.begin() + j);
    //                 }
    //             }
    //             for (size_t j = 0; j < game.controllerButton_esc.size(); j += 1)
    //             {
    //                 if i == game.controllerButton_esc[j] {
    //                     game.controllerButton_esc.erase(game.controllerButton_esc.begin() + j);
    //                 }
    //             }
    //         }
    //     }

    //     i = (SDL_GameControllerButton) (i + 1)
    // }
}

// static void toggleflipmode(void)
fn toggleflipmode() {
    // TODO @sx @impl
    println!("DEADBEEF: input::toggleflipmode is not implemented yet");
}

// static void initvolumeslider(const int menuoption)
fn initvolumeslider() {
    // TODO @sx @impl
    println!("DEADBEEF: input::initvolumeslider is not implemented yet");
}

// static void deinitvolumeslider(void)
fn deinitvolumeslider() {
    // TODO @sx @impl
    println!("DEADBEEF: input::deinitvolumeslider is not implemented yet");
}

// static void slidermodeinput(void)
fn slidermodeinput() {
    // TODO @sx @impl
    println!("DEADBEEF: input::slidermodeinput is not implemented yet");
}

// static void mapmenuactionpress(void)
fn mapmenuactionpress() {
    // TODO @sx @impl
    println!("DEADBEEF: input::mapmenuactionpress is not implemented yet");
}
