use sdl2::keyboard::Keycode;

use crate::{game, scenes::RenderResult, screen};
use crate::key_poll;

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

    pub fn titleinput (&mut self, game: &mut game::Game, screen: &mut screen::Screen, key: &mut key_poll::KeyPoll) -> Option<RenderResult> {
        let graphics = screen.render.graphics.as_mut();
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

            if game.menustart && game.menucountdown <= 0 && (key.isDownKeycode(Keycode::Escape) || key.isDownVec(&game.controllerButton_esc))
            {
                // music.playef(11);
                if game.currentmenuname == game::MenuName::mainmenu {
                    game.createmenu(game::MenuName::youwannaquit, false);
                    // map.nexttowercolour();
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
                        game.return_menu();
                        // map.nexttowercolour();
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
                    // music.play(6);
                    // music.playef(18);
                    game.screenshake = 10;
                    game.flashlight = 5;
                } else {
                    menuactionpress();
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

// static void startmode(const int mode)
fn startmode() {
    // TODO @sx @impl
    println!("DEADBEEF: input::startmode is not implemented yet");
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

// static void menuactionpress(void)
fn menuactionpress() {
    // TODO @sx @impl
    println!("DEADBEEF: input::menuactionpress is not implemented yet");
}

// static void mapmenuactionpress(void)
fn mapmenuactionpress() {
    // TODO @sx @impl
    println!("DEADBEEF: input::mapmenuactionpress is not implemented yet");
}
