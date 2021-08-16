// TODO: @sx: disable some rustc/clippy checks while this branch is
// mirroring original c++ codebase. Refactor on port completion.
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// fix in first place
#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Duration;

#[macro_use]
extern crate log;

extern crate sdl2;
use sdl2::EventPump;
mod sdl2u;

mod entity;
mod filesystem;
mod game;
use game::GameState;
#[macro_use]
mod helpers;
mod input;
mod key_poll;
mod logic;
mod map;
mod maths;
mod music;
mod rustutil;
mod scenes;
use scenes::{Fns, FuncType, IndexCode, InputTrait, RenderFixedTrait, RenderTrait, RenderResult, preloader::Preloader};
mod screen;
use screen::render::graphics;
mod script;
mod utility_class;
use utility_class::UtilityClass;
mod xml;

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

// static enum LoopCode (*const meta_funcs[])(void) = {
// static const struct ImplFunc** active_funcs = NULL;
// static int* num_active_funcs = NULL;
// static int* active_func_index = NULL;
// static enum IndexCode (*increment_func_index)(void) = NULL;
// static const struct ImplFunc* gamestate_funcs = NULL;
// static int num_gamestate_funcs = 0;
// static int gamestate_func_index = -1;
// static int meta_func_index = 0;

// static void runscript(void)
// static void teleportermodeinput(void)
fn teleportermodeinput(game: &mut game::Game, script: &mut script::ScriptClass, input: &mut input::Input, graphics: &mut graphics::Graphics, map: &mut map::Map, music: &mut music::Music, help: &mut utility_class::UtilityClass, key: &mut key_poll::KeyPoll, obj: &mut entity::EntityClass, fs: &mut filesystem::FileSystem) -> Result<Option<RenderResult>, i32> {
    if game.useteleporter {
        input.teleporterinput(game, graphics, map, key, obj)
    } else {
        script.run(game, obj, map, graphics, help, music, key, fs);
        input.gameinput(game, graphics, map, music, key, obj, script, help)
    }
}
// static void flipmodeoff(void)
// static const inline struct ImplFunc* get_gamestate_funcs(const int gamestate, int* num_implfuncs)
// static enum IndexCode increment_gamestate_func_index(void)
// static void unfocused_run(void);
// static enum IndexCode increment_unfocused_func_index(void)

struct Main {
    sdl_context: sdl2::Sdl,
    input: input::Input,
    scenes: scenes::Scenes,
    preloader_scene: Preloader,

    // #if !defined(NO_CUSTOM_LEVELS)
    // edentity: Vec<edentities>,
    // ed: editorclass;
    // #endif

    fs: filesystem::FileSystem,
    help: utility_class::UtilityClass,
    // graphics: graphics::Graphics,
    music: music::Music,
    game: game::Game,
    key: key_poll::KeyPoll,
    map: map::Map,
    obj: entity::EntityClass,
    gameScreen: screen::Screen,
    script: script::ScriptClass,

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

    exit_code: i32,
}

impl Main {
    fn new() -> Main {
        let sdl_context = Main::create_sdl_context();

        let baseDir = None;
        let assetsPath = None;
        let argvZero = std::env::args().nth(0).unwrap();
        let mut fs = match filesystem::FileSystem::new(argvZero, baseDir, assetsPath) {
            Ok(fs) => fs,
            Err(s) => panic!("Unable to initialize filesystem!: {}", s),
        };
        // NETWORK_init();

        let mut gameScreen = screen::Screen::new(&sdl_context);
        let screen_params = gameScreen.get_screen_params();
        let screen_settings = gameScreen.screen_settings;
        let mut map = map::Map::new(&mut gameScreen.render.graphics);
        let mut music = music::Music::new();
        let mut game = game::Game::new(&mut gameScreen.render.graphics, &mut music, screen_params, &mut map, &mut fs, screen_settings);

        //Set up screen
        //Load Ini
        // gameScreen.render.graphics.init(); // @sx: done at Graphics::new()
        //This loads music too...
        gameScreen.render.graphics.reload_resources(&mut music, &mut fs);

        // TODO: @sx load scene from argument
        game.gamestate = GameState::PRELOADER;
        game.menustart = false;

        //Initialize title screen to cyan
        // graphics.titlebg.colstate = 10; // @sx: done at struct init
        // map.nexttowercolour(); // @sx: done at map init

        map.ypos = (700-29) * 8;
        gameScreen.render.graphics.buffers.towerbg.bypos = map.ypos / 2;
        gameScreen.render.graphics.buffers.titlebg.bypos = map.ypos / 2;

        // Prioritize unlock.vvv first (2.2 and below),
        // but settings have been migrated to settings.vvv (2.3 and up)
        game.loadstats(screen_settings, &mut fs, &mut music);
        game.loadsettings(screen_settings);
        gameScreen.init(screen_settings);

        // graphics.create_buffers(gameScreen.GetFormat());

        // @sx: for skipfakeload see Game::init()
        // if (game.skipfakeload)
        //     game.gamestate = TITLEMODE;
        // if (game.slowdown == 0) game.slowdown = 30;
        // @sx: for unlockAchievement stuff see Game::init()

        // obj.init();

        Main {
            sdl_context,
            input: input::Input::new(),
            scenes: scenes::Scenes::new(),
            preloader_scene: scenes::preloader::Preloader::new(),

            // edentity: Vec<edentities>,
            // ed: editorclass;

            fs,
            help: utility_class::UtilityClass::new(),
            // graphics: graphics::Graphics::new(),
            music,
            game,
            key: key_poll::KeyPoll::new(),
            map,
            obj: entity::EntityClass::new(),
            gameScreen,
            script: script::ScriptClass::new(),

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

            exit_code: -1,
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

    fn main_loop(&mut self) -> i32 {
        let mut time_: u32 = 0;
        let mut timePrev: u32 = 0;
        let mut f_time: u32 = 0;
        let mut f_timePrev: u32 = 0;

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
        self.scenes.update_gamestate_funcs(self.game.gamestate);
        // loop_assign_active_funcs();

        loop {
            // let now = std::time::Instant::now();

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

            // crate::rustutil::dump_surface(&self.gameScreen.render.graphics.buffers.backBuffer, "buffer", "");
            // println!("main loop iter done in {:?}ms", now.elapsed().as_millis());

            if self.exit_code != -1 {
                return self.exit_code
            }
        }
    }

    // static void inline deltaloop(void);
    fn deltaloop(&mut self, rawdeltatime: u32, key: &mut key_poll::KeyPoll, event_pump: &mut EventPump) {
        // timestep limit to 30
        self.accumulator = self.accumulator + rawdeltatime as f32;
        let timesteplimit = self.game.get_timestep() as f32;

        while self.accumulator >= timesteplimit {
            // enum IndexCode index_code = increment_func_index();
            let index_code = self.scenes.increment_gamestate_func_index(&self.game);

            if index_code == IndexCode::IndexEnd {
                // loop_assign_active_funcs();
            }

            self.accumulator = sdl2u::sys_fmodf(self.accumulator, timesteplimit);

            /* We are done rendering. */
            self.gameScreen.render.graphics.renderfixedpost(&mut self.game);

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
        // const struct ImplFunc* implfunc = &(*active_funcs)[*active_func_index];
        let implfunc = self.scenes.get_current_gamestate_func();

        if implfunc.fntype == FuncType::FuncDelta {
            match invoke_scene_function(&mut self.preloader_scene, implfunc.fnname, &mut self.music, &mut self.map, &mut self.game, &mut self.gameScreen, key, &mut self.input, event_pump, &mut self.help, &mut self.script, &mut self.obj, &mut self.fs) {
                Ok(Some(rr)) => self.gameScreen.do_screen_render(rr, &mut self.game),
                Err(exit_code) => {
                    self.exit_code = exit_code;
                    return;
                },
                _ => (),
            }
            self.gameScreen.FlipScreen();
        }
        // }

        // println!("delta loop finish");
    }

    // static void inline fixedloop(void)
    fn fixedloop(&mut self, key: &mut key_poll::KeyPoll, event_pump: &mut EventPump) {
        let meta_funcs: Vec<&dyn Fn(&mut music::Music, &mut map::Map, &mut game::Game, &mut screen::Screen, &mut key_poll::KeyPoll, &mut input::Input, &mut EventPump, &mut scenes::Scenes, &mut scenes::preloader::Preloader, &mut utility_class::UtilityClass, &mut script::ScriptClass, &mut entity::EntityClass, &mut filesystem::FileSystem) -> LoopCode> = vec![
            &loop_begin,
            &loop_assign_active_funcs,
            &loop_run_active_funcs,
            &loop_end,
        ];

        'fixedloop: loop {
            for meta_func in &meta_funcs {
                match meta_func(&mut self.music, &mut self.map, &mut self.game, &mut self.gameScreen, key, &mut self.input, event_pump, &mut self.scenes, &mut self.preloader_scene, &mut self.help, &mut self.script, &mut self.obj, &mut self.fs) {
                    LoopCode::LoopContinue => (),
                    LoopCode::LoopStop => break 'fixedloop,
                    LoopCode::BreakTheMain(code) => {
                        self.exit_code = code;
                        break 'fixedloop;
                    }
                }
            }
        }
        // println!("fixed loop finish");
    }

    // void VVV_exit(const int exit_code)
    pub fn VVV_exit(&mut self, exit_code: i32) {
        self.exit_code = exit_code;
    }
}

impl Drop for Main {
    // static void cleanup(void);
    fn drop(&mut self) {
        eprintln!("Drop The Main!");
        /* Order matters! */
        self.game.savestatsandsettings(self.gameScreen.screen_settings, &mut self.fs, &mut self.music);
        // gameScreen.destroy();
        // graphics.grphx.destroy();
        // graphics.destroy_buffers();
        // graphics.destroy();
        // music.destroy();
        // NETWORK_shutdown();
        // SDL_Quit(); // @sx: impl drop
        // FILESYSTEM_deinit(); // @sx: impl drop
    }
}

fn main() {
    env_logger::init();
    let mut m = Main::new();
    m.init_arguments();
    print_logo();
    m.no_custom_levels();
    let code = m.main_loop();

    std::process::exit(code)
}

// static enum LoopCode loop_begin(void);
fn loop_begin(music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes, preloader: &mut scenes::preloader::Preloader, help: &mut utility_class::UtilityClass, script: &mut script::ScriptClass, obj: &mut entity::EntityClass, fs: &mut filesystem::FileSystem) -> LoopCode {
    // println!("loop_begin");
    if game.inputdelay {
        if let Err(exit_code) = key.Poll(event_pump, game, music, gameScreen) {
            return LoopCode::BreakTheMain(exit_code)
        }
    }

    // Update network per frame.
    // TODO @sx
    // NETWORK_update();

    LoopCode::LoopContinue
}

// static enum LoopCode loop_assign_active_funcs(void)
fn loop_assign_active_funcs(music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes, preloader: &mut scenes::preloader::Preloader, help: &mut utility_class::UtilityClass, script: &mut script::ScriptClass, obj: &mut entity::EntityClass, fs: &mut filesystem::FileSystem) -> LoopCode {
    // println!("loop_assign_active_funcs");
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
fn loop_run_active_funcs(music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes, preloader: &mut scenes::preloader::Preloader, help: &mut utility_class::UtilityClass, script: &mut script::ScriptClass, obj: &mut entity::EntityClass, fs: &mut filesystem::FileSystem) -> LoopCode {
    // println!("loop_run_active_funcs");

    // while (*active_funcs)[*active_func_index].type != Func_delta {
    // const struct ImplFunc* implfunc = &(*active_funcs)[*active_func_index];
    let mut implfunc = scenes.get_current_gamestate_func();
    while implfunc.fntype != FuncType::FuncDelta {
        // println!("loop_run_active_funcs: {:?} received", implfunc.fnname);

        if implfunc.fntype == FuncType::FuncInput && !game.inputdelay {
            if let Err(exit_code) = key.Poll(event_pump, game, music, gameScreen) {
                return LoopCode::BreakTheMain(exit_code)
            }
        }

        match invoke_scene_function(preloader, implfunc.fnname, music, map, game, gameScreen, key, input, event_pump, help, script, obj, fs) {
            Ok(Some(rr)) => gameScreen.do_screen_render(rr, game),
            Err(exit_code) => return LoopCode::BreakTheMain(exit_code),
            _ => (),
        };

        let index_code = scenes.increment_gamestate_func_index(game);

        if index_code == IndexCode::IndexEnd {
            return LoopCode::LoopContinue;
        }

        implfunc = scenes.get_current_gamestate_func();
    }

    /* About to switch over to rendering... but call this first. */
    let badSignalEffect = gameScreen.badSignalEffect;
    gameScreen.render.graphics.renderfixedpre(game, badSignalEffect);

    LoopCode::LoopStop
}

// static void focused_begin(void);
fn focused_begin() -> Result<Option<RenderResult>, i32> {
    Ok(None)
}

// static void focused_end(void);
fn focused_end(music: &mut music::Music, game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map) -> Result<Option<RenderResult>, i32> {
    game.gameclock();
    music.processmusic(map, game);
    graphics.processfade();

    Ok(None)
}

// static enum LoopCode loop_end(void);
fn loop_end(music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, scenes: &mut scenes::Scenes, preloader: &mut scenes::preloader::Preloader, help: &mut utility_class::UtilityClass, script: &mut script::ScriptClass, obj: &mut entity::EntityClass, fs: &mut filesystem::FileSystem) -> LoopCode {
    // println!("loop_end");

    // We did editorinput, now it's safe to turn this off
    key.linealreadyemptykludge = false;

    // Mute button
    if key.isDownKeycode(sdl2::keyboard::Keycode::M) && game.mutebutton <= 0 && !key.textentry() {
        game.mutebutton = 8;
        if game.muted {
            game.muted = false;
        } else {
            game.muted = true;
        }
    }
    if game.mutebutton > 0 {
        game.mutebutton -= 1;
    }

    if key.isDownKeycode(sdl2::keyboard::Keycode::N) && game.musicmutebutton <= 0 && !key.textentry() {
        game.musicmutebutton = 8;
        game.musicmuted = !game.musicmuted;
    }
    if game.musicmutebutton > 0 {
        game.musicmutebutton -= 1;
    }

    unsafe {
        if game.muted {
            sdl2_sys::mixer::Mix_VolumeMusic(0);
            sdl2_sys::mixer::Mix_Volume(-1, 0);
        } else {
            sdl2_sys::mixer::Mix_Volume(-1, sdl2_sys::mixer::MIX_MAX_VOLUME as i32 * *music.user_sound_volume / music::USER_VOLUME_MAX);

            if game.musicmuted {
                sdl2_sys::mixer::Mix_VolumeMusic(0);
            } else {
                sdl2_sys::mixer::Mix_VolumeMusic(music.musicVolume * *music.user_music_volume / music::USER_VOLUME_MAX);
            }
        }
    }

    if key.resetWindow {
        key.resetWindow = false;
        gameScreen.ResizeScreen(-1, -1);
    }

    LoopCode::LoopContinue
}

fn invoke_scene_function(preloader: &mut Preloader, fnname: Fns, music: &mut music::Music, map: &mut map::Map, game: &mut game::Game, gameScreen: &mut screen::Screen, key: &mut key_poll::KeyPoll, input: &mut input::Input, event_pump: &mut EventPump, help: &mut UtilityClass, script: &mut script::ScriptClass, obj: &mut entity::EntityClass, fs: &mut filesystem::FileSystem) -> Result<Option<RenderResult>, i32> {
    // println!("current scene function: {:?}", fnname);

    let screen_params = gameScreen.get_screen_params();
    let screen_settings = gameScreen.screen_settings;

    match fnname {
        Fns::focused_begin => focused_begin(),
        Fns::focused_end => focused_end(music, game, &mut gameScreen.render.graphics, map),

        // GameState::PRELOADER
        Fns::preloaderinput => preloader.input(game, key),
        Fns::preloaderrenderfixed => preloader.render_fixed(game),
        Fns::preloaderrender => preloader.render(&mut gameScreen.render.graphics),

        // GameState::TITLEMODE
        Fns::titleinput => input.titleinput(music, map, game, gameScreen, key, script, obj, help, fs),
        Fns::titlerenderfixed => gameScreen.renderfixed.titlerenderfixed(map, game, &mut gameScreen.render.graphics),
        Fns::titlerender => gameScreen.render.titlerender(game, music, map, help, key, screen_params),
        Fns::titlelogic => logic::titlelogic(map, music, game, &mut gameScreen.renderfixed, &mut gameScreen.render.graphics, screen_params, screen_settings, fs),

        // GameState::GAMEMODE
        Fns::runscript => script.run(game, obj, map, &mut gameScreen.render.graphics, help, music, key, fs),
        Fns::gamerenderfixed => gameScreen.renderfixed.gamerenderfixed(obj, game, map, &mut gameScreen.render.graphics, script, help),
        Fns::gamerender => gameScreen.render.gamerender(game, map, help, obj),
        Fns::gameinput => input.gameinput(game, &mut gameScreen.render.graphics, map, music, key, obj, script, help),
        Fns::gamelogic => logic::gamelogic(game, &mut gameScreen.render.graphics, map, music, obj, help, script, screen_params, fs, screen_settings),

        // GameState::MAPMODE
        Fns::maprenderfixed => gameScreen.renderfixed.maprenderfixed(&mut gameScreen.render.graphics, game, map, script),
        Fns::maprender => gameScreen.render.maprender(map, help, game, script, obj),
        Fns::mapinput => input.mapinput(game, &mut gameScreen.render.graphics, obj, script, music, map, help, key, fs, screen_params, screen_settings),
        Fns::maplogic => logic::maplogic(help),

        // GameState::TELEPORTERMODE
        Fns::teleporterrender => gameScreen.render.teleporterrender(game, map, help, obj),
        Fns::teleportermodeinput => teleportermodeinput(game, script, input, &mut gameScreen.render.graphics, map, music, help, key, obj, fs),

        // GameState::GAMECOMPLETE
        // GameState::GAMECOMPLETE2
        // GameState::EDITORMODE
        // GameState::CLICKTOSTART
        // GameState::FOCUSMODE
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LoopCode {
    LoopContinue,
    LoopStop,
    BreakTheMain(i32),
}
