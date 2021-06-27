use crate::{entity, game::{self, GameState}, map, music, scenes::RenderResult, screen::render::graphics};
mod scripts;

pub struct ScriptClass {
    //Script contents
    commands: Vec<String>,
    // std::string words[40];
    words: String,
    txt: Vec<String>,
    scriptname: String,
    position: i32,
    looppoint: i32,
    loopcount: i32,

    scriptdelay: i32,
    running: bool,

    //Textbox stuff
    textx: i32,
    texty: i32,
    r: i32,
    g: i32,
    b: i32,
    textflipme: bool,

    //Misc
    i: i32,
    j: i32,
    k: i32,

    //Custom level stuff
    // std::vector<Script> customscripts;
}

impl ScriptClass {
    // scriptclass::scriptclass(void)
    pub fn new() -> Self {
        Self {
            //Script contents
            commands: Vec::new(),
            // std::string words[40];
            words: String::new(),
            txt: Vec::new(),
            scriptname: String::new(),
            position: 0,
            looppoint: 0,
            loopcount: 0,

            scriptdelay: 0,
            running: false,

            //Textbox stuff
            textx: 0,
            texty: 0,
            r: 0,
            g: 0,
            b: 0,
            textflipme: false,

            //Misc
            i: 0,
            j: 0,
            k: 0,

            //Custom level stuff
            // std::vector<Script> customscripts;
        }
    }

    // void scriptclass::clearcustom(void)
    // void scriptclass::tokenize( const std::string& t )

    // void scriptclass::run(void)
    pub fn run(&mut self) -> Option<RenderResult> {
        println!("DEADBEEF: scriptclass::run() method not implemented yet");
        None
    }

    // void scriptclass::resetgametomenu(void)

    // void scriptclass::startgamemode( int t )
    pub fn startgamemode(&mut self, t: i32, game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, obj: &mut entity::EntityClass, music: &mut music::Music) {
        match t {
        0 => {
            game.gamestate = GameState::GAMEMODE;
            self.hardreset(game, map, graphics, obj);
            game.start(music);
            game.jumpheld = true;
            graphics.showcutscenebars = true;
            graphics.setbars(320);

            //set flipmode
            if graphics.setflipmode {
                graphics.flipmode = true;
            } else {
                obj.flags[73] = true;
            }

            if obj.entities.is_empty() {
                //In this game, constant, never destroyed
                obj.createentity(game.savex, game.savey, 0, Some(0), None, None, None, None, None, map, game);
            } else {
                map.resetplayer(None);
            }
            map.gotoroom(game.saverx, game.savery);
            map.initmapdata();

            scripts::load(self, "intro");
        },
        1..=19 | 100 => println!("gamemode {} not implemented yet", t),
        _ => println!("incorrect game mode"),
        }
    }

    // void scriptclass::teleport(void)

    // void scriptclass::hardreset(void)
    fn hardreset(&mut self, game: &mut game::Game, map: &mut map::Map, graphics: &mut graphics::Graphics, obj: &mut entity::EntityClass) {
        //Game:
        game.hascontrol = true;
        game.gravitycontrol = 0;
        game.teleport = false;
        game.companion = 0;
        game.roomchange = false;
        if !game.glitchrunnermode {
            // Ironically, resetting more variables makes the janky fadeout system in glitchrunnermode even more glitchy
            game.roomx = 0;
            game.roomy = 0;
        }
        game.prevroomx = 0;
        game.prevroomy = 0;
        game.teleport_to_new_area = false;
        game.teleport_to_x = 0;
        game.teleport_to_y = 0;
        game.teleportscript = "".to_string();

        game.tapleft = 0;
        game.tapright = 0;
        game.startscript = false;
        game.newscript = "".to_string();
        game.alarmon = false;
        game.alarmdelay = 0;
        game.blackout = false;
        game.useteleporter = false;
        game.teleport_to_teleporter = 0;

        game.nodeathmode = false;
        game.nocutscenes = false;

        for mut crewstat in game.crewstats.iter() {
            crewstat = &false;
        }
        game.crewstats[0] = true;
        game.lastsaved = 0;

        game.deathcounts = 0;
        game.gameoverdelay = 0;
        game.resetgameclock();
        game.gamesaved = false;
        game.gamesavefailed = false;
        game.savetime = "00:00".to_string();
        game.savearea = "nowhere".to_string();
        game.savetrinkets = 0;
        if !game.glitchrunnermode {
            // Ironically, resetting more variables makes the janky fadeout system in glitchrunnermode even more glitchy
            game.saverx = 0;
            game.savery = 0;
        }

        game.intimetrial = false;
        game.timetrialcountdown = 0;
        game.timetrialshinytarget = 0;
        game.timetrialparlost = false;
        game.timetrialpar = 0;

        game.totalflips = 0;
        game.hardestroom = "Welcome Aboard".to_string();
        game.hardestroomdeaths = 0;
        game.currentroomdeaths = 0;

        game.swnmode = false;
        game.swntimer = 0;
        game.swngame = 0;//Not playing sine wave ninja!
        game.swnstate = 0;
        game.swnstate2 = 0;
        game.swnstate3 = 0;
        game.swnstate4 = 0;
        game.swndelay = 0;
        game.swndeaths = 0;
        game.supercrewmate = false;
        game.scmhurt = false;
        game.scmprogress = 0;
        game.scmmoveme = false;
        game.swncolstate = 0;
        game.swncoldelay = 0;
        game.swnrank = 0;
        game.swnmessage = 0;
        game.creditposx = 0;
        game.creditposy = 0;
        game.creditposdelay = 0;

        game.inintermission = false;
        game.insecretlab = false;

        game.state = 0;
        game.statedelay = 0;

        game.hascontrol = true;
        if !game.glitchrunnermode {
            // Keep the "- Press ACTION to advance text -" prompt around,
            // apparently the speedrunners call it the "text storage" glitch
            game.advancetext = false;
        }

        game.pausescript = false;
        game.completestop = false;

        game.flashlight = 0;
        game.screenshake = 0;

        game.activeactivity = -1;
        game.act_fade = 5;

        //dwgraphicsclass
        graphics.backgrounddrawn = false;
        graphics.textbox.clear();
        graphics.flipmode = false; //This will be reset if needs be elsewhere
        graphics.showcutscenebars = false;
        graphics.setbars(0);

        //mapclass
        map.warpx = false;
        map.warpy = false;
        map.showteleporters = false;
        map.showtargets = false;
        map.showtrinkets = false;
        map.finalmode = false;
        map.finalstretch = false;
        map.final_colormode = false;
        map.final_colorframe = 0;
        map.final_colorframedelay = 0;
        map.final_mapcol = 0;
        map.final_aniframe = 0;
        map.final_aniframedelay = 0;
        map.rcol = 0;
        map.resetnames();
        map.custommode = false;
        map.custommodeforreal = false;
        if !game.glitchrunnermode {
            // Ironically, resetting more variables makes the janky fadeout system even more glitchy
            map.towermode = false;
        }
        map.cameraseekframe = 0;
        map.resumedelay = 0;
        graphics.buffers.towerbg.scrolldir = 0;
        map.customshowmm = true;

        map.roomdeaths = [0i32; 20*20];
        map.roomdeathsfinal = [0i32; 20*20];
        map.resetmap();
        //entityclass
        obj.nearelephant = false;
        obj.upsetmode = false;
        obj.upset = 0;

        obj.trophytext = 0 ;
        obj.trophytype = 0;
        obj.altstates = 0;

        obj.resetallflags();

        for mut customcrewmood in obj.customcrewmoods.iter() {
            customcrewmood = &true;
        }

        obj.collect = [false; 100];
        obj.customcollect = [false; 100];
        // @sx: what the use of next line?
        //i = 100; //previously a for-loop iterating over collect/customcollect set this to 100

        match obj.getplayer() {
            Some(theplayer) => {
                match obj.entities.get_mut(theplayer) {
                    Some(v) => {
                        v.tile = 0;
                    },
                    None => (),
                }
            },
            None => (),
        }

        /* Disable duplicate player entities */
        // for int i = 0; i < (int) obj.entities.size(); i++ {
        //     if obj.entities[i].rule == 0 && i != theplayer {
        //         obj.disableentity(i);
        //     }
        // }
        // for (i, mut entity) in obj.entities.iter().enumerate() {
        //     let i = i as i32;
        //     if entity.rule == 0 && i != theplayer {
        //         obj.disableentity(i);
        //     }
        // }
        println!("DEADBEEF: Disable duplicate player entities not implemented yet");

        //Script Stuff
        self.position = 0;
        self.commands.clear();
        self.scriptdelay = 0;
        self.scriptname = "null".to_string();
        self.running = false;
    }

    // void scriptclass::loadcustom(const std::string& t)

    fn filllines(&mut self, lines: Vec<&str>) {
        for line in lines {
            self.commands.push(line.to_string());
        }
    }

}
