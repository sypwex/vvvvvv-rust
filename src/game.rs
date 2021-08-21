use sdl2::controller::Button;
use crate::{INBOUNDS_VEC, entity, filesystem, key_poll, map, maths, music, screen::{self, render::graphics}, script, utility_class, xml};

pub const numcrew: usize = 6;
const numunlock: usize = 25;
const numtrials: usize = 6;

pub struct Game {
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
    pub kludge_ingametemp: MenuName,
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
    customscript: Vec<String>, // String[50]
    customcol: i32,
    pub levelpage: i32,
    playcustomlevel: i32,
    pub customleveltitle: String,
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

    pub fadetomenu: bool,
    pub fadetomenudelay: i32,
    pub fadetolab: bool,
    pub fadetolabdelay: i32,

    // #if !defined(NO_CUSTOM_LEVELS)
    //   void returntoeditor(),
    //   shouldreturntoeditor: bool,
    // #endif

    gametimer: i32,

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
    pub fn new(graphics: &mut graphics::Graphics, music: &mut music::Music, screen_params: screen::ScreenParams, map: &mut map::Map, fs: &mut filesystem::FileSystem, screen_settings: screen::ScreenSettings) -> Game {
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
            customscript: vec![String::new(); 50],
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

        game.createmenu(MenuName::mainmenu, Some(false), graphics, music, screen_params, map, screen_settings, fs);

        game
    }

    // inline: bool inspecial()
    pub fn inspecial(&self) -> bool {
        self.inintermission || self.insecretlab || self.intimetrial || self.nodeathmode
    }

    // void Game::init(void);
    pub fn init(&mut self, music: &mut music::Music) {
        // @sx this code was previously located in main.cpp
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
    pub fn crewrescued(&self) -> usize {
        self.crewstats.iter().filter(|x| **x == true).collect::<Vec<_>>().len()
    }

    // std::string Game::unrescued(void);
    pub fn unrescued(&mut self) -> &str {
        //Randomly return the name of an unrescued crewmate
        if maths::fRandom() * 100.0 > 50.0 {
            if !self.crewstats[5] { return "Victoria"; }
            if !self.crewstats[2] { return "Vitellary"; }
            if !self.crewstats[4] { return "Verdigris"; }
            if !self.crewstats[3] { return "Vermilion"; }
        } else {
            if maths::fRandom() * 100.0 > 50.0 {
                if !self.crewstats[2] { return "Vitellary"; }
                if !self.crewstats[4] { return "Verdigris"; }
                if !self.crewstats[3] { return "Vermilion"; }
                if !self.crewstats[5] { return "Victoria"; }
            } else {
                if !self.crewstats[4] { return "Verdigris"; }
                if !self.crewstats[3] { return "Vermilion"; }
                if !self.crewstats[5] { return "Victoria"; }
                if !self.crewstats[2] { return "Vitellary"; }
            }
        }

        return "you";
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
        warn!("DEADBEEF: Game::customsavequick() method not implemented yet");
        false
    }

    // bool Game::savequick(void);
    pub fn savequick(&mut self) -> bool {
        warn!("DEADBEEF: Game::savequick() method not implemented yet");
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
    pub fn timestring(&mut self, help: &mut utility_class::UtilityClass) -> String {
        let tempstring = if self.hours > 0 {
            [help.String(self.hours), ":".to_string()].concat()
        } else {
            String::new()
        };
        [tempstring, help.twodigits(self.minutes).to_string(), ":".to_string(), help.twodigits(self.seconds).to_string()].concat()
    }

    // std::string Game::partimestring(void);
    pub fn partimestring(&self) -> &str {
        warn!("DEADBEEF: partimestring not implemented yet");

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
        warn!("DEADBEEF: resulttimestring not implemented yet");
        &""
    }

    // std::string Game::timetstring(int t);
    pub fn timetstring(&self, t: i32) -> &str {
        warn!("DEADBEEF: timetstring not implemented yet");
        &""
    }

    // void Game::returnmenu(void);
    pub fn returnmenu(&mut self, graphics: &mut graphics::Graphics, music: &mut music::Music, screen_params: screen::ScreenParams, map: &mut map::Map, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem) {
        match self.menustack.pop() {
            Some(frame) => {
                // Store this in case createmenu() removes the stack frame
                let previousoption = frame.option;

                self.createmenu(frame.name, Some(true), graphics, music, screen_params, map, screen_settings, fs);
                self.currentmenuoption = previousoption;

                // @sx: looks like don't need it
                // Remove the stackframe now, but createmenu() might have already gotten to it
                // if we were returning to the main menu
                // if !self.menustack.empty() {
                //     self.menustack.pop_back();
                // }
            },
            None => error!("Error: returning to previous menu frame on empty stack!"),
        }
    }

    // void Game::returntomenu(enum Menu::MenuName t);
    pub fn returntomenu(&mut self, t: MenuName) {
        warn!("DEADBEEF: Game::returntomenu not implemented yet");
    }

    // void Game::createmenu( enum Menu::MenuName t, bool samemenu/*= false*/ )
    pub fn createmenu(&mut self, t: MenuName, samemenu: Option<bool>, graphics: &mut graphics::Graphics, music: &mut music::Music, screen_params: screen::ScreenParams, map: &mut map::Map, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem) {
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
                        self.createmenu(MenuName::unlocktimetrial, Some(true), graphics, music, screen_params, map, screen_settings, fs);
                        self.savestatsandsettings(screen_settings, fs, music, map);
                    } else if temp > 1 {
                        self.createmenu(MenuName::unlocktimetrials, Some(true), graphics, music, screen_params, map, screen_settings, fs);
                        self.savestatsandsettings(screen_settings, fs, music, map);
                    }
                } else {
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
                        self.createmenu(MenuName::unlocknodeathmode, Some(true), graphics, music, screen_params, map, screen_settings, fs);
                        self.savestatsandsettings(screen_settings, fs, music, map);
                    }
                    //Alright then! Flip mode?
                    else if self.unlock[5] && !self.unlocknotify[18] {
                        self.unlock[18] = true;
                        self.unlocknotify[18] = true;
                        self.createmenu(MenuName::unlockflipmode, Some(true), graphics, music, screen_params, map, screen_settings, fs);
                        self.savestatsandsettings(screen_settings, fs, music, map);
                    }
                    //What about the intermission levels?
                    else if self.unlock[7] && !self.unlocknotify[16] {
                        self.unlock[16] = true;
                        self.unlocknotify[16] = true;
                        self.createmenu(MenuName::unlockintermission, Some(true), graphics, music, screen_params, map, screen_settings, fs);
                        self.savestatsandsettings(screen_settings, fs, music, map);
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



            v => warn!("DEADBEEF: Game::createmenu({:?}) is not implemented yet", v),
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
    pub fn gethardestroom(&mut self, map: &mut map::Map) {
        if self.currentroomdeaths > self.hardestroomdeaths {
            self.hardestroomdeaths = self.currentroomdeaths;
            self.hardestroom = map.roomname.to_owned();
            if map.roomname == "glitch" {
                if self.roomx == 42 && self.roomy == 51 {
                    self.hardestroom = "Rear Vindow".to_string();
                } else if self.roomx == 48 && self.roomy == 51 {
                    self.hardestroom = "On the Vaterfront".to_string();
                } else if self.roomx == 49 && self.roomy == 51 {
                    self.hardestroom = "The Untouchavles".to_string();
                }
            } else if map.roomname == "change" {
                if self.roomx == 45 && self.roomy == 51 { self.hardestroom = map.specialnames[3].to_string(); }
                if self.roomx == 46 && self.roomy == 51 { self.hardestroom = map.specialnames[4].to_string(); }
                if self.roomx == 47 && self.roomy == 51 { self.hardestroom = map.specialnames[5].to_string(); }
                if self.roomx == 50 && self.roomy == 53 { self.hardestroom = map.specialnames[6].to_string(); }
                if self.roomx == 50 && self.roomy == 54 { self.hardestroom = map.specialnames[7].to_string(); }
            } else if map.roomname == "" {
                self.hardestroom = "Dimension VVVVVV".to_string();
            }
        }
    }

    // void Game::levelcomplete_textbox(void);
    pub fn levelcomplete_textbox(&mut self, graphics: &mut graphics::Graphics) {
        graphics.createtextboxflipme("", -1, 12, 165, 165, 255);
        graphics.addline("                                   ");
        graphics.addline("");
        graphics.addline("");
        graphics.textboxcenterx();
    }

    // void Game::crewmate_textbox(const int r, const int g, const int b);
    pub fn crewmate_textbox(&mut self, r: i32, g: i32, b: i32, graphics: &mut graphics::Graphics) {
        graphics.createtextboxflipme("", -1, 64 + 8 + 16, r, g, b);
        graphics.addline("     You have rescued  ");
        graphics.addline("      a crew member!   ");
        graphics.addline("");
        graphics.textboxcenterx();
    }

    // void Game::remaining_textbox(void);
    pub fn remaining_textbox(&mut self, graphics: &mut graphics::Graphics) {
        let remaining = 6 - self.crewrescued();

        let string = if remaining == 1 {
            "  One remains  ".to_string()
        } else if remaining > 0 {
            format!("  {} remain  ", remaining)
        } else {
            "  All Crew Members Rescued!  ".to_string()
        };

        graphics.createtextboxflipme(&string, -1, 128 + 16, 174, 174, 174);
        graphics.textboxcenterx();
    }

    // void Game::actionprompt_textbox(void);
    pub fn actionprompt_textbox(&mut self, graphics: &mut graphics::Graphics) {
        graphics.createtextboxflipme(" Press ACTION to continue ", -1, 196, 164, 164, 255);
        graphics.textboxcenterx();
    }

    // void Game::savetele_textbox(void);
    pub fn savetele_textbox(&mut self, graphics: &mut graphics::Graphics, map: &mut map::Map, fs: &mut filesystem::FileSystem) {
        if self.inspecial() || map.custommode {
            return;
        }

        match self.savetele(map, fs) {
            true => {
                graphics.createtextboxflipme("    Game Saved    ", -1, 12, 174, 174, 174);
                graphics.textboxtimer(25);
            },
            false => {
                graphics.createtextboxflipme("  ERROR: Could not save game!  ", -1, 12, 255, 60, 60);
                graphics.textboxtimer(50);
            },
        }
    }

    // void Game::updatestate(void);
    pub fn updatestate(&mut self, graphics: &mut graphics::Graphics, script: &mut script::ScriptClass, obj: &mut entity::EntityClass, music: &mut music::Music, map: &mut map::Map, screen_params: screen::ScreenParams, help: &mut utility_class::UtilityClass, fs: &mut filesystem::FileSystem, screen_settings: screen::ScreenSettings) {
        self.statedelay -= 1;
        if self.statedelay <= 0 {
            self.statedelay = 0;
            self.glitchrunkludge = false;
        }

        if self.state > 1 {
            trace!("updatestate {}", self.state);
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
                9 => {
                    //Start SWN Minigame Mode B
                    obj.removetrigger(9);

                    self.swnmode = true;
                    self.swngame = 6;
                    self.swndelay = 150;
                    self.swntimer = 60 * 30;

                    //set the checkpoint in the middle of the screen
                    self.savepoint = 0;
                    self.savex = 148;
                    self.savey = 100;
                    self.savegc = 0;
                    self.saverx = self.roomx;
                    self.savery = self.roomy;
                    self.savedir = 0;

                    self.state = 0;
                },
                10 => {
                    //Start SWN Minigame Mode A
                    obj.removetrigger(10);

                    self.swnmode = true;
                    self.swngame = 4;
                    self.swndelay = 150;
                    self.swntimer = 60 * 30;

                    //set the checkpoint in the middle of the screen
                    self.savepoint = 0;
                    self.savex = 148;
                    self.savey = 100;
                    self.savegc = 0;
                    self.saverx = self.roomx;
                    self.savery = self.roomy;
                    self.savedir = 0;

                    self.state = 0;
                },
                11 => {
                    //Intermission 1 instructional textbox, depends on last saved
                    graphics.textboxremovefast();
                    graphics.createtextbox("   When you're NOT standing on   ", -1, 3, 174, 174, 174);
                    if graphics.flipmode {
                        if self.lastsaved == 2 {
                            graphics.addline("   the ceiling, Vitellary will");
                        } else if self.lastsaved == 3 {
                            graphics.addline("   the ceiling, Vermilion will");
                        } else if self.lastsaved == 4 {
                            graphics.addline("   the ceiling, Verdigris will");
                        } else if self.lastsaved == 5 {
                            graphics.addline("   the ceiling, Victoria will");
                        }
                    } else {
                        if self.lastsaved == 2 {
                            graphics.addline("    the floor, Vitellary will");
                        } else if self.lastsaved == 3 {
                            graphics.addline("    the floor, Vermilion will");
                        } else if self.lastsaved == 4 {
                            graphics.addline("    the floor, Verdigris will");
                        } else if self.lastsaved == 5 {
                            graphics.addline("    the floor, Victoria will");
                        }
                    }

                    graphics.addline("     stop and wait for you.");
                    graphics.textboxtimer(180);
                    self.state = 0;
                },
                12 => {
                    //Intermission 1 instructional textbox, depends on last saved
                    obj.removetrigger(12);
                    if !obj.flags[61] {
                        obj.flags[61] = true;
                        graphics.textboxremovefast();
                        graphics.createtextbox("  You can't continue to the next   ", -1, 8, 174, 174, 174);
                        if self.lastsaved == 5 {
                            graphics.addline("  room until she is safely across. ");
                        } else {
                            graphics.addline("  room until he is safely across.  ");
                        }
                        graphics.textboxtimer(120);
                    }
                    self.state = 0;
                },
                13 => {
                    //textbox removal
                    obj.removetrigger(13);
                    graphics.textboxremovefast();
                    self.state = 0;
                },
                14 => {
                    //Intermission 1 instructional textbox, depends on last saved
                    if graphics.flipmode {
                        graphics.createtextbox(" When you're standing on the ceiling, ", -1, 3, 174, 174, 174);
                    } else {
                        graphics.createtextbox(" When you're standing on the floor, ", -1, 3, 174, 174, 174);
                    }

                    if self.lastsaved == 2 {
                        graphics.addline(" Vitellary will try to walk to you. ");
                    } else if self.lastsaved == 3 {
                        graphics.addline(" Vermilion will try to walk to you. ");
                    } else if self.lastsaved == 4 {
                        graphics.addline(" Verdigris will try to walk to you. ");
                    } else if self.lastsaved == 5 {
                        graphics.addline(" Victoria will try to walk to you. ");
                    }
                    graphics.textboxtimer(280);

                    self.state = 0;
                },
                15 => {
                    //leaving the naughty corner
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 0;
                    }
                    self.state = 0;
                },
                16 => {
                    //entering the naughty corner
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) && obj.entities[i].tile == 0 {
                        obj.entities[i].tile = 144;
                        music.playef(2);
                    }
                    self.state = 0;
                },
                17 => {
                    //Arrow key tutorial
                    obj.removetrigger(17);
                    graphics.createtextbox(" If you prefer, you can press UP or ", -1, 195, 174, 174, 174);
                    graphics.addline("   DOWN instead of ACTION to flip.");
                    graphics.textboxtimer(100);
                    self.state = 0;
                },
                20 => {
                    if !obj.flags[1] {
                        obj.flags[1] = true;
                        self.state = 0;
                        graphics.textboxremove();
                    }
                    obj.removetrigger(20);
                },
                21 => {
                    if !obj.flags[2] {
                        obj.flags[2] = true;
                        self.state = 0;
                        graphics.textboxremove();
                    }
                    obj.removetrigger(21);
                },
                22 => {
                    if !obj.flags[3] {
                        graphics.textboxremovefast();
                        obj.flags[3] = true;
                        self.state = 0;
                        graphics.createtextbox("  Press ACTION to flip  ", -1, 25, 174, 174, 174);
                        graphics.textboxtimer(60);
                    }
                    obj.removetrigger(22);
                },
                30 => {
                    //Generic "run script"
                    if !obj.flags[4] {
                        obj.flags[4] = true;
                        self.startscript = true;
                        self.newscript = "firststeps".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(30);
                    self.state = 0;
                },

                31 => {
                    //state = 55;  statedelay = 50;
                    self.state = 0;
                    self.statedelay = 0;
                    if !obj.flags[6] {
                        obj.flags[6] = true;

                        obj.flags[5] = true;
                        self.startscript = true;
                        self.newscript = "communicationstation".to_string();
                        self.state = 0;
                        self.statedelay = 0;
                    }
                    obj.removetrigger(31);
                },
                32 => {
                    //Generic "run script"
                    if !obj.flags[7] {
                        obj.flags[7] = true;
                        self.startscript = true;
                        self.newscript = "teleporterback".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(32);
                    self.state = 0;
                },
                33 => {
                    //Generic "run script"
                    if !obj.flags[9] {
                        obj.flags[9] = true;
                        self.startscript = true;
                        self.newscript = "rescueblue".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(33);
                    self.state = 0;
                },
                34 => {
                    //Generic "run script"
                    if !obj.flags[10] {
                        obj.flags[10] = true;
                        self.startscript = true;
                        self.newscript = "rescueyellow".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(34);
                    self.state = 0;
                },
                35 => {
                    //Generic "run script"
                    if !obj.flags[11] {
                        obj.flags[11] = true;
                        self.startscript = true;
                        self.newscript = "rescuegreen".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(35);
                    self.state = 0;
                },
                36 => {
                    //Generic "run script"
                    if !obj.flags[8] {
                        obj.flags[8] = true;
                        self.startscript = true;
                        self.newscript = "rescuered".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(36);
                    self.state = 0;
                },

                37 => {
                    //Generic "run script"
                    if self.companion == 0 {
                        self.startscript = true;
                        self.newscript = "int2_yellow".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(37);
                    self.state = 0;
                },
                38 => {
                    //Generic "run script"
                    if self.companion == 0 {
                        self.startscript = true;
                        self.newscript = "int2_red".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(38);
                    self.state = 0;
                },
                39 => {
                    //Generic "run script"
                    if self.companion == 0 {
                        self.startscript = true;
                        self.newscript = "int2_green".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(39);
                    self.state = 0;
                },
                40 => {
                    //Generic "run script"
                    if self.companion == 0 {
                        self.startscript = true;
                        self.newscript = "int2_blue".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(40);
                    self.state = 0;
                },

                41 => {
                    //Generic "run script"
                    if !obj.flags[60] {
                        obj.flags[60] = true;
                        self.startscript = true;
                        if self.lastsaved == 2 {
                            self.newscript = "int1yellow_2".to_string();
                        } else if self.lastsaved == 3 {
                            self.newscript = "int1red_2".to_string();
                        } else if self.lastsaved == 4 {
                            self.newscript = "int1green_2".to_string();
                        } else if self.lastsaved == 5 {
                            self.newscript = "int1blue_2".to_string();
                        }
                        self.state = 0;
                    }
                    obj.removetrigger(41);
                    self.state = 0;
                },
                42 => {
                    //Generic "run script"
                    if !obj.flags[62] {
                        obj.flags[62] = true;
                        self.startscript = true;
                        if self.lastsaved == 2 {
                            self.newscript = "int1yellow_3".to_string();
                        } else if self.lastsaved == 3 {
                            self.newscript = "int1red_3".to_string();
                        } else if self.lastsaved == 4 {
                            self.newscript = "int1green_3".to_string();
                        } else if self.lastsaved == 5 {
                            self.newscript = "int1blue_3".to_string();
                        }
                        self.state = 0;
                    }
                    obj.removetrigger(42);
                    self.state = 0;
                },
                43 => {
                    //Generic "run script"
                    if !obj.flags[63] {
                        obj.flags[63] = true;
                        self.startscript = true;
                        if self.lastsaved == 2 {
                            self.newscript = "int1yellow_4".to_string();
                        } else if self.lastsaved == 3 {
                            self.newscript = "int1red_4".to_string();
                        } else if self.lastsaved == 4 {
                            self.newscript = "int1green_4".to_string();
                        } else if self.lastsaved == 5 {
                            self.newscript = "int1blue_4".to_string();
                        }
                        self.state = 0;
                    }
                    obj.removetrigger(43);
                    self.state = 0;
                },
                44 => {
                    //Generic "run script"
                    if !obj.flags[64] {
                        obj.flags[64] = true;
                        self.startscript = true;
                        if self.lastsaved == 2 {
                            self.newscript = "int1yellow_5".to_string();
                        } else if self.lastsaved == 3 {
                            self.newscript = "int1red_5".to_string();
                        } else if self.lastsaved == 4 {
                            self.newscript = "int1green_5".to_string();
                        } else if self.lastsaved == 5 {
                            self.newscript = "int1blue_5".to_string();
                        }
                        self.state = 0;
                    }
                    obj.removetrigger(44);
                    self.state = 0;
                },
                45 => {
                    //Generic "run script"
                    if !obj.flags[65] {
                        obj.flags[65] = true;
                        self.startscript = true;
                        if self.lastsaved == 2 {
                            self.newscript = "int1yellow_6".to_string();
                        } else if self.lastsaved == 3 {
                            self.newscript = "int1red_6".to_string();
                        } else if self.lastsaved == 4 {
                            self.newscript = "int1green_6".to_string();
                        } else if self.lastsaved == 5 {
                            self.newscript = "int1blue_6".to_string();
                        }
                        self.state = 0;
                    }
                    obj.removetrigger(45);
                    self.state = 0;
                },
                46 => {
                    //Generic "run script"
                    if !obj.flags[66] {
                        obj.flags[66] = true;
                        self.startscript = true;
                        if self.lastsaved == 2 {
                            self.newscript = "int1yellow_7".to_string();
                        } else if self.lastsaved == 3 {
                            self.newscript = "int1red_7".to_string();
                        } else if self.lastsaved == 4 {
                            self.newscript = "int1green_7".to_string();
                        } else if self.lastsaved == 5 {
                            self.newscript = "int1blue_7".to_string();
                        }
                        self.state = 0;
                    }
                    obj.removetrigger(46);
                    self.state = 0;
                },

                47 => {
                    //Generic "run script"
                    if !obj.flags[69] {
                        obj.flags[69] = true;
                        self.startscript = true;
                        self.newscript = "trenchwarfare".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(47);
                    self.state = 0;
                },
                48 => {
                    //Generic "run script"
                    if !obj.flags[70] {
                        obj.flags[70] = true;
                        self.startscript = true;
                        self.newscript = "trinketcollector".to_string();
                        self.state = 0;
                    }
                    obj.removetrigger(48);
                    self.state = 0;
                },
                49 => {
                    //Start final level music
                    if !obj.flags[71] {
                        obj.flags[71] = true;
                        music.niceplay(15, self);  //Final level remix
                        self.state = 0;
                    }
                    obj.removetrigger(49);
                    self.state = 0;
                },

                50 => {
                    music.playef(15);
                    graphics.createtextbox("Help! Can anyone hear", 35, 15, 255, 134, 255);
                    graphics.addline("this message?");
                    graphics.textboxtimer(60);
                    self.state += 1;
                    self.statedelay = 100;
                },
                51 => {
                    music.playef(15);
                    graphics.createtextbox("Verdigris? Are you out", 30, 12, 255, 134, 255);
                    graphics.addline("there? Are you ok?");
                    graphics.textboxtimer(60);
                    self.state += 1;
                    self.statedelay = 100;
                },
                52 => {
                    music.playef(15);
                    graphics.createtextbox("Please help us! We've crashed", 5, 22, 255, 134, 255);
                    graphics.addline("and need assistance!");
                    graphics.textboxtimer(60);
                    self.state += 1;
                    self.statedelay = 100;
                },
                53 => {
                    music.playef(15);
                    graphics.createtextbox("Hello? Anyone out there?", 40, 15, 255, 134, 255);
                    graphics.textboxtimer(60);
                    self.state += 1;
                    self.statedelay = 100;
                },
                54 => {
                    music.playef(15);
                    graphics.createtextbox("This is Doctor Violet from the", 5, 8, 255, 134, 255);
                    graphics.addline("D.S.S. Souleye! Please respond!");
                    graphics.textboxtimer(60);
                    self.state += 1;
                    self.statedelay = 100;
                },
                55 => {
                    music.playef(15);
                    graphics.createtextbox("Please... Anyone...", 45, 14, 255, 134, 255);
                    graphics.textboxtimer(60);
                    self.state += 1;
                    self.statedelay = 100;
                },
                56 => {
                    music.playef(15);
                    graphics.createtextbox("Please be alright, everyone...", 25, 18, 255, 134, 255);
                    graphics.textboxtimer(60);
                    self.state=50;
                    self.statedelay = 100;
                },


                80 => {
                    //Used to return to menu from the game
                    if graphics.fademode == 1 {
                        self.state += 1;
                    }
                },
                81 => {
                    self.quittomenu(graphics, map, script, music, obj, screen_params, screen_settings, fs);
                    music.play(6, map, self); //should be after quittomenu()
                    self.state = 0;
                },

                82 => {
                    //Time Trial Complete!
                    obj.removetrigger(82);
                    self.hascontrol = false;

                    self.timetrialresulttime = self.seconds + (self.minutes * 60) + (self.hours * 60 * 60);
                    self.timetrialresultframes = self.frames;
                    self.timetrialresulttrinkets = self.trinkets(obj);
                    self.timetrialresultshinytarget = self.timetrialshinytarget;
                    self.timetrialresultpar = self.timetrialpar;
                    self.timetrialresultdeaths = self.deathcounts;

                    self.timetrialrank = 0;
                    if self.timetrialresulttime <= self.timetrialpar { self.timetrialrank += 1; }
                    if self.trinkets(obj) >= self.timetrialshinytarget { self.timetrialrank += 1; }
                    if self.deathcounts == 0 { self.timetrialrank += 1; }

                    if self.timetrialresulttime < self.besttimes[self.timetriallevel as usize] || (
                        self.timetrialresulttime == self.besttimes[self.timetriallevel as usize] && self.timetrialresultframes < self.bestframes[self.timetriallevel as usize]
                    ) || self.besttimes[self.timetriallevel as usize] == -1 {
                        self.besttimes[self.timetriallevel as usize] = self.timetrialresulttime;
                        self.bestframes[self.timetriallevel as usize] = self.timetrialresultframes;
                    }
                    if self.timetrialresulttrinkets > self.besttrinkets[self.timetriallevel as usize] || self.besttrinkets[self.timetriallevel as usize] == -1 {
                        self.besttrinkets[self.timetriallevel as usize] = self.trinkets(obj);
                    }
                    if self.deathcounts < self.bestlives[self.timetriallevel as usize] || self.bestlives[self.timetriallevel as usize] == -1 {
                        self.bestlives[self.timetriallevel as usize] = self.deathcounts;
                    }
                    if self.timetrialrank > self.bestrank[self.timetriallevel as usize] || self.bestrank[self.timetriallevel as usize] == -1 {
                        self.bestrank[self.timetriallevel as usize] = self.timetrialrank;
                        if self.timetrialrank >= 3 {
                            match self.timetriallevel {
                                0 => self.unlockAchievement("vvvvvvtimetrial_station1_fixed"),
                                1 => self.unlockAchievement("vvvvvvtimetrial_lab_fixed"),
                                2 => self.unlockAchievement("vvvvvvtimetrial_tower_fixed"),
                                3 => self.unlockAchievement("vvvvvvtimetrial_station2_fixed"),
                                4 => self.unlockAchievement("vvvvvvtimetrial_warp_fixed"),
                                5 => self.unlockAchievement("vvvvvvtimetrial_final_fixed"),
                                _ => (),
                            };
                        }
                    }

                    self.savestatsandsettings(screen_settings, fs, music, map);

                    graphics.fademode = 2;
                    music.fadeout(None, self);
                    self.state += 1;
                },
                83 => {
                    self.frames -= 1;
                    if graphics.fademode == 1 { self.state += 1; }
                },
                84 => {
                    self.quittomenu(graphics, map, script, music, obj, screen_params, screen_settings, fs);
                    self.createmenu(MenuName::timetrialcomplete, None, graphics, music, screen_params, map, screen_settings, fs);
                    self.state = 0;
                },


                85 => {
                    //Cutscene skip version of final level change
                    obj.removetrigger(85);
                    //Init final stretch
                    self.state += 1;
                    music.playef(9);
                    music.play(2, map, self);
                    obj.flags[72] = true;

                    self.screenshake = 10;
                    self.flashlight = 5;
                    map.finalstretch = true;
                    map.warpx = false;
                    map.warpy = false;
                    map.background = 6;

                    map.final_colormode = true;
                    map.final_colorframe = 1;

                    self.state = 0;
                },

                //From 90-100 are run scripts for the eurogamer expo only, remove later
                90 => {
                    //Generic "run script"
                    self.startscript = true;
                    self.newscript = "startexpolevel_station1".to_string();
                    obj.removetrigger(90);
                    self.state = 0;
                },
                91 => {
                    //Generic "run script"
                    self.startscript = true;
                    self.newscript = "startexpolevel_lab".to_string();
                    obj.removetrigger(91);
                    self.state = 0;
                },
                92 => {
                    //Generic "run script"
                    self.startscript = true;
                    self.newscript = "startexpolevel_warp".to_string();
                    obj.removetrigger(92);
                    self.state = 0;
                },
                93 => {
                    //Generic "run script"
                    self.startscript = true;
                    self.newscript = "startexpolevel_tower".to_string();
                    obj.removetrigger(93);
                    self.state = 0;
                },
                94 => {
                    //Generic "run script"
                    self.startscript = true;
                    self.newscript = "startexpolevel_station2".to_string();
                    obj.removetrigger(94);
                    self.state = 0;
                },
                95 => {
                    //Generic "run script"
                    self.startscript = true;
                    self.newscript = "startexpolevel_final".to_string();
                    obj.removetrigger(95);
                    self.state = 0;
                },

                96 => {
                    //Used to return to gravitron to game
                    if graphics.fademode == 1 { self.state += 1; }
                },
                97 => {
                    self.returntolab(graphics, map, music, obj, help);
                    self.state = 0;
                },

                100 => {
                    //
                    //                       Meeting crewmate in the warpzone
                    //
                    obj.removetrigger(100);
                    if !obj.flags[4] {
                        obj.flags[4] = true;
                        self.state += 1;
                    }
                },
                101 => {
                    let i = obj.getplayer() as usize;
                    self.hascontrol = false;
                    if INBOUNDS_VEC!(i, obj.entities) && obj.entities[i].onroof > 0 && self.gravitycontrol == 1 {
                        self.gravitycontrol = 0;
                        music.playef(1);
                    }
                    if INBOUNDS_VEC!(i, obj.entities) && obj.entities[i].onground > 0 {
                        self.state += 1;
                    }
                },
                102 => {
                    self.companion = 6;
                    let i = obj.getcompanion() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 0;
                        obj.entities[i].state = 1;
                    }

                    self.advancetext = true;
                    self.hascontrol = false;

                    graphics.createtextbox("Captain! I've been so worried!", 60, 90, 164, 255, 164);
                    self.state += 1;
                    music.playef(12);
                },
                104 => {
                    graphics.createtextbox("I'm glad you're ok!", 135, 152, 164, 164, 255);
                    self.state += 1;
                    music.playef(11);
                    graphics.textboxactive();
                },
                106 => {
                    graphics.createtextbox("I've been trying to find a", 74, 70, 164, 255, 164);
                    graphics.addline("way out, but I keep going");
                    graphics.addline("around in circles...");
                    self.state += 1;
                    music.playef(2);
                    graphics.textboxactive();
                    let i = obj.getcompanion() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 54;
                        obj.entities[i].state = 0;
                    }
                },
                108 => {
                    graphics.createtextbox("Don't worry! I have a", 125, 152, 164, 164, 255);
                    graphics.addline("teleporter key!");
                    self.state += 1;
                    music.playef(11);
                    graphics.textboxactive();
                },
                110 => {
                    let i = obj.getcompanion() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 0;
                        obj.entities[i].state = 1;
                    }
                    graphics.createtextbox("Follow me!", 185, 154, 164, 164, 255);
                    self.state += 1;
                    music.playef(11);
                    graphics.textboxactive();
                },
                112 => {
                    graphics.textboxremove();
                    self.hascontrol = true;
                    self.advancetext = false;

                    self.state = 0;
                },

                115 => {
                    //
                    //                       Test script for space station, totally delete me!
                    //
                    self.hascontrol = false;
                    self.state += 1;
                },
                116 => {
                    self.advancetext = true;
                    self.hascontrol = false;

                    graphics.createtextbox("Sorry Eurogamers! Teleporting around", 60 - 20, 200, 255, 64, 64);
                    graphics.addline("the map doesn't work in this version!");
                    graphics.textboxcenterx();
                    self.state += 1;
                },
                118 => {
                    graphics.textboxremove();
                    self.hascontrol = true;
                    self.advancetext = false;

                    self.state = 0;
                },

                120 => {
                    //
                    //                       Meeting crewmate in the space station
                    //
                    obj.removetrigger(120);
                    if !obj.flags[5] {
                        obj.flags[5] = true;
                        self.state += 1;
                    }
                },
                121 => {
                    let i = obj.getplayer() as usize;
                    self.hascontrol = false;
                    if INBOUNDS_VEC!(i, obj.entities) && obj.entities[i].onground > 0 && self.gravitycontrol == 0 {
                        self.gravitycontrol = 1;
                        music.playef(1);
                    }
                    if INBOUNDS_VEC!(i, obj.entities) && obj.entities[i].onroof > 0 {
                        self.state += 1;
                    }
                },
                122 => {
                    self.companion = 7;
                    let i = obj.getcompanion() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 6;
                        obj.entities[i].state = 1;
                    }

                    self.advancetext = true;
                    self.hascontrol = false;

                    graphics.createtextbox("Captain! You're ok!", 60-10, 90-40, 255, 255, 134);
                    self.state += 1;
                    music.playef(14);
                },
                124 => {
                    graphics.createtextbox("I've found a teleporter, but", 60-20, 90 - 40, 255, 255, 134);
                    graphics.addline("I can't get it to go anywhere...");
                    self.state += 1;
                    music.playef(2);
                    graphics.textboxactive();
                },
                126 => {
                    graphics.createtextbox("I can help with that!", 125, 152-40, 164, 164, 255);
                    self.state += 1;
                    music.playef(11);
                    graphics.textboxactive();
                },
                128 => {
                    graphics.createtextbox("I have the teleporter", 130, 152-35, 164, 164, 255);
                    graphics.addline("codex for our ship!");
                    self.state += 1;
                    music.playef(11);
                    graphics.textboxactive();
                },

                130 => {
                    graphics.createtextbox("Yey! Let's go home!", 60-30, 90-35, 255, 255, 134);
                    self.state += 1;
                    music.playef(14);
                    graphics.textboxactive();
                    let i = obj.getcompanion() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 6;
                        obj.entities[i].state = 1;
                    }
                },
                132 => {
                    graphics.textboxremove();
                    self.hascontrol = true;
                    self.advancetext = false;

                    self.state = 0;
                },

                200 => {
                    //Init final stretch
                    self.state += 1;
                    music.playef(9);
                    obj.flags[72] = true;

                    self.screenshake = 10;
                    self.flashlight = 5;
                    map.finalstretch = true;
                    map.warpx = false;
                    map.warpy = false;
                    map.background = 6;

                    map.final_colormode = true;
                    map.final_colorframe = 1;

                    self.startscript = true;
                    self.newscript = "finalterminal_finish".to_string();
                    self.state = 0;
                },

                // WARNING: If updating this code, make sure to update Map.cpp mapclass::twoframedelayfix()
                300..=336 => {
                    self.startscript = true;
                    self.newscript = ["custom_", &self.customscript[self.state as usize - 300]].concat();
                    obj.removetrigger(self.state);
                    self.state = 0;
                },
                1000 => {
                    graphics.showcutscenebars = true;
                    self.hascontrol = false;
                    self.completestop = true;
                    self.state += 1;
                    self.statedelay = 15;
                },
                1001 => {
                    //Found a trinket!
                    self.advancetext = true;
                    self.state += 1;
                    graphics.createtextboxflipme("        Congratulations!       ", 50, 85, 174, 174, 174);
                    graphics.addline("");
                    graphics.addline("You have found a shiny trinket!");
                    graphics.textboxcenterx();

                    // #if !defined(NO_CUSTOM_LEVELS)
                    // if map.custommode {
                    //     graphics.createtextboxflipme(" " + help.number(trinkets()) + " out of " + help.number(ed.numtrinkets())+ " ", 50, 135, 174, 174, 174);
                    //     graphics.textboxcenterx();
                    // } else
                    // #endif
                    {
                        graphics.createtextboxflipme(&format!(" {} out of Twenty ", help.number(self.trinkets(obj))), 50, 135, 174, 174, 174);
                        graphics.textboxcenterx();
                    }
                },
                1002 => {
                    if !self.advancetext {
                        // Prevent softlocks if we somehow don't have advancetext
                        self.state += 1;
                    }
                },
                1003 => {
                    graphics.textboxremove();
                    self.hascontrol = true;
                    self.advancetext = false;
                    self.completestop = false;
                    self.state = 0;
                    if !self.muted && music.currentsong > -1 { music.fadeMusicVolumeIn(3000, self); }
                    graphics.showcutscenebars = false;
                },

                1010 => {
                    graphics.showcutscenebars = true;
                    self.hascontrol = false;
                    self.completestop = true;
                    self.state += 1;
                    self.statedelay = 15;
                },
                // // #if !defined(NO_CUSTOM_LEVELS)
                // 1011 => {
                //     //Found a crewmate!
                //     self.advancetext = true;
                //     self.state += 1;
                //     graphics.createtextboxflipme("        Congratulations!       ", 50, 85, 174, 174, 174);
                //     graphics.addline("");
                //     graphics.addline("You have found a lost crewmate!");
                //     graphics.textboxcenterx();
                //
                //     if ed.numcrewmates() - crewmates() == 0 {
                //         graphics.createtextboxflipme("     All crewmates rescued!    ", 50, 135, 174, 174, 174);
                //     } else if ed.numcrewmates()-crewmates() == 1 {
                //         graphics.createtextboxflipme("    " + help.number(ed.numcrewmates()-crewmates())+ " remains    ", 50, 135, 174, 174, 174);
                //     } else {
                //         graphics.createtextboxflipme("     " + help.number(ed.numcrewmates()-crewmates())+ " remain    ", 50, 135, 174, 174, 174);
                //     }
                //     graphics.textboxcenterx();
                // },
                // 1012 => {
                //     if !self.advancetext {
                //         // Prevent softlocks if we somehow don't have advancetext
                //         self.state += 1;
                //     }
                // },
                // 1013 => {
                //     graphics.textboxremove();
                //     self.hascontrol = true;
                //     self.advancetext = false;
                //     self.completestop = false;
                //     self.state = 0;
                //
                //     if ed.numcrewmates()-crewmates() == 0 {
                //         if map.custommodeforreal {
                //             graphics.fademode = 2;
                //             if !muted && ed.levmusic > 0 { music.fadeMusicVolumeIn(3000); }
                //             if ed.levmusic > 0 { music.fadeout(); }
                //             self.state = 1014;
                //         } else {
                //             returntoeditor();
                //             if !muted && ed.levmusic > 0 { music.fadeMusicVolumeIn(3000); }
                //             if ed.levmusic > 0 { music.fadeout(); }
                //         }
                //     } else {
                //         if !muted && ed.levmusic > 0 { music.fadeMusicVolumeIn(3000); }
                //     }
                //     graphics.showcutscenebars = false;
                // },
                // // #endif
                1014 => {
                    self.frames -= 1;
                    if graphics.fademode == 1 { self.state += 1; }
                },
                1015 => {
                    // // #if !defined(NO_CUSTOM_LEVELS)
                    // //Update level stats
                    // if ed.numcrewmates()-crewmates() == 0 {
                    //     //Finished level
                    //     if ed.numtrinkets()-trinkets() == 0 {
                    //         //and got all the trinkets!
                    //         updatecustomlevelstats(customlevelfilename, 3);
                    //     } else {
                    //         updatecustomlevelstats(customlevelfilename, 1);
                    //     }
                    // }
                    // // #endif
                    self.quittomenu(graphics, map, script, music, obj, screen_params, screen_settings, fs);
                    music.play(6, map, self); //should be after quittomenu()
                    self.state = 0;
                },

                2000 => {
                    //Game Saved!
                    self.savetele_textbox(graphics, map, fs);
                    self.state = 0;
                },

                2500 => {
                    music.play(5, map, self);
                    //Activating a teleporter (appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                2501 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    //we're done here!
                    music.playef(10);
                },
                2502 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;

                        let j = obj.getteleporter() as usize;
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                        }
                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = 6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = 6.0;
                    }

                    let i = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 1;
                        obj.entities[i].colour = 101;
                    }
                },
                2503 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                2504 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        //obj.entities[i].xp += 10;
                    }
                },
                2505 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 8;
                    }
                },
                2506 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 6;
                    }
                },
                2507 => {
                    self.state += 1;
                },
                2508 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 2;
                    }
                },
                2509 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 1;
                    }
                },
                2510 => {
                    self.advancetext = true;
                    self.hascontrol = false;
                    graphics.createtextbox("Hello?", 125+24, 152-20, 164, 164, 255);
                    self.state += 1;
                    music.playef(11);
                    graphics.textboxactive();
                },
                2512 => {
                    self.advancetext = true;
                    self.hascontrol = false;
                    graphics.createtextbox("Is anyone there?", 125+8, 152-24, 164, 164, 255);
                    self.state += 1;
                    music.playef(11);
                    graphics.textboxactive();
                },
                2514 => {
                    graphics.textboxremove();
                    self.hascontrol = true;
                    self.advancetext = false;

                    self.state = 0;
                    music.play(3, map, self);
                },


                3000 => {
                    //Activating a teleporter (long version for level complete)
                    self.state += 1;
                    self.statedelay = 30;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                3001 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    music.playef(9);
                },
                3002 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    music.playef(9);
                },
                3003 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    music.playef(9);
                },
                3004 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    //we're done here!
                    music.playef(10);
                },
                3005 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 50;
                    match self.companion {
                        6 => self.state = 3006, //Warp Zone
                        7 => self.state = 3020, //Space Station
                        8 => self.state = 3040, //Lab
                        9 => self.state = 3060, //Tower
                        10 => self.state = 3080, //Intermission 2
                        11 => self.state = 3085, //Intermission 1
                        _ => (),
                    }

                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = true;
                    }

                    let i = obj.getcompanion() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.disableentity(i);
                    }

                    let i = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 1;
                        obj.entities[i].colour = 100;
                    }
                },

                3006 => {
                    //Level complete! (warp zone)
                    self.unlocknum(4);
                    self.lastsaved = 4;
                    music.play(0, map, self);
                    self.state += 1;
                    self.statedelay = 75;

                    self.levelcomplete_textbox(graphics);
                },
                3007 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.crewmate_textbox(175, 174, 174, graphics);
                },
                3008 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.remaining_textbox(graphics);
                },
                3009 => {
                    self.state += 1;
                    self.statedelay = 0;

                    self.actionprompt_textbox(graphics);
                },
                3010 => {
                    if self.jumppressed != 0 {
                        self.state += 1;
                        self.statedelay = 30;
                        graphics.textboxremove();
                    }
                },
                3011 => {
                    self.state = 3070;
                    self.statedelay = 0;
                },

                3020 => {
                    //Level complete! (Space Station 2)
                    self.unlocknum(3);
                    self.lastsaved = 2;
                    music.play(0, map, self);
                    self.state += 1;
                    self.statedelay = 75;

                    self.levelcomplete_textbox(graphics);
                },
                3021 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.crewmate_textbox(174, 175, 174, graphics);
                },
                3022 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.remaining_textbox(graphics);
                },
                3023 => {
                    self.state += 1;
                    self.statedelay = 0;

                    self.actionprompt_textbox(graphics);
                },
                3024 => {
                    if self.jumppressed != 0{
                        self.state += 1;
                        self.statedelay = 30;
                        graphics.textboxremove();
                    }
                },
                3025 => {
                    self.state = 3070;
                    self.statedelay = 0;
                },

                3040 => {
                    //Level complete! (Lab)
                    self.unlocknum(1);
                    self.lastsaved = 5;
                    music.play(0, map, self);
                    self.state += 1;
                    self.statedelay = 75;

                    self.levelcomplete_textbox(graphics);
                },
                3041 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.crewmate_textbox(174, 174, 175, graphics);
                },
                3042 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.remaining_textbox(graphics);
                },
                3043 => {
                    self.state += 1;
                    self.statedelay = 0;

                    self.actionprompt_textbox(graphics);
                },
                3044 => {
                    if self.jumppressed != 0 {
                        self.state += 1;
                        self.statedelay = 30;
                        graphics.textboxremove();
                    }
                },
                3045 => {
                    self.state = 3070;
                    self.statedelay = 0;
                },

                3050 => {
                    //Level complete! (Space Station 1)
                    self.unlocknum(0);
                    self.lastsaved = 1;
                    music.play(0, map, self);
                    self.state += 1;
                    self.statedelay = 75;

                    self.levelcomplete_textbox(graphics);
                },
                3051 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.crewmate_textbox(175, 175, 174, graphics);
                },
                3052 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.remaining_textbox(graphics);
                },
                3053 => {
                    self.state += 1;
                    self.statedelay = 0;

                    self.actionprompt_textbox(graphics);
                },
                3054 => {
                    if self.jumppressed != 0 {
                        self.state += 1;
                        self.statedelay = 30;
                        graphics.textboxremove();
                        self.teleportscript = "".to_string();
                    }
                },
                3055 => {
                    graphics.fademode = 2;
                    self.state += 1;
                    self.statedelay = 10;
                },
                3056 => {
                    if graphics.fademode == 1 {
                        self.startscript = true;
                        self.newscript = match (self.crewrescued() == 6, self.nocutscenes) {
                            (true, _) => "startlevel_final".to_string(),
                            (false, true) => "bigopenworldskip".to_string(),
                            (_, _) => "bigopenworld".to_string(),
                        };
                        self.state = 0;
                    }
                },

                3060 => {
                    //Level complete! (Tower)
                    self.unlocknum(2);
                    self.lastsaved = 3;
                    music.play(0, map, self);
                    self.state += 1;
                    self.statedelay = 75;

                    self.levelcomplete_textbox(graphics);
                },
                3061 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.crewmate_textbox(175, 174, 175, graphics);
                },
                3062 => {
                    self.state += 1;
                    self.statedelay = 45;

                    self.remaining_textbox(graphics);
                },
                3063 => {
                    self.state += 1;
                    self.statedelay = 0;

                    self.actionprompt_textbox(graphics);
                },
                3064 => {
                    if self.jumppressed != 0 {
                        self.state += 1;
                        self.statedelay = 30;
                        graphics.textboxremove();
                    }
                },
                3065 => {
                    self.state = 3070;
                    self.statedelay = 0;
                },

                3070 => {
                    graphics.fademode = 2;
                    self.state += 1;
                },
                3071 => {
                    if graphics.fademode == 1 { self.state += 1; }
                },
                3072 => {
                    //Ok, we need to adjust some flags based on who've we've rescued. Some of there conversation options
                    //change depending on when they get back to the ship.
                    if self.lastsaved == 2 {
                        if self.crewstats[3] { obj.flags[25] = true; }
                        if self.crewstats[4] { obj.flags[26] = true; }
                        if self.crewstats[5] { obj.flags[24] = true; }
                    } else if self.lastsaved == 3 {
                        if self.crewstats[2] { obj.flags[50] = true; }
                        if self.crewstats[4] { obj.flags[49] = true; }
                        if self.crewstats[5] { obj.flags[48] = true; }
                    } else if self.lastsaved == 4 {
                        if self.crewstats[2] { obj.flags[54] = true; }
                        if self.crewstats[3] { obj.flags[55] = true; }
                        if self.crewstats[5] { obj.flags[56] = true; }
                    } else if self.lastsaved == 5 {
                        if self.crewstats[2] { obj.flags[37] = true; }
                        if self.crewstats[3] { obj.flags[38] = true; }
                        if self.crewstats[4] { obj.flags[39] = true; }
                    }
                    //We're pitch black now, make a decision
                    self.companion = 0;
                    if self.crewrescued() == 6 {
                        self.startscript = true;
                        self.newscript = "startlevel_final".to_string();
                        self.state = 0;
                    } else if self.crewrescued() == 4 {
                        self.companion = 11;
                        self.supercrewmate = true;
                        self.scmprogress = 0;

                        self.startscript = true;
                        self.newscript = "intermission_1".to_string();
                        obj.flags[19] = true;
                        if self.lastsaved == 2 { obj.flags[32] = true; }
                        if self.lastsaved == 3 { obj.flags[35] = true; }
                        if self.lastsaved == 4 { obj.flags[34] = true; }
                        if self.lastsaved == 5 { obj.flags[33] = true; }
                        self.state = 0;
                    } else if self.crewrescued() == 5 {
                        self.startscript = true;
                        self.newscript = "intermission_2".to_string();
                        obj.flags[20] = true;
                        if self.lastsaved == 2 { obj.flags[32] = true; }
                        if self.lastsaved == 3 { obj.flags[35] = true; }
                        if self.lastsaved == 4 { obj.flags[34] = true; }
                        if self.lastsaved == 5 { obj.flags[33] = true; }
                        self.state = 0;
                    } else {
                        self.startscript = true;
                        self.newscript = "regularreturn".to_string();
                        self.state = 0;
                    }
                },

                3080 => {
                    //returning from an intermission, very like 3070
                    if self.inintermission {
                        graphics.fademode = 2;
                        self.companion = 0;
                        self.state=3100;
                    } else {
                        self.unlocknum(7);
                        graphics.fademode = 2;
                        self.companion = 0;
                        self.state += 1;
                    }
                },
                3081 => {
                    if graphics.fademode == 1 { self.state += 1; }
                },
                3082 => {
                    map.finalmode = false;
                    self.startscript = true;
                    self.newscript = "regularreturn".to_string();
                    self.state = 0;
                },

                3085 => {
                    //returning from an intermission, very like 3070
                    //return to menu from here
                    if self.inintermission {
                        self.companion = 0;
                        self.supercrewmate = false;
                        self.state += 1;
                        graphics.fademode = 2;
                        music.fadeout(None, self);
                        self.state = 3100;
                    } else {
                        self.unlocknum(6);
                        graphics.fademode = 2;
                        self.companion = 0;
                        self.supercrewmate = false;
                        self.state += 1;
                    }
                },
                3086 => {
                    if graphics.fademode == 1 { self.state += 1; }
                },
                3087 => {
                    map.finalmode = false;
                    self.startscript = true;
                    self.newscript = "regularreturn".to_string();
                    self.state = 0;
                },

                3100 => {
                    if graphics.fademode == 1 { self.state += 1; }
                },
                3101 => {
                    self.quittomenu(graphics, map, script, music, obj, screen_params, screen_settings, fs);
                    music.play(6, map, self); //should be after quittomenu();
                    self.state = 0;
                },

                3500 => {
                    music.fadeout(None, self);
                    self.state += 1;
                    self.statedelay = 120;
                },
                3501 => {
                    //Game complete!
                    self.unlockAchievement("vvvvvvgamecomplete");
                    self.unlocknum(5);
                    self.crewstats[0] = true;
                    self.state += 1;
                    self.statedelay = 75;
                    music.play(7, map, self);

                    graphics.createtextboxflipme("", -1, 12, 164, 165, 255);
                    graphics.addline("                                   ");
                    graphics.addline("");
                    graphics.addline("");
                    graphics.textboxcenterx();
                },
                3502 => {
                    self.state += 1;
                    self.statedelay = 45+15;

                    graphics.createtextboxflipme("  All Crew Members Rescued!  ", -1, 64, 0, 0, 0);
                    self.savetime = [self.timestring(help), ".".to_string(), help.twodigits(self.frames * 100 / 30)].concat();
                },
                3503 => {
                    self.state += 1;
                    self.statedelay = 45;

                    let tempstring = help.number(self.trinkets(obj));
                    graphics.createtextboxflipme("Trinkets Found:", 48, 84, 0,0,0);
                    graphics.createtextboxflipme(&tempstring, 180, 84, 0, 0, 0);
                },
                3504 => {
                    self.state += 1;
                    self.statedelay = 45+15;

                    graphics.createtextboxflipme("   Game Time:", 64, 96, 0,0,0);
                    graphics.createtextboxflipme(&self.savetime, 180, 96, 0, 0, 0);
                },
                3505 => {
                    self.state += 1;
                    self.statedelay = 45;

                    graphics.createtextboxflipme(" Total Flips:", 64, 123, 0,0,0);
                    graphics.createtextboxflipme(&self.totalflips.to_string(), 180, 123, 0, 0, 0);
                },
                3506 => {
                    self.state += 1;
                    self.statedelay = 45+15;

                    graphics.createtextboxflipme("Total Deaths:", 64, 135, 0,0,0);
                    graphics.createtextboxflipme(&self.deathcounts.to_string(), 180, 135, 0, 0, 0);
                },
                3507 => {
                    self.state += 1;
                    self.statedelay = 45+15;

                    let tempstring = format!("Hardest Room (with {} deaths)", self.hardestroomdeaths);
                    graphics.createtextboxflipme(&tempstring, -1, 158, 0,0,0);
                    graphics.createtextboxflipme(&self.hardestroom, -1, 170, 0, 0, 0);
                },
                3508 => {
                    self.state += 1;
                    self.statedelay = 0;

                    self.actionprompt_textbox(graphics);
                },
                3509 => {
                    if self.jumppressed != 0 {
                        self.state += 1;
                        self.statedelay = 30;
                        graphics.textboxremove();
                    }
                },
                3510 => {
                    //Save stats and stuff here
                    if !obj.flags[73] {
                        //flip mode complete
                        self.unlockAchievement("vvvvvvgamecompleteflip");
                        self.unlocknum(19);
                    }

                    if self.bestgamedeaths == -1 {
                        self.bestgamedeaths = self.deathcounts;
                    } else {
                        if self.deathcounts < self.bestgamedeaths {
                            self.bestgamedeaths = self.deathcounts;
                        }
                    }

                    if self.bestgamedeaths > -1 {
                        if self.bestgamedeaths <= 500 {
                            self.unlockAchievement("vvvvvvcomplete500");
                        }
                        if self.bestgamedeaths <= 250 {
                            self.unlockAchievement("vvvvvvcomplete250");
                        }
                        if self.bestgamedeaths <= 100 {
                            self.unlockAchievement("vvvvvvcomplete100");
                        }
                        if self.bestgamedeaths <= 50 {
                            self.unlockAchievement("vvvvvvcomplete50");
                        }
                    }

                    self.savestatsandsettings(screen_settings, fs, music, map);
                    if self.nodeathmode {
                        self.unlockAchievement("vvvvvvmaster"); //bloody hell
                        self.unlocknum(20);
                        self.state = 3520;
                        self.statedelay = 0;
                    } else {
                        self.statedelay = 120;
                        self.state += 1;
                    }
                },
                3511 => {
                    //Activating a teleporter (long version for level complete)
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].colour = 102;
                    }

                    self.state += 1;
                    self.statedelay = 30;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                3512 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    music.playef(9);
                },
                3513 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    music.playef(9);
                },
                3514 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    music.playef(9);
                },
                3515 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;

                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = true;
                    }

                    //we're done here!
                    music.playef(10);
                    self.statedelay = 60;
                },
                3516 => {
                    graphics.fademode = 2;
                    self.state += 1;
                },
                3517 => {
                    if graphics.fademode == 1 {
                        self.state += 1;
                        self.statedelay = 30;
                    }
                },
                3518 => {
                    graphics.fademode = 4;
                    self.state = 0;
                    self.statedelay = 30;

                    map.finalmode = false;
                    map.final_colormode = false;
                    map.final_mapcol = 0;
                    map.final_colorframe = 0;
                    map.finalstretch = false;

                    graphics.setbars(320);

                    self.teleport_to_new_area = true;
                    self.teleportscript = "gamecomplete".to_string();
                },

                3520 => {
                    //NO DEATH MODE COMPLETE JESUS
                    self.hascontrol = false;
                    self.crewstats[0] = true;

                    graphics.fademode = 2;
                    self.state += 1;
                },
                3521 => {
                    if graphics.fademode == 1 { self.state += 1; }
                },
                3522 => {
                    self.copyndmresults();
                    self.quittomenu(graphics, map, script, music, obj, screen_params, screen_settings, fs);
                    self.createmenu(MenuName::nodeathmodecomplete, None, graphics, music, screen_params, map, screen_settings, fs);
                    self.state = 0;
                },

                4000 => {
                    //Activating a teleporter (short version)
                    self.state += 1;
                    self.statedelay = 10;
                    self.flashlight = 5;
                    self.screenshake = 10;
                    music.playef(9);
                },
                4001 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    //we're done here!
                    music.playef(10);
                },
                4002 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 10;

                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = true;
                    }

                    let i = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 1;
                        obj.entities[i].colour = 100;
                    }
                },
                4003 => {
                    self.state = 0;
                    self.statedelay = 0;
                    self.teleport_to_new_area = true;
                },

                4010 => {
                    //Activating a teleporter (default appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4011 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4012 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 1;

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = 6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = 6.0;
                    }
                },
                4013 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4014 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4015 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 8;
                    }
                },
                4016 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 6;
                    }
                },
                4017 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 3;
                    }
                },
                4018 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 1;
                    }
                },
                4019 => {
                    if self.intimetrial || self.nodeathmode || self.inintermission {
                    } else {
                        self.savetele(map, fs);
                    }
                    let i = obj.getteleporter() as usize;
                    self.activetele = true;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        self.teleblock.x = obj.entities[i].xp - 32;
                        self.teleblock.y = obj.entities[i].yp - 32;
                    }
                    self.teleblock.w = 160;
                    self.teleblock.h = 160;
                    self.hascontrol = true;
                    self.advancetext = false;
                    self.state = 0;
                },

                4020 => {
                    //Activating a teleporter (default appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4021 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4022 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 1;

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = 6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = 6.0;
                    }
                },
                4023 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 12;
                    }
                },
                4024 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 12;
                    }
                },
                4025 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4026 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 8;
                    }
                },
                4027 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 5;
                    }
                },
                4028 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 2;
                    }
                },
                4029 => {
                    self.hascontrol = true;
                    self.advancetext = false;
                    self.state = 0;
                },

                4030 => {
                    //Activating a teleporter (default appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4031 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4032 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 0;

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = -6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = -6.0;
                    }
                },
                4033 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 12;
                    }
                },
                4034 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 12;
                    }
                },
                4035 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 10;
                    }
                },
                4036 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 8;
                    }
                },
                4037 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 5;
                    }
                },
                4038 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 2;
                    }
                },
                4039 => {
                    self.hascontrol = true;
                    self.advancetext = false;
                    self.state = 0;
                },

                4040 => {
                    //Activating a teleporter (default appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4041 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4042 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 1;

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = 6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = 6.0;
                    }
                },
                4043 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 12;
                        obj.entities[i].yp -= 15;
                    }
                },
                4044 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 12;
                        obj.entities[i].yp -= 10;
                    }
                },
                4045 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 12;
                        obj.entities[i].yp -= 10;
                    }
                },
                4046 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 8;
                        obj.entities[i].yp -= 8;
                    }
                },
                4047 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 6;
                        obj.entities[i].yp -= 8;
                    }
                },
                4048 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 3;
                    }
                },
                4049 => {
                    self.hascontrol = true;
                    self.advancetext = false;
                    self.state = 0;
                },

                4050 => {
                    //Activating a teleporter (default appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4051 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4052 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 1;

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = 6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = 6.0;
                    }
                },
                4053 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 4;
                        obj.entities[i].yp -= 15;
                    }
                },
                4054 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 4;
                        obj.entities[i].yp -= 10;
                    }
                },
                4055 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 4;
                        obj.entities[i].yp -= 10;
                    }
                },
                4056 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 4;
                        obj.entities[i].yp -= 8;
                    }
                },
                4057 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 2;
                        obj.entities[i].yp -= 8;
                    }
                },
                4058 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 1;
                    }
                },
                4059 => {
                    self.hascontrol = true;
                    self.advancetext = false;
                    self.state = 0;
                },

                4060 => {
                    //Activating a teleporter (default appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4061 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4062 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 0;

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = -6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = -6.0;
                    }
                },
                4063 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 28;
                        obj.entities[i].yp -= 8;
                    }
                },
                4064 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 28;
                        obj.entities[i].yp -= 8;
                    }
                },
                4065 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 25;
                    }
                },
                4066 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 25;
                    }
                },
                4067 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 20;
                    }
                },
                4068 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp -= 16;
                    }
                },
                4069 => {
                    self.hascontrol = true;
                    self.advancetext = false;
                    self.state = 0;
                },


                4070 => {
                    //Activating a teleporter (special for final script, player has colour changed to match rescued crewmate)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4071 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4072 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 1;
                        obj.entities[i].colour = obj.crewcolour(self.lastsaved);

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = 6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = 6.0;
                    }
                },
                4073 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4074 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4075 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 8;
                    }
                },
                4076 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 6;
                    }
                },
                4077 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 3;
                    }
                },
                4078 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 1;
                    }
                },
                4079 => {
                    self.state = 0;
                    self.startscript = true;
                    self.newscript = "finallevel_teleporter".to_string();
                },

                4080 => {
                    //Activating a teleporter (default appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4081 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4082 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 1;

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = 6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = 6.0;
                    }
                },
                4083 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4084 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4085 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 8;
                    }
                },
                4086 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 6;
                    }
                },
                4087 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 3;
                    }
                },
                4088 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 1;
                    }
                },
                4089 => {
                    self.startscript = true;
                    self.newscript = "gamecomplete_ending".to_string();
                    self.state = 0;
                },

                4090 => {
                    //Activating a teleporter (default appear)
                    self.state += 1;
                    self.statedelay = 15;
                    self.flashlight = 5;
                    self.screenshake = 90;
                    music.playef(9);
                },
                4091 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 0;
                    self.flashlight = 5;
                    self.screenshake = 0;
                    music.playef(10);
                },
                4092 => {
                    //Activating a teleporter 2
                    self.state += 1;
                    self.statedelay = 5;

                    let i = obj.getplayer() as usize;
                    let j = obj.getteleporter() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[i].xp = obj.entities[j].xp + 44;
                            obj.entities[i].yp = obj.entities[j].yp + 44;
                            obj.entities[i].lerpoldxp = obj.entities[i].xp;
                            obj.entities[i].lerpoldyp = obj.entities[i].yp;
                            obj.entities[j].tile = 2;
                            obj.entities[j].colour = 101;
                        }
                        obj.entities[i].colour = 0;
                        obj.entities[i].invis = false;
                        obj.entities[i].dir = 1;

                        obj.entities[i].ay = -6.0;
                        obj.entities[i].ax = 6.0;
                        obj.entities[i].vy = -6.0;
                        obj.entities[i].vx = 6.0;
                    }
                },
                4093 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4094 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 10;
                    }
                },
                4095 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 8;
                    }
                },
                4096 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 6;
                    }
                },
                4097 => {
                    self.state += 1;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 3;
                    }
                },
                4098 => {
                    self.state += 1;
                    self.statedelay = 15;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp += 1;
                    }
                },
                4099 => {
                    if self.nocutscenes {
                        self.startscript = true;
                        self.newscript = "levelonecompleteskip".to_string();
                    } else {
                        self.startscript = true;
                        self.newscript = "levelonecomplete_ending".to_string();
                    }
                    self.state = 0;
                },

                _ => warn!("DEADBEEF: Game::updatestate(): state {} not implemented yet", self.state),
            }
        }
    }

    // void Game::unlocknum(int t);
    pub fn unlocknum(&mut self, t: i32) {
        warn!("DEADBEEF: Game::unlocknum() not implemented yet");
    }

    // void Game::loadstats(ScreenSettings* screen_settings);
    pub fn loadstats(&mut self, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem, music: &mut music::Music, map: &mut map::Map) {
        trace!("loading stats");
        let doc = fs.FILESYSTEM_loadTiXml2Document("unlock.vvv");

        match doc {
            Ok(mut reader) => {
                reader.trim_text(true);
                let mut buf = Vec::new();

                let mut tag: String = String::new();
                loop {
                    match reader.read_event(&mut buf) {
                        Ok(quick_xml::events::Event::Start(ref e)) => {
                            match String::from_utf8(e.name().to_vec()) {
                                Ok(v) => tag = v,
                                Err(_) => tag = String::new(),
                            }
                        },
                        Ok(quick_xml::events::Event::Text(text)) => {
                            if tag.len() == 0 {
                                continue;
                            }

                            match text.unescape_and_decode(&reader) {
                                Ok(text) => {
                                    let v = text.as_str();
                                    match tag.as_str() {
                                        // "unlock"
                                        // "unlocknotify"
                                        // "besttimes"
                                        // "bestframes"
                                        // "besttrinkets"
                                        // "bestlives"
                                        // "bestrank"
                                        // "bestgamedeaths"
                                        // "stat_trinkets"
                                        // "swnbestrank"
                                        // "swnrecord"
                                        // "fullscreen"
                                        // "stretch"
                                        // "useLinearFilter"
                                        // "window_width"
                                        // "window_height"
                                        // "noflashingmode"
                                        // "colourblindmode"
                                        // "setflipmode"
                                        "invincibility" => {
                                            match i32::from_str_radix(v, 10) {
                                                Ok(v) => {
                                                    map.invincibility = v != 0;
                                                },
                                                Err(s) => error!("error while parsing invincibility value: {}", s),
                                            };

                                        }
                                        // "slowdown"
                                        // "advanced_smoothing"
                                        // "usingmmmmmm"
                                        // "ghostsenabled"
                                        "skipfakeload" => {
                                            match i32::from_str_radix(v, 10) {
                                                Ok(v) => {
                                                    self.skipfakeload = v != 0;
                                                },
                                                Err(s) => error!("error while parsing musicvolume value: {}", s),
                                            };
                                        },
                                        // "disablepause"
                                        // "notextoutline"
                                        // "translucentroomname"
                                        // "over30mode"
                                        // "inputdelay"
                                        // "glitchrunnermode"
                                        // "vsync"
                                        "musicvolume" => {
                                            match i32::from_str_radix(v, 10) {
                                                Ok(v) => music.user_music_volume = Box::new(v),
                                                Err(s) => error!("error while parsing musicvolume value: {}", s),
                                            };
                                        },
                                        "soundvolume" => {
                                            match i32::from_str_radix(v, 10) {
                                                Ok(v) => music.user_sound_volume = Box::new(v),
                                                Err(s) => error!("error while parsing soundvolume value: {}", s),
                                            };
                                        },
                                        // "controllerSensitivity"
                                        // "flipButton"
                                        // "enterButton"
                                        // "escButton"
                                        // "restartButton"
                                        _ => info!("parsing {:?} tag not implemented", tag),
                                    }

                                },
                                Err(_) => continue,
                            }
                        },
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        Ok(quick_xml::events::Event::Eof) => break,
                        _ => (),
                    }
                    buf.clear();
                }

                // let pElem = hDoc.FirstChildElement().ToElement();
                // // should always have a valid root but handle gracefully if it does
                // // save this for later
                // let hRoot = tinyxml2::XMLHandle(pElem);
                //
                // tinyxml2::XMLElement* dataNode = hRoot.FirstChildElement("Data").FirstChild().ToElement();
                //
                // for pElem = dataNode; pElem; pElem=pElem.NextSiblingElement() {
                //     const char* pKey = pElem.Value();
                //     const char* pText = pElem.GetText() ;
                //
                //     if pText == NULL {
                //         pText = "";
                //     }
                //
                //     LOAD_ARRAY(unlock)
                //     LOAD_ARRAY(unlocknotify)
                //     LOAD_ARRAY(besttimes)
                //     LOAD_ARRAY(bestframes)
                //     LOAD_ARRAY(besttrinkets)
                //     LOAD_ARRAY(bestlives)
                //     LOAD_ARRAY(bestrank)
                //
                //     if SDL_strcmp(pKey, "bestgamedeaths") == 0 {
                //         bestgamedeaths = help.Int(pText);
                //     }
                //
                //     if SDL_strcmp(pKey, "stat_trinkets") == 0 {
                //         stat_trinkets = help.Int(pText);
                //     }
                //
                //     if SDL_strcmp(pKey, "swnbestrank") == 0 {
                //         swnbestrank = help.Int(pText);
                //     }
                //
                //     if SDL_strcmp(pKey, "swnrecord") == 0 {
                //         swnrecord = help.Int(pText);
                //     }
                // }

                // self.deserializesettings(dataNode, screen_settings);
            },
            Err(xml_err) => {
                match xml_err {
                    quick_xml::Error::Io(io_err) => {
                        match io_err.kind() {
                            std::io::ErrorKind::NotFound => {
                                trace!("{:?}", io_err);
                                info!("No Stats found. Assuming a new player");

                                // Save unlock.vvv only. Maybe we have a settings.vvv laying around too,
                                // and we don't want to overwrite that!
                                self.savestats(screen_settings, fs, music, map);
                            },
                            _ => panic!(io_err),
                        }
                    },
                    _ => panic!(xml_err), // @sx probably this should not happen
                }
            },
        }

    }

    // bool Game::savestats(const ScreenSettings* screen_settings);
    // bool Game::savestats(void);
    pub fn savestats(&mut self, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem, music: &mut music::Music, map: &mut map::Map) -> bool {
        trace!("savestats");
        if fs.savefile_exists("unlock.vvv") {
            info!("No unlock.vvv found. Creating new file");
        }

        let mut wrapper = xml::xml::new();
        wrapper.update_declaration();
        wrapper.write_start_tag("Save");
        wrapper.update_comment(" Save file ");
        wrapper.write_start_tag("Data");

        // let s_unlock = String::new();
        // for el in self.unlock {
        //     s_unlock = [ s_unlock, help.String(el), "," ].concat();
        // }
        // xml::update_tag(dataNode, "unlock", s_unlock);
        // let tag = "unlock";
        // let s = self.s_unlock;
        // writer.write_event(quick_xml::events::Event::Start(quick_xml::events::BytesStart::borrowed(tag.as_bytes(), tag.len())));
        // writer.write_event(quick_xml::events::Event::Text(quick_xml::events::BytesText::from_plain_str(s)));
        // writer.write_event(quick_xml::events::Event::End(quick_xml::events::BytesEnd::borrowed(tag.as_bytes())));

        // let s_unlocknotify = String::new();
        // for el in self.unlocknotify {
        //     s_unlocknotify = [ s_unlocknotify, help.String(el), "," ].concat();
        // }
        // xml::update_tag(dataNode, "unlocknotify", s_unlocknotify);

        // let s_besttimes = String::new();
        // for el in self.besttimes  {
        //     s_besttimes = [ s_besttimes, help.String(el), "," ].concat();
        // }
        // xml::update_tag(dataNode, "besttimes", s_besttimes);

        // let s_bestframes = String::new();
        // for el in self.bestframes {
        //     s_bestframes = [s_bestframes, help.String(el), ","].concat();
        // }
        // xml::update_tag(dataNode, "bestframes", s_bestframes);





        // let s_besttrinkets = String::new();
        // for(size_t i = 0; i < SDL_arraysize(besttrinkets); i++ ) {
        //     s_besttrinkets += help.String(besttrinkets[i]) + ",";
        // }
        // xml::update_tag(dataNode, "besttrinkets", s_besttrinkets);

        // let s_bestlives = String::new();
        // for(size_t i = 0; i < SDL_arraysize(bestlives); i++ ) {
        //     s_bestlives += help.String(bestlives[i]) + ",";
        // }
        // xml::update_tag(dataNode, "bestlives", s_bestlives);

        // let s_bestrank = String::new();
        // for(size_t i = 0; i < SDL_arraysize(bestrank); i++ ) {
        //     s_bestrank += help.String(bestrank[i]) + ",";
        // }
        // xml::update_tag(dataNode, "bestrank", s_bestrank);
        // xml::update_tag(dataNode, "bestgamedeaths", bestgamedeaths);
        // xml::update_tag(dataNode, "stat_trinkets", stat_trinkets);
        // xml::update_tag(dataNode, "swnbestrank", swnbestrank);
        // xml::update_tag(dataNode, "swnrecord", swnrecord);

        self.serializesettings(&mut wrapper, screen_settings, music, map);
        wrapper.write_end_tag("Data");
        wrapper.write_end_tag("Save");

        return fs.FILESYSTEM_saveTiXml2Document("unlock.vvv", wrapper.writer.into_inner().into_inner());
    }

    pub fn savestats_settings(&mut self, screen_settings: screen::ScreenSettings) {
       warn!("DEADBEEF: Game::savestats_settings() method not implemented yet");
    }

    // void Game::deletestats(void);
    pub fn deletestats(&mut self) {
        warn!("DEADBEEF: Game::deletestats() not implemented yet");
    }

    // void Game::deserializesettings(tinyxml2::XMLElement* dataNode, ScreenSettings* screen_settings);
    fn deserializesettings(&mut self, reader: &mut quick_xml::Reader<std::io::BufReader<std::fs::File>>, music: &mut music::Music, map: &mut map::Map, gameScreen: &mut screen::Screen, help: &mut utility_class::UtilityClass, key: &mut key_poll::KeyPoll) {
        warn!("DEADBEEF: Game::deserializesettings not fully implemented yet");

        // Don't duplicate controller buttons!
        self.controllerButton_flip.clear();
        self.controllerButton_map.clear();
        self.controllerButton_esc.clear();
        self.controllerButton_restart.clear();

        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut tag: String = String::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(quick_xml::events::Event::Start(ref e)) => {
                    match String::from_utf8(e.name().to_vec()) {
                        Ok(v) => tag = v,
                        Err(_) => tag = String::new(),
                    }
                },
                Ok(quick_xml::events::Event::Text(text)) => {
                    if tag.len() == 0 {
                        continue;
                    }

                    match text.unescape_and_decode(&reader) {
                        Ok(text) => {
                            let v = text.as_str();
                            info!("parsing value ({}) into ({}) tag", v, tag);
                            match tag.as_str() {
                                "fullscreen" => gameScreen.screen_settings.fullscreen = v == "1",
                                "stretch" => gameScreen.screen_settings.stretch = i32::from_str_radix(v, 10).unwrap_or(0),
                                "useLinearFilter" => gameScreen.screen_settings.linearFilter = v == "1",
                                "window_width" => gameScreen.screen_settings.windowWidth = i32::from_str_radix(v, 10).unwrap_or(320),
                                "window_height" => gameScreen.screen_settings.windowHeight = i32::from_str_radix(v, 10).unwrap_or(240),
                                "noflashingmode" => self.noflashingmode = v == "1",
                                "colourblindmode" => self.colourblindmode = v == "1",
                                "setflipmode" => gameScreen.render.graphics.setflipmode = v == "1",
                                "invincibility" => map.invincibility = v == "1",
                                "slowdown" => self.slowdown = i32::from_str_radix(v, 10).unwrap_or(30),
                                "advanced_smoothing" => gameScreen.screen_settings.badSignal = v == "1",
                                "usingmmmmmm" => music.usingmmmmmm = v == "1",
                                "ghostsenabled" => self.ghostsenabled = v == "1",
                                "skipfakeload" => self.skipfakeload = v == "1",
                                "disablepause" => self.disablepause = v == "1",
                                "over30mode" => self.over30mode = v == "1",
                                "inputdelay" => self.inputdelay = v == "1",
                                "glitchrunnermode" => self.glitchrunnermode = v == "1",
                                "vsync" => gameScreen.screen_settings.useVsync = v == "1",
                                "notextoutline" => gameScreen.render.graphics.notextoutline = v == "1",
                                "translucentroomname" => gameScreen.render.graphics.translucentroomname = v == "1",
                                "musicvolume" => music.user_music_volume = Box::new(i32::from_str_radix(v, 10).unwrap_or(music::USER_VOLUME_MAX)),
                                "soundvolume" => music.user_sound_volume = Box::new(i32::from_str_radix(v, 10).unwrap_or(music::USER_VOLUME_MAX)),

                                // if SDL_strcmp(pKey, "flipButton") == 0 {
                                //     SDL_GameControllerButton newButton;
                                //     if GetButtonFromString(pText, &newButton) {
                                //         controllerButton_flip.push_back(newButton);
                                //     }
                                // }

                                // if SDL_strcmp(pKey, "enterButton") == 0 {
                                //     SDL_GameControllerButton newButton;
                                //     if GetButtonFromString(pText, &newButton) {
                                //         controllerButton_map.push_back(newButton);
                                //     }
                                // }

                                // if SDL_strcmp(pKey, "escButton") == 0 {
                                //     SDL_GameControllerButton newButton;
                                //     if GetButtonFromString(pText, &newButton) {
                                //         controllerButton_esc.push_back(newButton);
                                //     }
                                // }

                                // if SDL_strcmp(pKey, "restartButton") == 0 {
                                //     SDL_GameControllerButton newButton;
                                //     if GetButtonFromString(pText, &newButton)
                                //     {
                                //         controllerButton_restart.push_back(newButton);
                                //     }
                                // }

                                "controllerSensitivity" => key.sensitivity = i32::from_str_radix(v, 10).unwrap_or(2),

                                _ => info!("parsing {:?} tag not implemented", tag),
                            }

                        },
                        Err(_) => continue,
                    }
                },
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(quick_xml::events::Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        if self.controllerButton_flip.len() < 1 {
            self.controllerButton_flip.push(Button::A);
        }
        if self.controllerButton_map.len() < 1 {
            self.controllerButton_map.push(Button::Y);
        }
        if self.controllerButton_esc.len() < 1 {
            self.controllerButton_esc.push(Button::B);
        }
        if self.controllerButton_restart.len() < 1 {
            self.controllerButton_restart.push(Button::RightShoulder);
        }
    }

    // void Game::serializesettings(tinyxml2::XMLElement* dataNode, const ScreenSettings* screen_settings);
    fn serializesettings(&mut self, wrapper: &mut xml::xml, screen_settings: screen::ScreenSettings, music: &mut music::Music, map: &mut map::Map) {
        warn!("DEADBEEF: Game::serializesettings not implemented yet");

        // xml::update_tag(dataNode, "fullscreen", (int) screen_settings->fullscreen);
        // xml::update_tag(dataNode, "stretch", screen_settings->stretch);
        // xml::update_tag(dataNode, "useLinearFilter", (int) screen_settings->linearFilter);
        // xml::update_tag(dataNode, "window_width", screen_settings->windowWidth);
        // xml::update_tag(dataNode, "window_height", screen_settings->windowHeight);
        // xml::update_tag(dataNode, "noflashingmode", noflashingmode);
        // xml::update_tag(dataNode, "colourblindmode", colourblindmode);
        // xml::update_tag(dataNode, "setflipmode", graphics.setflipmode);
        wrapper.update_tag("invincibility", if map.invincibility { "1" } else { "0" });
        // xml::update_tag(dataNode, "slowdown", slowdown);
        // xml::update_tag(dataNode, "advanced_smoothing", (int) screen_settings->badSignal);
        // xml::update_tag(dataNode, "usingmmmmmm", music.usingmmmmmm);
        // xml::update_tag(dataNode, "ghostsenabled", (int) ghostsenabled);
        wrapper.update_tag("skipfakeload", if self.skipfakeload { "1" } else { "0" });
        // xml::update_tag(dataNode, "disablepause", (int) disablepause);
        // xml::update_tag(dataNode, "notextoutline", (int) graphics.notextoutline);
        // xml::update_tag(dataNode, "translucentroomname", (int) graphics.translucentroomname);
        // xml::update_tag(dataNode, "over30mode", (int) over30mode);
        // xml::update_tag(dataNode, "inputdelay", (int) inputdelay);
        // xml::update_tag(dataNode, "glitchrunnermode", (int) glitchrunnermode);
        // xml::update_tag(dataNode, "vsync", (int) screen_settings->useVsync);
        wrapper.update_tag("musicvolume", &music.user_music_volume.to_string());
        wrapper.update_tag("soundvolume", &music.user_sound_volume.to_string());

        // Delete all controller buttons we had previously.
        // dataNode->FirstChildElement() shouldn't be NULL at this point...
        // we've already added a bunch of elements
        // for (tinyxml2::XMLElement* element = dataNode->FirstChildElement();
        // element != NULL;
        // /* Increment code handled separately */) {
        //     const char* name = element->Name();

        //     if (SDL_strcmp(name, "flipButton") == 0
        //     || SDL_strcmp(name, "enterButton") == 0
        //     || SDL_strcmp(name, "escButton") == 0
        //     || SDL_strcmp(name, "restartButton") == 0)
        //     {
        //         // Can't just doc.DeleteNode(element) and then go to next,
        //         // element->NextSiblingElement() will be NULL.
        //         // Instead, store pointer of element we want to delete. Then
        //         // increment `element`. And THEN delete the element.
        //         tinyxml2::XMLElement* delete_this = element;

        //         element = element->NextSiblingElement();

        //         doc.DeleteNode(delete_this);
        //         continue;
        //     }

        //     element = element->NextSiblingElement();
        // }

        // Now add them
        // for (size_t i = 0; i < controllerButton_flip.size(); i += 1) {
        //     tinyxml2::XMLElement* msg = doc.NewElement("flipButton");
        //     msg->LinkEndChild(doc.NewText(help.String((int) controllerButton_flip[i]).c_str()));
        //     dataNode->LinkEndChild(msg);
        // }
        // for (size_t i = 0; i < controllerButton_map.size(); i += 1) {
        //     tinyxml2::XMLElement* msg = doc.NewElement("enterButton");
        //     msg->LinkEndChild(doc.NewText(help.String((int) controllerButton_map[i]).c_str()));
        //     dataNode->LinkEndChild(msg);
        // }
        // for (size_t i = 0; i < controllerButton_esc.size(); i += 1) {
        //     tinyxml2::XMLElement* msg = doc.NewElement("escButton");
        //     msg->LinkEndChild(doc.NewText(help.String((int) controllerButton_esc[i]).c_str()));
        //     dataNode->LinkEndChild(msg);
        // }
        // for (size_t i = 0; i < controllerButton_restart.size(); i += 1) {
        //     tinyxml2::XMLElement* msg = doc.NewElement("restartButton");
        //     msg->LinkEndChild(doc.NewText(help.String((int) controllerButton_restart[i]).c_str()));
        //     dataNode->LinkEndChild(msg);
        // }

        // xml::update_tag(dataNode, "controllerSensitivity", key.sensitivity);
    }

    // void Game::loadsettings(ScreenSettings* screen_settings);
    pub fn loadsettings(&mut self, fs: &mut filesystem::FileSystem, music: &mut music::Music, map: &mut map::Map, gameScreen: &mut screen::Screen, help: &mut utility_class::UtilityClass, key: &mut key_poll::KeyPoll) {
        trace!("loadsettings");
        let doc = fs.FILESYSTEM_loadTiXml2Document("settings.vvv");

        match doc {
            Ok(mut reader) => {
                self.deserializesettings(&mut reader, music, map, gameScreen, help, key);
            },
            Err(xml_err) => {
                match xml_err {
                    quick_xml::Error::Io(io_err) => {
                        match io_err.kind() {
                            std::io::ErrorKind::NotFound => {
                                self.savesettings(gameScreen.screen_settings, fs, music, map);
                                info!("No settings.vvv found");
                            },
                            _ => panic!("{:?}", io_err),
                        }
                    },
                    _ => panic!("{:?}", xml_err),
                }
            },
        }
    }

    // bool Game::savesettings(const ScreenSettings* screen_settings);
    // bool Game::savesettings(void);
    pub fn savesettings(&mut self, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem, music: &mut music::Music, map: &mut map::Map) -> bool {
        warn!("DEADBEEF: Game::savesettings not implemented yet");

        if !fs.savefile_exists("settings.vvv") {
            info!("No settings.vvv found. Creating new file");
        }

        let mut wrapper = xml::xml::new();
        wrapper.update_declaration();
        wrapper.write_start_tag("Settings");
        wrapper.update_comment(" Settings (duplicated from unlock.vvv) ");
        wrapper.write_start_tag("Data");
        self.serializesettings(&mut wrapper, screen_settings, music, map);
        wrapper.write_end_tag("Data");
        wrapper.write_end_tag("Settings");

        return fs.FILESYSTEM_saveTiXml2Document("settings.vvv", wrapper.writer.into_inner().into_inner());
    }

    // bool Game::savestatsandsettings(void);
    pub fn savestatsandsettings(&mut self, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem, music: &mut music::Music, map: &mut map::Map) -> bool {
        let stats_saved = self.savestats(screen_settings, fs, music, map);
        let settings_saved = self.savesettings(screen_settings, fs, music, map);
        return stats_saved && settings_saved; // Not the same as `savestats() && savesettings()`!
    }

    // void Game::savestatsandsettings_menu(void);
    pub fn savestatsandsettings_menu(&mut self, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem, music: &mut music::Music, map: &mut map::Map) {
        warn!("DEADBEEF: Game::savestatsandsettings_menu not implemented yet");

        self.savestatsandsettings(screen_settings, fs, music, map);

        // // Call Game::savestatsandsettings(), but upon failure, go to the save error screen
        // if !self.savestatsandsettings() && !silence_settings_error {
        //     self.createmenu(MenuName::errorsavingsettings);
        //     map.nexttowercolour();
        // }
    }

    // void Game::deletesettings(void);
    pub fn deletesettings(&mut self) {
        warn!("DEADBEEF: Game::deletesettings() method not implemented yet");
    }

    // void Game::deletequick(void);
    pub fn deletequick(&mut self) {
        warn!("DEADBEEF: Game::deletequick() method not implemented yet");
    }

    // bool Game::savetele(void);
    pub fn savetele(&mut self, map: &mut map::Map, fs: &mut filesystem::FileSystem) -> bool {
        if map.custommode || self.inspecial() {
            //Don't trash save data!
            return false;
        }

        warn!("DEADBEEF: Game::savetele() method not fully implemented yet");
        // let already_exists = fs.FILESYSTEM_loadTiXml2Document("tsave.vvv");
        // if !already_exists {
        //     info!("No tsave.vvv found. Creating new file");
        // }
        // self.telesummary = self.writemaingamesave(doc);

        // if !fs.FILESYSTEM_saveTiXml2Document("tsave.vvv", doc) {
        //     error!("Could Not Save game!\n");
        //     error!("Failed: {}{}\n", self.saveFilePath, "tsave.vvv");
        //     return false;
        // }

        // info!("Game saved\n");
        return true;
    }

    // void Game::loadtele(void);
    pub fn loadtele(&mut self) {
        warn!("DEADBEEF: Game::loadtele() method not implemented yet");
    }

    // void Game::deletetele(void);
    pub fn deletetele(&mut self) {
        warn!("DEADBEEF: Game::deletetele() method not implemented yet");
    }

    // void Game::customstart(void);
    pub fn customstart(&mut self) {
        warn!("DEADBEEF: Game::customstart() method not implemented yet");
    }

    // void Game::start(void);
    pub fn start(&mut self, map: &mut map::Map, music: &mut music::Music) {
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
            music.play(5, map, self);
        }
    }

    // void Game::startspecial(int t);
    pub fn startspecial(&mut self, t: i32) {
        warn!("DEADBEEF: Game::startspecial() method not implemented yet");
    }

    // void Game::starttrial(int t);
    pub fn starttrial(&mut self, t: i32) {
        warn!("DEADBEEF: Game::starttrial() method not implemented yet");
    }

    // void Game::swnpenalty(void);
    pub fn swnpenalty(&mut self) {
        warn!("DEADBEEF: Game::swnpenalty() method not implemented yet");
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
                music.fadeout(None, self);
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
        warn!("DEADBEEF: Game::customloadquick() method not implemented yet");
    }

    // void Game::loadquick(void);
    pub fn loadquick(&mut self) {
        warn!("DEADBEEF: Game::loadquick() method not implemented yet");
    }

    // void Game::loadsummary(void);
    pub fn loadsummary(&mut self) {
        warn!("DEADBEEF: Game::loadsummary() method not implemented yet");
    }

    // void Game::readmaingamesave(tinyxml2::XMLDocument& doc);
    pub fn readmaingamesave(&mut self, doc: i32) {
        warn!("DEADBEEF: Game::readmaingamesave() method not implemented yet");
    }

    // std::string Game::writemaingamesave(tinyxml2::XMLDocument& doc);
    pub fn writemaingamesave(&mut self, doc: i32) -> &'static str {
        warn!("DEADBEEF: Game::writemaingamesave() method not implemented yet");
        &""
    }

    // void Game::initteleportermode(void);
    pub fn initteleportermode(&mut self) {
        warn!("DEADBEEF: Game::initteleportermode() method not implemented yet");
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
        warn!("DEADBEEF: Game::copyndmresults() method not implemented yet");
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
        warn!("DEADBEEF: Game::crewmates method not implemented yet");

        // let temp = 0;
        // // for (size_t i = 0; i < SDL_arraysize(obj.customcollect); i++)
        // for i in 0..obj.customcollect.len() {
        //     if obj.customcollect[i] {
        //         temp += 1;
        //     }
        // }

        // temp
        0
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
        warn!("DEADBEEF: Game::loadcustomlevelstats() method not implemented yet");
    }

    // void Game::savecustomlevelstats(void);
    // void Game::updatecustomlevelstats(std::string clevel, int cscore);

    // void Game::quittomenu(void);
    pub fn quittomenu(&mut self, graphics: &mut graphics::Graphics, map: &mut map::Map, script: &mut script::ScriptClass, music: &mut music::Music, obj: &mut entity::EntityClass, screen_params: screen::ScreenParams, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem) {
        self.gamestate = GameState::TITLEMODE;
        graphics.fademode = 4;
        // FILESYSTEM_unmountAssets();
        self.cliplaytest = false;
        graphics.buffers.titlebg.tdrawback = true;
        graphics.flipmode = false;

        //Don't be stuck on the summary screen,
        //or "who do you want to play the level with?"
        //or "do you want cutscenes?"
        //or the confirm-load-quicksave menu
        if self.intimetrial {
            self.returntomenu(MenuName::timetrials);
        } else if self.inintermission {
            self.returntomenu(MenuName::intermissionmenu);
        } else if self.nodeathmode {
            self.returntomenu(MenuName::playmodes);
        } else if map.custommode {
            if map.custommodeforreal {
                self.returntomenu(MenuName::levellist);
            } else {
                //Returning from editor
                self.returntomenu(MenuName::playerworlds);
            }
        } else if self.save_exists() || self.anything_unlocked() {
            self.returntomenu(MenuName::play);
            if !self.insecretlab {
                //Select "continue"
                self.currentmenuoption = 0;
            }
        } else {
            self.createmenu(MenuName::mainmenu, Some(false), graphics, music, screen_params, map, screen_settings, fs);
        }

        script.hardreset(self, map, graphics, obj);
    }

    // void Game::returntolab(void);
    pub fn returntolab(&mut self, graphics: &mut graphics::Graphics, map: &mut map::Map, music: &mut music::Music, obj: &mut entity::EntityClass, help: &mut utility_class::UtilityClass) {
        self.gamestate = GameState::GAMEMODE;
        graphics.fademode = 4;
        map.gotoroom(119, 107, self, graphics, music, obj, help);
        let player = obj.getplayer() as usize;
        if INBOUNDS_VEC!(player, obj.entities) {
            obj.entities[player].xp = 132;
            obj.entities[player].yp = 137;
        }
        self.gravitycontrol = 0;

        self.savepoint = 0;
        self.saverx = 119;
        self.savery = 107;
        self.savex = 132;
        self.savey = 137;
        self.savegc = 0;
        if INBOUNDS_VEC!(player, obj.entities) {
            self.savedir = obj.entities[player].dir;
        }

        music.play(11, map, self);
    }

    // #if !defined(NO_CUSTOM_LEVELS)
    // void Game::returntoeditor(void);
    // #endif

    // bool inline inspecial(void)

    // void Game::returntoingame(void)
    pub fn returntoingame(&mut self, graphics: &mut graphics::Graphics) {
        // TODO @sx @impl
        warn!("DEADBEEF: Game::returntoingame is not implemented yet");

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
        warn!("DEADBEEF: Game::unlockAchievement() method not implemented yet");
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
