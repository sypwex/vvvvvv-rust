extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod screen;
mod rustutil;

fn main() {
    init_arguments();

    // if !FILESYSTEM_init(argv[0], baseDir, assetsPath) {
    //     puts("Unable to initialize filesystem!");
    //     return 1;
    // }

    let sdl_context = sdl2::init().unwrap();
    // SDL_Init(
    //     SDL_INIT_VIDEO |
    //     SDL_INIT_AUDIO |
    //     SDL_INIT_JOYSTICK |
    //     SDL_INIT_GAMECONTROLLER
    // );
    // if SDL_IsTextInputActive() == SDL_TRUE {
    //     SDL_StopTextInput();

    // NETWORK_init();
    print_logo();

    // Load Ini, reloadresources loads music too...

    // graphics.init();
    // game.init();
    // graphics.reloadresources();
    fill_initial_values();

    // Prioritize unlock.vvv first (2.2 and below), but settings have been migrated to settings.vvv (2.3 and up)
    // game.loadstats(&width, &height, &vsync);
    // game.loadsettings(&width, &height, &vsync);

    let mut gameScreen = screen::Screen::new(&sdl_context);
    // gameScreen.init(&sdl_context);

    // gameScreen.init(
    //     width,
    //     height,
    //     game.fullscreen,
    //     vsync,
    //     game.stretchMode,
    //     game.useLinearFilter,
    //     game.fullScreenEffect_badSignal
    // );

    init_graphics();
    // obj.init();
    no_custom_levels();
    main_loop(&sdl_context, &mut gameScreen);
    free_assets();
}

fn init_arguments() {
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

fn print_logo() {
    println!("\t\t");
    println!("\t\t");
    println!("\t\t       VVVVVV");
    println!("\t\t rust implementation");
    println!("\t\t");
    println!("\t\t");
    println!("\t\t  8888888888888888  ");
    println!("\t\t88888888888888888888");
    println!("\t\t888888    8888    88");
    println!("\t\t888888    8888    88");
    println!("\t\t88888888888888888888");
    println!("\t\t88888888888888888888");
    println!("\t\t888888            88");
    println!("\t\t88888888        8888");
    println!("\t\t  8888888888888888  ");
    println!("\t\t      88888888      ");
    println!("\t\t  8888888888888888  ");
    println!("\t\t88888888888888888888");
    println!("\t\t88888888888888888888");
    println!("\t\t88888888888888888888");
    println!("\t\t8888  88888888  8888");
    println!("\t\t8888  88888888  8888");
    println!("\t\t    888888888888    ");
    println!("\t\t    8888    8888    ");
    println!("\t\t  888888    888888  ");
    println!("\t\t  888888    888888  ");
    println!("\t\t  888888    888888  ");
    println!("\t\t");
    println!("\t\t");
}

fn fill_initial_values() {
    // // previously were here but now in game.rs
    // game.gamestate = PRELOADER;
    // game.menustart = false;
    // game.mainmenu = 0;

    // TODO:
    // map.ypos = (700-29) * 8;
    // graphics.towerbg.bypos = map.ypos / 2;
    // graphics.titlebg.bypos = map.ypos / 2;

    // TODO:
    // // Moved screensetting init here from main menu V2.1
    // int width = 320;
    // int height = 240;
    // bool vsync = false;
}

fn init_graphics() {
    // graphics.screenbuffer = &gameScreen;

    // const SDL_PixelFormat* fmt = gameScreen.GetFormat();
    // #define CREATE_SURFACE(w, h) \
    //     SDL_CreateRGBSurface( \
    //         SDL_SWSURFACE, \
    //         w, h, \
    //         fmt->BitsPerPixel, \
    //         fmt->Rmask, fmt->Gmask, fmt->Bmask, fmt->Amask \
    //     )
    // graphics.backBuffer = CREATE_SURFACE(320, 240);
    // SDL_SetSurfaceBlendMode(graphics.backBuffer, SDL_BLENDMODE_NONE);

    // graphics.footerbuffer = CREATE_SURFACE(320, 10);
    // SDL_SetSurfaceBlendMode(graphics.footerbuffer, SDL_BLENDMODE_BLEND);
    // SDL_SetSurfaceAlphaMod(graphics.footerbuffer, 127);
    // FillRect(graphics.footerbuffer, SDL_MapRGB(fmt, 0, 0, 0));

    // graphics.ghostbuffer = CREATE_SURFACE(320, 240);
    // SDL_SetSurfaceBlendMode(graphics.ghostbuffer, SDL_BLENDMODE_BLEND);
    // SDL_SetSurfaceAlphaMod(graphics.ghostbuffer, 127);

    // graphics.Makebfont();

    // graphics.foregroundBuffer =  CREATE_SURFACE(320, 240);
    // SDL_SetSurfaceBlendMode(graphics.foregroundBuffer, SDL_BLENDMODE_BLEND);

    // graphics.menubuffer = CREATE_SURFACE(320, 240);
    // SDL_SetSurfaceBlendMode(graphics.menubuffer, SDL_BLENDMODE_NONE);

    // graphics.warpbuffer = CREATE_SURFACE(320 + 16, 240 + 16);
    // SDL_SetSurfaceBlendMode(graphics.warpbuffer, SDL_BLENDMODE_NONE);

    // graphics.warpbuffer_lerp = CREATE_SURFACE(320 + 16, 240 + 16);
    // SDL_SetSurfaceBlendMode(graphics.warpbuffer_lerp, SDL_BLENDMODE_NONE);

    // graphics.towerbg.buffer =  CREATE_SURFACE(320 + 16, 240 + 16);
    // SDL_SetSurfaceBlendMode(graphics.towerbg.buffer, SDL_BLENDMODE_NONE);

    // graphics.towerbg.buffer_lerp = CREATE_SURFACE(320 + 16, 240 + 16);
    // SDL_SetSurfaceBlendMode(graphics.towerbg.buffer_lerp, SDL_BLENDMODE_NONE);

    // graphics.titlebg.buffer = CREATE_SURFACE(320 + 16, 240 + 16);
    // SDL_SetSurfaceBlendMode(graphics.titlebg.buffer, SDL_BLENDMODE_NONE);

    // graphics.titlebg.buffer_lerp = CREATE_SURFACE(320 + 16, 240 + 16);
    // SDL_SetSurfaceBlendMode(graphics.titlebg.buffer_lerp, SDL_BLENDMODE_NONE);

    // graphics.tempBuffer = CREATE_SURFACE(320, 240);
    // SDL_SetSurfaceBlendMode(graphics.tempBuffer, SDL_BLENDMODE_NONE);
}

fn no_custom_levels() {
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

fn main_loop(sdl_context: &sdl2::Sdl, gameScreen: &mut screen::Screen) {
    let mut time_: u32 = 0;
    let mut timePrev: u32 = 0;
    // let accumulator: u32 = 0;
    let mut f_time: u32 = 0;
    let mut f_timePrev: u32 = 0;

    gameScreen.init_canvas();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let timer = match sdl_context.timer() {
        Ok(v) => v,
        Err(s) => panic!(s),
    };
    // key.isActive = true;
    'running: loop {
        f_time = timer.ticks();
        let f_timetaken: u32 = f_time - f_timePrev;
        if !gameScreen.getOver30mode() && f_timetaken < 34 {
            let f_delay: u32 = 34 - f_timetaken;
            ::std::thread::sleep(Duration::new(0, f_delay));
            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            f_time = timer.ticks();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Event::KeyDown { keycode: Some(Keycode::F), .. } => { toggleFullscreen() },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        gameScreen.deltaloop();
        gameScreen.present_canvas();

        f_timePrev = f_time;
        timePrev = time_;
        time_ = timer.ticks();
    }
}

fn free_assets() {
    // game.savestats();
    // NETWORK_shutdown();
    // SDL_Quit();
    // FILESYSTEM_deinit();
}
