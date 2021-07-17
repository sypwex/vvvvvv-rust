use sdl2::controller::Button;
use crate::{INBOUNDS_VEC, entity, map, music, screen::{self, render::graphics}, script, utility_class};

pub const numcrew: usize = 6;
const numunlock: usize = 25;
const numtrials: usize = 6;

pub struct Game {
    // saveFilePath: String,

    pub door_left: i32,
    pub door_right: i32,
    pub door_up: i32,
    pub door_down: i32,
    pub roomx: i32,
    pub roomy: i32,
    pub roomchangedir: i32,
    pub prevroomx: i32,
    pub prevroomy: i32,

    pub savex: i32,
    pub savey: i32,
    pub saverx: i32,
    pub savery: i32,
    pub savegc: i32,
    pub savedir: i32,

    // Added for port
    edsavex: i32,
    edsavey: i32,
    edsaverx: i32,
    edsavery: i32,
    edsavegc: i32,
    edsavedir: i32,

    // State logic stuff
    pub state: i32,
    pub statedelay: i32,

    pub glitchrunkludge: bool,

    usingmmmmmm: i32,

    pub gamestate: GameState,
    pub prevgamestate: GameState, //only used sometimes
    pub hascontrol: bool,
    pub jumpheld: bool,
    pub jumppressed: i32,
    pub gravitycontrol: i32,

    pub muted: bool,
    pub mutebutton: i32,
    pub musicmuted: bool,
    pub musicmutebutton: i32,

    pub tapleft: i32,
    pub tapright: i32,

    // Menu interaction stuff
    pub mapheld: bool,
    pub menupage: i32,
    pub lastsaved: i32,
    pub deathcounts: i32,
    pub silence_settings_error: bool,

    frames: i32,
    pub seconds: i32,
    pub minutes: i32,
    hours: i32,
    pub gamesaved: bool,
    pub gamesavefailed: bool,
    pub savetime: String,
    pub savearea: String,
    pub savetrinkets: i32,
    pub startscript: bool,
    pub newscript: String,

    mainmenu: i32,
    pub menustart: bool,

    // Teleporting
    pub teleport_to_new_area: bool,
    pub teleport_to_x: i32,
    pub teleport_to_y: i32,
    pub teleportscript: String,
    pub useteleporter: bool,
    pub teleport_to_teleporter: i32,

    // Main Menu Variables
    pub menuoptions: Vec<MenuOption>,
    pub currentmenuoption: i32,
    pub currentmenuname: MenuName,
    kludge_ingametemp: MenuName,
    pub current_credits_list_index: i32,
    pub menuxoff: i32,
    pub menuyoff: i32,
    pub menuspacing: i32,
    // static const menutextbytes: i32: 161, // this is just sizeof(MenuOption::text), but doing that is non-standard
    menustack: Vec<MenuStackFrame>,

    pub menucountdown: i32,
    pub menudest: MenuName,

    pub creditposx: i32,
    pub creditposy: i32,
    pub creditposdelay: i32,
    oldcreditposx: i32,


    // Sine Wave Ninja Minigame
    pub swnmode: bool,
    pub swngame: i32,
    pub swnstate: i32,
    pub swnstate2: i32,
    pub swnstate3: i32,
    pub swnstate4: i32,
    pub swndelay: i32,
    pub swndeaths: i32,
    pub swntimer: i32,
    pub swncolstate: i32,
    pub swncoldelay: i32,
    pub swnrecord: i32,
    pub swnbestrank: i32,
    pub swnrank: i32,
    pub swnmessage: i32,

    // SuperCrewMate Stuff
    pub supercrewmate: bool,
    pub scmhurt: bool,
    pub scmmoveme: bool,
    pub scmprogress: i32,

    // Accessibility Options
    pub colourblindmode: bool,
    pub noflashingmode: bool,
    pub slowdown: i32,
    pub gameframerate: u32,

    pub nodeathmode: bool,
    pub gameoverdelay: i32,
    pub nocutscenes: bool,
    pub ndmresultcrewrescued: i32,
    pub ndmresulttrinkets: i32,
    pub ndmresulthardestroom: String,

    // Time Trials
    pub intimetrial: bool,
    pub timetrialparlost: bool,
    pub timetrialcountdown: i32,
    pub timetrialshinytarget: i32,
    pub timetriallevel: i32,
    pub timetrialpar: i32,
    pub timetrialresulttime: i32,
    pub timetrialresultframes: i32,
    pub timetrialrank: i32,
    pub timetrialresultshinytarget: i32,
    pub timetrialresulttrinkets: i32,
    pub timetrialresultpar: i32,
    pub timetrialresultdeaths: i32,

    pub creditposition: i32,
    oldcreditposition: i32,
    pub insecretlab: bool,

    pub inintermission: bool,

    // numcrew: i32, // const
    pub crewstats: [bool; numcrew],
    pub ndmresultcrewstats: [bool; numcrew],
    pub tele_crewstats: [bool; numcrew],
    pub quick_crewstats: [bool; numcrew],
    // static const numtrials: i32 = 6,
    pub besttimes: [i32; numtrials],
    pub bestframes: [i32; numtrials],
    pub besttrinkets: [i32; numtrials],
    pub bestlives: [i32; numtrials],
    pub bestrank: [i32; numtrials],

    pub alarmon: bool,
    pub alarmdelay: i32,
    pub blackout: bool,

    // numunlock: i32, // static const
    pub unlock: [bool; numunlock],
    pub unlocknotify: [bool; numunlock],
    pub stat_trinkets: i32,
    fullscreen: bool,
    pub bestgamedeaths: i32,

    pub tele_gametime: String,
    pub tele_trinkets: i32,
    pub tele_currentarea: String,
    pub quick_gametime: String,
    pub quick_trinkets: i32,
    pub quick_currentarea: String,

    mx: i32,
    my: i32,
    pub screenshake: i32,
    pub flashlight: i32,
    pub advancetext: bool,
    pub pausescript: bool,

    pub deathseq: i32,
    pub lifeseq: i32,

    pub savepoint: i32,
    pub teleportxpos: i32,
    pub teleport: bool,
    pub edteleportent: i32,
    pub completestop: bool,

    pub inertia: f32,

    pub companion: i32,
    pub roomchange: bool,
    pub teleblock: sdl2::rect::Rect,
    pub activetele: bool,
    pub readytotele: i32,
    pub oldreadytotele: i32,
    pub activity_r: i32,
    pub activity_g: i32,
    pub activity_b: i32,
    pub activity_lastprompt: String,

    pub telesummary: String,
    pub quicksummary: String,
    customquicksummary: String,
    // save_exists: bool(),

    pub backgroundtext: bool,

    pub activeactivity: i32,
    pub act_fade: i32,
    pub prev_act_fade: i32,

    pub press_left: bool,
    pub press_right: bool,
    pub press_action: bool,
    pub press_map: bool,

    // Some stats:
    pub totalflips: i32,
    pub hardestroom: String,
    pub hardestroomdeaths: i32,
    pub currentroomdeaths: i32,

    savemystats: bool,

    fullScreenEffect_badSignal: bool,
    useLinearFilter: bool,
    stretchMode: i32,
    controllerSensitivity: i32,

    pub quickrestartkludge: bool,

    // Custom stuff
    // customscript: String[50],
    customcol: i32,
    pub levelpage: i32,
    playcustomlevel: i32,
    customleveltitle: String,
    customlevelfilename: String,

    // std::vector<CustomLevelStat> customlevelstats,
    customlevelstatsloaded: bool,

    pub controllerButton_map: Vec<Button>,
    pub controllerButton_flip: Vec<Button>,
    pub controllerButton_esc: Vec<Button>,
    pub controllerButton_restart: Vec<Button>,

    pub skipfakeload: bool,
    ghostsenabled: bool,

    cliplaytest: bool,
    playx: i32,
    playy: i32,
    playrx: i32,
    playry: i32,
    playgc: i32,
    playmusic: i32,
    playassets: String,

    // void quittomenu(),
    // void returntolab(),
    fadetomenu: bool,
    fadetomenudelay: i32,
    fadetolab: bool,
    fadetolabdelay: i32,

    // #if !defined(NO_CUSTOM_LEVELS)
    //   void returntoeditor(),
    //   shouldreturntoeditor: bool,
    // #endif

    gametimer: i32,

    //   inline: bool inspecial() {
    //       return inintermission || insecretlab || intimetrial || nodeathmode,
    //   }

    pub over30mode: bool,
    pub glitchrunnermode: bool, // Have fun speedrunners! <3 Misa

    pub ingame_titlemode: bool,
    // #if !defined(NO_CUSTOM_LEVELS) && !defined(NO_EDITOR)
    ingame_editormode: bool,
    // #endif
    pub slidermode: SLIDERMODE,

    pub disablepause: bool,
    pub inputdelay: bool,
}

impl Game {
    pub fn new(graphics: &mut graphics::Graphics, music: &music::Music, screen_params: screen::ScreenParams, map: &mut map::Map) -> Game {
        let mut game = Game {
            door_left: 0,
            door_right: 0,
            door_up: 0,
            door_down: 0,
            roomx: 0,
            roomy: 0,
            roomchangedir: 0,
            prevroomx: 0,
            prevroomy: 0,

            savex: 0,
            savey: 0,
            saverx: 0,
            savery: 0,
            savegc: 0,
            savedir: 0,

            // Added for port
            edsavex: 0,
            edsavey: 0,
            edsaverx: 0,
            edsavery: 0,
            edsavegc: 0,
            edsavedir: 0,

            mutebutton: 0,
            muted: false,
            musicmuted: false,
            musicmutebutton: 0,

            glitchrunkludge: false,

            usingmmmmmm: 0,

            gamestate: GameState::PRELOADER,
            prevgamestate: GameState::PRELOADER,

            hascontrol: true,
            jumpheld: false,
            advancetext: false,
            jumppressed: 0,
            gravitycontrol: 0,
            companion: 0,
            roomchange: false,
            teleblock: sdl2::rect::Rect::new(0, 0, 0, 0),

            savemystats: false,
            quickrestartkludge: false,

            // Custom stuff
            // customscript: String[50],
            customcol: 0,
            levelpage: 0,
            playcustomlevel: 0,
            customleveltitle: String::new(),
            customlevelfilename: String::new(),

            tapleft: 0,
            tapright: 0,

            // Menu interaction stuff
            mapheld: false,
            menupage: 0,
            lastsaved: 0,
            deathcounts: 0,
            silence_settings_error: false,

            press_left: false,
            press_right: false,
            press_action: false,
            press_map: false,

            pausescript: false,

            deathseq: 0,
            lifeseq: 0,

            savepoint: 0,
            teleportxpos: 0,
            teleport: false,
            edteleportent: 0, // Added in the port!
            completestop: false,

            activeactivity: -1,
            act_fade: 0,
            prev_act_fade: 0,
            backgroundtext: false,
            inintermission: false,

            alarmon: false,
            alarmdelay: 0,
            blackout: false,
            creditposx: 0,
            creditposy: 0,
            creditposdelay: 0,
            oldcreditposx: 0,

            menuoptions: vec![],
            currentmenuoption: 0,
            currentmenuname: MenuName::mainmenu,
            kludge_ingametemp: MenuName::mainmenu,
            current_credits_list_index: 0,
            menuxoff: 0,
            menuyoff: 0,
            menuspacing: 0,

            activetele: false,
            readytotele: 0,
            oldreadytotele: 0,
            activity_r: 0,
            activity_g: 0,
            activity_b: 0,
            activity_lastprompt: String::new(),

            telesummary: String::new(),
            quicksummary: String::new(),
            customquicksummary: String::new(),

            bestgamedeaths: -1,

            fullScreenEffect_badSignal: false,

            //Accessibility Options
            colourblindmode: false,
            noflashingmode: false,
            slowdown: 30,
            gameframerate: 34,

            fullscreen: false, // true, // Assumed true at first unless overwritten at some point!
            stretchMode: 0,
            useLinearFilter: false,
            // 0..5
            controllerSensitivity: 2,

            nodeathmode: false,
            gameoverdelay: 0,
            nocutscenes: false,
            ndmresultcrewrescued: 0,
            ndmresulttrinkets: 0,
            ndmresulthardestroom: String::new(),

            // Time Trials
            intimetrial: false,
            timetrialparlost: false,
            timetrialcountdown: 0,
            timetrialshinytarget: 0,
            timetriallevel: 0,
            timetrialpar: 0,
            timetrialresulttime: 0,
            timetrialresultframes: 0,
            timetrialrank: 0,
            timetrialresultshinytarget: 0,
            timetrialresulttrinkets: 0,
            timetrialresultpar: 0,
            timetrialresultdeaths: 0,

            creditposition: 0,
            oldcreditposition: 0,
            insecretlab: false,

            crewstats: [false; numcrew],
            ndmresultcrewstats: [false; numcrew],
            tele_crewstats: [false; numcrew],
            quick_crewstats: [false; numcrew],
            besttimes: [-1; numcrew],
            bestframes: [-1; numcrew],
            besttrinkets: [-1; numcrew],
            bestlives: [-1; numcrew],
            bestrank: [-1; numcrew],

            tele_gametime: "00:00".to_owned(),
            tele_trinkets: 0,
            tele_currentarea: "Error! Error!".to_owned(),
            quick_gametime: "00:00".to_owned(),
            quick_trinkets: 0,
            quick_currentarea: "Error! Error!".to_owned(),

            // Menu stuff initiliased here:
            // SDL_memset(unlock, false, sizeof(unlock));
            // SDL_memset(unlocknotify, false, sizeof(unlock));
            unlock: [false; numunlock],
            unlocknotify: [false; numunlock],

            menustack: vec![],

            menucountdown: 0,
            menudest: MenuName::accessibility,

            startscript: false,
            newscript: String::new(),

            mainmenu: 0,
            menustart: false,

            // Teleporting
            teleport_to_new_area: false,
            teleport_to_x: 0,
            teleport_to_y: 0,
            teleportscript: String::new(),
            useteleporter: false,
            teleport_to_teleporter: 0,

            frames: 0,
            seconds: 0,
            minutes: 0,
            hours: 0,
            gamesaved: false,
            gamesavefailed: false,
            savetime: "00:00".to_owned(),
            savearea: "nowhere".to_owned(),
            savetrinkets: 0,

            totalflips: 0,
            hardestroom: "Welcome Aboard".to_owned(),
            hardestroomdeaths: 0,
            currentroomdeaths: 0,

            inertia: 1.1,
            swnmode: false,
            swntimer: 0,
            swngame: 0, // Not playing sine wave ninja!
            swnstate: 0,
            swnstate2: 0,
            swnstate3: 0,
            swnstate4: 0,
            swndelay: 0,
            swndeaths: 0,
            supercrewmate: false,
            scmhurt: false,
            scmprogress: 0,
            scmmoveme: false,
            swncolstate: 0,
            swncoldelay: 0,
            swnrecord: 0,
            swnbestrank: 0,
            swnrank: 0,
            swnmessage: 0,

            // clearcustomlevelstats(),

            // saveFilePath: FILESYSTEM_getUserSaveDirectory(),

            // tinyxml2::XMLDocument doc;
            // if !FILESYSTEM_loadTiXml2Document("saves/qsave.vvv", doc))
            // {
            //     quicksummary = "";
            //     printf("Quick Save Not Found\n");
            // }
            // else
            // {
            //     tinyxml2::XMLHandle hDoc(&doc);
            //     tinyxml2::XMLElement* pElem;
            //     tinyxml2::XMLHandle hRoot(NULL);
            //     pElem=hDoc.FirstChildElement().ToElement();
            //     if !pElem)
            //     {
            //         printf("Quick Save Appears Corrupted: No XML Root\n");
            //     }
            //     // save this for later
            //     hRoot=tinyxml2::XMLHandle(pElem);
            //     for( pElem = hRoot.FirstChildElement( "Data" ).FirstChild().ToElement(); pElem; pElem=pElem->NextSiblingElement())
            //     {
            //         std::string pKey(pElem->Value());
            //         const char* pText = pElem->GetText() ;
            //         if pKey == "summary")
            //         {
            //             quicksummary = pText;
            //         }
            //     }
            // }


            // tinyxml2::XMLDocument docTele;
            // if !FILESYSTEM_loadTiXml2Document("saves/tsave.vvv", docTele)) {
            //     telesummary = "";
            //     printf("Teleporter Save Not Found\n");
            // } else {
            //     tinyxml2::XMLHandle hDoc(&docTele);
            //     tinyxml2::XMLElement* pElem;
            //     tinyxml2::XMLHandle hRoot(NULL); {
            //         pElem=hDoc.FirstChildElement().ToElement();
            //         // should always have a valid root but handle gracefully if it does
            //         if !pElem) {
            //             printf("Teleporter Save Appears Corrupted: No XML Root\n");
            //         }
            //         // save this for later
            //         hRoot=tinyxml2::XMLHandle(pElem);
            //     }
            //     for( pElem = hRoot.FirstChildElement( "Data" ).FirstChild().ToElement(); pElem; pElem=pElem->NextSiblingElement())
            //     {
            //         std::string pKey(pElem->Value());
            //         const char* pText = pElem->GetText() ;
            //         if pKey == "summary")
            //         {
            //             telesummary = pText;
            //         }
            //     }
            // }

            mx: 0,
            my: 0,
            screenshake: 0,
            flashlight: 0,

            stat_trinkets: 0,

            state: 1,
            statedelay: 0,
            //updatestate(),

            customlevelstatsloaded: false,

            controllerButton_map: vec![],
            controllerButton_flip: vec![],
            controllerButton_esc: vec![],
            controllerButton_restart: vec![],

            skipfakeload: false,

            ghostsenabled: false,
            gametimer: 0,

            cliplaytest: false,
            playx: 0,
            playy: 0,
            playrx: 0,
            playry: 0,
            playgc: 0,
            playmusic: 0,
            playassets: String::new(),

            fadetomenu: false,
            fadetomenudelay: 0,
            fadetolab: false,
            fadetolabdelay: 0,

            // #if !defined(NO_CUSTOM_LEVELS)
            //     shouldreturntoeditor = false;
            // #endif

            over30mode: false,
            glitchrunnermode: false,

            ingame_titlemode: false,
            // #if !defined(NO_CUSTOM_LEVELS) && !defined(NO_EDITOR)
            ingame_editormode: false,
            // #endif
            slidermode: SLIDERMODE::SLIDER_NONE,

            disablepause: false,
            inputdelay: false,
        };

        game.createmenu(MenuName::mainmenu, Some(false), graphics, music, screen_params, map);

        game
    }

    // void Game::init(void);
    pub fn init(&mut self, music: &mut music::Music) {
        // static inline int get_framerate(const int slowdown)
        self.gameframerate = match self.slowdown {
            30 => 34,
            24 => 41,
            18 => 55,
            12 => 83,
            _  => 34,
        };

        if self.skipfakeload     { self.gamestate = GameState::TITLEMODE };
        if self.usingmmmmmm == 0 { music.usingmmmmmm = false; }
        if self.usingmmmmmm == 1 { music.usingmmmmmm = true; }
        if self.slowdown == 0    { self.slowdown = 30; }

        // // Check to see if you've already unlocked some achievements here from before the update
        // if self.swnbestrank > 0 {
        //     if self.swnbestrank >= 1 { self.unlockAchievement("vvvvvvsupgrav5") };
        //     if self.swnbestrank >= 2 { self.unlockAchievement("vvvvvvsupgrav10") };
        //     if self.swnbestrank >= 3 { self.unlockAchievement("vvvvvvsupgrav15") };
        //     if self.swnbestrank >= 4 { self.unlockAchievement("vvvvvvsupgrav20") };
        //     if self.swnbestrank >= 5 { self.unlockAchievement("vvvvvvsupgrav30") };
        //     if self.swnbestrank >= 6 { self.unlockAchievement("vvvvvvsupgrav60") };
        // }

        // if self.unlock[ 5] { self.unlockAchievement("vvvvvvgamecomplete") }
        // if self.unlock[19] { self.unlockAchievement("vvvvvvgamecompleteflip") }
        // if self.unlock[20] { self.unlockAchievement("vvvvvvmaster") }

        // if self.bestgamedeaths > -1 {
        //     if self.bestgamedeaths <= 500 { self.unlockAchievement("vvvvvvcomplete500"); }
        //     if self.bestgamedeaths <= 250 { self.unlockAchievement("vvvvvvcomplete250"); }
        //     if self.bestgamedeaths <= 100 { self.unlockAchievement("vvvvvvcomplete100"); }
        //     if self.bestgamedeaths <= 50 { self.unlockAchievement("vvvvvvcomplete50"); }
        // }

        // if self.bestrank[0]>=3 { self.unlockAchievement("vvvvvvtimetrial_station1_fixed"); }
        // if self.bestrank[1]>=3 { self.unlockAchievement("vvvvvvtimetrial_lab_fixed"); }
        // if self.bestrank[2]>=3 { self.unlockAchievement("vvvvvvtimetrial_tower_fixed"); }
        // if self.bestrank[3]>=3 { self.unlockAchievement("vvvvvvtimetrial_station2_fixed"); }
        // if self.bestrank[4]>=3 { self.unlockAchievement("vvvvvvtimetrial_warp_fixed"); }
        // if self.bestrank[5]>=3 { self.unlockAchievement("vvvvvvtimetrial_final_fixed"); }
    }

    // int Game::crewrescued(void);
    pub fn crewrescued(&mut self) -> i32 {
        println!("DEADBEEF: Game::crewrescued() method not implemented yet");
        0
    }

    // std::string Game::unrescued(void);
    pub fn unrescued(&mut self) -> &str {
        println!("DEADBEEF: Game::unrescued() method not implemented yet");
        &""
    }

    // void Game::resetgameclock(void);
    pub fn resetgameclock(&mut self) {
        self.frames = 0;
        self.seconds = 0;
        self.minutes = 0;
        self.hours = 0;
    }

    // bool Game::customsavequick(std::string savfile);
    pub fn customsavequick(&mut self, savfile: &str) -> bool {
        println!("DEADBEEF: Game::customsavequick() method not implemented yet");
        false
    }

    // bool Game::savequick(void);
    pub fn savequick(&mut self) -> bool {
        println!("DEADBEEF: Game::savequick() method not implemented yet");
        false
    }

    // void Game::gameclock(void);
    pub fn gameclock (&mut self) {
        if self.timetrialcountdown > 0 {
            return;
        }

        self.frames += 1;
        if self.frames >= 30 {
            self.frames -= 30;
            self.seconds += 1;

            if self.seconds >= 60 {
                self.seconds -= 60;
                self.minutes += 1;

                if self.minutes >= 60 {
                    self.minutes -= 60;
                    self.hours += 1;
                }
            }
        }
    }

    // std::string Game::giventimestring(int hrs, int min, int sec);

    // std::string Game::timestring(void);
    pub fn timestring(&mut self, help: &mut utility_class::UtilityClass) -> &str {
        println!("DEADBEEF: timestring not implemented yet");

        // std::string tempstring = "";
        // if hours > 0 {
        //     tempstring += help.String(hours) + ":";
        // }
        // tempstring += help.twodigits(minutes) + ":" + help.twodigits(seconds);
        // return tempstring;

        // let mut tempstring = String::new();
        // if self.hours > 0 {
        //     tempstring = format!("{}{}", help.String(self.hours), ":");
        // }
        // tempstring = format!("{}{}{}{}", tempstring, help.twodigits(self.minutes), ":", help.twodigits(self.seconds));

        &""
    }

    // std::string Game::partimestring(void);
    pub fn partimestring(&self) -> &str {
        println!("DEADBEEF: partimestring not implemented yet");

        //given par time in seconds:
        let tempstring = "";
        // if timetrialpar >= 60 {
        //     tempstring = help.twodigits(int((timetrialpar - (timetrialpar % 60)) / 60)) + ":" + help.twodigits(timetrialpar % 60);
        // } else {
        //     tempstring = "00:" + help.twodigits(timetrialpar);
        // }

        tempstring
    }

    // std::string Game::resulttimestring(void);
    pub fn resulttimestring(&self) -> &str {
        println!("DEADBEEF: resulttimestring not implemented yet");
        &""
    }

    // std::string Game::timetstring(int t);
    pub fn timetstring(&self, t: i32) -> &str {
        println!("DEADBEEF: timetstring not implemented yet");
        &""
    }

    // void Game::returnmenu(void);
    pub fn returnmenu(&mut self, graphics: &mut graphics::Graphics, music: &mut music::Music, screen_params: screen::ScreenParams, map: &mut map::Map) {
        match self.menustack.pop() {
            Some(frame) => {
                // Store this in case createmenu() removes the stack frame
                let previousoption = frame.option;

                self.createmenu(frame.name, Some(true), graphics, music, screen_params, map);
                self.currentmenuoption = previousoption;

                // @sx: looks like don't need it
                // Remove the stackframe now, but createmenu() might have already gotten to it
                // if we were returning to the main menu
                // if !self.menustack.empty() {
                //     self.menustack.pop_back();
                // }
            },
            None => println!("Error: returning to previous menu frame on empty stack!"),
        }
    }

    // void Game::returntomenu(enum Menu::MenuName t);
    pub fn returntomenu(&mut self, t: MenuName) {
        println!("DEADBEEF: Game::returntomenu not implemented yet");
    }

    // void Game::createmenu( enum Menu::MenuName t, bool samemenu/*= false*/ )
    pub fn createmenu(&mut self, t: MenuName, samemenu: Option<bool>, graphics: &mut graphics::Graphics, music: &music::Music, screen_params: screen::ScreenParams, map: &mut map::Map) {
        let samemenu = samemenu.unwrap_or(false);
        if t == MenuName::mainmenu {
            //Either we've just booted up the game or returned from gamemode
            //Whichever it is, we shouldn't have a stack,
            //and most likely don't have a current stackframe
            self.menustack = vec![];
        } else if !samemenu {
            let frame = MenuStackFrame{
                option: self.currentmenuoption,
                name: self.currentmenuname,
            };
            self.menustack.push(frame);
            self.currentmenuoption = 0;
        }

        self.currentmenuname = t;
        self.menuyoff = 0;
        let mut maxspacing = 30; // maximum value for menuspacing, can only become lower.
        self.menucountdown = 0;
        self.menuoptions = vec![];

        match self.currentmenuname {
            MenuName::mainmenu => {
                // #if !defined(MAKEANDPLAY)
                self.add_menu_option("play", None);
                // #endif

                // #if !defined(NO_CUSTOM_LEVELS)
                self.add_menu_option("levels", None);
                // #endif

                self.add_menu_option("options", None);

                // #if !defined(MAKEANDPLAY)
                self.add_menu_option("credits", None);
                // #endif

                self.add_menu_option("quit", None);

                self.menuyoff = -10;
                maxspacing = 15;
            },
            MenuName::quickloadlevel => {
                self.add_menu_option("continue from save", None);
                self.add_menu_option("start from beginning", None);
                self.add_menu_option("back to levels", None);
                self.menuyoff = -30;
            },
            MenuName::youwannaquit => {
                self.add_menu_option("yes, quit", None);
                self.add_menu_option("no, return", None);
                self.menuyoff = -20;
            },
            MenuName::errornostart => {
                self.add_menu_option("ok", None);
                self.menuyoff = -20;
            },
            MenuName::gameplayoptions => {
                // #if !defined(MAKEANDPLAY)
                if self.ingame_titlemode && self.unlock[18] {
                // #endif
                    self.add_menu_option("flip mode", None);
                }
                self.add_menu_option("toggle fps", None);
                self.add_menu_option("speedrun options", None);
                self.add_menu_option("advanced options", None);
                self.add_menu_option("clear data", None);
                self.add_menu_option("return", None);
                self.menuyoff = -10;
                maxspacing = 15;
            },
            MenuName::graphicoptions => {
                self.add_menu_option("toggle fullscreen", None);
                self.add_menu_option("scaling mode", None);
                self.add_menu_option("resize to nearest", Some(screen_params.isWindowed));
                self.add_menu_option("toggle filter", None);
                self.add_menu_option("toggle analogue", None);
                self.add_menu_option("toggle vsync", None);
                self.add_menu_option("return", None);
                self.menuyoff = -10;
                maxspacing = 15;
            },
            MenuName::ed_settings => {
                self.add_menu_option("change description", None);
                self.add_menu_option("edit scripts", None);
                self.add_menu_option("change music", None);
                self.add_menu_option("editor ghosts", None);
                self.add_menu_option("load level", None);
                self.add_menu_option("save level", None);
                self.add_menu_option("options", None);
                self.add_menu_option("quit to main menu", None);

                self.menuyoff = -20;
                maxspacing = 15;
            },
            MenuName::ed_desc => {
                self.add_menu_option("change name", None);
                self.add_menu_option("change author", None);
                self.add_menu_option("change description", None);
                self.add_menu_option("change website", None);
                self.add_menu_option("back to settings", None);

                self.menuyoff = 6;
                maxspacing = 15;
            },
            MenuName::ed_music => {
                self.add_menu_option("next song", None);
                self.add_menu_option("previous song", None);
                self.add_menu_option("back", None);
                self.menuyoff = 16;
                maxspacing = 15;
            },
            MenuName::ed_quit => {
                self.add_menu_option("yes, save and quit", None);
                self.add_menu_option("no, quit without saving", None);
                self.add_menu_option("return to editor", None);
                self.menuyoff = 8;
                maxspacing = 15;
            },
            MenuName::options => {
                self.add_menu_option("gameplay", None);
                self.add_menu_option("graphics", None);
                self.add_menu_option("audio", None);
                self.add_menu_option("game pad", None);
                self.add_menu_option("accessibility", None);
                self.add_menu_option("return", None);
                self.menuyoff = 0;
                maxspacing = 15;
            },
            MenuName::speedrunneroptions => {
                self.add_menu_option("glitchrunner mode", None);
                self.add_menu_option("input delay", None);
                self.add_menu_option("fake load screen", None);
                self.add_menu_option("return", None);
                self.menuyoff = 0;
                maxspacing = 15;
            },
            MenuName::advancedoptions => {
                self.add_menu_option("unfocus pause", None);
                self.add_menu_option("room name background", None);
                self.add_menu_option("return", None);
                self.menuyoff = 0;
                maxspacing = 15;
            },
            MenuName::audiooptions => {
                self.add_menu_option("music volume", None);
                self.add_menu_option("sound volume", None);
                if music.mmmmmm {
                    self.add_menu_option("soundtrack", None);
                }
                self.add_menu_option("return", None);
                self.menuyoff = 0;
                maxspacing = 15;
            },


            MenuName::accessibility => {
                // #if !defined(MAKEANDPLAY)
                self.add_menu_option("unlock play modes", None);
                // #endif
                self.add_menu_option("invincibility", Some(!self.ingame_titlemode || (!self.insecretlab && !self.intimetrial && !self.nodeathmode)));
                self.add_menu_option("slowdown", Some(!self.ingame_titlemode || (!self.insecretlab && !self.intimetrial && !self.nodeathmode)));
                self.add_menu_option("animated backgrounds", None);
                self.add_menu_option("screen effects", None);
                self.add_menu_option("text outline", None);
                self.add_menu_option("return", None);
                self.menuyoff = 0;
                maxspacing = 15;
            },
            MenuName::controller => {
                self.add_menu_option("analog stick sensitivity", None);
                self.add_menu_option("bind flip", None);
                self.add_menu_option("bind enter", None);
                self.add_menu_option("bind menu", None);
                self.add_menu_option("bind restart", None);
                self.add_menu_option("return", None);
                self.menuyoff = 0;
                maxspacing = 10;
            },
            MenuName::cleardatamenu => {
                self.add_menu_option("no! don't delete", None);
                self.add_menu_option("yes, delete everything", None);
                self.menuyoff = 64;
            },
            MenuName::setinvincibility => {
                self.add_menu_option("no, return to options", None);
                self.add_menu_option("yes, enable", None);
                self.menuyoff = 64;
            },
            MenuName::setslowdown => {
                self.add_menu_option("normal speed", None);
                self.add_menu_option("80% speed", None);
                self.add_menu_option("60% speed", None);
                self.add_menu_option("40% speed", None);
                self.menuyoff = 16;
            },
            MenuName::unlockmenu => {
                self.add_menu_option("unlock time trials", None);
                self.add_menu_option("unlock intermissions", Some(!self.unlock[16]));
                self.add_menu_option("unlock no death mode", Some(!self.unlock[17]));
                self.add_menu_option("unlock flip mode", Some(!self.unlock[18]));
                self.add_menu_option("unlock ship jukebox", Some(self.stat_trinkets < 20));
                self.add_menu_option("unlock secret lab", Some(!self.unlock[8]));
                self.add_menu_option("return", None);
                self.menuyoff = -20;
            },
            MenuName::credits => {
                self.add_menu_option("next page", None);
                self.add_menu_option("last page", None);
                self.add_menu_option("return", None);
                self.menuyoff = 64;
            },
            MenuName::credits2 => {
                self.add_menu_option("next page", None);
                self.add_menu_option("previous page", None);
                self.add_menu_option("return", None);
                self.menuyoff = 64;
            },
            MenuName::credits25 => {
                self.add_menu_option("next page", None);
                self.add_menu_option("previous page", None);
                self.add_menu_option("return", None);
                self.menuyoff = 64;
            },
            MenuName::credits3 => {
                self.add_menu_option("next page", None);
                self.add_menu_option("previous page", None);
                self.add_menu_option("return", None);
                self.menuyoff = 64;
            },
            MenuName::credits4 => {
                self.add_menu_option("next page", None);
                self.add_menu_option("previous page", None);
                self.add_menu_option("return", None);
                self.menuyoff = 64;
            },
            MenuName::credits5 => {
                self.add_menu_option("next page", None);
                self.add_menu_option("previous page", None);
                self.add_menu_option("return", None);
                self.menuyoff = 64;
            },
            MenuName::credits6 => {
                self.add_menu_option("first page", None);
                self.add_menu_option("previous page", None);
                self.add_menu_option("return", None);
                self.menuyoff = 64;
            },
            MenuName::play => {
                //Ok, here's where the self.unlock stuff comes into it:
                //First up, time trials:
                let mut temp = 0;
                if self.unlock[0] && self.stat_trinkets >= 3 && !self.unlocknotify[9] { temp += 1 };
                if self.unlock[1] && self.stat_trinkets >= 6 && !self.unlocknotify[10] { temp += 1 };
                if self.unlock[2] && self.stat_trinkets >= 9 && !self.unlocknotify[11] { temp += 1 };
                if self.unlock[3] && self.stat_trinkets >= 12 && !self.unlocknotify[12] { temp += 1 };
                if self.unlock[4] && self.stat_trinkets >= 15 && !self.unlocknotify[13] { temp += 1 };
                if self.unlock[5] && self.stat_trinkets >= 18 && !self.unlocknotify[14] { temp += 1 };
                if temp > 0 {
                    //you've self.unlocked a time trial!
                    if self.unlock[0] && self.stat_trinkets >= 3 {
                        self.unlocknotify[9] = true;
                        self.unlock[9] = true;
                    }
                    if self.unlock[1] && self.stat_trinkets >= 6 {
                        self.unlocknotify[10] = true;
                        self.unlock[10] = true;
                    }
                    if self.unlock[2] && self.stat_trinkets >= 9 {
                        self.unlocknotify[11] = true;
                        self.unlock[11] = true;
                    }
                    if self.unlock[3] && self.stat_trinkets >= 12 {
                        self.unlocknotify[12] = true;
                        self.unlock[12] = true;
                    }
                    if self.unlock[4] && self.stat_trinkets >= 15 {
                        self.unlocknotify[13] = true;
                        self.unlock[13] = true;
                    }
                    if self.unlock[5] && self.stat_trinkets >= 18 {
                        self.unlocknotify[14] = true;
                        self.unlock[14] = true;
                    }

                    if temp == 1 {
                        self.createmenu(MenuName::unlocktimetrial, Some(true), graphics, music, screen_params, map);
                        self.savestatsandsettings();
                    }
                    else if temp > 1 {
                        self.createmenu(MenuName::unlocktimetrials, Some(true), graphics, music, screen_params, map);
                        self.savestatsandsettings();
                    }
                }
                else
                {
                    //Alright, we haven't self.unlocked any time trials. How about no death mode?
                    temp = 0;
                    if self.bestrank[0] >= 2 { temp += 1 }
                    if self.bestrank[1] >= 2 { temp += 1 }
                    if self.bestrank[2] >= 2 { temp += 1 }
                    if self.bestrank[3] >= 2 { temp += 1 }
                    if self.bestrank[4] >= 2 { temp += 1 }
                    if self.bestrank[5] >= 2 { temp += 1 }
                    if temp >= 4 && !self.unlocknotify[17] {
                        //Unlock No Death Mode
                        self.unlocknotify[17] = true;
                        self.unlock[17] = true;
                        self.createmenu(MenuName::unlocknodeathmode, Some(true), graphics, music, screen_params, map);
                        self.savestatsandsettings();
                    }
                    //Alright then! Flip mode?
                    else if self.unlock[5] && !self.unlocknotify[18] {
                        self.unlock[18] = true;
                        self.unlocknotify[18] = true;
                        self.createmenu(MenuName::unlockflipmode, Some(true), graphics, music, screen_params, map);
                        self.savestatsandsettings();
                    }
                    //What about the intermission levels?
                    else if self.unlock[7] && !self.unlocknotify[16] {
                        self.unlock[16] = true;
                        self.unlocknotify[16] = true;
                        self.createmenu(MenuName::unlockintermission, Some(true), graphics, music, screen_params, map);
                        self.savestatsandsettings();
                    } else {
                        if self.save_exists() {
                            self.add_menu_option("continue", None);
                        } else {
                            self.add_menu_option("new game", None);
                        }
                        //ok, secret lab! no notification, but test:
                        if self.unlock[8] {
                            self.add_menu_option("secret lab", Some(!map.invincibility && self.slowdown == 30));
                        }
                        self.add_menu_option("play modes", None);
                        if self.save_exists() {
                            self.add_menu_option("new game", None);
                        }
                        self.add_menu_option("return", None);
                        if self.unlock[8] {
                            self.menuyoff = -30;
                        } else {
                            self.menuyoff = -40;
                        }
                    }
                }
            },
            MenuName::unlocktimetrial | MenuName::unlocktimetrials | MenuName::unlocknodeathmode | MenuName::unlockintermission | MenuName::unlockflipmode => {
                self.add_menu_option("continue", None);
                self.menuyoff = 70;
            },
            MenuName::newgamewarning => {
                self.add_menu_option("start new game", None);
                self.add_menu_option("return to menu", None);
                self.menuyoff = 64;
            },
            MenuName::playmodes => {
                self.add_menu_option("time trials", Some(!map.invincibility && self.slowdown == 30));
                self.add_menu_option("intermissions", Some(self.unlock[16]));
                self.add_menu_option("no death mode", Some(self.unlock[17] && !map.invincibility && self.slowdown == 30));
                self.add_menu_option("flip mode", Some(self.unlock[18]));
                self.add_menu_option("return to play menu", None);
                self.menuyoff = 8;
                maxspacing = 20;
            },
            MenuName::intermissionmenu => {
                self.add_menu_option("play intermission 1", None);
                self.add_menu_option("play intermission 2", None);
                self.add_menu_option("return to play menu", None);
                self.menuyoff = -35;
            },
            MenuName::playint1 => {
                self.add_menu_option("Vitellary", None);
                self.add_menu_option("Vermilion", None);
                self.add_menu_option("Verdigris", None);
                self.add_menu_option("Victoria", None);
                self.add_menu_option("return", None);
                self.menuyoff = 10;
            },
            MenuName::playint2 => {
                self.add_menu_option("Vitellary", None);
                self.add_menu_option("Vermilion", None);
                self.add_menu_option("Verdigris", None);
                self.add_menu_option("Victoria", None);
                self.add_menu_option("return", None);
                self.menuyoff = 10;
            },
            MenuName::continuemenu => {
                map.settowercolour(3, graphics);
                self.add_menu_option("continue from teleporter", None);
                self.add_menu_option("continue from quicksave", None);
                self.add_menu_option("return to play menu", None);
                self.menuyoff = 20;
            },
            MenuName::startnodeathmode => {
                self.add_menu_option("disable cutscenes", None);
                self.add_menu_option("enable cutscenes", None);
                self.add_menu_option("return to play menu", None);
                self.menuyoff = 40;
            },
            MenuName::gameover => {
                self.menucountdown = 120;
                self.menudest = MenuName::gameover2;
            },
            MenuName::gameover2 => {
                self.add_menu_option("return to play menu", None);
                self.menuyoff = 80;
            },
            MenuName::unlockmenutrials => {
                self.add_menu_option("space station 1", Some(!self.unlock[9]));
                self.add_menu_option("the laboratory", Some(!self.unlock[10]));
                self.add_menu_option("the tower", Some(!self.unlock[11]));
                self.add_menu_option("space station 2", Some(!self.unlock[12]));
                self.add_menu_option("the warp zone", Some(!self.unlock[13]));
                self.add_menu_option("the final level", Some(!self.unlock[14]));

                self.add_menu_option("return to self.unlock menu", None);
                self.menuyoff = 0;
            },
            MenuName::timetrials => {
                self.add_menu_option(if self.unlock[9]  { "space station 1" } else { "???" }, Some(self.unlock[9]));
                self.add_menu_option(if self.unlock[10] { "the laboratory"  } else { "???" }, Some(self.unlock[10]));
                self.add_menu_option(if self.unlock[11] { "the tower"       } else { "???" }, Some(self.unlock[11]));
                self.add_menu_option(if self.unlock[12] { "space station 2" } else { "???" }, Some(self.unlock[12]));
                self.add_menu_option(if self.unlock[13] { "the warp zone"   } else { "???" }, Some(self.unlock[13]));
                self.add_menu_option(if self.unlock[14] { "the final level" } else { "???" }, Some(self.unlock[14]));

                self.add_menu_option("return to play menu", None);
                self.menuyoff = 0;
                maxspacing = 15;
            },
            MenuName::nodeathmodecomplete => {
                self.menucountdown = 90;
                self.menudest = MenuName::nodeathmodecomplete2;
            },
            MenuName::nodeathmodecomplete2 => {
                self.add_menu_option("return to play menu", None);
                self.menuyoff = 70;
            },
            MenuName::timetrialcomplete => {
                self.menucountdown = 90;
                self.menudest = MenuName::timetrialcomplete2;
            },
            MenuName::timetrialcomplete2 => {
                self.menucountdown = 60;
                self.menudest = MenuName::timetrialcomplete3;
            },
            MenuName::timetrialcomplete3 => {
                self.add_menu_option("return to play menu", None);
                self.add_menu_option("try again", None);
                self.menuyoff = 70;
            },
            MenuName::gamecompletecontinue => {
                self.add_menu_option("return to play menu", None);
                self.menuyoff = 70;
            },
            MenuName::errorsavingsettings => {
                self.add_menu_option("ok", None);
                self.add_menu_option("silence", None);
                self.menuyoff = 10;
            },



            v => println!("DEADBEEF: Game::createmenu({:?}) is not implemented yet", v),
        }

        // Automatically center the menu. We must check the width of the menu with the initial horizontal spacing.
        // If it's too wide, reduce the horizontal spacing by 5 and retry.
        // Try to limit the menu width to 272 pixels: 320 minus 16*2 for square brackets, minus 8*2 padding.
        // The square brackets fall outside the menu width (i.e. selected menu options are printed 16 pixels to the left)
        let mut done_once = false;
        let mut menuwidth = 0;
        while !done_once || (menuwidth > 272 && self.menuspacing > 0) {
            done_once = true;
            self.menuspacing = maxspacing;
            menuwidth = 0;

            for i in 0..self.menuoptions.len() as i32 {
                let width = i*self.menuspacing + self.menuoptions[i as usize].text.len() as i32;
                if width > menuwidth {
                    menuwidth = width;
                }
            }

            maxspacing -= 5;
        }

        self.menuxoff = (320 - menuwidth)/2;
    }

    // void inline option(const char* text, bool active = true)
    fn add_menu_option (&mut self, text: &str, active: Option<bool>) {
        let active = active.unwrap_or(true);

        let menuoption = MenuOption {
            text: text.to_string(),
            active,
        };
        self.menuoptions.push(menuoption);
    }

    // void Game::lifesequence(void);
    pub fn lifesequence(&mut self, obj: &mut entity::EntityClass) {
        if self.lifeseq > 0 {
            let i = obj.getplayer() as usize;
            if INBOUNDS_VEC!(i, obj.entities) {
                obj.entities[i].invis = false;
                if self.lifeseq == 2 {
                    obj.entities[i].invis = true;
                }
                if self.lifeseq == 6 {
                    obj.entities[i].invis = true;
                }
                if self.lifeseq >= 8 {
                    obj.entities[i].invis = true;
                }
            }

            if self.lifeseq > 5 {
                self.gravitycontrol = self.savegc;
            }

            self.lifeseq -= 1;
            if INBOUNDS_VEC!(i, obj.entities) && self.lifeseq <= 0 {
                obj.entities[i].invis = false;
            }
        }
    }

    // void Game::gethardestroom(void);
    pub fn gethardestroom(&mut self) {
        println!("DEADBEEF: Game::gethardestroom() not implemented yet");
    }

    // void Game::levelcomplete_textbox(void);
    pub fn levelcomplete_textbox(&mut self) {
        println!("DEADBEEF: Game::levelcomplete_textbox() not implemented yet");
    }

    // void Game::crewmate_textbox(const int r, const int g, const int b);
    pub fn crewmate_textbox(&mut self, r: i32, g: i32, b: i32) {
        println!("DEADBEEF: Game::crewmate_textbox() not implemented yet");
    }

    // void Game::remaining_textbox(void);
    pub fn remaining_textbox(&mut self) {
        println!("DEADBEEF: Game::remaining_textbox() not implemented yet");
    }

    // void Game::actionprompt_textbox(void);
    pub fn actionprompt_textbox(&mut self) {
        println!("DEADBEEF: Game::actionprompt_textbox() not implemented yet");
    }

    // void Game::savetele_textbox(void);
    pub fn savetele_textbox(&mut self) {
        println!("DEADBEEF: Game::savetele_textbox() not implemented yet");
    }

    // void Game::updatestate(void);
    pub fn updatestate(&mut self, graphics: &mut graphics::Graphics, script: &mut script::ScriptClass, obj: &mut entity::EntityClass) {
        self.statedelay -= 1;
        if self.statedelay <= 0 {
            self.statedelay = 0;
            self.glitchrunkludge = false;
        }

        if self.statedelay <= 0 {
            match self.state {
                0 => {
                    //Do nothing here! Standard game state

                    if script.running {
                        if self.pausescript && !self.advancetext {
                            /* Prevent softlocks if we somehow don't have advancetext */
                            self.pausescript = false;
                        }
                    } else {
                        /* Prevent softlocks if there's no cutscene running right now */
                        self.hascontrol = true;
                        self.completestop = false;
                    }
                },
                1 => {
                    //Game initilisation
                    self.state = 0;
                },
                2 => {
                    //Opening cutscene
                    self.advancetext = true;
                    self.hascontrol = false;
                    self.state = 3;
                    graphics.createtextbox("To do: write quick", 50, 80, 164, 164, 255);
                    graphics.addline("intro to story!");
                    //Oh no! what happen to rest of crew etc crash into dimension
                },
                4 => {
                    //End of opening cutscene for now
                    graphics.createtextbox("  Press arrow keys or WASD to move  ", -1, 195, 174, 174, 174);
                    graphics.textboxtimer(60);
                    self.state = 0;
                },
                5 => {
                    //Demo over
                    self.advancetext = true;
                    self.hascontrol = false;

                    self.startscript = true;
                    self.newscript = "returntohub".into();
                    obj.removetrigger(5);
                    self.state = 6;
                },
                7 => {
                    //End of opening cutscene for now
                    graphics.textboxremove();
                    self.hascontrol = true;
                    self.advancetext = false;
                    self.state = 0;
                },
                8 => {
                    //Enter dialogue
                    obj.removetrigger(8);
                    if !obj.flags[13] {
                        obj.flags[13] = true;
                        graphics.createtextbox("  Press ENTER to view map  ", -1, 155, 174, 174, 174);
                        graphics.addline("      and quicksave");
                        graphics.textboxtimer(60);
                    }
                    self.state = 0;
                },
                _ => println!("DEADBEEF: Game::updatestate(): state {} not implemented yer", self.state),
            }
        }
    }

    // void Game::unlocknum(int t);
    pub fn unlocknum(&mut self, t: i32) {
        println!("DEADBEEF: Game::unlocknum() not implemented yet");
    }

    // void Game::loadstats(ScreenSettings* screen_settings);
    pub fn loadstats(&mut self, screen_settings: &mut screen::ScreenSettings) {
        println!("DEADBEEF: Game::loadstats() not implemented yet");
    }

    // bool Game::savestats(const ScreenSettings* screen_settings);
    // bool Game::savestats(void);
    pub fn savestats(&mut self, screen_settings: &mut screen::ScreenSettings) {
        println!("DEADBEEF: Game::savestats() not implemented yet");
    }

    // void Game::deletestats(void);
    pub fn deletestats(&mut self) {
        println!("DEADBEEF: Game::deletestats() not implemented yet");
    }

    // void Game::deserializesettings(tinyxml2::XMLElement* dataNode, ScreenSettings* screen_settings);

    // void Game::serializesettings(tinyxml2::XMLElement* dataNode, const ScreenSettings* screen_settings);

    // void Game::loadsettings(ScreenSettings* screen_settings);
    pub fn loadsettings(&mut self, screen_settings: &mut screen::ScreenSettings) {
        println!("DEADBEEF: Game::loadsettings not implemented yet");
    }

    // bool Game::savesettings(const ScreenSettings* screen_settings);
    // bool Game::savesettings(void);

    // bool Game::savestatsandsettings(void);
    pub fn savestatsandsettings(&mut self) {
        println!("DEADBEEF: Game::savestatsandsettings not implemented yet");
    }

    // void Game::savestatsandsettings_menu(void);
    pub fn savestatsandsettings_menu(&mut self) {
        println!("DEADBEEF: Game::savestatsandsettings_menu not implemented yet");
    }

    // void Game::deletesettings(void);
    pub fn deletesettings(&mut self) {
        println!("DEADBEEF: Game::deletesettings() method not implemented yet");
    }

    // void Game::deletequick(void);
    pub fn deletequick(&mut self) {
        println!("DEADBEEF: Game::deletequick() method not implemented yet");
    }

    // bool Game::savetele(void);
    pub fn savetele(&mut self) {
        println!("DEADBEEF: Game::savetele() method not implemented yet");
    }

    // void Game::loadtele(void);
    pub fn loadtele(&mut self) {
        println!("DEADBEEF: Game::loadtele() method not implemented yet");
    }

    // void Game::deletetele(void);
    pub fn deletetele(&mut self) {
        println!("DEADBEEF: Game::deletetele() method not implemented yet");
    }

    // void Game::customstart(void);
    pub fn customstart(&mut self) {
        println!("DEADBEEF: Game::customstart() method not implemented yet");
    }

    // void Game::start(void);
    pub fn start(&mut self, music: &mut music::Music) {
        self.jumpheld = true;

        self.savex = 232;
        self.savey = 113;
        self.saverx = 104;
        self.savery = 110;
        self.savegc = 0;
        self.savedir = 1; //Worldmap Start
        self.savepoint = 0;
        self.gravitycontrol = self.savegc;

        self.state = 0;
        self.deathseq = -1;
        self.lifeseq = 0;

        if !self.nocutscenes {
            music.play(5);
        }
    }

    // void Game::startspecial(int t);
    pub fn startspecial(&mut self, t: i32) {
        println!("DEADBEEF: Game::startspecial() method not implemented yet");
    }

    // void Game::starttrial(int t);
    pub fn starttrial(&mut self, t: i32) {
        println!("DEADBEEF: Game::starttrial() method not implemented yet");
    }

    // void Game::swnpenalty(void);
    pub fn swnpenalty(&mut self) {
        println!("DEADBEEF: Game::swnpenalty() method not implemented yet");
    }

    // void Game::deathsequence(void);
    pub fn deathsequence(&mut self, map: &mut map::Map, music: &mut music::Music, obj: &mut entity::EntityClass) {
        let i = if self.supercrewmate && self.scmhurt {
            obj.getscm()
        } else {
            obj.getplayer()
        } as usize;

        if INBOUNDS_VEC!(i, obj.entities) {
            obj.entities[i].colour = 1;
            obj.entities[i].invis = false;
        }

        if self.deathseq == 30 {
            if self.nodeathmode {
                music.fadeout(None);
                self.gameoverdelay = 60;
            }

            self.deathcounts += 1;
            music.playef(2);
            if INBOUNDS_VEC!(i, obj.entities) {
                obj.entities[i].invis = true;
            }

            if map.finalmode {
                if self.roomx - 41 >= 0 && self.roomx - 41 < 20 && self.roomy - 48 >= 0 && self.roomy - 48 < 20 {
                    let i = (self.roomx - 41 + (20 * (self.roomy - 48))) as usize;
                    map.roomdeathsfinal[i] += 1;
                    self.currentroomdeaths = map.roomdeathsfinal[i];
                }
            } else {
                if self.roomx - 100 >= 0 && self.roomx - 100 < 20 && self.roomy - 100 >= 0 && self.roomy - 100 < 20 {
                    let i = (self.roomx - 100 + (20*(self.roomy - 100))) as usize;
                    map.roomdeaths[i] += 1;
                    self.currentroomdeaths = map.roomdeaths[i];
                }
            }
        }

        if INBOUNDS_VEC!(i, obj.entities) {
            if self.deathseq == 25 { obj.entities[i].invis = true; }
            if self.deathseq == 20 { obj.entities[i].invis = true; }
            if self.deathseq == 16 { obj.entities[i].invis = true; }
            if self.deathseq == 14 { obj.entities[i].invis = true; }
            if self.deathseq == 12 { obj.entities[i].invis = true; }
            if self.deathseq  < 10 { obj.entities[i].invis = true; }
        }

        if !self.nodeathmode {
            if INBOUNDS_VEC!(i, obj.entities) && self.deathseq <= 1 {
                obj.entities[i].invis = false;
            }
        } else {
            self.gameoverdelay -= 1;
        }
    }

    // void Game::customloadquick(std::string savfile);
    pub fn customloadquick(&mut self, savfile: &str) {
        println!("DEADBEEF: Game::customloadquick() method not implemented yet");
    }

    // void Game::loadquick(void);
    pub fn loadquick(&mut self) {
        println!("DEADBEEF: Game::loadquick() method not implemented yet");
    }

    // void Game::loadsummary(void);
    pub fn loadsummary(&mut self) {
        println!("DEADBEEF: Game::loadsummary() method not implemented yet");
    }

    // void Game::readmaingamesave(tinyxml2::XMLDocument& doc);
    pub fn readmaingamesave(&mut self, doc: i32) {
        println!("DEADBEEF: Game::readmaingamesave() method not implemented yet");
    }

    // std::string Game::writemaingamesave(tinyxml2::XMLDocument& doc);
    pub fn Game(&mut self, doc: i32) -> &'static str {
        println!("DEADBEEF: ::string Game() method not implemented yet");
        &""
    }

    // void Game::initteleportermode(void);
    pub fn initteleportermode(&mut self) {
        println!("DEADBEEF: Game::initteleportermode() method not implemented yet");
    }

    // void Game::mapmenuchange(const int newgamestate);
    pub fn mapmenuchange(&mut self, newgamestate: GameState, graphics: &mut graphics::Graphics, map: &mut map::Map) {
        self.prevgamestate = self.gamestate;
        self.gamestate = newgamestate;
        graphics.resumegamemode = false;
        self.mapheld = true;

        if self.prevgamestate == GameState::GAMEMODE {
            graphics.menuoffset = 240;
            if map.extrarow != 0 {
                graphics.menuoffset -= 10;
            }
        } else {
            graphics.menuoffset = 0;
        }
        graphics.oldmenuoffset = graphics.menuoffset;
    }

    // int Game::get_timestep(void);
    pub fn get_timestep(&mut self) -> u32 {
        match self.gamestate {
            GameState::EDITORMODE => 24,
            GameState::GAMEMODE => self.gameframerate,
            _ => 34,
        }
    }

    // void Game::copyndmresults(void);
    pub fn copyndmresults(&mut self) {
        println!("DEADBEEF: Game::copyndmresults() method not implemented yet");
    }

    // int Game::trinkets(void);
    pub fn trinkets(&self, obj: &entity::EntityClass) -> i32 {
        let mut temp = 0;
        for ob in obj.collect.iter() {
            // if obj.collect[i] {
            // TODO @sx @impl
            if true {
                temp += 1;
            }
        }
        temp
    }

    // int Game::crewmates(void);
    pub fn crewmates(&self, obj: &entity::EntityClass) -> i32 {
        let mut temp = 0;
        // for (size_t i = 0; i < SDL_arraysize(obj.customcollect); i++) {
        for ob in obj.customcollect.iter() {
            // if (obj.collect[i])
            // TODO @sx @impl
            if true {
                temp += 1;
            }
        }
        temp
    }

    // bool Game::anything_unlocked(void)
    pub fn anything_unlocked (&self) -> bool {
        // for (size_t i = 0; i < SDL_arraysize(unlock); i++) {
        for i in 0..self.unlock.len() {
            if self.unlock[i] && (
                i == 8 || // Secret Lab
                (i >= 9 && i <= 14) || // any Time Trial
                i == 16 || // Intermission replays
                i == 17 || // No Death Mode
                i == 18 // Flip Mode
            ) {
                return true
            }
        }
        false
    }

    // bool Game::save_exists(void);
    pub fn save_exists (&self) -> bool {
        self.telesummary != "" || self.quicksummary != ""
    }

    // void Game::clearcustomlevelstats(void);
    // void Game::loadcustomlevelstats(void);
    pub fn loadcustomlevelstats(&self) {
        println!("DEADBEEF: Game::loadcustomlevelstats() method not implemented yet");
    }

    // void Game::savecustomlevelstats(void);
    // void Game::updatecustomlevelstats(std::string clevel, int cscore);

    // void Game::quittomenu(void);
    // void Game::returntolab(void);

    // #if !defined(NO_CUSTOM_LEVELS)
    // void Game::returntoeditor(void);
    // #endif

    // bool inline inspecial(void)

    // void Game::returntoingame(void)
    pub fn returntoingame(&mut self, graphics: &mut graphics::Graphics) {
        // TODO @sx @impl
        println!("DEADBEEF: Game::returntoingame is not implemented yet");

        self.ingame_titlemode = false;
        self.mapheld = true;
        // // #if !defined(NO_CUSTOM_LEVELS) && !defined(NO_EDITOR)
        if self.ingame_editormode {
            self.ingame_editormode = false;
            // DEFER_CALLBACK(returntoedsettings);
            // gamestate = EDITORMODE;
            // ed.settingskey = true;
        } else { // #endif
            // DEFER_CALLBACK(returntoingametemp);
            self.gamestate = GameState::MAPMODE;
            graphics.flipmode = graphics.setflipmode;
            // DEFER_CALLBACK(setfademode);
            // if !map.custommode && !graphics.flipmode {
            //     obj.flags[73] = true;
            // }
        }
        // DEFER_CALLBACK(nextbgcolor);
    }

    // void Game::unlockAchievement(const char *name);
    pub fn unlockAchievement(&mut self, name: &str) {
        println!("DEADBEEF: Game::unlockAchievement() method not implemented yet");
    }
}

/* 40 chars (160 bytes) covers the entire screen, + 1 more for null terminator */
pub const MENU_TEXT_BYTES: usize = 161;

pub struct MenuOption {
    // text: char[161], // 40 chars (160 bytes) covers the entire screen, + 1 more for null terminator
    pub text: String,
    // WARNING: should match Game::menutextbytes below
    pub active: bool,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum MenuName {
    mainmenu,
    playerworlds,
    levellist,
    quickloadlevel,
    youwannaquit,
    errornostart,
    errorsavingsettings,
    graphicoptions,
    ed_settings,
    ed_desc,
    ed_music,
    ed_quit,
    options,
    gameplayoptions,
    speedrunneroptions,
    advancedoptions,
    audiooptions,
    accessibility,
    controller,
    cleardatamenu,
    setinvincibility,
    setslowdown,
    unlockmenu,
    credits,
    credits2,
    credits25,
    credits3,
    credits4,
    credits5,
    credits6,
    play,
    unlocktimetrial,
    unlocktimetrials,
    unlocknodeathmode,
    unlockintermission,
    unlockflipmode,
    newgamewarning,
    playmodes,
    intermissionmenu,
    playint1,
    playint2,
    continuemenu,
    startnodeathmode,
    gameover,
    gameover2,
    unlockmenutrials,
    timetrials,
    nodeathmodecomplete,
    nodeathmodecomplete2,
    timetrialcomplete,
    timetrialcomplete2,
    timetrialcomplete3,
    gamecompletecontinue,
}

#[derive(PartialEq, Eq)]
pub enum SLIDERMODE {
    SLIDER_NONE,
    SLIDER_MUSICVOLUME,
    SLIDER_SOUNDVOLUME
}

struct MenuStackFrame {
    option: i32,
    name: MenuName,
}

struct CustomLevelStat {
    name: String,
    // 0 - not played, 1 - finished, 2 - all trinkets, 3 - finished, all trinkets
    score: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    GAMEMODE,
    TITLEMODE,
    CLICKTOSTART,
    FOCUSMODE,
    MAPMODE,
    TELEPORTERMODE,
    GAMECOMPLETE,
    GAMECOMPLETE2,
    EDITORMODE,
    PRELOADER,
}
