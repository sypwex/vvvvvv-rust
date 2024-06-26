use sdl2::keyboard::Keycode;

use crate::{INBOUNDS_VEC, entity, filesystem, game::{self, GameState, MenuName, SLIDERMODE}, key_poll, map, maths, music, scenes::RenderResult, screen::{self, ScreenParams, render::graphics::{self, graphics_util}}, script::{self, scripts}, utility_class};

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

    pub fn titleinput (&mut self, music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, screen: &mut screen::Screen, key: &mut key_poll::KeyPoll, script: &mut script::ScriptClass, obj: &mut entity::EntityClass, help: &mut utility_class::UtilityClass, fs: &mut filesystem::FileSystem) -> Result<Option<RenderResult>, i32> {
        let screen_params = screen.get_screen_params();
        let screen_settings = screen.screen_settings;
        // @sx: disabled in original code
        // game.mx = (mouseX / 4);
        // game.my = (mouseY / 4);

        game.press_left = false;
        game.press_right = false;
        game.press_action = false;
        game.press_map = false;

        if screen.render.graphics.flipmode {
            if key.isDownKeycode(Keycode::Left) || key.isDownKeycode(Keycode::Down) || key.isDownKeycode(Keycode::A) ||  key.isDownKeycode(Keycode::S) || key.controllerWantsRight(true) {
                game.press_left = true;
            }

            if key.isDownKeycode(Keycode::Right) || key.isDownKeycode(Keycode::Up)  || key.isDownKeycode(Keycode::D) ||  key.isDownKeycode(Keycode::W) || key.controllerWantsLeft(true) {
                game.press_right = true;
            }
        } else {
            if key.isDownKeycode(Keycode::Left) || key.isDownKeycode(Keycode::Up) || key.isDownKeycode(Keycode::A) ||  key.isDownKeycode(Keycode::W) || key.controllerWantsLeft(true) {
                game.press_left = true;
            }
            if key.isDownKeycode(Keycode::Right) || key.isDownKeycode(Keycode::Down)  || key.isDownKeycode(Keycode::D) ||  key.isDownKeycode(Keycode::S) || key.controllerWantsRight(true) {
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

        if !game.jumpheld && screen.render.graphics.fademode == 0 {
            if game.press_action || game.press_left || game.press_right || game.press_map || key.isDownKeycode(Keycode::Escape) || key.isDownVec(&game.controllerButton_esc) {
                game.jumpheld = true;
            }

            if game.menustart && game.menucountdown <= 0 && (key.isDownKeycode(Keycode::Escape) || key.isDownVec(&game.controllerButton_esc)) {
                music.playef(11);
                if game.currentmenuname == game::MenuName::mainmenu {
                    game.createmenu(game::MenuName::youwannaquit, Some(false), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else {
                    if game.slidermode != game::SLIDERMODE::SLIDER_NONE {
                        match game.slidermode {
                            /* Cancel volume change. */
                            game::SLIDERMODE::SLIDER_MUSICVOLUME | game::SLIDERMODE::SLIDER_SOUNDVOLUME => {
                                match self.user_changing_volume {
                                    Some(ref mut volume) => {
                                        *volume = self.previous_volume;
                                        self.deinitvolumeslider(game, screen_settings, fs, music, map);
                                    },
                                    None => {
                                        warn!("user_changing_volume is NULL!");
                                    },
                                }
                            },
                            _ => warn!("Unhandled slider mode!"),
                        }
                    } else if game.ingame_titlemode && game.currentmenuname == game::MenuName::options {
                        game.returntoingame(&mut screen.render.graphics);
                    } else {
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
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
                    self.slidermodeinput(game, music);
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
                    music.play(6, map, game);
                    music.playef(18);
                    game.screenshake = 10;
                    game.flashlight = 5;
                } else {
                    self.menuactionpress(music, map, game, screen, key, fs, help, obj);
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
                if let Err(code) = script.startgamemode(self.gotomode, game, &mut screen.render.graphics, map, obj, music, help, fs) {
                    return Err(code)
                }
            }
        }

        Ok(None)
    }

    pub fn gameinput (&mut self, game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, music: &mut music::Music, key: &mut key_poll::KeyPoll, obj: &mut entity::EntityClass, script: &mut script::ScriptClass, help: &mut utility_class::UtilityClass, fs: &mut filesystem::FileSystem) -> Result<Option<RenderResult>, i32> {
        //TODO mouse input
        //game.mx = (mouseX / 2);
        //game.my = (mouseY / 2);

        if !script.running {
            game.press_left = false;
            game.press_right = false;
            game.press_action = false;

            if key.isDownKeycode(Keycode::Left) || key.isDownKeycode(Keycode::A) || key.controllerWantsLeft(false) {
                game.press_left = true;
            }
            if key.isDownKeycode(Keycode::Right) || key.isDownKeycode(Keycode::D) || key.controllerWantsRight(false) {
                game.press_right = true;
            }
            if key.isDownKeycode(Keycode::Z) || key.isDownKeycode(Keycode::Space) || key.isDownKeycode(Keycode::V)
                    || key.isDownKeycode(Keycode::Up) || key.isDownKeycode(Keycode::Down) || key.isDownKeycode(Keycode::W) || key.isDownKeycode(Keycode::S)|| key.isDownVec(&game.controllerButton_flip) {
                game.press_action = true;
            };
        }

        game.press_map = false;
        if key.isDownKeycode(Keycode::KpEnter) || key.isDownKeycode(Keycode::Return) || key.isDownVec(&game.controllerButton_map) {
            game.press_map = true;
        }

        if game.advancetext {
            if game.pausescript {
                game.press_action = false;
                if key.isDownKeycode(Keycode::Z) || key.isDownKeycode(Keycode::Space) || key.isDownKeycode(Keycode::V) || key.isDownKeycode(Keycode::Up) || key.isDownKeycode(Keycode::Down) || key.isDownKeycode(Keycode::W) || key.isDownKeycode(Keycode::S) || key.isDownVec(&game.controllerButton_flip) {
                    game.press_action = true;
                }
            }

            if game.press_action && !game.jumpheld {
                if game.pausescript {
                    game.pausescript = false;
                    game.hascontrol = true;
                    game.jumpheld = true;
                } else {
                    if game.glitchrunnermode || !game.glitchrunkludge {
                        game.state += 1;
                    }

                    game.jumpheld = true;
                    game.glitchrunkludge = true;
                    //Bug fix! You should only be able to do this ONCE.
                    //...Unless you're in glitchrunner mode
                }
            }
        }

        if !game.press_map
        //Extra conditionals as a kludge fix so if you open the quit menu during
        //the script command gamemode(teleporter) and close it with Esc, it won't
        //immediately open again
        //We really need a better input system soon...
        && !key.isDownKeycode(Keycode::Escape)
        && !key.isDownVec(&game.controllerButton_esc) {
            game.mapheld = false;
        }

        if game.intimetrial && graphics.fademode == 1 && game.quickrestartkludge {
            //restart the time trial
            game.quickrestartkludge = false;
            script.startgamemode(game.timetriallevel + 3, game, graphics, map, obj, music, help, fs)?;
            game.deathseq = -1;
            game.completestop = false;
        }

        //Returning to editor mode must always be possible
        // #if !defined(NO_CUSTOM_LEVELS)
        // if map.custommode && !map.custommodeforreal {
        //     if (game.press_map || key.isDownKeycode(Keycode::Escape)) && !game.mapheld {
        //         //Return to level editor
        //         if INBOUNDS_VEC!(game.activeactivity, obj.blocks) && game.press_map {
        //             //pass, let code block below handle it
        //         } else if game.activetele && game.readytotele > 20 && game.press_map {
        //             //pass, let code block below handle it
        //         } else {
        //             game.returntoeditor();
        //             game.mapheld = true;
        //         }
        //     }
        // }
        // #endif

        //Entity type 0 is player controled
        let mut has_control = false;
        let enter_pressed = game.press_map && !game.mapheld;
        let mut enter_already_processed = false;

        for ie in 0..obj.entities.len() {
            if obj.entities[ie].rule == 0 {
                if game.hascontrol && game.deathseq == -1 && game.lifeseq <= 5 {
                    has_control = true;
                    if enter_pressed {
                        game.mapheld = true;
                    }

                    if enter_pressed && !script.running {
                        if game.activetele && game.readytotele > 20 && !game.intimetrial {
                            enter_already_processed = true;
                            if (obj.entities[ie].vx).abs() <= 1.0 && obj.entities[ie].vy as i32 == 0 {
                                //wait! space station 2 debug thingy
                                if game.teleportscript != "" {
                                    //trace(game.recordstring);
                                    //We're teleporting! Yey!
                                    game.activetele = false;
                                    game.hascontrol = false;
                                    music.fadeout(None, game);

                                    let player = obj.getplayer() as usize;
                                    if INBOUNDS_VEC!(player, obj.entities) {
                                        obj.entities[player].colour = 102;
                                    }

                                    let teleporter = obj.getteleporter() as usize;
                                    if INBOUNDS_VEC!(teleporter, obj.entities) {
                                        obj.entities[teleporter].tile = 6;
                                        obj.entities[teleporter].colour = 102;
                                    }
                                    //which teleporter script do we use? it depends on the companion!
                                    game.state = 4000;
                                    game.statedelay = 0;
                                } else if game.companion == 0 {
                                    //Alright, normal teleporting
                                    game.mapmenuchange(GameState::TELEPORTERMODE, graphics, map);

                                    game.useteleporter = true;
                                    game.initteleportermode();
                                } else {
                                    //We're teleporting! Yey!
                                    game.activetele = false;
                                    game.hascontrol = false;
                                    music.fadeout(None, game);

                                    let player = obj.getplayer() as usize;
                                    if INBOUNDS_VEC!(player, obj.entities) {
                                        obj.entities[player].colour = 102;
                                    }
                                    let companion = obj.getcompanion() as usize;
                                    if INBOUNDS_VEC!(companion, obj.entities) {
                                        obj.entities[companion].colour = 102;
                                    }

                                    let teleporter = obj.getteleporter() as usize;
                                    if INBOUNDS_VEC!(teleporter, obj.entities) {
                                        obj.entities[teleporter].tile = 6;
                                        obj.entities[teleporter].colour = 102;
                                    }
                                    //which teleporter script do we use? it depends on the companion!
                                    game.state = 3000;
                                    game.statedelay = 0;
                                }
                            }
                        } else if INBOUNDS_VEC!(game.activeactivity, obj.blocks) {
                            enter_already_processed = true;
                            if (obj.entities[ie].vx).abs() <= 1.0 && obj.entities[ie].vy as i32 == 0 {
                                scripts::load(script, &obj.blocks[game.activeactivity as usize].script);
                                obj.disableblock(game.activeactivity as usize);
                                game.activeactivity = -1;
                            }
                        }
                    }

                    if game.press_left {
                        game.tapleft += 1;
                    } else {
                        if game.tapleft <= 4 && game.tapleft > 0 {
                            if obj.entities[ie].vx < 0.0 {
                                obj.entities[ie].vx = 0.0;
                            }
                        }
                        game.tapleft = 0;
                    }
                    if game.press_right {
                        game.tapright += 1;
                    } else {
                        if game.tapright <= 4 && game.tapright > 0 {
                            if obj.entities[ie].vx > 0.0 {
                                obj.entities[ie].vx = 0.0;
                            }
                        }
                        game.tapright = 0;
                    }

                    if game.press_right {
                        game.tapright += 1;
                    } else {
                        if game.tapright <= 4 && game.tapright > 0 {
                            if obj.entities[ie].vx > 0.0 {
                                obj.entities[ie].vx = 0.0;
                            }
                        }
                        game.tapright = 0;
                    }

                    if game.press_left {
                        obj.entities[ie].ax = -3.0;
                        obj.entities[ie].dir = 0;
                    } else if game.press_right {
                        obj.entities[ie].ax = 3.0;
                        obj.entities[ie].dir = 1;
                    }

                    if !game.press_action {
                        game.jumppressed = 0;
                        game.jumpheld = false;
                    }

                    if game.press_action && !game.jumpheld {
                        game.jumppressed = 5;
                        game.jumpheld = true;
                    }

                    if game.jumppressed > 0 {
                        game.jumppressed -= 1;
                        if obj.entities[ie].onground > 0 && game.gravitycontrol == 0 {
                            game.gravitycontrol = 1;
                            obj.entities[ie].vy = -4.0;
                            obj.entities[ie].ay = -3.0;
                            music.playef(0);
                            game.jumppressed = 0;
                            game.totalflips += 1;
                        }
                        if obj.entities[ie].onroof > 0 && game.gravitycontrol == 1 {
                            game.gravitycontrol = 0;
                            obj.entities[ie].vy = 4.0;
                            obj.entities[ie].ay = 3.0;
                            music.playef(1);
                            game.jumppressed = 0;
                            game.totalflips += 1;
                        }
                    }
                }
            }
        }

        if !has_control {
            //Simple detection of keypresses outside player control, will probably scrap this (expand on
            //advance text function)
            if !game.press_action {
                game.jumppressed = 0;
                game.jumpheld = false;
            }

            if game.press_action && !game.jumpheld {
                game.jumppressed = 5;
                game.jumpheld = true;
            }
        }

        // Continuation of Enter processing. The rest of the if-tree runs only if
        // enter_pressed && !enter_already_pressed
        if !enter_pressed || enter_already_processed {
            // Do nothing
        } else if game.swnmode && game.swngame == 1 {
            //quitting the super gravitron
            game.mapheld = true;
            //Quit menu, same conditions as in game menu
            game.mapmenuchange(GameState::MAPMODE, graphics, map);
            game.gamesaved = false;
            game.gamesavefailed = false;
            game.menupage = 20; // The Map Page
        } else if game.intimetrial && graphics.fademode == 0 {
            //Quick restart of time trial
            graphics.fademode = 2;
            game.completestop = true;
            music.fadeout(None, game);
            game.quickrestartkludge = true;
        } else if game.intimetrial {
            //Do nothing if we're in a Time Trial but a fade animation is playing
        } else {
            //Normal map screen, do transition later
            game.mapmenuchange(GameState::MAPMODE, graphics, map);
            map.cursordelay = 0;
            map.cursorstate = 0;
            game.gamesaved = false;
            game.gamesavefailed = false;
            if script.running {
                game.menupage = 3; // Only allow saving
            } else {
                game.menupage = 0; // The Map Page
            }
        }

        if !game.mapheld && (key.isDownKeycode(Keycode::Escape) || key.isDownVec(&game.controllerButton_esc)) && (!map.custommode || map.custommodeforreal) {
            game.mapheld = true;
            //Quit menu, same conditions as in game menu
            game.mapmenuchange(GameState::MAPMODE, graphics, map);
            game.gamesaved = false;
            game.gamesavefailed = false;
            game.menupage = 30; // Pause screen
        }

        if game.deathseq == -1 && (key.isDownKeycode(Keycode::R) || key.isDownVec(&game.controllerButton_restart)) && !game.nodeathmode {
        // && map.custommode) //Have fun glitchrunners!
            game.deathseq = 30;
        }

        Ok(None)
    }

    pub fn mapinput (&mut self, game: &mut game::Game, graphics: &mut graphics::Graphics, obj: &mut entity::EntityClass, script: &mut script::ScriptClass, music: &mut music::Music, map: &mut map::Map, help: &mut utility_class::UtilityClass, key: &mut key_poll::KeyPoll, fs: &mut filesystem::FileSystem, screen_params: ScreenParams, screen_settings: screen::ScreenSettings) -> Result<Option<RenderResult>, i32> {
        //TODO Mouse Input!
        //game.mx = (mouseX / 2);
        //game.my = (mouseY / 2);

        game.press_left = false;
        game.press_right = false;
        game.press_action = false;
        game.press_map = false;

        if game.glitchrunnermode && graphics.fademode == 1 && graphics.menuoffset == 0 {
            // Deliberate re-addition of the glitchy gamestate-based fadeout!

            // First of all, detecting a black screen means if the glitchy fadeout
            // gets interrupted but you're still on a black screen, opening a menu
            // immediately quits you to the title. This has the side effect that if
            // you accidentally press Esc during a cutscene when it's black, you'll
            // immediately be quit and lose all your progress, but that's fair in
            // glitchrunner mode.
            // Also have to check graphics.menuoffset so this doesn't run every frame

            // Have to close the menu in order to run gamestates
            graphics.resumegamemode = true;
            // Remove half-second delay
            graphics.menuoffset = 250;

            // Technically this was in <=2.2 as well
            obj.removeallblocks();

            if game.menupage >= 20 && game.menupage <= 21 {
                game.state = 96;
                game.statedelay = 0;
            } else {
                // Produces more glitchiness! Necessary for credits warp to work.
                script.running = false;
                graphics.textbox.clear();

                game.state = 80;
                game.statedelay = 0;
            }
        }

        if game.fadetomenu && !game.glitchrunnermode {
            if game.fadetomenudelay > 0 {
                game.fadetomenudelay -= 1;
            } else {
                game.quittomenu(graphics, map, script, music, obj, screen_params, screen_settings, fs);
                music.play(6, map, game); //should be after game.quittomenu()
                game.fadetomenu = false;
            }
        }

        if game.fadetolab && !game.glitchrunnermode {
            if game.fadetolabdelay > 0 {
                game.fadetolabdelay -= 1;
            } else {
                game.returntolab(graphics, map, music, obj, help);
                game.fadetolab = false;
            }
        }

        if graphics.menuoffset == 0
        && ((!game.glitchrunnermode && !game.fadetomenu && game.fadetomenudelay <= 0 && !game.fadetolab && game.fadetolabdelay <= 0)
        || graphics.fademode == 0) {
            if key.isDownKeycode(Keycode::Left) || key.isDownKeycode(Keycode::Up) || key.isDownKeycode(Keycode::A) || key.isDownKeycode(Keycode::W) || key.controllerWantsLeft(true) {
                game.press_left = true;
            }
            if key.isDownKeycode(Keycode::Right) || key.isDownKeycode(Keycode::Down) || key.isDownKeycode(Keycode::D) || key.isDownKeycode(Keycode::S) || key.controllerWantsRight(true) {
                game.press_right = true;
            }
            if key.isDownKeycode(Keycode::Z) || key.isDownKeycode(Keycode::Space) || key.isDownKeycode(Keycode::V) || key.isDownVec(&game.controllerButton_flip) {
                game.press_action = true;
            }
            if game.menupage < 12 || (game.menupage >= 30 && game.menupage <= 32) {
                if key.isDownKeycode(Keycode::KpEnter) || key.isDownVec(&game.controllerButton_map) {
                    game.press_map = true;
                }

                if key.isDownKeycode(Keycode::Escape) && !game.mapheld {
                    game.mapheld = true;
                    if game.menupage < 9 {
                        game.menupage = 30;
                    } else if game.menupage < 12 {
                        game.menupage = 32;
                    } else {
                        graphics.resumegamemode = true;
                    }
                    music.playef(11);
                }
            } else {
                if key.isDownKeycode(Keycode::KpEnter) || key.isDownKeycode(Keycode::Escape) || key.isDownVec(&game.controllerButton_map) {
                    game.press_map = true;
                }
            }

            //In the menu system, all keypresses are single taps rather than holds. Therefore this test has to be done for all presses
            if !game.press_action && !game.press_left && !game.press_right {
                game.jumpheld = false;
            }
            if !game.press_map && !key.isDownKeycode(Keycode::Escape) {
                game.mapheld = false;
            }
        } else {
            game.mapheld = true;
            game.jumpheld = true;
        }

        if !game.mapheld {
            if game.press_map && game.menupage < 10 {
                //Normal map screen, do transition later
                graphics.resumegamemode = true;
            }
        }

        if !game.jumpheld {
            if game.press_action || game.press_left || game.press_right || game.press_map {
                game.jumpheld = true;
            }

            if script.running && game.menupage == 3 {
                // Force the player to stay in the SAVE tab while in a cutscene
            } else if game.press_left {
                game.menupage -= 1;
            } else if game.press_right {
                game.menupage += 1;
            }

            if game.press_action {
                mapmenuactionpress(game, graphics, map, music, obj, help, fs, screen_params, screen_settings);
            }

            if game.menupage < 0 { game.menupage = 3; }
            if game.menupage > 3 && game.menupage < 9 { game.menupage = 0; }

            if game.menupage == 9 { game.menupage = 11; }
            if game.menupage == 12 { game.menupage = 10; }

            if game.menupage == 19 { game.menupage = 21; }
            if game.menupage == 22 { game.menupage = 20; }

            if game.menupage == 29 { game.menupage = 32; }
            if game.menupage == 33 { game.menupage = 30; }
        }

        Ok(None)
    }

    pub fn teleporterinput (&mut self, game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, key: &mut key_poll::KeyPoll, obj: &mut entity::EntityClass) -> Result<Option<RenderResult>, i32> {
        //Todo Mouseinput!
        //game.mx = (mouseX / 2);
        //game.my = (mouseY / 2);

        game.press_left = false;
        game.press_right = false;
        game.press_action = false;
        game.press_map = false;

        if graphics.menuoffset == 0 {
            if key.isDownKeycode(Keycode::Left) || key.isDownKeycode(Keycode::A) || key.controllerWantsLeft(false) {
                game.press_left = true;
            }
            if key.isDownKeycode(Keycode::Right) || key.isDownKeycode(Keycode::D) || key.controllerWantsRight(false) {
                game.press_right = true;
            }
            if key.isDownKeycode(Keycode::Z) || key.isDownKeycode(Keycode::Space) || key.isDownKeycode(Keycode::V) ||
               key.isDownKeycode(Keycode::Up) || key.isDownKeycode(Keycode::Down) || key.isDownKeycode(Keycode::W) ||
               key.isDownKeycode(Keycode::S) || key.isDownVec(&game.controllerButton_flip) {
                game.press_action = true;
            }
            if key.isDownKeycode(Keycode::KpEnter) || key.isDownVec(&game.controllerButton_map) {
                game.press_map = true;
            }

            //In the menu system, all keypresses are single taps rather than holds. Therefore this test has to be done for all presses
            if !game.press_action && !game.press_left && !game.press_right {
                game.jumpheld = false
            };
            if !game.press_map { game.mapheld = false };

            if key.isDown(27) {
                if !map.custommode || map.custommodeforreal {
                    // Go to pause menu
                    game.mapheld = true;
                    game.menupage = 30;
                    game.gamestate = GameState::MAPMODE;
                } else {
                    // Close teleporter menu
                    graphics.resumegamemode = true;
                }
            }
        } else {
            game.mapheld = true;
            game.jumpheld = true;
        }

        if !game.jumpheld {
            if game.press_action || game.press_left || game.press_right || game.press_map {
                game.jumpheld = true;
            }

            let mut any_tele_unlocked = false;
            if game.press_left || game.press_right {
                for i in 0..map.teleporters.len() {
                    let tele = &map.teleporters[i];

                    if map.isexplored(tele.x, tele.y) {
                        any_tele_unlocked = true;
                        break;
                    }
                }
            }

            if game.press_left && any_tele_unlocked {
                loop {
                    game.teleport_to_teleporter -= 1;
                    if game.teleport_to_teleporter < 0 {
                        game.teleport_to_teleporter = map.teleporters.len() as i32 - 1;
                    }
                    let tempx = map.teleporters[game.teleport_to_teleporter as usize].x;
                    let tempy = map.teleporters[game.teleport_to_teleporter as usize].y;
                    if !map.isexplored(tempx, tempy) { break }
                }
            } else if game.press_right && any_tele_unlocked {
                loop {
                    game.teleport_to_teleporter += 1;
                    if game.teleport_to_teleporter >= map.teleporters.len() as i32 {
                        game.teleport_to_teleporter = 0;
                    }
                    let tempx = map.teleporters[game.teleport_to_teleporter as usize].x;
                    let tempy = map.teleporters[game.teleport_to_teleporter as usize].y;
                    if !map.isexplored(tempx, tempy) { break }
                }
            }

            if game.press_map {
                let tempx = map.teleporters[game.teleport_to_teleporter as usize].x;
                let tempy = map.teleporters[game.teleport_to_teleporter as usize].y;
                if game.roomx == tempx + 100 && game.roomy == tempy + 100 {
                    //cancel!
                    graphics.resumegamemode = true;
                } else {
                    //teleport
                    graphics.resumegamemode = true;
                    game.teleport_to_x = tempx;
                    game.teleport_to_y = tempy;

                    //trace(game.recordstring);
                    //We're teleporting! Yey!
                    game.activetele = false;
                    game.hascontrol = false;

                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].colour = 102;
                    }

                    let i = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 6;
                        obj.entities[i].colour = 102;
                    }
                    //which teleporter script do we use? it depends on the companion!
                    game.state = 4000;
                    game.statedelay = 0;
                }
            }
        }

        Ok(None)
    }

    pub fn gamecompleteinput (&mut self) -> Result<Option<RenderResult>, i32> {
        println!("DEADBEEF(input.rs): input::Input::gamecompleteinput is not implemented yet");

        Ok(None)
    }

    pub fn gamecompleteinput2 (&mut self) -> Result<Option<RenderResult>, i32> {
        println!("DEADBEEF(input.rs): input::Input::gamecompleteinput2 is not implemented yet");

        Ok(None)
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
    fn menuactionpress(&mut self, music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, screen: &mut screen::Screen, key: &mut key_poll::KeyPoll, fs: &mut filesystem::FileSystem, help: &utility_class::UtilityClass, obj: &mut entity::EntityClass) {
        trace!("menuactionpress: menuname={:?} option={})", game.currentmenuname, game.currentmenuoption);

        let screen_settings = screen.screen_settings;
        let screen_params = screen.get_screen_params();

        match game.currentmenuname {
            MenuName::mainmenu => {
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
                            self.startmode(0, &mut screen.render.graphics);
                        } else {
                            // Bring you to the normal playmenu
                            music.playef(11);
                            game.createmenu(MenuName::play, Some(false), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                            map.nexttowercolour(&mut screen.render.graphics);
                        }
                    },
                    // #endif
                    // #if !defined(NO_CUSTOM_LEVELS)
                    1 => { // OFFSET+1
                        // Bring you to the normal playmenu
                        music.playef(11);
                        game.createmenu(MenuName::playerworlds, Some(false), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    // #endif
                    2 => { // OFFSET+2
                        // Options
                        music.playef(11);
                        game.createmenu(MenuName::options, Some(false), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    // #if !defined(MAKEANDPLAY)
                    3 => { // OFFSET+3
                        // Credits
                        music.playef(11);
                        game.createmenu(MenuName::credits, Some(false), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    // #else
                    //     #undef MPOFFSET
                    //     #define MPOFFSET -2
                    // #endif
                    4 => { // OFFSET+4
                        music.playef(11);
                        game.createmenu(MenuName::youwannaquit, Some(false), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    // #undef OFFSET
                    // #undef NOCUSTOMSOFFSET
                    // #undef MPOFFSET
                    _ => println!("unknown menuoption"),
                }
            },

            // #if !defined(NO_CUSTOM_LEVELS)
            MenuName::levellist => {
                println!("DEADBEEF(input.rs): not fully implemented yet");

                if game.currentmenuoption == game.menuoptions.len() as i32 - 1 {
                    //go back to menu
                    music.playef(11);
                    game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == game.menuoptions.len() as i32 - 2 {
                    //previous page
                    music.playef(11);
                    // if game.levelpage == 0 {
                    //     game.levelpage = (ed.ListOfMetaData.size()-1)/8;
                    // } else {
                    //     game.levelpage -= 1;
                    // }
                    game.createmenu(MenuName::levellist, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    game.currentmenuoption = game.menuoptions.len() as i32 - 2;
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == game.menuoptions.len() as i32 - 3 {
                    //next page
                    music.playef(11);
                    // if game.levelpage*8)+8 >= ed.ListOfMetaData.len() {
                    //     game.levelpage = 0;
                    // } else {
                    //     game.levelpage += 1;
                    // }
                    println!("method not fully implemented");
                    game.createmenu(MenuName::levellist, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    game.currentmenuoption = game.menuoptions.len() as i32 - 3;
                    map.nexttowercolour(&mut screen.render.graphics);
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
                    //     self.startmode(22);
                    // } else {
                    //     game.createmenu(MenuName::quickloadlevel);
                    //     map.nexttowercolour(&mut screen.render.graphics);
                    // }
                    println!("method not fully implemented");
                }
            },
            // #endif
            MenuName::quickloadlevel => {
                match game.currentmenuoption {
                    0 => {
                        // continue save
                        music.playef(11);
                        self.startmode(23, &mut screen.render.graphics);
                    },
                    1 => {
                        music.playef(11);
                        self.startmode(22, &mut screen.render.graphics);
                    },
                    2 => {
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => panic!("incorrect menuoption"),
                };
            },
            // #if !defined(NO_CUSTOM_LEVELS)
            MenuName::playerworlds => {
                println!("DEADBEEF(input.rs): playerworlds menu implemented yet");

                // #if defined(NO_EDITOR)
                //     #define OFFSET -1
                // #else
                //     #define OFFSET 0
                // #endif
                match game.currentmenuoption {
                    0 => {
                        music.playef(11);
                        game.levelpage = 0;
                        // ed.getDirectoryData();
                        game.loadcustomlevelstats(); // Should only load a file if it's needed
                        game.createmenu(MenuName::levellist, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    // #if !defined(NO_EDITOR)
                    1 => { // OFFSET+1
                        // LEVEL EDITOR HOOK
                        music.playef(11);
                        self.startmode(20, &mut screen.render.graphics);
                        // ed.filename = "";
                    },
                    // #endif
                    2 => { // OFFSET+2
                        // "OPENFOLDERHOOK"
                        // if FILESYSTEM_openDirectoryEnabled() && FILESYSTEM_openDirectory(FILESYSTEM_getUserLevelDirectory()) {
                        //     music.playef(11);
                        //     SDL_MinimizeWindow(screen.m_window);
                        // } else {
                        //     music.playef(2);
                        // }
                    },
                    3 => { // OFFSET+3
                        // back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => (),
                };
                // #undef OFFSET
                // #endif
                println!("DEADBEEF(input.rs): input::menuactionpress({:?}) is not fully implemented yet", game.currentmenuname);
            },
            MenuName::graphicoptions => {
                match game.currentmenuoption {
                    0 => {
                        music.playef(11);
                        screen.toggleFullScreen();

                        // Recreate menu to update "resize to nearest"
                        game.createmenu(game.currentmenuname, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);

                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    1 => {
                        music.playef(11);
                        screen.toggleStretchMode();
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    2 => {
                        // resize to nearest multiple
                        if screen.isWindowed {
                            music.playef(11);
                            screen.ResizeToNearestMultiple();
                            game.savestatsandsettings_menu(screen_settings, fs, music, map);
                        } else {
                            music.playef(2);
                        }
                    },
                    3 => {
                        music.playef(11);
                        screen.toggleLinearFilter();
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    4 => {
                        //change smoothing
                        music.playef(11);
                        screen.badSignalEffect= !screen.badSignalEffect;
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    5 => {
                        //toggle vsync
                        music.playef(11);
                        // #ifndef __HAIKU__ // FIXME: Remove after SDL VSync bug is fixed! -flibit
                        screen.vsync = !screen.vsync;
                        screen.resetRendererWorkaround();
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                        // #endif
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::youwannaquit => {
                match game.currentmenuoption {
                    0 => {
                        //bye!
                        music.playef(2);
                        self.startmode(100, &mut screen.render.graphics);
                    },
                    _ => {
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::setinvincibility => {
                match game.currentmenuoption {
                    0 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        map.invincibility = !map.invincibility;
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                };
            },
            MenuName::setslowdown => {
                match game.currentmenuoption {
                    0 => {
                        //back
                        game.slowdown = 30;
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    1 => {
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    2 => {
                        game.slowdown = 18;
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    3 => {
                        game.slowdown = 12;
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    _ => println!("unknown menu option"),
                };
            },
            MenuName::speedrunneroptions => {
                match game.currentmenuoption {
                    0 => {
                        // Glitchrunner mode
                        music.playef(11);
                        game.glitchrunnermode = !game.glitchrunnermode;
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    1 => {
                        /* Input delay */
                        music.playef(11);
                        game.inputdelay = !game.inputdelay;
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    2 => {
                        // toggle fake load screen
                        game.skipfakeload = !game.skipfakeload;
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                        music.playef(11);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                };
            },
            MenuName::advancedoptions => {
                match game.currentmenuoption {
                    0 => {
                        // toggle unfocus pause
                        game.disablepause = !game.disablepause;
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                        music.playef(11);
                    },
                    1 => {
                        // toggle translucent roomname BG
                        screen.render.graphics.translucentroomname = !screen.render.graphics.translucentroomname;
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                        music.playef(11);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                };
            },


            MenuName::accessibility => {
                let mut accessibilityoffset = 0;
                // #if !defined(MAKEANDPLAY)
                accessibilityoffset = 1;
                if game.currentmenuoption == 0 {
                    //unlock play options
                    music.playef(11);
                    game.createmenu(MenuName::unlockmenu, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                }
                // #endif
                if game.currentmenuoption == accessibilityoffset + 0 {
                    //invincibility
                    if !game.ingame_titlemode || (!game.insecretlab && !game.intimetrial && !game.nodeathmode) {
                        if !map.invincibility {
                            game.createmenu(MenuName::setinvincibility, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                            map.nexttowercolour(&mut screen.render.graphics);
                        } else {
                            map.invincibility = !map.invincibility;
                            game.savestatsandsettings_menu(screen_settings, fs, music, map);
                        }
                        music.playef(11);
                    } else {
                        music.playef(2);
                        map.invincibility = false;
                    }
                } else if game.currentmenuoption == accessibilityoffset + 1 {
                    //change game speed
                    if !game.ingame_titlemode || (!game.insecretlab && !game.intimetrial && !game.nodeathmode) {
                        game.createmenu(MenuName::setslowdown, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                        music.playef(11);
                    } else {
                        music.playef(2);
                        game.slowdown = 30;
                    }
                } else if game.currentmenuoption == accessibilityoffset + 2 {
                    //disable animated backgrounds
                    game.colourblindmode = !game.colourblindmode;
                    game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    screen.render.graphics.buffers.towerbg.tdrawback = true;
                    screen.render.graphics.buffers.titlebg.tdrawback = true;
                    music.playef(11);
                } else if game.currentmenuoption == accessibilityoffset + 3 {
                    //disable screeneffects
                    game.noflashingmode = !game.noflashingmode;
                    game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    if !game.noflashingmode {
                        music.playef(18);
                        game.screenshake = 10;
                        game.flashlight = 5;
                    } else {
                        music.playef(11);
                    }
                } else if game.currentmenuoption == accessibilityoffset + 4 {
                    //disable text outline
                    screen.render.graphics.notextoutline = !screen.render.graphics.notextoutline;
                    game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    music.playef(11);
                } else if game.currentmenuoption == accessibilityoffset + 5 {
                    //back
                    music.playef(11);
                    game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                }
            },
            MenuName::gameplayoptions => {
                let mut gameplayoptionsoffset = 0;
                // #if !defined(MAKEANDPLAY)
                if game.ingame_titlemode && game.unlock[18] {
                // #endif
                    gameplayoptionsoffset = 1;
                    if game.currentmenuoption == 0 {
                        toggleflipmode(game, &mut screen.render.graphics, music, screen_settings, fs, map);
                        // Fix wrong area music in Tower (Positive Force vs. ecroF evitisoP)
                        if !map.custommode {
                            let area = map.area(game.roomx, game.roomy);
                            if area == 3 || area == 11 {
                                if screen.render.graphics.setflipmode {
                                    music.play(9, map, game); // ecroF evitisoP
                                } else {
                                    music.play(2, map, game); // Positive Force
                                }
                            }
                        }
                    }
                }

                if game.currentmenuoption == gameplayoptionsoffset + 0 {
                    //Toggle 30+ FPS
                    music.playef(11);
                    game.over30mode = !game.over30mode;
                    game.savestatsandsettings_menu(screen_settings, fs, music, map);
                } else if game.currentmenuoption == gameplayoptionsoffset + 1 {
                    //Speedrunner options
                    music.playef(11);
                    game.createmenu(MenuName::speedrunneroptions, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == gameplayoptionsoffset + 2 {
                    //Advanced options
                    music.playef(11);
                    game.createmenu(MenuName::advancedoptions, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == gameplayoptionsoffset + 3 {
                    //Clear Data
                    music.playef(11);
                    game.createmenu(MenuName::cleardatamenu, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == gameplayoptionsoffset + 4 {
                    //return to previous menu
                    music.playef(11);
                    game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                }
            },
            MenuName::options => {
                match game.currentmenuoption {
                    0 => {
                        //gameplay options
                        music.playef(11);
                        game.createmenu(MenuName::gameplayoptions, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //graphic options
                        music.playef(11);
                        game.createmenu(MenuName::graphicoptions, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    2 => {
                        /* Audio options */
                        music.playef(11);
                        game.createmenu(MenuName::audiooptions, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    3 => {
                        //gamepad options
                        music.playef(11);
                        game.createmenu(MenuName::controller, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    4 => {
                        //accessibility options
                        music.playef(11);
                        game.createmenu(MenuName::accessibility, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        /* Return */
                        music.playef(11);
                        if game.ingame_titlemode {
                            game.returntoingame(&mut screen.render.graphics);
                        } else {
                            game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                            map.nexttowercolour(&mut screen.render.graphics);
                        }
                    }
                };
            },
            MenuName::audiooptions => {
                match game.currentmenuoption {
                    0 | 1 => {
                        music.playef(11);
                        if game.slidermode == SLIDERMODE::SLIDER_NONE {
                            self.initvolumeslider(game.currentmenuoption, game, music);
                        } else {
                            self.deinitvolumeslider(game, screen_settings, fs, music, map);
                        }
                    },
                    2 => {
                        if !music.mmmmmm {
                        }

                        /* Toggle MMMMMM */
                        music.usingmmmmmm = !music.usingmmmmmm;
                        music.playef(11);
                        if music.currentsong > -1 {
                            music.play(music.currentsong, map, game);
                        }
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    _ => warn!("unknown menu option"),
                };

                if game.currentmenuoption == 2 + music.mmmmmm as i32 {
                    /* Return */
                    game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                }
            },
            MenuName::unlockmenutrials => {
                match game.currentmenuoption {
                    0 => {
                        //unlock 1
                        game.unlock[9] = true;
                        game.unlocknotify[9] = true;
                        music.playef(11);
                        game.createmenu(MenuName::unlockmenutrials, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    1 => {
                        //unlock 2
                        game.unlock[10] = true;
                        game.unlocknotify[10] = true;
                        music.playef(11);
                        game.createmenu(MenuName::unlockmenutrials, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    2 => {
                        //unlock 3
                        game.unlock[11] = true;
                        game.unlocknotify[11] = true;
                        music.playef(11);
                        game.createmenu(MenuName::unlockmenutrials, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    3 => {
                        //unlock 4
                        game.unlock[12] = true;
                        game.unlocknotify[12] = true;
                        music.playef(11);
                        game.createmenu(MenuName::unlockmenutrials, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    4 => {
                        //unlock 5
                        game.unlock[13] = true;
                        game.unlocknotify[13] = true;
                        music.playef(11);
                        game.createmenu(MenuName::unlockmenutrials, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    5 => {
                        //unlock 6
                        game.unlock[14] = true;
                        game.unlocknotify[14] = true;
                        music.playef(11);
                        game.createmenu(MenuName::unlockmenutrials, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    6 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => println!("unkown menu option"),
                };
            },
            MenuName::unlockmenu => {
                match game.currentmenuoption {
                    0 => {
                        //unlock time trials separately...
                        music.playef(11);
                        game.createmenu(MenuName::unlockmenutrials, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //unlock intermissions
                        music.playef(11);
                        game.unlock[16] = true;
                        game.unlocknotify[16] = true;
                        game.unlock[6] = true;
                        game.unlock[7] = true;
                        game.createmenu(MenuName::unlockmenu, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    2 => {
                        //unlock no death mode
                        music.playef(11);
                        game.unlock[17] = true;
                        game.unlocknotify[17] = true;
                        game.createmenu(MenuName::unlockmenu, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    3 => {
                        //unlock flip mode
                        music.playef(11);
                        game.unlock[18] = true;
                        game.unlocknotify[18] = true;
                        game.createmenu(MenuName::unlockmenu, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    4 => {
                        //unlock jukebox
                        music.playef(11);
                        game.stat_trinkets = 20;
                        game.createmenu(MenuName::unlockmenu, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    5 => {
                        //unlock secret lab
                        music.playef(11);
                        game.unlock[8] = true;
                        game.unlocknotify[8] = true;
                        game.createmenu(MenuName::unlockmenu, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },

            MenuName::credits => {
                match game.currentmenuoption {
                    0 => {
                        //next page
                        music.playef(11);
                        game.createmenu(MenuName::credits2, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //last page
                        music.playef(11);
                        game.createmenu(MenuName::credits6, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::credits2 => {
                match game.currentmenuoption {
                    0 => {
                        //next page
                        music.playef(11);
                        game.createmenu(MenuName::credits25, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //previous page
                        music.playef(11);
                        game.createmenu(MenuName::credits, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    2 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                    _ => (),
                };
            },
            MenuName::credits25 => {
                match game.currentmenuoption {
                    0 => {
                        //next page
                        music.playef(11);
                        game.createmenu(MenuName::credits3, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //previous page
                        music.playef(11);
                        game.createmenu(MenuName::credits2, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::credits3 => {
                match game.currentmenuoption {
                    0 => {
                        //next page
                        music.playef(11);
                        game.current_credits_list_index += 9;

                        // if game.current_credits_list_index >= SDL_arraysize(Credits::superpatrons) {
                        //     // No more super patrons. Move to the next credits section
                        //     game.current_credits_list_index = 0;
                        //     game.createmenu(MenuName::credits4, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // } else {
                        //     // There are more super patrons. Refresh the menu with the next ones
                        //     game.createmenu(MenuName::credits3, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // }
                        println!("DEADBEEF(input.rs): not implemented yet");

                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //previous page
                        music.playef(11);
                        game.current_credits_list_index -= 9;

                        if game.current_credits_list_index < 0 {
                            //No more super patrons. Move to the previous credits section
                            game.current_credits_list_index = 0;
                            game.createmenu(MenuName::credits25, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        } else {
                            //There are more super patrons. Refresh the menu with the next ones
                            game.createmenu(MenuName::credits3, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        }

                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.current_credits_list_index = 0;
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::credits4 => {
                match game.currentmenuoption {
                    0 => {
                        //next page
                        music.playef(11);
                        game.current_credits_list_index += 14;

                        // if game.current_credits_list_index >= SDL_arraysize(Credits::patrons) {
                        //     // No more patrons. Move to the next credits section
                        //     game.current_credits_list_index = 0;
                        //     game.createmenu(MenuName::credits5, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // } else {
                        //     // There are more patrons. Refresh the menu with the next ones
                        //     game.createmenu(MenuName::credits4, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // }
                        println!("DEADBEEF(input.rs): not implemented yet");

                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //previous page
                        music.playef(11);
                        game.current_credits_list_index -= 14;

                        // if game.current_credits_list_index < 0 {
                        //     //No more patrons. Move to the previous credits section
                        //     game.current_credits_list_index = SDL_arraysize(Credits::superpatrons) - 1 - (SDL_arraysize(Credits::superpatrons)-1)%9;
                        //     game.createmenu(MenuName::credits3, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // } else {
                        //     //There are more patrons. Refresh the menu with the next ones
                        //     game.createmenu(MenuName::credits4, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // }
                        println!("DEADBEEF(input.rs): not implemented yet");

                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.current_credits_list_index = 0;
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::credits5 => {
                match game.currentmenuoption {
                    0 => {
                        //next page
                        music.playef(11);
                        game.current_credits_list_index += 9;

                        // if game.current_credits_list_index >= SDL_arraysize(Credits::githubfriends) {
                        //     // No more GitHub contributors. Move to the next credits section
                        //     game.current_credits_list_index = 0;
                        //     game.createmenu(MenuName::credits6, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // } else {
                        //     // There are more GitHub contributors. Refresh the menu with the next ones
                        //     game.createmenu(MenuName::credits5, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // }
                        println!("DEADBEEF(input.rs): not implemented yet");

                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //previous page
                        music.playef(11);
                        game.current_credits_list_index -= 9;

                        // if game.current_credits_list_index < 0 {
                        //     //No more GitHub contributors. Move to the previous credits section
                        //     game.current_credits_list_index = SDL_arraysize(Credits::patrons) - 1 - (SDL_arraysize(Credits::patrons)-1)%14;
                        //     game.createmenu(MenuName::credits4, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // } else {
                        //     //There are more GitHub contributors. Refresh the menu with the next ones
                        //     game.createmenu(MenuName::credits5, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        // }
                        println!("DEADBEEF(input.rs): not implemented yet");

                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.current_credits_list_index = 0;
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::credits6 => {
                match game.currentmenuoption {
                    0 => {
                        //first page
                        music.playef(11);
                        game.createmenu(MenuName::credits, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //previous page
                        music.playef(11);
                        // game.current_credits_list_index = SDL_arraysize(Credits::githubfriends) - 1 - (SDL_arraysize(Credits::githubfriends)-1)%9;
                        println!("DEADBEEF(input.rs): not implemented yet");
                        game.createmenu(MenuName::credits5, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },


            MenuName::play => {
                //Do we have the Secret Lab option?
                let sloffset = if game.unlock[8] { 0 } else { -1 };
                //Do we have a telesave or quicksave?
                let ngoffset = if game.save_exists() { 0 } else { -1 };
                if game.currentmenuoption == 0 {
                    //continue
                    //right, this depends on what saves you've got
                    if !game.save_exists() {
                        //You have no saves but have something unlocked, or you couldn't have gotten here
                        music.playef(11);
                        self.startmode(0, &mut screen.render.graphics);
                    } else if game.telesummary == "" {
                        //You at least have a quicksave, or you couldn't have gotten here
                        music.playef(11);
                        self.startmode(2, &mut screen.render.graphics);
                    } else if game.quicksummary == "" {
                        //You at least have a telesave, or you couldn't have gotten here
                        music.playef(11);
                        self.startmode(1, &mut screen.render.graphics);
                    } else {
                        //go to a menu!
                        music.playef(11);
                        game.loadsummary(fs, map, music, obj, help); //Prepare save slots to display
                        game.createmenu(MenuName::continuemenu, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    }
                } else if game.currentmenuoption == 1 && game.unlock[8] {
                    if !map.invincibility && game.slowdown == 30 {
                        music.playef(11);
                        self.startmode(11, &mut screen.render.graphics);
                    } else {
                        //Can't do yet! play sad sound
                        music.playef(2);
                    }
                } else if game.currentmenuoption == sloffset+2 {
                    //play modes
                    music.playef(11);
                    game.createmenu(MenuName::playmodes, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == sloffset+3 && game.save_exists() {
                    //newgame
                    music.playef(11);
                    game.createmenu(MenuName::newgamewarning, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == sloffset+ngoffset+4 {
                    //back
                    music.playef(11);
                    game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                }
            },
            MenuName::newgamewarning => {
                match game.currentmenuoption {
                    0 => {
                        //yep
                        music.playef(11);
                        self.startmode(0, &mut screen.render.graphics);
                        game.deletequick();
                        game.deletetele();
                    },
                    _ => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::controller => {
                match game.currentmenuoption {
                    0 => {
                        key.sensitivity += 1;
                        music.playef(11);
                        if key.sensitivity > 4 {
                            key.sensitivity = 0;
                        }
                        game.savestatsandsettings_menu(screen_settings, fs, music, map);
                    },
                    5 => {
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => (),
                };
            },
            MenuName::cleardatamenu => {
                match game.currentmenuoption {
                    0 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => {
                        //yep
                        music.playef(23);
                        game.deletequick();
                        game.deletetele();
                        game.deletestats();
                        game.deletesettings();
                        game.flashlight = 5;
                        game.screenshake = 15;
                        game.createmenu(MenuName::mainmenu, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    }
                };
            },
            MenuName::playmodes => {
                if game.currentmenuoption == 0 && game.slowdown == 30 && !map.invincibility {
                    //go to the time trial menu
                    music.playef(11);
                    game.createmenu(MenuName::timetrials, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == 1 && game.unlock[16] {
                    //intermission mode menu
                    music.playef(11);
                    game.createmenu(MenuName::intermissionmenu, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == 2 && game.unlock[17] && game.slowdown == 30 && !map.invincibility {
                    //start a game in no death mode
                    music.playef(11);
                    game.createmenu(MenuName::startnodeathmode, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else if game.currentmenuoption == 3 && game.unlock[18] {
                    //enable/disable flip mode
                    toggleflipmode(game, &mut screen.render.graphics, music, screen_settings, fs, map);
                } else if game.currentmenuoption == 4 {
                    //back
                    music.playef(11);
                    game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else {
                    //Can't do yet! play sad sound
                    music.playef(2);
                }
            },
            MenuName::startnodeathmode => {
                match game.currentmenuoption {
                    0 => {
                        //start no death mode, disabling cutscenes
                        music.playef(11);
                        self.startmode(10, &mut screen.render.graphics);
                    },
                    1 => {
                        music.playef(11);
                        self.startmode(9, &mut screen.render.graphics);
                    },
                    2 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => (),
                };
            },
            MenuName::continuemenu => {
                match game.currentmenuoption {
                    0 => {
                        music.playef(11);
                        self.startmode(1, &mut screen.render.graphics);
                    },
                    1 => {
                        music.playef(11);
                        self.startmode(2, &mut screen.render.graphics);
                    },
                    2 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => (),
                };
            },
            MenuName::intermissionmenu => {
                match game.currentmenuoption {
                    0 => {
                        music.playef(11);
                        music.play(6, map, game);
                        game.createmenu(MenuName::playint1, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        music.playef(11);
                        music.play(6, map, game);
                        game.createmenu(MenuName::playint2, None, &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    2 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => (),
                };
            },
            MenuName::playint1 => {
                match game.currentmenuoption {
                    0 => {
                        music.playef(11);
                        self.startmode(12, &mut screen.render.graphics);
                    },
                    1 => {
                        music.playef(11);
                        self.startmode(13, &mut screen.render.graphics);
                    },
                    2 => {
                        music.playef(11);
                        self.startmode(14, &mut screen.render.graphics);
                    },
                    3 => {
                        music.playef(11);
                        self.startmode(15, &mut screen.render.graphics);
                    },
                    4 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => (),
                };
            },
            MenuName::playint2 => {
                match game.currentmenuoption {
                    0 => {
                        music.playef(11);
                        self.startmode(16, &mut screen.render.graphics);
                    },
                    1 => {
                        music.playef(11);
                        self.startmode(17, &mut screen.render.graphics);
                    },
                    2 => {
                        music.playef(11);
                        self.startmode(18, &mut screen.render.graphics);
                    },
                    3 => {
                        music.playef(11);
                        self.startmode(19, &mut screen.render.graphics);
                    },
                    4 => {
                        //back
                        music.playef(11);
                        game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    _ => (),
                };
            },
            MenuName::gameover2 => {
                //back
                music.playef(11);
                music.play(6, map, game);
                game.returntomenu(MenuName::playmodes);
                map.nexttowercolour(&mut screen.render.graphics);
            },
            MenuName::unlocktimetrials | MenuName::unlocktimetrial | MenuName::unlocknodeathmode | MenuName::unlockintermission | MenuName::unlockflipmode => {
                //back
                music.playef(11);
                game.createmenu(MenuName::play, Some(true), &mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                map.nexttowercolour(&mut screen.render.graphics);
            },
            MenuName::timetrials => {
                if game.currentmenuoption == 0 && game.unlock[9] {
                    //space station 1
                    music.playef(11);
                    self.startmode(3, &mut screen.render.graphics);
                } else if game.currentmenuoption == 1 && game.unlock[10] {
                    //lab
                    music.playef(11);
                    self.startmode(4, &mut screen.render.graphics);
                } else if game.currentmenuoption == 2 && game.unlock[11] {
                    //tower
                    music.playef(11);
                    self.startmode(5, &mut screen.render.graphics);
                } else if game.currentmenuoption == 3 && game.unlock[12] {
                    //station 2
                    music.playef(11);
                    self.startmode(6, &mut screen.render.graphics);
                } else if game.currentmenuoption == 4 && game.unlock[13] {
                    //warp
                    music.playef(11);
                    self.startmode(7, &mut screen.render.graphics);
                } else if game.currentmenuoption == 5 && game.unlock[14] {
                    //final
                    music.playef(11);
                    self.startmode(8, &mut screen.render.graphics);
                } else if game.currentmenuoption == 6 {
                    //go to the time trial menu
                    //back
                    music.playef(11);
                    game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                    map.nexttowercolour(&mut screen.render.graphics);
                } else {
                    //Can't do yet! play sad sound
                    music.playef(2);
                }
            },
            MenuName::timetrialcomplete3 => {
                match game.currentmenuoption {
                    0 => {
                        //back
                        music.playef(11);
                        music.play(6, map, game);
                        game.returntomenu(MenuName::timetrials);
                        map.nexttowercolour(&mut screen.render.graphics);
                    },
                    1 => {
                        //duplicate the above based on given time trial level!
                        if game.timetriallevel == 0 {
                            //space station 1
                            music.playef(11);
                            self.startmode(3, &mut screen.render.graphics);
                        } else if game.timetriallevel == 1 {
                            //lab
                            music.playef(11);
                            self.startmode(4, &mut screen.render.graphics);
                        } else if game.timetriallevel == 2 {
                            //tower
                            music.playef(11);
                            self.startmode(5, &mut screen.render.graphics);
                        } else if game.timetriallevel == 3 {
                            //station 2
                            music.playef(11);
                            self.startmode(6, &mut screen.render.graphics);
                        } else if game.timetriallevel == 4 {
                            //warp
                            music.playef(11);
                            self.startmode(7, &mut screen.render.graphics);
                        } else if game.timetriallevel == 5 {
                            //final
                            music.playef(11);
                            self.startmode(8, &mut screen.render.graphics);
                        }
                    }
                    _ => (),
                };
            },
            MenuName::gamecompletecontinue | MenuName::nodeathmodecomplete2 => {
                music.play(6, map, game);
                music.playef(11);
                game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                map.nexttowercolour(&mut screen.render.graphics);
            },
            MenuName::errorsavingsettings => {
                if game.currentmenuoption == 1 {
                    game.silence_settings_error = true;
                }
                music.playef(11);
                game.returnmenu(&mut screen.render.graphics, music, screen_params, map, screen_settings, fs);
                map.nexttowercolour(&mut screen.render.graphics);
            },

            _ => println!("{:?} menu option not implemented yet", game.currentmenuname),
        };
    }

    // static void initvolumeslider(const int menuoption)
    fn initvolumeslider(&mut self, menuoption: i32, game: &mut game::Game, music: &mut music::Music) {
        match menuoption {
            0 => {
                game.slidermode = game::SLIDERMODE::SLIDER_MUSICVOLUME;
                self.user_changing_volume = Some(*music.user_music_volume);
                self.previous_volume = *music.user_music_volume;
            },
            1 => {
                game.slidermode = game::SLIDERMODE::SLIDER_SOUNDVOLUME;
                self.user_changing_volume = Some(*music.user_sound_volume);
                self.previous_volume = *music.user_sound_volume;
            },
            _ => {
                warn!("Unhandled volume slider option!");
                game.slidermode = game::SLIDERMODE::SLIDER_NONE;
                self.user_changing_volume = None;
            }
        }
    }

    // static void deinitvolumeslider(void)
    fn deinitvolumeslider(&mut self, game: &mut game::Game, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem, music: &mut music::Music, map: &mut map::Map) {
        self.user_changing_volume = None;
        game.savestatsandsettings_menu(screen_settings, fs, music, map);
        game.slidermode = game::SLIDERMODE::SLIDER_NONE;
    }

    // static void slidermodeinput(void)
    fn slidermodeinput(&mut self, game: &mut game::Game, music: &mut music::Music) {
        let volume = match game.slidermode {
            game::SLIDERMODE::SLIDER_MUSICVOLUME => &mut music.user_music_volume,
            game::SLIDERMODE::SLIDER_SOUNDVOLUME => &mut music.user_sound_volume,
            _ => {
                warn!("Unhandled volume slider option!");
                // warn!("user_changing_volume is NULL!");
                return
            },
        };

        if game.press_left {
            **volume = **volume - music::USER_VOLUME_STEP;
        } else if game.press_right {
            **volume = **volume + music::USER_VOLUME_STEP;
        };

        **volume = maths::clamp(**volume, 0, music::USER_VOLUME_MAX);
        self.user_changing_volume = Some(**volume);
    }
}

// static void updatebuttonmappings(int bind)
fn updatebuttonmappings(key: &mut key_poll::KeyPoll, game: &mut game::Game, bind: i32) {
    println!("DEADBEEF(input.rs): input::Input::updatebuttonmappings is not implemented yet");

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
fn toggleflipmode(game: &mut game::Game, graphics: &mut graphics::Graphics, music: &mut music::Music, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem, map: &mut map::Map) {
    graphics.setflipmode = !graphics.setflipmode;
    game.savestatsandsettings_menu(screen_settings, fs, music, map);

    if graphics.setflipmode {
        music.playef(18);
        game.screenshake = 10;
        game.flashlight = 5;
    } else {
        music.playef(11);
    }
}

// static void mapmenuactionpress(void)
fn mapmenuactionpress(game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, music: &mut music::Music, obj: &mut entity::EntityClass, help: &mut utility_class::UtilityClass, fs: &mut filesystem::FileSystem, screen_params: ScreenParams, screen_settings: screen::ScreenSettings) {
    match game.menupage {
        1 => {
            if obj.flags[67] && !game.inspecial() && !map.custommode {
                //Warp back to the ship
                graphics.resumegamemode = true;

                game.teleport_to_x = 2;
                game.teleport_to_y = 11;

                //trace(game.recordstring);
                //We're teleporting! Yey!
                game.activetele = false;
                game.hascontrol = false;

                let i = obj.getplayer() as usize;
                if INBOUNDS_VEC!(i, obj.entities) {
                    obj.entities[i].colour = 102;
                }

                //which teleporter script do we use? it depends on the companion!
                game.state = 4000;
                game.statedelay = 0;
            }
        },
        3 => {
            if !game.gamesaved && !game.gamesavefailed && !game.inspecial() {
                game.flashlight = 5;
                game.screenshake = 10;
                music.playef(18);

                game.savetime = game.timestring(help);
                game.savearea = map.currentarea(map.area(game.roomx, game.roomy)).to_string();
                game.savetrinkets = game.trinkets(obj);

                if game.roomx >= 102 && game.roomx <= 104 && game.roomy >= 110 && game.roomy <= 111 {
                    game.savearea = "The Ship".to_string();
                }

                let success =
                // #if !defined(NO_CUSTOM_LEVELS)
                // if map.custommodeforreal {
                //     game.customsavequick(ed.ListOfMetaData[game.playcustomlevel].filename)
                // }
                // else
                // #endif
                {
                    game.savequick(fs, map, music, obj, help)
                };
                game.gamesaved = success;
                game.gamesavefailed = !success;
            }
        },
        10 => {
            //return to pause menu
            music.playef(11);
            game.menupage = 32;
        },
        11 => {
            //quit to menu

            //Kill contents of offset render buffer, since we do that for some reason.
            //This fixes an apparent frame flicker.
            graphics_util::ClearSurface(&mut graphics.buffers.tempBuffer);
            graphics.fademode = 2;
            music.fadeout(None, game);
            map.nexttowercolour(graphics);
            if !game.glitchrunnermode {
                game.fadetomenu = true;
                game.fadetomenudelay = 16;
            }
        },
        20 => {
            //return to game
            graphics.resumegamemode = true;
        },
        21 => {
            //quit to menu
            game.swnmode = false;
            graphics.fademode = 2;
            music.fadeout(None, game);
            if !game.glitchrunnermode {
                game.fadetolab = true;
                game.fadetolabdelay = 16;
            }
        },
        30 => {
            // Return to game
            graphics.resumegamemode = true;
            music.playef(11);
        },
        31 => {
            // Graphic options and game options
            music.playef(11);
            game.gamestate = GameState::TITLEMODE;
            graphics.flipmode = false;
            game.ingame_titlemode = true;
            graphics.ingame_fademode = graphics.fademode;
            graphics.fademode = 0;

            // Set this before we create the menu
            game.kludge_ingametemp = game.currentmenuname;
            game.createmenu(MenuName::options, None, graphics, music, screen_params, map, screen_settings, fs);
            map.nexttowercolour(graphics);
        },
        32 => {
            // Go to quit prompt
            music.playef(11);
            game.menupage = 10;
        },
        _ => eprintln!("unknown menupage {}", game.menupage),
    };
}
