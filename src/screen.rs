extern crate sdl2;
use sdl2::pixels::Color;
mod render;
mod game;

pub struct Screen {
    render: Box<render::Render>,
    game: Box<game::Game>,
    // window: Box<sdl2::video::Window>,
    canvas: Box<sdl2::render::Canvas<sdl2::video::Window>>,
    m_screen: Box<sdl2::surface::Surface<'static>>,
}

impl Screen {
    pub fn new (sdl_context: &sdl2::Sdl) -> Screen {
        // m_window = NULL;
        // m_renderer = NULL;
        // m_screenTexture = NULL;
        // m_screen = NULL;
        // isWindowed = !fullscreen;
        // stretchMode = stretch;
        // isFiltered = linearFilter;
        // vsync = useVsync;
        // filterSubrect.x = 1;
        // filterSubrect.y = 1;
        // filterSubrect.w = 318;
        // filterSubrect.h = 238;

        // SDL_SetHintWithPriority(
        //     SDL_HINT_RENDER_SCALE_QUALITY,
        //     isFiltered ? "linear" : "nearest",
        //     SDL_HINT_OVERRIDE
        // );
        // SDL_SetHintWithPriority(
        //     SDL_HINT_RENDER_VSYNC,
        //     vsync ? "1" : "0",
        //     SDL_HINT_OVERRIDE
        // );

        // // Uncomment this next line when you need to debug -flibit
        // // SDL_SetHintWithPriority(SDL_HINT_RENDER_DRIVER, "software", SDL_HINT_OVERRIDE);
        // // FIXME: m_renderer is also created in Graphics::processVsync()!
        // SDL_CreateWindowAndRenderer(
        //     640,
        //     480,
        //     SDL_WINDOW_HIDDEN | SDL_WINDOW_RESIZABLE,
        //     &m_window,
        //     &m_renderer
        // );
        // SDL_SetWindowTitle(m_window, "VVVVVV");

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("VVVVVV on Rust", 640, 480)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        // LoadIcon();

        // // FIXME: This surface should be the actual backbuffer! -flibit
        // m_screen = SDL_CreateRGBSurface(
        //     0,
        //     320,
        //     240,
        //     32,
        //     0x00FF0000,
        //     0x0000FF00,
        //     0x000000FF,
        //     0xFF000000
        // );

        let m_screen = sdl2::surface::Surface::new(320, 240, sdl2::pixels::PixelFormatEnum::RGBX8888).unwrap();

        // // ALSO FIXME: This SDL_CreateTexture() is duplicated in Graphics::processVsync()!
        // m_screenTexture = SDL_CreateTexture(
        //     m_renderer,
        //     SDL_PIXELFORMAT_ARGB8888,
        //     SDL_TEXTUREACCESS_STREAMING,
        //     320,
        //     240
        // );

        // badSignalEffect = badSignal;

        // ResizeScreen(windowWidth, windowHeight);

        let mut game = game::Game::new();
        game.init_game();

        Screen {
            render: Box::new(render::Render::new()),
            game: Box::new(game),
            // window: Box::new(window),
            canvas: Box::new(canvas),
            m_screen: Box::new(m_screen),
        }
    }

    pub fn init (
        // int windowWidth,
        // int windowHeight,
        // bool fullscreen,
        // bool useVsync,
        // int stretch,
        // bool linearFilter,
        // bool badSignal
    ) {

    }

    pub fn getOver30mode (&self) -> bool {
        self.game.over30mode
    }

    pub fn init_canvas (&mut self) {
        self.canvas.set_draw_color(Color::RGB(128, 128, 128));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn present_canvas (&mut self) {
        self.canvas.present();
    }

    pub fn deltaloop(&mut self) {
        // //timestep limit to 30
        // const float rawdeltatime = static_cast<float>(time_ - timePrev);
        // accumulator += rawdeltatime;

        let timesteplimit: u32 = match self.game.gamestate {
            game::GameState::EDITORMODE => 24,
            game::GameState::GAMEMODE => self.game.gameframerate,
            _ => 34,
        };

        // while (accumulator >= timesteplimit) {
        //     accumulator = SDL_fmodf(accumulator, timesteplimit);
            self.fixedloop();
        // }
        // const float alpha = game.over30mode ? static_cast<float>(accumulator) / timesteplimit : 1.0f;
        // graphics.alpha = alpha;

        // if (key.isActive) {
            match self.game.gamestate {
                game::GameState::PRELOADER => self.render.preloaderrender(),
        // #if !defined(NO_CUSTOM_LEVELS) && !defined(NO_EDITOR)
        //         game::GameState::EDITORMODE: {
        //             graphics.flipmode = false;
        //             editorrender();
        //         }
        // #endif
                game::GameState::TITLEMODE => {
                    self.render.title_render();
                    self.update_screen()
                },
                game::GameState::GAMEMODE => self.render.gamerender(),
                game::GameState::MAPMODE => self.render.maprender(),
                game::GameState::TELEPORTERMODE => self.render.teleporterrender(),
                game::GameState::GAMECOMPLETE => self.render.gamecompleterender(),
                game::GameState::GAMECOMPLETE2 => self.render.gamecompleterender2(),
                // game::GameState::CLICKTOSTART => help.updateglow(), // TODO:
                _ => (),
            }
            // gameScreen.FlipScreen(); TODO:
        // }
    }

    pub fn fixedloop(&self) {
        //     // Update network per frame.
        //     NETWORK_update();

        //     key.Poll();
        //     if(key.toggleFullscreen)
        //     {
        //         gameScreen.toggleFullScreen();
        //         game.fullscreen = !game.fullscreen;
        //         key.toggleFullscreen = false;

        //         key.keymap.clear(); //we lost the input due to a new window.
        //         if (game.glitchrunnermode)
        //         {
        //             game.press_left = false;
        //             game.press_right = false;
        //             game.press_action = true;
        //             game.press_map = false;
        //         }
        //     }

        //     if(!key.isActive)
        //     {
        //         Mix_Pause(-1);
        //         Mix_PauseMusic();

        //         if (!game.blackout)
        //         {
        //             FillRect(graphics.backBuffer, 0x00000000);
        //             graphics.bprint(5, 110, "Game paused", 196 - help.glow, 255 - help.glow, 196 - help.glow, true);
        //             graphics.bprint(5, 120, "[click to resume]", 196 - help.glow, 255 - help.glow, 196 - help.glow, true);
        //             graphics.bprint(5, 220, "Press M to mute in game", 164 - help.glow, 196 - help.glow, 164 - help.glow, true);
        //             graphics.bprint(5, 230, "Press N to mute music only", 164 - help.glow, 196 - help.glow, 164 - help.glow, true);
        //         }
        //         graphics.render();
        //         gameScreen.FlipScreen();
        //         //We are minimised, so lets put a bit of a delay to save CPU
        //         SDL_Delay(100);
        //     }
        //     else
        //     {
        //         Mix_Resume(-1);
        //         Mix_ResumeMusic();
        //         game.gametimer++;
        //         graphics.cutscenebarstimer();

        //         switch(game.gamestate)
        //         {
        //         case PRELOADER:
        //             preloaderlogic();
        //             break;
        // #if !defined(NO_CUSTOM_LEVELS) && !defined(NO_EDITOR)
        //         case EDITORMODE:
        //             //Input
        //             editorinput();
        //             ////Logic
        //             editorlogic();
        //             break;
        // #endif
        //         case TITLEMODE:
        //             //Input
        //             titleinput();
        //             ////Logic
        //             titlelogic();
        //             break;
        //         case GAMEMODE:
        //             // WARNING: If updating this code, don't forget to update Map.cpp mapclass::twoframedelayfix()

        //             // Ugh, I hate this kludge variable but it's the only way to do it
        //             if (script.dontrunnextframe)
        //             {
        //                 script.dontrunnextframe = false;
        //             }
        //             else if (script.running)
        //             {
        //                 script.run();
        //             }

        //             //Update old lerp positions of entities - has to be done BEFORE gameinput!
        //             for (size_t i = 0; i < obj.entities.size(); i++)
        //             {
        //                 obj.entities[i].lerpoldxp = obj.entities[i].xp;
        //                 obj.entities[i].lerpoldyp = obj.entities[i].yp;
        //             }

        //             gameinput();
        //             gamelogic();


        //             break;
        //         case MAPMODE:
        //             mapinput();
        //             maplogic();
        //             break;
        //         case TELEPORTERMODE:
        //             if(game.useteleporter)
        //             {
        //                 teleporterinput();
        //             }
        //             else
        //             {
        //                 if (script.running)
        //                 {
        //                     script.run();
        //                 }
        //                 gameinput();
        //             }
        //             maplogic();
        //             break;
        //         case GAMECOMPLETE:
        //             //Input
        //             gamecompleteinput();
        //             //Logic
        //             gamecompletelogic();
        //             break;
        //         case GAMECOMPLETE2:
        //             //Input
        //             gamecompleteinput2();
        //             //Logic
        //             gamecompletelogic2();
        //             break;
        //         case CLICKTOSTART:
        //             break;
        //         default:

        //             break;

        //         }

        //     }

        //     //Screen effects timers
        //     if (key.isActive && game.flashlight > 0)
        //     {
        //         game.flashlight--;
        //     }
        //     if (key.isActive && game.screenshake > 0)
        //     {
        //         game.screenshake--;
        //         graphics.updatescreenshake();
        //     }

        //     if (graphics.screenbuffer->badSignalEffect)
        //     {
        //         UpdateFilter();
        //     }

        //     //We did editorinput, now it's safe to turn this off
        //     key.linealreadyemptykludge = false;

        //     if (game.savemystats)
        //     {
        //         game.savemystats = false;
        //         game.savestats();
        //     }

        //     //Mute button
        //     if (key.isDown(KEYBOARD_m) && game.mutebutton<=0 && !key.textentry())
        //     {
        //         game.mutebutton = 8;
        //         if (game.muted)
        //         {
        //             game.muted = false;
        //         }
        //         else
        //         {
        //             game.muted = true;
        //         }
        //     }
        //     if(game.mutebutton>0)
        //     {
        //         game.mutebutton--;
        //     }

        //     if (key.isDown(KEYBOARD_n) && game.musicmutebutton <= 0 && !key.textentry())
        //     {
        //         game.musicmutebutton = 8;
        //         game.musicmuted = !game.musicmuted;
        //     }
        //     if (game.musicmutebutton > 0)
        //     {
        //         game.musicmutebutton--;
        //     }

        //     if (game.muted)
        //     {
        //         Mix_VolumeMusic(0) ;
        //         Mix_Volume(-1,0);
        //     }
        //     else
        //     {
        //         Mix_Volume(-1,MIX_MAX_VOLUME);

        //         if (game.musicmuted)
        //         {
        //             Mix_VolumeMusic(0);
        //         }
        //         else
        //         {
        //             Mix_VolumeMusic(music.musicVolume);
        //         }
        //     }

        //     if (key.resetWindow)
        //     {
        //         key.resetWindow = false;
        //         gameScreen.ResizeScreen(-1, -1);
        //     }

        //     music.processmusic();
        //     graphics.processfade();
        //     game.gameclock();
    }

    // void Screen::UpdateScreen(SDL_Surface* buffer, SDL_Rect* rect )
    pub fn update_screen(&mut self) {
        let rect_src = self.render.get_render_rect();
        let surf_src = &self.render.graphics.backBuffer;

        // FillRect(m_screen, 0x000);
        let rect_dst = sdl2::rect::Rect::new(0, 0, self.m_screen.width(), self.m_screen.height());
        match self.m_screen.fill_rect(rect_dst, sdl2::pixels::Color::BLACK) {
            Ok(_x) => (),
            Err(s) => panic!(s),
        };

        // BlitSurfaceStandard(buffer, NULL, m_screen, rect);

        match surf_src.blit(rect_src, &mut self.m_screen, rect_dst) {
            Ok(_x) => (),
            Err(s) => panic!(s),
        };

        match self.m_screen.save_bmp("backbuffer.bmp") {
            Ok(_x) => (),
            Err(s) => panic!(s),
        };

        let texture_creator = self.canvas.texture_creator();
        let texture = self.m_screen.as_texture(&texture_creator).unwrap();
        self.canvas.copy(&texture, None, None);

        // self.canvas.set_draw_color(Color::RGB(128, 128, 128));
        // self.canvas.clear();
    }
}
