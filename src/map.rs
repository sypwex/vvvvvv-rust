use crate::{INBOUNDS_VEC, entity, game, music, screen::render::graphics::{self, towerbg}};
mod towerclass;

// Areamap starts at 100,100 and extends 20x20
pub const areamap: [i32; 20*20] = [
    1,2,2,2,2,2,2,2,0,3,0,0,0,4,4,4,4,4,4,4,
    1,2,2,2,2,2,2,0,0,3,0,0,0,0,4,4,4,4,4,4,
    0,1,0,0,2,0,0,0,0,3,0,0,0,0,4,4,4,4,4,4,
    0,0,0,0,2,0,0,0,0,3,0,0,5,5,5,5,4,4,4,4,
    0,0,2,2,2,0,0,0,0,3,11,11,5,5,5,5,0,0,0,0,
    0,0,0,0,0,0,0,0,0,3,5,5,5,5,5,5,0,0,0,0,
    0,0,0,0,0,0,0,0,0,3,5,5,5,5,5,5,5,0,0,0,
    0,0,0,0,0,0,0,0,0,3,5,5,5,5,5,5,5,5,5,0,
    0,0,0,0,0,0,0,0,0,3,0,0,0,5,5,5,5,5,5,0,
    0,0,0,0,0,0,0,0,11,3,0,0,0,5,5,5,5,5,5,0,
    0,0,0,0,0,0,0,0,0,3,0,0,0,5,5,5,5,5,5,0,
    0,0,0,0,0,0,0,0,0,3,0,5,5,5,5,5,5,5,5,0,
    0,0,0,0,0,0,0,0,0,3,0,5,5,5,5,5,5,0,5,0,
    0,0,0,0,0,0,0,0,0,3,0,5,5,5,5,5,5,0,5,0,
    0,0,0,0,0,0,0,0,0,3,0,5,5,0,0,0,0,0,5,0,
    0,0,0,0,0,0,0,2,0,3,0,0,0,0,0,0,0,0,0,0,
    0,0,2,2,2,2,2,2,0,3,0,0,0,0,0,0,0,0,0,0,
    0,2,2,2,2,2,2,2,0,3,0,0,0,0,0,0,0,0,0,0,
    2,2,2,2,2,0,0,2,0,3,0,0,0,0,0,0,0,0,0,0,
    2,2,2,2,2,0,0,2,0,3,0,0,0,0,0,0,0,0,0,0,
];

pub struct Roomtext {
    pub x: i32,
    pub y: i32,
    pub text: String,
}

pub struct Map {
    pub roomdeaths: [i32; 20*20],
    pub roomdeathsfinal: [i32; 20*20],
    pub contents: [i16; 40*30],
    explored: [bool; 20*20],
    pub vmult: [i32; 30],

    //Start here!
    colstatedelay: i32,
    pub colsuperstate: i32,
    pub spikeleveltop: i32,
    pub spikelevelbottom: i32,
    pub oldspikeleveltop: i32,
    pub oldspikelevelbottom: i32,
    pub warpx: bool,
    pub warpy: bool,

    pub showteleporters: bool,
    pub showtargets: bool,
    pub showtrinkets: bool,

    pub finalmode: bool,
    pub finalstretch: bool,

    //Special tower stuff
    pub towermode: bool,
    pub ypos: i32,
    pub oldypos: i32,
    pub cameraseek: i32,
    pub cameraseekframe: i32,
    pub resumedelay: i32,
    pub minitowermode: bool,

    specialnames: Vec<String>, // size of 8
    glitchmode: i32,
    glitchdelay: i32,
    pub glitchname: String,

    //final level colour cycling stuff
    pub final_colormode: bool,
    pub final_colorframe: i32,
    pub final_colorframedelay: i32,
    pub final_mapcol: i32,
    pub final_aniframe: i32,
    pub final_aniframedelay: i32,

    pub custommode: bool,
    pub custommodeforreal: bool,
    customwidth: i32,
    customheight: i32,

    custommmxoff: i32,
    custommmyoff: i32,
    custommmxsize: i32,
    custommmysize: i32,

    customzoom: i32,
    pub customshowmm: bool,

    pub rcol: i32,

    pub tileset: i32,

    pub background: i32,
    pub cameramode: i32,

    pub roomname: String,
    hiddenname: String,

    //Roomtext
    pub roomtexton: bool,
    pub roomtext: Vec<Roomtext>,

    //Levels
    // otherlevel: otherlevelclass,
    // spacestation2: spacestation2class,
    // lablevel: labclass,
    // finallevel: finalclass,
    // warplevel: warpclass,
    pub tower: towerclass::TowerClass,
    pub extrarow: i32,

    //Accessibility options
    pub invincibility: bool,

    //Map cursor
    cursorstate: i32,
    cursordelay: i32,
}

impl Map {
    // mapclass::mapclass(void)
    pub fn new(graphics: &mut graphics::Graphics) -> Self {
        let mut map = Map {
            roomdeaths: [0i32; 20*20],
            roomdeathsfinal: [0i32; 20*20],
            contents: [0i16; 40*30],
            explored: [false; 20*20],
            vmult: [0; 30],

            //Start here!
            colstatedelay: 0,
            colsuperstate: 0,
            spikeleveltop: 0,
            spikelevelbottom: 0,
            oldspikeleveltop: 0,
            oldspikelevelbottom: 0,
            warpx: false,
            warpy: false,
            extrarow: 0,

            showteleporters: false,
            showtargets: false,
            showtrinkets: false,

            finalmode: false,
            finalstretch: false,

            //Special tower stuff
            towermode: false,
            ypos: 0,
            oldypos: 0,
            cameraseek: 0,
            cameraseekframe: 0,
            resumedelay: 0,
            minitowermode: false,

            specialnames: Vec::new(),
            glitchmode: 0,
            glitchdelay: 0,
            glitchname: String::new(),

            //final level colour cycling stuff
            final_colormode: false,
            final_colorframe: 0,
            final_colorframedelay: 0,
            final_mapcol: 0,
            final_aniframe: 0,
            final_aniframedelay: 0,

            custommode: false,
            custommodeforreal: false,
            customwidth: 20,
            customheight: 20,

            custommmxoff: 0,
            custommmyoff: 0,
            custommmxsize: 0,
            custommmysize: 0,

            customzoom: 0,
            customshowmm: true,

            rcol: 0,

            tileset: 0,

            background: 0,
            cameramode: 0,

            //Levels
            // otherlevel: otherlevelclass,
            // spacestation2: spacestation2class,
            // lablevel: labclass,
            // finallevel: finalclass,
            // warplevel: warpclass,
            tower: towerclass::TowerClass::new(),

            roomname: String::new(),
            hiddenname: String::new(),

            //Roomtext
            roomtexton: false,
            roomtext: Vec::new(),

            //Accessibility options
            invincibility: false,

            //Map cursor
            cursorstate: 0,
            cursordelay: 0,
        };

        map.initmapdata();
        map.resetnames();
        map.nexttowercolour(graphics);

        map
    }

    // int mapclass::intpol(int a, int b, float c)
    fn intpol (a: i32, b: i32, c: i32) -> i32 {
        a + ((b - a) * c)
    }

    // void mapclass::setteleporter(int x, int y)
    pub fn setteleporter(&mut self, x: i32, y: i32) {
        println!("DEADBEEF: mapclass::setteleporter() method not implemented yet");
    }

    // void mapclass::settrinket(int x, int y)
    pub fn settrinket(&mut self, x: i32, y: i32) {
        println!("DEADBEEF: mapclass::settrinket() method not implemented yet");
    }

    // void mapclass::resetmap(void)
    pub fn resetmap(&mut self) {
        self.explored = [false; 20*20];
    }

    // void mapclass::resetnames(void)
    pub fn resetnames (&mut self) {

    }

    // void mapclass::transformname(int t)
    pub fn transformname(&mut self, t: i32) {
        println!("DEADBEEF: mapclass::transformname() method not implemented yet");
    }

    // std::string mapclass::getglitchname(int x, int y)
    pub fn getglitchname(&self, x: i32, y: i32) -> String {
        println!("DEADBEEF: mapclass::getglitchname() method not implemented yet");
        String::new()
    }

    // void mapclass::initmapdata(void)
    pub fn initmapdata (&mut self) {

    }

    // void mapclass::initcustommapdata(void)
    pub fn initcustommapdata(&mut self) {
        println!("DEADBEEF: mapclass::initcustommapdata() method not implemented yet");
    }

    // int mapclass::finalat(int x, int y)
    pub fn finalat(&mut self, x: i32, y: i32) -> i32 {
        println!("DEADBEEF: mapclass::finalat() method not implemented yet");
        0
    }

    // int mapclass::maptiletoenemycol(int t)
    pub fn maptiletoenemycol(&mut self, t: i32) -> i32 {
        println!("DEADBEEF: mapclass::maptiletoenemycol() method not implemented yet");
        0
    }

    // void mapclass::changefinalcol(int t)
    pub fn changefinalcol(&mut self, t: i32) {
        println!("DEADBEEF: mapclass::changefinalcol() method not implemented yet");
    }

    // void mapclass::setcol(TowerBG& bg_obj, const int r1, const int g1, const int b1 , const int r2, const int g2, const int b2, const int c)
    fn setcol (bg_obj: &mut towerbg::TowerBG, r1: i32, g1: i32, b1: i32, r2: i32, g2: i32, b2: i32, check: i32) {
        bg_obj.r = Map::intpol(r1, r2, check / 5);
        bg_obj.g = Map::intpol(g1, g2, check / 5);
        bg_obj.b = Map::intpol(b1, b2, check / 5);
    }

    // void mapclass::updatebgobj(TowerBG& bg_obj)
    fn updatebgobj (bg_obj: &mut towerbg::TowerBG) {
        let check = bg_obj.colstate % 5; //current state of phase
        let cmode = (bg_obj.colstate - check) / 5; //current colour transition;

        match cmode {
            0 => Map::setcol(bg_obj, 255, 093, 107, 255, 255, 093, check),
            1 => Map::setcol(bg_obj, 255, 255, 093, 159, 255, 093, check),
            2 => Map::setcol(bg_obj, 159, 255, 093, 093, 245, 255, check),
            3 => Map::setcol(bg_obj, 093, 245, 255, 177, 093, 255, check),
            4 => Map::setcol(bg_obj, 177, 093, 255, 255, 093, 255, check),
            5 => Map::setcol(bg_obj, 255, 093, 255, 255, 093, 107, check),
            _ => panic!("wrong cmode {:?}", cmode),
        };

        bg_obj.tdrawback = true;
    }

    // void mapclass::updatetowerglow(TowerBG& bg_obj)
    pub fn updatetowerglow (&mut self, bg_obj: &mut towerbg::TowerBG) {
        if self.colstatedelay <= 0 || self.colsuperstate > 0 {
            if self.colsuperstate > 0 {
                bg_obj.colstate -= 1;
            }
            bg_obj.colstate += 1;
            if bg_obj.colstate >= 30 {
                bg_obj.colstate = 0;
            }

            let check: i32 = bg_obj.colstate % 5;
            Map::updatebgobj(bg_obj);

            if check == 0 {
                self.colstatedelay = 45;
            } else {
                self.colstatedelay = 0;
            }
            if self.colsuperstate > 0 {
                self.colstatedelay = 0;
            }
        } else {
            self.colstatedelay -= 1;
        }
    }

    // void mapclass::nexttowercolour(void)
    pub fn nexttowercolour (&mut self, graphics: &mut graphics::Graphics) {
        graphics.buffers.titlebg.colstate += 5;

        if graphics.buffers.titlebg.colstate >= 30 {
            graphics.buffers.titlebg.colstate = 0;
        }

        Map::updatebgobj(&mut graphics.buffers.titlebg);
    }

    // void mapclass::settowercolour(int t)
    pub fn settowercolour(&mut self, t: i32, graphics: &mut graphics::Graphics) {
        graphics.buffers.titlebg.colstate = t*5;
        if graphics.buffers.titlebg.colstate >= 30 {
            graphics.buffers.titlebg.colstate = 0;
        }

        Map::updatebgobj(&mut graphics.buffers.titlebg);
    }

    // bool mapclass::spikecollide(int x, int y)
    pub fn spikecollide(&mut self, x: i32, y: i32) -> bool {
        println!("DEADBEEF: mapclass::spikecollide() method not implemented yet");
        false
    }

    // bool mapclass::collide(int x, int y)
    pub fn collide(&self, x: i32, y: i32) -> bool {
        if self.towermode {
            if self.tower.at(x, y, 0) >= 12 && self.tower.at(x, y, 0) <= 27 {
                return true
            }
            if self.invincibility {
                if self.tower.at(x, y, 0) >= 6 && self.tower.at(x, y, 0) <= 11 {
                    return true
                }
            }
        } else if self.tileset == 2 {
            if y == -1                  { return self.collide(x, y + 1); }
            if y == 29 + self.extrarow  { return self.collide(x, y - 1); }
            if x == -1                  { return self.collide(x + 1, y); }
            if x == 40                  { return self.collide(x - 1, y); }
            if x < 0 || y < 0 || x >= 40 || y >= 29 + self.extrarow {
                return false;
            }
            if self.contents[(x + self.vmult[y as usize]) as usize] >= 12 && self.contents[(x + self.vmult[y as usize]) as usize] <= 27 { return true; }
            if self.invincibility {
                if self.contents[(x + self.vmult[y as usize]) as usize] >= 6 && self.contents[(x + self.vmult[y as usize]) as usize] <= 11 { return true; }
            }
        } else {
            if y == -1                 { return self.collide(x, y + 1); }
            if y == 29 + self.extrarow { return self.collide(x, y - 1); }
            if x == -1                 { return self.collide(x + 1, y); }
            if x == 40                 { return self.collide(x - 1, y); }
            if x < 0 || y < 0 || x >= 40 || y >= 29 + self.extrarow { return false; }
            if self.contents[(x + self.vmult[y as usize]) as usize] == 1 { return true; }
            if self.tileset == 0 && self.contents[(x + self.vmult[y as usize]) as usize] == 59 { return true; }
            if self.contents[(x + self.vmult[y as usize]) as usize] >= 80 && self.contents[(x + self.vmult[y as usize]) as usize] < 680 { return true; }
            if self.contents[(x + self.vmult[y as usize]) as usize] == 740 && self.tileset == 1 { return true; }
            if self.invincibility {
                if self.contents[(x + self.vmult[y as usize]) as usize] >= 6 && self.contents[(x + self.vmult[y as usize]) as usize] <= 9 { return true; }
                if self.contents[(x + self.vmult[y as usize]) as usize] >= 49 && self.contents[(x + self.vmult[y as usize]) as usize] <= 50 { return true; }
                if self.tileset == 1 {
                    if self.contents[(x + self.vmult[y as usize]) as usize] >= 49 && self.contents[(x + self.vmult[y as usize]) as usize] < 80 { return true; }
                }
            }
        }

        false
    }

    // void mapclass::settile(int xp, int yp, int t)
    pub fn settile(&mut self, xp: i32, yp: i32, t: i32) {
        println!("DEADBEEF: mapclass::settile() method not implemented yet");
    }

    // int mapclass::area(int _rx, int _ry)
    pub fn area(&self, _rx: i32, _ry: i32) -> i32 {
        println!("DEADBEEF: mapclass::area method not implemented yet");
        0
    }

    // bool mapclass::isexplored(const int rx, const int ry)
    pub fn isexplored(&mut self, rx: i32, ry: i32) -> bool {
        println!("DEADBEEF: mapclass::isexplored() method not implemented yet");
        false
    }

    // void mapclass::setexplored(const int rx, const int ry, const bool status)
    pub fn setexplored(&mut self, rx: i32, ry: i32, status: bool) {
        println!("DEADBEEF: mapclass::setexplored() method not implemented yet");
    }

    // void mapclass::exploretower(void)
    pub fn exploretower(&mut self) {
        println!("DEADBEEF: mapclass::exploretower() method not implemented yet");
    }

    // void mapclass::hideship(void)
    pub fn hideship(&mut self) {
        println!("DEADBEEF: mapclass::hideship() method not implemented yet");
    }

    // void mapclass::showship(void)
    pub fn showship(&mut self) {
        println!("DEADBEEF: mapclass::showship() method not implemented yet");
    }

    // void mapclass::resetplayer(void)
    // void mapclass::resetplayer(const bool player_died)
    pub fn resetplayer(&mut self, player_died: Option<bool>) {
        println!("DEADBEEF: mapclass::resetplayer method not implemented yet");
    }

    // void mapclass::warpto(int rx, int ry , int t, int tx, int ty)
    pub fn warpto(&mut self, rx: i32, ry: i32, t: i32, tx: i32, ty: i32) {
        println!("DEADBEEF: mapclass::warpto() method not implemented yet");
    }

    // void mapclass::gotoroom(int rx, int ry)
    pub fn gotoroom(&mut self, rx: i32, ry: i32, game: &mut game::Game, graphics: &mut graphics::Graphics, music: &mut music::Music, obj: &mut entity::EntityClass) {
        //First, destroy the current room
        obj.removeallblocks();
        game.activetele = false;
        game.readytotele = 0;
        game.oldreadytotele = 0;

        //Ok, let's save the position of all lines on the screen
        obj.linecrosskludge.clear();
        // for size_t i = 0; i < obj.entities.size(); i++ {
        for i in 0..obj.entities.len() {
            if obj.entities[i].r#type == 9 {
                //It's a horizontal line
                if obj.entities[i].xp <= 0 || (obj.entities[i].xp + obj.entities[i].w) >= 312 {
                    //it's on a screen edge
                    obj.copylinecross(i);
                }
            }
        }

        /* Disable all entities in the room, and deallocate any unnecessary entity slots. */
        /* However don't disable player entities, but do preserve holes between them (if any). */
        let mut player_found = false;
        // for int i = obj.entities.size() - 1; i >= 0; --i {
        for i in obj.entities.len()..=0 {
            /* Iterate in reverse order to prevent unnecessary indice shifting */
            if obj.entities[i].rule == 0 {
                player_found = true;
                continue;
            }

            if !player_found {
                obj.entities.remove(i);
            } else {
                obj.disableentity(i);
            }
        }

        game.door_up = rx + ((ry - 1) * 100);
        game.door_down = rx + ((ry + 1) * 100);
        game.door_right = rx + 1 + (ry * 100);
        game.door_left = rx -1 + (ry * 100);

        game.roomchangedir = match rx < game.roomx {
            true => 0,
            false => 1,
        };

        if self.finalmode {
            //Ok, what way are we moving?
            game.roomx = rx;
            game.roomy = ry;
            game.roomchange = true;

            if game.roomy < 10 {
                game.roomy = 11;
            }

            game.currentroomdeaths = match game.roomx >= 41 && game.roomy >= 48 && game.roomx < 61 && game.roomy < 68 {
                true => self.roomdeathsfinal[(game.roomx - 41 + (20 * (game.roomy - 48))) as usize],
                false => 0,
            };

            //Final level for time trial
            if game.intimetrial {
                if game.roomx == 46 && game.roomy == 54 {
                    //Final level remix
                    music.niceplay(15);
                }
            }
        }
        // #if !defined(NO_CUSTOM_LEVELS)
        // else if self.custommode {
        //     game.roomx = rx;
        //     game.roomy = ry;
        //     game.roomchange = true;
        //     if game.roomx < 100 { game.roomx = 100 + ed.mapwidth-1; }
        //     if game.roomy < 100 { game.roomy = 100 + ed.mapheight-1; }
        //     if game.roomx > 100 + ed.mapwidth-1 { game.roomx = 100; }
        //     if game.roomy > 100 + ed.mapheight-1 { game.roomy = 100; }
        // }
        // #endif
        else {
            game.roomx = rx;
            game.roomy = ry;
            game.roomchange = true;
            if game.roomx < 100 { game.roomx = 119; }
            if game.roomy < 100 { game.roomy = 119; }
            if game.roomx > 119 { game.roomx = 100; }
            if game.roomy > 119 { game.roomy = 100; }

            game.currentroomdeaths = self.roomdeaths[(game.roomx - 100 + (20 * (game.roomy - 100))) as usize];

            //Alright, change music depending on where we are:
            //Tower
            if game.roomx == 107 && game.roomy == 106 { music.niceplay(4); }
            if game.roomx == 107 && game.roomy == 107 { music.niceplay(4); }
            if game.roomx == 107 && game.roomy == 108 { music.niceplay(4); }
            if game.roomx == 107 && game.roomy == 109 { music.niceplay(4); }
            if game.roomx == 108 && game.roomy == 109 {
                if graphics.setflipmode {
                    music.niceplay(9);
                } else {
                    music.niceplay(2);
                }
            }
            if game.roomx == 109 {
                if graphics.setflipmode {
                    music.niceplay(9);
                } else {
                    music.niceplay(2);
                }
            }
            //Warp Zone
            if game.roomx == 112 && game.roomy == 101 { music.niceplay(4); }
            if game.roomx == 113 && game.roomy == 101 { music.niceplay(4); }
            if game.roomx == 113 && game.roomy == 102 { music.niceplay(4); }
            if game.roomx == 114 && game.roomy == 101 { music.niceplay(12); }
            if game.roomx == 115 && game.roomy == 101 { music.niceplay(12); }
            if game.roomx == 115 && game.roomy == 102 { music.niceplay(12); }
            //Lab
            if game.roomx == 101 && game.roomy == 115 { music.niceplay(4); }
            if game.roomx == 100 && game.roomy == 115 { music.niceplay(4); }
            if game.roomx == 101 && game.roomy == 116 { music.niceplay(4); }
            if game.roomx == 100 && game.roomy == 116 { music.niceplay(4); }
            if game.roomx == 102 && game.roomy == 116 { music.niceplay(3); }
            if game.roomx == 102 && game.roomy == 117 { music.niceplay(3); }
            if game.roomx == 101 && game.roomy == 117 { music.niceplay(3); }
            //Space Station
            if game.intimetrial {
                if game.roomx == 111 && game.roomy == 112 { music.niceplay(1); }
                if game.roomx == 111 && game.roomy == 113 { music.niceplay(1); }
                if game.roomx == 112 && game.roomy == 114 { music.niceplay(1); }
                if game.roomx == 112 && game.roomy == 115 { music.niceplay(1); }
            } else {
                if game.roomx == 111 && game.roomy == 112 { music.niceplay(1); }
                if game.roomx == 111 && game.roomy == 113 { music.niceplay(1); }
                if game.roomx == 112 && game.roomy == 114 { music.niceplay(4); }
                if game.roomx == 112 && game.roomy == 115 { music.niceplay(4); }
            }
            //Leaving the Ship
            if game.roomx == 104 && game.roomy == 112 { music.niceplay(4); }
        }
        // @sx: looks like orphaned code, because variable will be redefined with another value later without being used
        // let temp = rx + (ry * 100);
        self.loadlevel(game.roomx, game.roomy);

        //Do we need to reload the background?
        let redrawbg = (game.roomx != game.prevroomx) | (game.roomy != game.prevroomy);

        if redrawbg {
            graphics.backgrounddrawn = false; //Used for background caching speedup
        }
        graphics.foregrounddrawn = false; //Used for background caching speedup

        game.prevroomx = game.roomx;
        game.prevroomy = game.roomy;

        //a very special case: if entering the communication room, room 13,4 before tag 5 is set, set the game state to a background
        //textbox thingy. if tag five is not set when changing room, reset the game state. (tag 5 is set when you get back to the ship)
        if !game.intimetrial && !self.custommode {
            if !obj.flags[5] && !self.finalmode {
                game.state = 0;
                if game.roomx == 113 && game.roomy == 104 {
                    game.state = 50;
                }
            }
        }

        //Ok, kludge to fix lines in crossing rooms - if we're intersecting a gravity line right now, let's
        //set it to an inactive state.

        //Alright! So, let's look at our lines from the previous rooms, and determine if any of them are actually
        //continuations!

        let temp = obj.getplayer() as usize;
        if INBOUNDS_VEC!(temp, obj.entities) {
            obj.entities[temp].oldxp = obj.entities[temp].xp;
            obj.entities[temp].oldyp = obj.entities[temp].yp;
            obj.entities[temp].lerpoldxp = obj.entities[temp].xp - obj.entities[temp].vx as i32;
            obj.entities[temp].lerpoldyp = obj.entities[temp].yp - obj.entities[temp].vy as i32;
        }

        // for size_t i = 0; i < obj.entities.size(); i++ {
        for i in 0..obj.entities.len() {
            if obj.entities[i].r#type == 9 {
                //It's a horizontal line
                if obj.entities[i].xp <= 0 || obj.entities[i].xp + obj.entities[i].w >= 312 {
                    //it's on a screen edge
                    // for size_t j = 0; j < obj.linecrosskludge.size(); j++ {
                    for j in 0..obj.linecrosskludge.len() {
                        if obj.entities[i].yp == obj.linecrosskludge[j].yp {
                            //y's match, how about x's?
                            //we're moving left:
                            if game.roomchangedir == 0 {
                                if obj.entities[i].xp + obj.entities[i].w >= 312 && obj.linecrosskludge[j].xp <= 0 {
                                    obj.revertlinecross(i, j);
                                }
                            } else {
                                if obj.entities[i].xp <= 0 && obj.linecrosskludge[j].xp + obj.linecrosskludge[j].w >= 312 {
                                    obj.revertlinecross(i, j);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // std::string mapclass::currentarea(int t)
    pub fn  method(&mut self, t: i32) -> &str {
        println!("DEADBEEF: mapclass() method not implemented yet");
        &""
    }

    // void mapclass::loadlevel(int rx, int ry)
    pub fn loadlevel(&mut self, rx: i32, ry: i32) {
        println!("DEADBEEF: mapclass::loadlevel() method not implemented yet");


    }

    // void mapclass::twoframedelayfix(void)
    pub fn twoframedelayfix(&mut self) {
        println!("DEADBEEF: mapclass::twoframedelayfix() method not implemented yet");
    }
}
