const numcrew: usize = 6;
const numunlock: usize = 25;
const numtrials: usize = 6;

pub struct Game {
    // saveFilePath: String,

    door_left: i32,
    door_right: i32,
    door_up: i32,
    door_down: i32,
    roomx: i32,
    roomy: i32,
    roomchangedir: i32,
    prevroomx: i32,
    prevroomy: i32,

    savex: i32,
    savey: i32,
    saverx: i32,
    savery: i32,
    savegc: i32,
    savedir: i32,

    // Added for port
    edsavex: i32,
    edsavey: i32,
    edsaverx: i32,
    edsavery: i32,
    edsavegc: i32,
    edsavedir: i32,

    // State logic stuff
    state: i32,
    statedelay: i32,

    glitchrunkludge: bool,

    usingmmmmmm: i32,

    pub gamestate: GameState,
    hascontrol: bool,
    jumpheld: bool,
    jumppressed: i32,
    gravitycontrol: i32,

    muted: bool,
    mutebutton: i32,
    musicmuted: bool,
    musicmutebutton: i32,

    tapleft: i32,
    tapright: i32,

    // Menu interaction stuff
    mapheld: bool,
    menupage: i32,
    lastsaved: i32,
    deathcounts: i32,

    frames: i32,
    seconds: i32,
    minutes: i32,
    hours: i32,
    gamesaved: bool,
    gamesavefailed: bool,
    savetime: String,
    savearea: String,
    savetrinkets: i32,
    startscript: bool,
    newscript: String,

    mainmenu: i32,
    menustart: bool,

    // Teleporting
    teleport_to_new_area: bool,
    teleport_to_x: i32,
    teleport_to_y: i32,
    teleportscript: String,
    useteleporter: bool,
    teleport_to_teleporter: i32,

    // Main Menu Variables
    // menuoptions: [MenuOption],
    currentmenuoption: i32,
    currentmenuname: MenuName,
    kludge_ingametemp: MenuName,
    current_credits_list_index: i32,
    menuxoff: i32,
    menuyoff: i32,
    menuspacing: i32,
    // static const menutextbytes: i32: 161, // this is just sizeof(MenuOption::text), but doing that is non-standard
    // menustack: [MenuStackFrame],

    // void inline option(const char* text, active: bool = true) {
    //     MenuOption menuoption,
    //     SDL_strlcpy(menuoption.text, text, sizeof(menuoption.text)),
    //     menuoption.active: active,
    //     menuoptions.push_back(menuoption),
    // }

    menucountdown: i32,
    menudest: MenuName,

    creditposx: i32,
    creditposy: i32,
    creditposdelay: i32,
    oldcreditposx: i32,


    // Sine Wave Ninja Minigame
    swnmode: bool,
    swngame: i32,
    swnstate: i32,
    swnstate2: i32,
    swnstate3: i32,
    swnstate4: i32,
    swndelay: i32,
    swndeaths: i32,
    swntimer: i32,
    swncolstate: i32,
    swncoldelay: i32,
    swnrecord: i32,
    swnbestrank: i32,
    swnrank: i32,
    swnmessage: i32,

    // SuperCrewMate Stuff
    supercrewmate: bool,
    scmhurt: bool,
    scmmoveme: bool,
    scmprogress: i32,

    // Accessibility Options
    colourblindmode: bool,
    noflashingmode: bool,
    slowdown: i32,
    pub gameframerate: u32,

    nodeathmode: bool,
    gameoverdelay: i32,
    nocutscenes: bool,

    // Time Trials
    intimetrial: bool,
    timetrialparlost: bool,
    timetrialcountdown: i32,
    timetrialshinytarget: i32,
    timetriallevel: i32,
    timetrialpar: i32,
    timetrialresulttime: i32,
    timetrialresultframes: i32,
    timetrialrank: i32,

    creditposition: i32,
    oldcreditposition: i32,
    insecretlab: bool,

    inintermission: bool,

    // numcrew: i32, // const
    crewstats: [bool; numcrew],

    alarmon: bool,
    alarmdelay: i32,
    blackout: bool,

    tele_crewstats: [bool; numcrew],
    quick_crewstats: [bool; numcrew],

    // numunlock: i32, // static const
    unlock: [bool; numunlock],
    unlocknotify: [bool; numunlock],
    // anything_unlocked: bool(),
    stat_trinkets: i32,
    fullscreen: bool,
    bestgamedeaths: i32,

    // static const numtrials: i32 = 6,
    besttimes: [i32; numtrials],
    bestframes: [i32; numtrials],
    besttrinkets: [i32; numtrials],
    bestlives: [i32; numtrials],
    bestrank: [i32; numtrials],

    tele_gametime: String,
    tele_trinkets: i32,
    tele_currentarea: String,
    quick_gametime: String,
    quick_trinkets: i32,
    quick_currentarea: String,

    mx: i32,
    my: i32,
    screenshake: i32,
    flashlight: i32,
    advancetext: bool,
    pausescript: bool,

    deathseq: i32,
    lifeseq: i32,

    // trinkets: i32(),
    // crewmates: i32(),
    savepoint: i32,
    teleportxpos: i32,
    teleport: bool,
    edteleportent: i32,
    completestop: bool,

    inertia: f32,

    companion: i32,
    roomchange: bool,
    // SDL_Rect teleblock,
    activetele: bool,
    readytotele: i32,
    oldreadytotele: i32,
    activity_r: i32,
    activity_g: i32,
    activity_b: i32,
    activity_lastprompt: String,

    telesummary: String,
    quicksummary: String,
    customquicksummary: String,
    // save_exists: bool(),

    backgroundtext: bool,

    activeactivity: i32,
    act_fade: i32,
    prev_act_fade: i32,

    press_left: bool,
    press_right: bool,
    press_action: bool,
    press_map: bool,

    // Some stats:
    totalflips: i32,
    hardestroom: String,
    hardestroomdeaths: i32,
    currentroomdeaths: i32,

    savemystats: bool,


    fullScreenEffect_badSignal: bool,
    useLinearFilter: bool,
    stretchMode: i32,
    controllerSensitivity: i32,

    quickrestartkludge: bool,

    // Custom stuff
    // customscript: String[50],
    customcol: i32,
    levelpage: i32,
    playcustomlevel: i32,
    customleveltitle: String,
    customlevelfilename: String,

    // void clearcustomlevelstats(),
    // void loadcustomlevelstats(),
    // void savecustomlevelstats(),
    // void updatecustomlevelstats(clevel: String, cscore: i32),

    // std::vector<CustomLevelStat> customlevelstats,
    customlevelstatsloaded: bool,


    // std::vector<SDL_GameControllerButton> controllerButton_map,
    // std::vector<SDL_GameControllerButton> controllerButton_flip,
    // std::vector<SDL_GameControllerButton> controllerButton_esc,
    // std::vector<SDL_GameControllerButton> controllerButton_restart,

    skipfakeload: bool,
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
    glitchrunnermode: bool, // Have fun speedrunners! <3 Misa

    ingame_titlemode: bool,

    // void returntopausemenu(),
    // void unlockAchievement(const char *name),

    disablepause: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
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

            gamestate: GameState::TITLEMODE,

            hascontrol: true,
            jumpheld: false,
            advancetext: false,
            jumppressed: 0,
            gravitycontrol: 0,
            companion: 0,
            roomchange: false,


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
            // save_exists: bool(),

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
            nocutscenes: false,

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

            creditposition: 0,
            oldcreditposition: 0,
            insecretlab: false,

            // SDL_memset(crewstats, false, sizeof(crewstats));
            // SDL_memset(tele_crewstats, false, sizeof(tele_crewstats));
            // SDL_memset(quick_crewstats, false, sizeof(quick_crewstats));
            // SDL_memset(besttimes, -1, sizeof(besttimes));
            // SDL_memset(bestframes, -1, sizeof(bestframes));
            // SDL_memset(besttrinkets, -1, sizeof(besttrinkets));
            // SDL_memset(bestlives, -1, sizeof(bestlives));
            // SDL_memset(bestrank, -1, sizeof(bestrank));
            crewstats: [false; numcrew],
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

            menucountdown: 0,
            menudest: MenuName::accessibility, // TODO:
            // createmenu(Menu::mainmenu),

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

            gameoverdelay: 0,
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
            // if (!FILESYSTEM_loadTiXml2Document("saves/qsave.vvv", doc))
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
            //     if (!pElem)
            //     {
            //         printf("Quick Save Appears Corrupted: No XML Root\n");
            //     }
            //     // save this for later
            //     hRoot=tinyxml2::XMLHandle(pElem);
            //     for( pElem = hRoot.FirstChildElement( "Data" ).FirstChild().ToElement(); pElem; pElem=pElem->NextSiblingElement())
            //     {
            //         std::string pKey(pElem->Value());
            //         const char* pText = pElem->GetText() ;
            //         if (pKey == "summary")
            //         {
            //             quicksummary = pText;
            //         }
            //     }
            // }


            // tinyxml2::XMLDocument docTele;
            // if (!FILESYSTEM_loadTiXml2Document("saves/tsave.vvv", docTele)) {
            //     telesummary = "";
            //     printf("Teleporter Save Not Found\n");
            // } else {
            //     tinyxml2::XMLHandle hDoc(&docTele);
            //     tinyxml2::XMLElement* pElem;
            //     tinyxml2::XMLHandle hRoot(NULL); {
            //         pElem=hDoc.FirstChildElement().ToElement();
            //         // should always have a valid root but handle gracefully if it does
            //         if (!pElem) {
            //             printf("Teleporter Save Appears Corrupted: No XML Root\n");
            //         }
            //         // save this for later
            //         hRoot=tinyxml2::XMLHandle(pElem);
            //     }
            //     for( pElem = hRoot.FirstChildElement( "Data" ).FirstChild().ToElement(); pElem; pElem=pElem->NextSiblingElement())
            //     {
            //         std::string pKey(pElem->Value());
            //         const char* pText = pElem->GetText() ;
            //         if (pKey == "summary")
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

            disablepause: false,
        }
    }

    pub fn init_game(&mut self) {
        if self.skipfakeload     { self.gamestate = GameState::TITLEMODE };
        // if self.usingmmmmmm == 0 { music.usingmmmmmm=false; }
        // if self.usingmmmmmm == 1 { music.usingmmmmmm=true; }
        if self.slowdown == 0    { self.slowdown = 30; }

        self.gameframerate = match self.slowdown {
            30 => 34,
            24 => 41,
            18 => 55,
            12 => 83,
            _  => 34,
        }

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
}
struct MenuOption {
    // text: char[161], // 40 chars (160 bytes) covers the entire screen, + 1 more for null terminator
    // WARNING: should match Game::menutextbytes below
    active: bool,
}

enum MenuName {
    mainmenu,
    playerworlds,
    levellist,
    quickloadlevel,
    youwannaquit,
    errornostart,
    graphicoptions,
    ed_settings,
    ed_desc,
    ed_music,
    ed_quit,
    options,
    advancedoptions,
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

struct MenuStackFrame {
    option: i32,
    name: MenuName,
}

struct CustomLevelStat {
    name: String,
    // 0 - not played, 1 - finished, 2 - all trinkets, 3 - finished, all trinkets
    score: u8,
}

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
