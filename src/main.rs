use std::time::Duration;

extern crate sdl2;
use sdl2::EventPump;
mod sdl2u;

mod game;
use game::GameState;
mod input;
mod key_poll;
mod logic;
mod scenes;
use scenes::{InputTrait, RenderFixedTrait, RenderTrait};

use crate::scenes::RenderResult;
mod screen;

mod rustutil;

fn print_logo() {
    println!("\t\t");
    println!("\t\t");
    println!("\t\t       VVVVVV");
    println!("\t\t rust implementation");
    println!("\t\t");
    println!("\t\t");
    println!("\t\t  8888888888888888  ");
    println!("\t\t88888888888888888888");
    println!("\t\t  888888    8888    88");
    // println!("\t\t888888    8888    88");
    // println!("\t\t88888888888888888888");
    println!("\t\t  88888888888888888888");
    println!("\t\t888888            88");
    println!("\t\t88888888        8888");
    println!("\t\t  8888888888888888  ");
    println!("\t\t      88888888      ");
    println!("\t\t  8888888888888888  ");
    println!("\t\t   88888888888888888888");
    // println!("\t\t88888888888888888888");
    // println!("\t\t88888888888888888888");
    // println!("\t\t8888  88888888  8888");
    println!("\t\t   8888  88888888  8888");
    println!("\t\t    888888888888    ");
    println!("\t\t    8888    8888    ");
    println!("\t\t     888888    888888  ");
    // println!("\t\t  888888    888888  ");
    // println!("\t\t  888888    888888  ");
    println!("\t\t");
    println!("\t\t");
}

struct Main {
    sdl_context: sdl2::Sdl,
    input: input::Input,
    scenes: scenes::Scenes,

    // script: scriptclass,

    // #if !defined(NO_CUSTOM_LEVELS)
    // edentity: Vec<edentities>,
    // ed: editorclass;
    // #endif

    // help: UtilityClass,
    // graphics: graphics::Graphics,
    // music: musicclass,
    game: game::Game,
    key: key_poll::KeyPoll,
    // map: mapclass,
    // obj: entityclass,
    gameScreen: screen::Screen,

    startinplaytest: bool,
    savefileplaytest: bool,
    savex: i32,
    savey: i32,
    saverx: i32,
    savery: i32,
    savegc: i32,
    savemusic: i32,
    playassets: String,

    playtestname: String,
    accumulator: f32,
}

impl Main {
    fn new() -> Main {
        let sdl_context = Main::create_sdl_context();

        // if !FILESYSTEM_init(argv[0], baseDir, assetsPath) {
        //     puts("Unable to initialize filesystem!");
        //     return 1;
        // }
        // NETWORK_init();
        // Load Ini, reloadresources loads music too...

        let mut game = game::Game::new();
        // graphics.init();
        // graphics.reloadresources();

        // game.gamestate = PRELOADER;
        // game.menustart = false;
        // game.mainmenu = 0;

        // TODO @sx
        // map.ypos = (700-29) * 8;
        // graphics.towerbg.bypos = map.ypos / 2;
        // graphics.titlebg.bypos = map.ypos / 2;

        // TODO @sx
        // // Moved screensetting init here from main menu V2.1
        // int width = 320;
        // int height = 240;
        // bool vsync = false;

        // Prioritize unlock.vvv first (2.2 and below), but settings have been migrated to settings.vvv (2.3 and up)
        // game.loadstats(&width, &height, &vsync);
        // game.loadsettings(&width, &height, &vsync);

        let gameScreen = screen::Screen::new(&sdl_context);
        // gameScreen.init(
        //     width,
        //     height,
        //     game.fullscreen,
        //     vsync,
        //     game.stretchMode,
        //     game.useLinearFilter,
        //     game.fullScreenEffect_badSignal
        // );

        // obj.init();

        Main {
            sdl_context,
            input: input::Input::new(),
            scenes: scenes::Scenes::new(),

            // script: scriptclass,
            // edentity: Vec<edentities>,
            // ed: editorclass;

            // help: UtilityClass,
            // graphics: graphics::Graphics::new(),
            // music: musicclass,
            game: game::Game::new(),
            key: key_poll::KeyPoll::new(),
            // map: mapclass,
            // obj: entityclass,
            gameScreen,

            startinplaytest: false,
            savefileplaytest: false,
            savex: 0,
            savey: 0,
            saverx: 0,
            savery: 0,
            savegc: 0,
            savemusic: 0,
            playassets: String::new(),

            playtestname: String::new(),
            accumulator: 0f32,
        }
    }

    /* methods extracted from original main.cpp::main() */

    fn init_arguments(&mut self) {
        //     for (int i = 1; i < argc; ++i)
        //     {
        // #define ARG(name) (strcmp(argv[i], name) == 0)
        // #define ARG_INNER(code) \
        //     if (i + 1 < argc) \
        //     { \
        //         code \
        //     } \
        //     else \
        //     { \
        //         printf("%s option requires one argument.\n", argv[i]); \
        //         return 1; \
        //     }

        //         if (ARG("-renderer"))
        //         {
        //             ARG_INNER({
        //                 i++;
        //                 SDL_SetHintWithPriority(SDL_HINT_RENDER_DRIVER, argv[i], SDL_HINT_OVERRIDE);
        //             })
        //         }
        //         else if (ARG("-basedir"))
        //         {
        //             ARG_INNER({
        //                 i++;
        //                 baseDir = argv[i];
        //             })
        //         }
        //         else if (ARG("-assets"))
        //         {
        //             ARG_INNER({
        //                 i++;
        //                 assetsPath = argv[i];
        //             })
        //         }
        //         else if (ARG("-playing") || ARG("-p"))
        //         {
        //             ARG_INNER({
        //                 i++;
        //                 startinplaytest = true;
        //                 playtestname = std::string("levels/");
        //                 playtestname.append(argv[i]);
        //                 playtestname.append(std::string(".vvvvvv"));
        //             })
        //         }
        //         else if (ARG("-playx") || ARG("-playy") ||
        //         ARG("-playrx") || ARG("-playry") ||
        //         ARG("-playgc") || ARG("-playmusic"))
        //         {
        //             ARG_INNER({
        //                 savefileplaytest = true;
        //                 int v = help.Int(argv[i+1]);
        //                 if (ARG("-playx")) savex = v;
        //                 else if (ARG("-playy")) savey = v;
        //                 else if (ARG("-playrx")) saverx = v;
        //                 else if (ARG("-playry")) savery = v;
        //                 else if (ARG("-playgc")) savegc = v;
        //                 else if (ARG("-playmusic")) savemusic = v;
        //                 i++;
        //             })
        //         }
        //         else if (ARG("-playassets"))
        //         {
        //             ARG_INNER({
        //                 i++;
        //                 // Even if this is a directory, FILESYSTEM_mountassets() expects '.vvvvvv' on the end
        //                 playassets = "levels/" + std::string(argv[i]) + ".vvvvvv";
        //             })
        //         }
        // #undef ARG_INNER
        // #undef ARG
        //         else
        //         {
        //             printf("Error: invalid option: %s\n", argv[i]);
        //             return 1;
        //         }
        //     }
    }

    pub fn create_sdl_context() -> sdl2::Sdl {
        // let sdl_context = sdl2::init().unwrap();
        match sdl2::init() {
            Ok(sdl_context) => {
                unsafe {
                    sdl2_sys::SDL_Init( sdl2_sys::SDL_INIT_VIDEO | sdl2_sys::SDL_INIT_AUDIO | sdl2_sys::SDL_INIT_JOYSTICK | sdl2_sys::SDL_INIT_GAMECONTROLLER);
                    if sdl2_sys::SDL_IsTextInputActive() == sdl2_sys::SDL_bool::SDL_TRUE {
                        sdl2_sys::SDL_StopTextInput();
                    }
                }

                return sdl_context;
            },
            Err(_) => panic!("unable to create SDL context"),
        };
    }

    fn no_custom_levels(&mut self) {
        // #if !defined(NO_CUSTOM_LEVELS)
        // if (startinplaytest) {
        //     game.levelpage = 0;
        //     game.playcustomlevel = 0;
        //     game.playassets = playassets;
        //     game.menustart = true;

        //     ed.directoryList.clear();
        //     ed.directoryList.push_back(playtestname);

        //     LevelMetaData meta;
        //     if (ed.getLevelMetaData(playtestname, meta)) {
        //         ed.ListOfMetaData.clear();
        //         ed.ListOfMetaData.push_back(meta);
        //     } else {
        //         ed.loadZips();
        //         if (ed.getLevelMetaData(playtestname, meta)) {
        //             ed.ListOfMetaData.clear();
        //             ed.ListOfMetaData.push_back(meta);
        //         } else {
        //             printf("Level not found\n");
        //             return 1;
        //         }
        //     }

        //     game.loadcustomlevelstats();
        //     game.customleveltitle=ed.ListOfMetaData[game.playcustomlevel].title;
        //     game.customlevelfilename=ed.ListOfMetaData[game.playcustomlevel].filename;
        //     if (savefileplaytest) {
        //         game.playx = savex;
        //         game.playy = savey;
        //         game.playrx = saverx;
        //         game.playry = savery;
        //         game.playgc = savegc;
        //         game.playmusic = savemusic;
        //         game.cliplaytest = true;
        //         script.startgamemode(23);
        //     } else {
        //         script.startgamemode(22);
        //     }

        //     graphics.fademode = 0;
        // }
        // #endif
    }

    fn main_loop(&mut self) {
        let mut time_: u32 = 0;
        let mut timePrev: u32 = 0;
        let mut f_time: u32 = 0;
        let mut f_timePrev: u32 = 0;

        self.gameScreen.init_canvas();

        let mut event_pump = match self.sdl_context.event_pump() {
            Ok(v) => v,
            Err(s) => panic!("{}", s),
        };
        let timer = match self.sdl_context.timer() {
            Ok(v) => v,
            Err(s) => panic!("{}", s),
        };

        // key.isActive = true;
        let mut key = key_poll::KeyPoll::new();
        // gamestate_funcs = get_gamestate_funcs(game.gamestate, &num_gamestate_funcs);
        // loop_assign_active_funcs();

        'running: loop {
            let now = std::time::Instant::now();

            f_time = timer.ticks();
            let f_timetaken: u32 = f_time - f_timePrev;
            if !self.game.over30mode && f_timetaken < 34 {
                let f_delay: u32 = 34 - f_timetaken;
                ::std::thread::sleep(Duration::new(0, f_delay));
                // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
                f_time = timer.ticks();
            }

            self.deltaloop(time_ - timePrev, &mut key, &mut event_pump);

            f_timePrev = f_time;
            timePrev = time_;
            time_ = timer.ticks();

            println!("main loop iter done in {:?}ms", now.elapsed().as_millis());
        }
    }

    fn deltaloop(&mut self, rawdeltatime: u32, key: &mut key_poll::KeyPoll, event_pump: &mut EventPump) {
        // timestep limit to 30
        self.accumulator = self.accumulator + rawdeltatime as f32;
        let timesteplimit = self.game.get_timestep() as f32;

        while self.accumulator >= timesteplimit {
            // let index_code: IndexCode = increment_func_index();

            // if index_code == Index_end {
            //     loop_assign_active_funcs();
            // }

            self.accumulator = sdl2u::sys_fmodf(self.accumulator, timesteplimit);

            /* We are done rendering. */
            // graphics.renderfixedpost();

            self.fixedloop(key, event_pump);
        }

        // const float alpha = game.over30mode ? static_cast<float>(accumulator) / timesteplimit : 1.0f;
        let alpha: f32;
        if self.game.over30mode {
            alpha = (self.accumulator / timesteplimit) as f32;
        } else {
            alpha = 1.0f32;
        }
        self.gameScreen.render.graphics.alpha = alpha;

        // if active_func_index == NULL || *active_func_index == -1 || active_funcs == NULL {
        //     0;
        //     /* Somehow the first deltatime has been too small and things haven't
        //     * initialized. We'll just no-op for now.
        //         */
        // } else {
        //     // const struct ImplFunc* implfunc = &(*active_funcs)[*active_func_index];

        //     if implfunc.type == Func_delta && implfunc.func != NULL {
        //         implfunc.func();
        //         gameScreen.FlipScreen();
        //     }
        // }

        println!("delta loop finish");
    }

    fn fixedloop(&mut self, key: &mut key_poll::KeyPoll, event_pump: &mut EventPump) {
        let meta_funcs: Vec<&dyn Fn(&mut game::Game, &mut screen::Screen, &mut key_poll::KeyPoll, &mut input::Input, &mut EventPump, &mut scenes::Scenes) -> LoopCode> = vec![
            &loop_begin,
            &loop_assign_active_funcs,
            &loop_run_active_funcs,
            &loop_end,
        ];

        'fixedloop: loop {
            for meta_func in &meta_funcs {
                match meta_func(&mut self.game, &mut self.gameScreen, key, &mut self.input, event_pump, &mut self.scenes) {
                    LoopCode::LoopContinue => (),
                    LoopCode::LoopStop => break 'fixedloop,
                }
            }
        }
        println!("fixed loop finish");
    }

}

impl Drop for Main {
    // free_assets
    fn drop(&mut self) {
        // game.savestats();
        // NETWORK_shutdown();
        // SDL_Quit();
        // FILESYSTEM_deinit();
    }
}

fn main() {
    let mut m = Main::new();
    m.init_arguments();
    print_logo();
    m.no_custom_levels();
    m.main_loop();
}

fn loop_begin(game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes) -> LoopCode {
    println!("loop_begin");
    if game.inputdelay {
        key.Poll(event_pump, game);
    }

    // Update network per frame.
    // TODO @sx
    // NETWORK_update();

    LoopCode::LoopContinue
}

fn loop_assign_active_funcs(game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes) -> LoopCode {
    println!("loop_assign_active_funcs");
    // TODO @sx
    // if key.isActive {
    //     active_funcs = &gamestate_funcs;
    //     num_active_funcs = &num_gamestate_funcs;
    //     active_func_index = &gamestate_func_index;
    //     increment_func_index = &increment_gamestate_func_index;
    // } else {
    //     active_funcs = &unfocused_funcs;
    //     num_active_funcs = &num_unfocused_funcs;
    //     active_func_index = &unfocused_func_index;
    //     increment_func_index = &increment_unfocused_func_index;
    // }

    LoopCode::LoopContinue
}

// static enum LoopCode loop_run_active_funcs(void)
fn loop_run_active_funcs(game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes) -> LoopCode {
    println!("loop_run_active_funcs");
    // while (*active_funcs)[*active_func_index].type != Func_delta {
    //     // const struct ImplFunc* implfunc = &(*active_funcs)[*active_func_index];
    //     let implfunc = &(*gamestate_funcs)[*active_func_index];

    //     if implfunc.type == Func_input && !game.inputdelay {
            key.Poll(event_pump, game);
    //     }

    //     if implfunc.type != Func_null && implfunc.func != NULL {
    //         implfunc.func();
    //     }

    //     // index_code = increment_func_index();
    //     let index_code = increment_gamestate_func_index();

    //     if index_code == IndexCode::IndexEnd {
    //         return LoopCode::LoopContinue;
    //     }
    // }

    // // /* About to switch over to rendering... but call this first. */
    // gameScreen.render.graphics.renderfixedpre();

    // // return Loop_stop;

    loop_run_active_funcs_modified(game, gameScreen, key, input, event_pump, scenes)
}

fn loop_run_active_funcs_modified(game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes) -> LoopCode {
    println!("loop_run_active_funcs_modified");

    game.gamestate = GameState::TITLEMODE; // TODO @sx
    let mut rr = RenderResult::None;

    if key.isActive {
        match game.gamestate {
            GameState::PRELOADER => {
                // {Func_input, preloaderinput},
                scenes.preloader.input(game, key);
                // {Func_fixed, preloaderrenderfixed},
                scenes.preloader.render_fixed(game);
                // {Func_delta, preloaderrender},
                rr = scenes.preloader.render(&mut gameScreen.render.graphics);
            },
            GameState::TITLEMODE => {
                // {Func_input, titleinput},
                input.titleinput(game, gameScreen, key);
                // {Func_fixed, titlerenderfixed},
                gameScreen.renderfixed.title_render_fixed(game, &mut gameScreen.render.graphics);
                // // {Func_delta, titlerender},
                rr = gameScreen.render.title_render(game);
                // // {Func_fixed, titlelogic},
                logic::title_logic(game, &mut gameScreen.renderfixed, &mut gameScreen.render.graphics);
            },
            GameState::GAMEMODE => {
                // {Func_fixed, runscript},
                // {Func_fixed, gamerenderfixed},
                // {Func_delta, gamerender},
                // {Func_input, gameinput},
                // {Func_fixed, gamelogic},
            },
            GameState::MAPMODE => {
                // {Func_fixed, maprenderfixed},
                // {Func_delta, maprender},
                // {Func_input, mapinput},
                // {Func_fixed, maplogic},
            },
            GameState::TELEPORTERMODE => {
                // {Func_fixed, maprenderfixed},
                // {Func_delta, teleporterrender},
                // {Func_input, teleportermodeinput},
                // {Func_fixed, maplogic},
            },
            GameState::GAMECOMPLETE => {
                // {Func_fixed, gamecompleterenderfixed},
                // {Func_delta, gamecompleterender},
                // {Func_input, gamecompleteinput},
                // {Func_fixed, gamecompletelogic},
            },
            GameState::GAMECOMPLETE2 => {
                // {Func_delta, gamecompleterender2},
                // {Func_input, gamecompleteinput2},
                // {Func_fixed, gamecompletelogic2},
            },
            // #if !defined(NO_CUSTOM_LEVELS) && !defined(NO_EDITOR)
            GameState::EDITORMODE => {
                // {Func_fixed, flipmodeoff},
                // {Func_input, editorinput},
                // {Func_fixed, editorrenderfixed},
                // {Func_delta, editorrender},
                // {Func_fixed, editorlogic},
            },
            // #endif
            // TODO @sx
            GameState::CLICKTOSTART => {
                // help.updateglow();
            },
            // TODO @sx
            GameState::FOCUSMODE => {
            },
        };
    }

    gameScreen.do_screen_render(rr, game);
    LoopCode::LoopStop
}

fn get_gamestate_funcs(
    gamestate: GameState, num_implfuncs: i32,
    game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump
) -> Vec<&'static dyn Fn(&mut game::Game, &mut screen::Screen, &mut key_poll::KeyPoll, &mut input::Input, &mut EventPump)> {
    println!("get_gamestate_funcs");

    game.gamestate = GameState::TITLEMODE; // TODO @sx

    if key.isActive {
        match game.gamestate {
            // GameState::PRELOADER => {
            //     vec![
            //         // {Func_input, preloaderinput},
            //         // {Func_fixed, preloaderrenderfixed},
            //         // {Func_delta, preloaderrender},
            //         gameScreen.render.preloaderrender,
            //     ]
            // },
            GameState::TITLEMODE => {
                return vec![
                    // // {Func_input, titleinput},
                    // input.titleinput,
                    // // {Func_fixed, titlerenderfixed},
                    // gameScreen.renderfixed.title_render_fixed,
                    // // {Func_delta, titlerender},
                    // gameScreen.render.title_render,
                    // // {Func_fixed, titlelogic},
                    // &logic::title_logic,
                ]
            },
            _ => return vec![],
        }
    }
    vec![]
}

fn loop_end(game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes) -> LoopCode {
    println!("loop_end");

    // We did editorinput, now it's safe to turn this off
    key.linealreadyemptykludge = false;

    // TODO @sx
    // Mute button
    // if key.isDown(KEYBOARD_m) && game.mutebutton<=0 && !key.textentry() {
    //     game.mutebutton = 8;
    //     if game.muted {
    //         game.muted = false;
    //     } else {
    //         game.muted = true;
    //     }
    // }
    // if game.mutebutton > 0 {
    //     game.mutebutton -= 1;
    // }

    // if key.isDown(KEYBOARD_n) && game.musicmutebutton <= 0 && !key.textentry() {
    //     game.musicmutebutton = 8;
    //     game.musicmuted = !game.musicmuted;
    // }
    // if game.musicmutebutton > 0 {
    //     game.musicmutebutton -= 1;
    // }

    // if game.muted {
    //     Mix_VolumeMusic(0);
    //     Mix_Volume(-1,0);
    // } else {
    //     Mix_Volume(-1,MIX_MAX_VOLUME * music.user_sound_volume / USER_VOLUME_MAX);

    //     if game.musicmuted {
    //         Mix_VolumeMusic(0);
    //     } else {
    //         Mix_VolumeMusic(music.musicVolume * music.user_music_volume / USER_VOLUME_MAX);
    //     }
    // }

    // if key.resetWindow {
    //     key.resetWindow = false;
    //     gameScreen.ResizeScreen(-1, -1);
    // }

    LoopCode::LoopContinue
}

// static enum IndexCode increment_gamestate_func_index(void)
fn increment_gamestate_func_index() -> IndexCode {
    println!("increment_gamestate_func_index");

    // gamestate_func_index += 1;

    // if gamestate_func_index == num_gamestate_funcs {
    //     /* Reached the end of current gamestate order.
    //      * Re-fetch for new order if gamestate changed.
    //      */
    //     gamestate_funcs = get_gamestate_funcs(
    //         game.gamestate,
    //         &num_gamestate_funcs
    //     );

    //     /* Also run callbacks that were deferred to end of func sequence. */
    //     DEFER_execute_callbacks();

    //     gamestate_func_index = 0;

    //     IndexCode::IndexEnd
    // }

    IndexCode::IndexNone
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LoopCode {
    LoopContinue,
    LoopStop
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum IndexCode {
    IndexNone,
    IndexEnd
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FuncType {
    FuncNull,
    FuncFixed,
    FuncInput,
    FuncDelta
}
struct ImplFunc {
    r#type: FuncType,
    func: Vec<&'static dyn Fn(&mut game::Game, &mut screen::Screen, &mut key_poll::KeyPoll, &mut input::Input, &mut EventPump) -> LoopCode>,
}

// impl ImplFunc {
//     fn new() -> ImplFunc {
//         ImplFunc {

//         }
//     }
// }
