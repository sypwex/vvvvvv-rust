use crate::{INBOUNDS_ARR, INBOUNDS_VEC, entity, game, music, screen::render::graphics::{self, towerbg}, script::{self, scripts}, utility_class};
mod towerclass;
mod otherlevel;
mod spacestation;

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
    otherlevel: otherlevel::otherlevelclass,
    spacestation2: spacestation::spacestation2class,
    // lablevel: labclass,
    // finallevel: finalclass,
    // warplevel: warpclass,
    pub tower: towerclass::TowerClass,
    pub extrarow: i32,

    //Accessibility options
    pub invincibility: bool,

    //Map cursor
    pub cursorstate: i32,
    pub cursordelay: i32,
}

impl Map {
    // mapclass::mapclass(void)
    pub fn new(graphics: &mut graphics::Graphics) -> Self {
        let mut vmult: [i32; 30] = [0; 30];
        //We init the lookup table:
	    for i in 0..30 {
		    vmult[i] = i as i32 * 40;
	    }

        let mut map = Map {
            roomdeaths: [0i32; 20*20],
            roomdeathsfinal: [0i32; 20*20],
            contents: [0i16; 40*30],
            explored: [false; 20*20],
            vmult,

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
            otherlevel: otherlevel::otherlevelclass::new(),
            spacestation2: spacestation::spacestation2class::new(),
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
    pub fn changefinalcol(&mut self, t: i32, obj: &mut entity::EntityClass) {
        //change the map to colour t - for the game's final stretch.
        //First up, the tiles. This is just a setting:
        self.final_mapcol = t;
        let temp = 5 - t;

        //Next, entities
        for i in 0..obj.entities.len() {
            if obj.entities[i].r#type == 1 {
                //something with a movement behavior
                if obj.entities[i].animate == 10 || obj.entities[i].animate == 11 {
                    //treadmill
                    if temp < 3 {
                        obj.entities[i].tile = 907 + (temp * 80);
                    } else {
                        obj.entities[i].tile = 911 + ((temp-3) * 80);
                    }
                    if obj.entities[i].animate == 10 {
                        obj.entities[i].tile += 40
                    };
                } else if obj.entities[i].isplatform {
                    obj.entities[i].tile = 915 + (temp*40);
                } else {
                    //just an enemy
                    obj.entities[i].colour = self.maptiletoenemycol(temp);
                }
            } else if obj.entities[i].r#type == 2 {
                //disappearing platforms
                obj.entities[i].tile = 915 + (temp*40);
            }
        }
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
        //THIS IS THE BUG
        if self.finalmode {
            return 6
        } else {
            let lookup = (_rx - 100) + ((_ry - 100) * 20);
            if _rx-100 >= 0 && _rx-100 < 20 && _ry-100 >= 0 && _ry-100 < 20 {
                return areamap[lookup as usize]
            } else {
                return 6
            }
        }
    }

    // bool mapclass::isexplored(const int rx, const int ry)
    pub fn isexplored(&mut self, rx: i32, ry: i32) -> bool {
        println!("DEADBEEF: mapclass::isexplored() method not implemented yet");
        false
    }

    // void mapclass::setexplored(const int rx, const int ry, const bool status)
    pub fn setexplored(&mut self, rx: i32, ry: i32, status: bool) {
        let roomnum = (rx + ry*20) as usize;
        if INBOUNDS_ARR!(roomnum, self.explored) {
            self.explored[roomnum] = status;
        }
    }

    // void mapclass::exploretower(void)
    pub fn exploretower(&mut self) {
        println!("DEADBEEF: mapclass::exploretower() method not implemented yet");
    }

    // void mapclass::hideship(void)
    pub fn hideship(&mut self) {
        //remove the ship from the explored areas
        self.setexplored(2, 10, false);
        self.setexplored(3, 10, false);
        self.setexplored(4, 10, false);
        self.setexplored(2, 11, false);
        self.setexplored(3, 11, false);
        self.setexplored(4, 11, false);
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
    pub fn gotoroom(&mut self, rx: i32, ry: i32, game: &mut game::Game, graphics: &mut graphics::Graphics, music: &mut music::Music, obj: &mut entity::EntityClass, help: &mut utility_class::UtilityClass) {
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
        // for let i = obj.entities.size() - 1; i >= 0; --i {
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
        self.loadlevel(game.roomx, game.roomy, game, graphics, obj, help);

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
    pub fn loadlevel(&mut self, rx: i32, ry: i32, game: &mut game::Game, graphics: &mut graphics::Graphics, obj: &mut entity::EntityClass, help: &mut utility_class::UtilityClass) {
        let mut t;
        if !self.finalmode {
            self.setexplored(rx - 100, ry - 100, true);
            if rx == 109 && !self.custommode {
                self.exploretower();
            }
        }

        self.roomtexton = false;
        self.roomtext.clear();

        obj.platformtile = 0;
        obj.customplatformtile = 0;
        obj.vertplatforms = false;
        obj.horplatforms = false;
        self.roomname = "".to_string();
        self.hiddenname = "".to_string();
        self.background = 1;
        self.warpx = false;
        self.warpy = false;

        self.towermode = false;
        self.ypos = 0;
        self.oldypos = 0;
        self.extrarow = 0;
        self.spikeleveltop = 0;
        self.spikelevelbottom = 0;
        self.oldspikeleveltop = 0;
        self.oldspikelevelbottom = 0;

        //Custom stuff for warplines
        obj.customwarpmode=false;
        obj.customwarpmodevon=false;
        obj.customwarpmodehon=false;

        if self.finalmode {
            t = 6;
            //check if we're in the towers
            if rx == 49 && ry == 52 {
                //entered tower 1
                t = 7;
            } else if rx == 49 && ry == 53 {
                //re entered tower 1
                t = 8;
            } else if rx == 51 && ry == 54 {
                //entered tower 2
                t = 9;
            } else if rx == 51 && ry == 53 {
                //re entered tower 2
                t = 10;
            }
        } else if self.custommode {
            t = 12;
        } else {
            t = self.area(rx, ry);

            if t == 3 {
                //correct position for tower
                if ry == 109 {
                    //entered from ground floor
                    let player = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(player, obj.entities) {
                        obj.entities[player].yp += 671 * 8;
                    }

                    self.ypos = (700-29) * 8;
                    self.oldypos = self.ypos;
                    graphics.buffers.towerbg.bypos = self.ypos / 2;
                    self.cameramode = 0;
                    graphics.buffers.towerbg.colstate = 0;
                    self.colsuperstate = 0;
                } else if ry == 104 {
                    //you've entered from the top floor
                    self.ypos = 0;
                    self.oldypos = self.ypos;
                    graphics.buffers.towerbg.bypos = 0;
                    self.cameramode = 0;
                    graphics.buffers.towerbg.colstate = 0;
                    self.colsuperstate = 0;
                }
            }

            if t < 2 {
                //on the world map, want to test if we're in the secret lab
                if rx >= 116 {
                    if ry >= 105 {
                        if ry <= 107 {
                            if rx == 119 && ry == 105 {
                                //Ah, this is just a normal area
                            } else {
                                //in the secret lab! Crazy background!
                                self.background = 2;
                                if rx == 116 && ry == 105 { graphics.rcol = 1; }
                                if rx == 117 && ry == 105 { graphics.rcol = 5; }
                                if rx == 118 && ry == 105 { graphics.rcol = 4; }
                                if rx == 117 && ry == 106 { graphics.rcol = 2; }
                                if rx == 118 && ry == 106 { graphics.rcol = 0; }
                                if rx == 119 && ry == 106 { graphics.rcol = 3; }
                                if rx == 119 && ry == 107 { graphics.rcol = 1; }
                            }
                        }
                    }
                }
            }
        }

        if rx == 119 && ry == 108 && !self.custommode {
            self.background = 5;
            graphics.rcol = 3;
            self.warpx = true;
            self.warpy = true;
        }

        match t {
            // #if !defined(MAKEANDPLAY)
            0 | 1 => {
                //World Map
                self.tileset = 1;
                self.extrarow = 1;
                self.contents = self.otherlevel.loadlevel(rx, ry, game, obj);
                self.roomname = self.otherlevel.roomname.to_owned();
                self.tileset = self.otherlevel.roomtileset;
                //do the appear/remove roomname here

                if game.roomx >= 102 && game.roomx <= 104 && game.roomy >= 110 && game.roomy <= 111 {
                    self.hiddenname = "The Ship".to_string();
                } else {
                    self.hiddenname = "Dimension VVVVVV".to_string();
                }
            },
            // 2 => {
            //     //The Lab
            //     self.contents = self.lablevel.loadlevel(rx, ry);
            //     self.roomname = self.lablevel.roomname;
            //     self.tileset = 1;
            //     self.background = 2;
            //     graphics.rcol = self.lablevel.rcol;
            // },
            3 => {
                //The Tower
                graphics.buffers.towerbg.tdrawback = true;
                self.minitowermode = false;
                self.tower.minitowermode = false;
                graphics.buffers.towerbg.bscroll = 0;
                graphics.buffers.towerbg.scrolldir = 0;

                self.roomname = "The Tower".to_string();
                self.tileset = 1;
                self.background = 3;
                self.towermode = true;
                //graphics.buffers.towerbg.bypos = 0; ypos = 0; cameramode = 0;

                //All the entities for here are just loaded here; it's essentially one room after all
                obj.createentity(48, 5456, 10, Some(1), Some(505007), None, None, None, None, game); // (savepoint)
                obj.createentity(224, 4528, 10, Some(1), Some(505017), None, None, None, None, game); // (savepoint)
                obj.createentity(232, 4168, 10, Some(0), Some(505027), None, None, None, None, game); // (savepoint)
                obj.createentity(280, 3816, 10, Some(1), Some(505037), None, None, None, None, game); // (savepoint)
                obj.createentity(152, 3552, 10, Some(1), Some(505047), None, None, None, None, game); // (savepoint)
                obj.createentity(216, 3280, 10, Some(0), Some(505057), None, None, None, None, game); // (savepoint)
                obj.createentity(216, 4808, 10, Some(1), Some(505067), None, None, None, None, game); // (savepoint)
                obj.createentity(72, 3096, 10, Some(0), Some(505077), None, None, None, None, game); // (savepoint)
                obj.createentity(176, 2600, 10, Some(0), Some(505087), None, None, None, None, game); // (savepoint)
                obj.createentity(216, 2392, 10, Some(0), Some(505097), None, None, None, None, game); // (savepoint)
                obj.createentity(152, 1184, 10, Some(1), Some(505107), None, None, None, None, game); // (savepoint)
                obj.createentity(152, 912, 10, Some(1), Some(505117), None, None, None, None, game); // (savepoint)
                obj.createentity(152, 536, 10, Some(1), Some(505127), None, None, None, None, game); // (savepoint)
                obj.createentity(120, 5136, 10, Some(0), Some(505137), None, None, None, None, game); // (savepoint)
                obj.createentity(144, 1824, 10, Some(0), Some(505147), None, None, None, None, game); // (savepoint)
                obj.createentity(72, 2904, 10, Some(0), Some(505157), None, None, None, None, game); // (savepoint)
                obj.createentity(224, 1648, 10, Some(1), Some(505167), None, None, None, None, game); // (savepoint)
                obj.createentity(112, 5280, 10, Some(1), Some(50517), None, None, None, None, game); // (savepoint)

                obj.createentity(24, 4216, 9, Some(7), None, None, None, None, None, game); // (shiny trinket)
                obj.createentity(280, 3216, 9, Some(8), None, None, None, None, None, game); // (shiny trinket)
            },
            // 4 => {
            //     //The Warpzone
            //     self.contents = self.warplevel.loadlevel(rx, ry);
            //     self.roomname = self.warplevel.roomname;
            //     self.tileset = 1;
            //     self.background = 3;
            //     graphics.rcol = self.warplevel.rcol;
            //     graphics.backgrounddrawn = false;

            //     self.warpx = self.warplevel.warpx;
            //     self.warpy = self.warplevel.warpy;
            //     self.background = 5;
            //     if self.warpy { self.background = 4; }
            //     if self.warpx { self.background = 3; }
            //     if self.warpx && self.warpy { self.background = 5; }
            // },
            5 => {
                //Space station
                self.contents = self.spacestation2.loadlevel(rx, ry, game, help, obj);
                self.roomname = self.spacestation2.roomname.to_owned();
                self.tileset = 0;
            },
            // 6 => {
            //     //final level
            //     self.contents = self.finallevel.loadlevel(rx, ry);
            //     self.roomname = self.finallevel.roomname;
            //     self.tileset = 1;
            //     self.background = 3;
            //     graphics.backgrounddrawn = false;

            //     if self.finalstretch {
            //         self.background = 6;
            //     } else {
            //         self.warpx = self.finallevel.warpx;
            //         self.warpy = self.finallevel.warpy;
            //         self.background = 5;
            //         if self.warpy { self.background = 4; }
            //         if self.warpx { self.background = 3; }
            //         if self.warpx && self.warpy { self.background = 5; }
            //     }

            //     graphics.rcol = 6;
            //     self.changefinalcol(self.final_mapcol, obj);
            // },
            7 => {
                //Final Level, Tower 1
                graphics.buffers.towerbg.tdrawback = true;
                self.minitowermode = true;
                self.tower.minitowermode = true;
                graphics.buffers.towerbg.bscroll = 0;
                graphics.buffers.towerbg.scrolldir = 1;

                self.roomname = "Panic Room".to_string();
                self.tileset = 1;
                self.background = 3;
                self.towermode = true;

                self.tower.loadminitower1();

                self.ypos = 0;
                self.oldypos = 0;
                graphics.buffers.towerbg.bypos = 0;
                self.cameramode = 0;
                graphics.buffers.towerbg.colstate = 0;
                self.colsuperstate = 0;
            },
            8 => {
                //Final Level, Tower 1 (reentered from below)
                graphics.buffers.towerbg.tdrawback = true;
                self.minitowermode = true;
                self.tower.minitowermode = true;
                graphics.buffers.towerbg.bscroll = 0;
                graphics.buffers.towerbg.scrolldir = 1;

                self.roomname = "Panic Room".to_string();
                self.tileset = 1;
                self.background = 3;
                self.towermode = true;

                self.tower.loadminitower1();

                let i = obj.getplayer() as usize;
                if INBOUNDS_VEC!(i, obj.entities) {
                    obj.entities[i].yp += 71 * 8;
                }
                game.roomy -= 1;

                self.ypos = (100-29) * 8;
                self.oldypos = self.ypos;
                graphics.buffers.towerbg.bypos = self.ypos/2;
                self.cameramode = 0;
                graphics.buffers.towerbg.colstate = 0;
                self.colsuperstate = 0;
            },
            9 => {
                //Final Level, Tower 2
                graphics.buffers.towerbg.tdrawback = true;
                self.minitowermode = true;
                self.tower.minitowermode = true;
                graphics.buffers.towerbg.bscroll = 0;
                graphics.buffers.towerbg.scrolldir = 0;
                self.final_colorframe = 2;

                self.roomname = "The Final Challenge".to_string();
                self.tileset = 1;
                self.background = 3;
                self.towermode = true;

                self.tower.loadminitower2();

                obj.createentity(56, 556, 11, Some(136), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(184, 592, 10, Some(0), Some(50500), None, None, None, None, game); // (savepoint)
                obj.createentity(184, 644, 11, Some(88), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(56, 460, 11, Some(136), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(216, 440, 10, Some(0), Some(50501), None, None, None, None, game); // (savepoint)
                obj.createentity(104, 508, 11, Some(168), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(219, 264, 12, Some(56), None, None, None, None, None, game); // (vertical gravity line)
                obj.createentity(120, 332, 11, Some(96), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(219, 344, 12, Some(56), None, None, None, None, None, game); // (vertical gravity line)
                obj.createentity(224, 332, 11, Some(48), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(56, 212, 11, Some(144), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(32, 20, 11, Some(96), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(72, 156, 11, Some(200), None, None, None, None, None, game); // (horizontal gravity line)

                let i = obj.getplayer() as usize;
                if INBOUNDS_VEC!(i, obj.entities) {
                    obj.entities[i].yp += 71 * 8;
                }
                game.roomy -= 1;

                self.ypos = (100-29) * 8;
                self.oldypos = self.ypos;
                graphics.buffers.towerbg.bypos = self.ypos/2;
                self.cameramode = 0;
                graphics.buffers.towerbg.colstate = 0;
                self.colsuperstate = 0;
            },
            10 => {
                //Final Level, Tower 2
                graphics.buffers.towerbg.tdrawback = true;
                self.minitowermode = true;
                self.tower.minitowermode = true;
                graphics.buffers.towerbg.bscroll = 0;
                graphics.buffers.towerbg.scrolldir = 0;
                self.final_colorframe = 2;

                self.roomname = "The Final Challenge".to_string();
                self.tileset = 1;
                self.background = 3;
                self.towermode = true;

                self.tower.loadminitower2();

                obj.createentity(56, 556, 11, Some(136), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(184, 592, 10, Some(0), Some(50500), None, None, None, None, game); // (savepoint)
                obj.createentity(184, 644, 11, Some(88), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(56, 460, 11, Some(136), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(216, 440, 10, Some(0), Some(50501), None, None, None, None, game); // (savepoint)
                obj.createentity(104, 508, 11, Some(168), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(219, 264, 12, Some(56), None, None, None, None, None, game); // (vertical gravity line)
                obj.createentity(120, 332, 11, Some(96), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(219, 344, 12, Some(56), None, None, None, None, None, game); // (vertical gravity line)
                obj.createentity(224, 332, 11, Some(48), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(56, 212, 11, Some(144), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(32, 20, 11, Some(96), None, None, None, None, None, game); // (horizontal gravity line)
                obj.createentity(72, 156, 11, Some(200), None, None, None, None, None, game); // (horizontal gravity line)

                self.ypos = 0;
                self.oldypos = 0;
                graphics.buffers.towerbg.bypos = 0;
                self.cameramode = 0;
                graphics.buffers.towerbg.colstate = 0;
                self.colsuperstate = 0;
            },
            // 11 => {
            //     //Tower Hallways //Content is held in final level routine
            //     self.contents = self.finallevel.loadlevel(rx, ry);
            //     self.roomname = self.finallevel.roomname;
            //     self.tileset = 2;
            //     if rx == 108 {
            //         self.background = 7;
            //         self.rcol = 15;
            //     }
            //     if rx == 110 {
            //         self.background = 8;
            //         self.rcol = 10;
            //     }
            //     if rx == 111 {
            //         self.background = 9;
            //         self.rcol = 0;
            //     }
            // },
            // #endif
            // #if !defined(NO_CUSTOM_LEVELS)
            12 => {
                // //Custom level
                // let room = ed.getroomprop(rx - 100, ry - 100);
                // game.customcol = ed.getlevelcol(room.tileset, room.tilecol) + 1;
                // obj.customplatformtile = game.customcol * 12;

                // match room.tileset {
                //     0 => {
                //         // Space Station
                //         self.tileset = 0;
                //         self.background = 1;
                //     },
                //     1 => {
                //         // Outside
                //         self.tileset = 1;
                //         self.background = 1;
                //     },
                //     2 => {
                //         // Lab
                //         self.tileset = 1;
                //         self.background = 2;
                //         graphics.rcol = room.tilecol;
                //     },
                //     3 => {
                //         // Warp Zone/intermission
                //         self.tileset = 1;
                //         self.background = 6;
                //     },
                //     4 => {
                //         // Ship
                //         self.tileset = 1;
                //         self.background = 1;
                //     },
                //     _ => {
                //         self.tileset = 1;
                //         self.background = 1;
                //     },
                // };

                // // If screen warping, then override all that:
                // let redrawbg = game.roomx != game.prevroomx || game.roomy != game.prevroomy;
                // if redrawbg {
                //     graphics.backgrounddrawn = false;
                // }

                // match room.warpdir {
                //     1 => {
                //         self.warpx = true;
                //         self.background = 3;
                //         graphics.rcol = ed.getwarpbackground(rx - 100, ry - 100);
                //     },
                //     2 => {
                //         self.warpy = true;
                //         self.background = 4;
                //         graphics.rcol = ed.getwarpbackground(rx - 100, ry - 100);
                //     },
                //     3 => {
                //         self.warpx = true;
                //         self.warpy = true;
                //         self.background = 5;
                //         graphics.rcol = ed.getwarpbackground(rx - 100, ry - 100);
                //     },
                //     _ => (),
                // };

                // self.roomname = room.roomname;
                // self.extrarow = 1;
                // let tmap = ed.loadlevel(rx, ry);
                // SDL_memcpy(contents, tmap, sizeof(contents));

                // self.roomtexton = false;
                // self.roomtext.clear();

                // // Entities have to be created HERE, akwardly
                // let mut tempcheckpoints = 0;
                // let mut tempscriptbox = 0;
                // // for (size_t edi = 0; edi < edentity.size(); edi++) {
                // for edi in 0..edentity.len() {
                //     // If entity is in this room, create it
                //     let tsx = ent.x / 40;
                //     let tsy = ent.y / 30;

                //     if tsx != rx-100 || tsy != ry-100 {
                //         continue;
                //     }

                //     let ex = (edentity[edi].x % 40) * 8;
                //     let ey = (edentity[edi].y % 30) * 8;

                //     // Platform and enemy bounding boxes
                //     let mut bx1 = 0;
                //     let mut by1 = 0;
                //     let mut bx2 = 0;
                //     let mut by2 = 0;

                //     let enemy = edentity[edi].t == 1;
                //     let moving_plat = edentity[edi].t == 2 && edentity[edi].p1 <= 4;
                //     if enemy || moving_plat {
                //         if enemy {
                //             bx1 = room.enemyx1;
                //             by1 = room.enemyy1;
                //             bx2 = room.enemyx2;
                //             by2 = room.enemyy2;
                //         }
                //         else if moving_plat {
                //             bx1 = room.platx1;
                //             by1 = room.platy1;
                //             bx2 = room.platx2;
                //             by2 = room.platy2;
                //         }

                //         // Enlarge bounding boxes to fix warping entities
                //         if self.warpx && bx1 == 0 && bx2 == 320 {
                //             bx1 -= 100;
                //             bx2 += 100;
                //         }
                //         if self.warpy && by1 == 0 && by2 == 240 {
                //             by1 -= 100;
                //             by2 += 100;
                //         }
                //     }

                //     match edentity[edi].t {
                //         1 => {
                //             // Enemies
                //             obj.customenemy = room.enemytype;
                //             obj.createentity(ex, ey, 56, edentity[edi].p1, 4, bx1, by1, bx2, by2);
                //         },
                //         2 => {
                //             // Platforms and conveyors
                //             if edentity[edi].p1 <= 4 {
                //                 obj.createentity(ex, ey, 2, edentity[edi].p1, room.platv, bx1, by1, bx2, by2);
                //             } else if edentity[edi].p1 >= 5 && edentity[edi].p1 <= 8 {
                //                 // Conveyor
                //                 obj.createentity(ex, ey, 2, edentity[edi].p1 + 3, 4);
                //             }
                //         },
                //         3 => {
                //             // Disappearing platforms
                //             obj.createentity(ex, ey, 3);
                //         },
                //         9 => {
                //             // Trinkets
                //             obj.createentity(ex, ey, 9, ed.findtrinket(edi));
                //         },
                //         10 => {
                //             // Checkpoints
                //             obj.createentity(ex, ey, 10, edentity[edi].p1, (rx + ry*100) * 20 + tempcheckpoints);
                //             tempcheckpoints += 1;
                //         },
                //         11 => {
                //             // Gravity Lines
                //             if edentity[edi].p1 == 0 {
                //                 //Horizontal
                //                 obj.createentity(edentity[edi].p2 * 8, ey + 4, 11, edentity[edi].p3);
                //             } else {
                //                 //Vertical
                //                 obj.createentity(ex + 3, edentity[edi].p2 * 8, 12, edentity[edi].p3);
                //             }
                //         },
                //         13 => {
                //             // Warp Tokens
                //             obj.createentity(ex, ey, 13, edentity[edi].p1, edentity[edi].p2);
                //         },
                //         15 => {
                //             // Collectable crewmate
                //             obj.createentity(ex - 4, ey + 1, 55, ed.findcrewmate(edi), edentity[edi].p1, edentity[edi].p2);
                //         },
                //         17 => {
                //             // Roomtext!
                //             self.roomtexton = true;
                //             let text = Roomtext {
                //                 x: ex / 8,
                //                 y: ey / 8,
                //                 text: edentity[edi].scriptname,
                //             };

                //             self.roomtext.push(text);
                //         },
                //         18 => {
                //             // Terminals
                //             obj.customscript = edentity[edi].scriptname;

                //             let mut usethistile = edentity[edi].p1;
                //             let mut usethisy = ey;

                //             // This isn't a boolean: we just swap 0 and 1 around and leave the rest alone
                //             if usethistile == 0 {
                //                 usethistile = 1; // Unflipped
                //             } else if usethistile == 1 {
                //                 usethistile = 0; // Flipped;
                //                 usethisy -= 8;
                //             }

                //             obj.createentity(ex, usethisy + 8, 20, usethistile);
                //             obj.createblock(ACTIVITY, ex - 8, usethisy + 8, 20, 16, 35);
                //         },
                //         19 => {
                //             //Script Box
                //             if INBOUNDS_ARR!(tempscriptbox, game.customscript) {
                //                 game.customscript[tempscriptbox] = edentity[edi].scriptname;
                //             }
                //             obj.createblock(TRIGGER, ex, ey, edentity[edi].p1 * 8, edentity[edi].p2 * 8, 300 + tempscriptbox, "custom_" + edentity[edi].scriptname);
                //             tempscriptbox += 1;
                //         },
                //         50 => {
                //             // Warp Lines
                //             obj.customwarpmode = true;
                //             match edentity[edi].p1 {
                //                 0 => {
                //                     // Vertical, left
                //                     obj.createentity(ex + 4, edentity[edi].p2 * 8, 51, edentity[edi].p3);
                //                 },
                //                 1 => {
                //                     //Horizontal, right
                //                     obj.createentity(ex + 4, edentity[edi].p2 * 8, 52, edentity[edi].p3);
                //                 },
                //                 2 => {
                //                     //Vertical, top
                //                     obj.createentity(edentity[edi].p2 * 8, ey + 7, 53, edentity[edi].p3);
                //                 },
                //                 3 => {
                //                     // Horizontal, bottom
                //                     obj.createentity(edentity[edi].p2 * 8, ey, 54, edentity[edi].p3);
                //                 },
                //                 _ => (),
                //             }
                //         },
                //     }
                // }

                // //do the appear/remove roomname here
            }
            // #endif

            _ => println!("map::loadlevel(): loading level {} not implemented yet", t),
        }

        println!("map::loadlevel(): region ({}) - current room ({}) @ ({},{}) with tileset ({})", t, self.roomname, rx, ry, self.tileset);

        //The room's loaded: now we fill out damage blocks based on the tiles.
        if !self.towermode {
            // for int j = 0; j < 29 + extrarow; j++ {
            for j in 0..(29+self.extrarow) {
                // for (let i = 0; i < 40; i++
                for i in 0..40 {
                    //Damage blocks
                    if self.tileset == 0 {
                        if self.contents[i + self.vmult[j as usize] as usize] == 6 || self.contents[i + self.vmult[j as usize] as usize] == 8 {
                            //sticking up
                            obj.createblock(2, (i * 8) as i32, (j * 8)+4, 8, 4, None, None);
                        }
                        if self.contents[i + self.vmult[j as usize] as usize] == 7 || self.contents[i + self.vmult[j as usize] as usize] == 9 {
                            //Sticking down
                            obj.createblock(2, (i * 8) as i32, (j * 8) as i32, 8, 4, None, None);
                        }
                        if self.contents[i + self.vmult[j as usize] as usize] == 49 || self.contents[i + self.vmult[j as usize] as usize] == 50 {
                            //left or right
                            obj.createblock(2, (i * 8) as i32, (j * 8)+3, 8, 2, None, None);
                        }
                    } else if self.tileset == 1 {
                        //if self.contents[i + self.vmult[j as usize] as usize] >= 6 && self.contents[i + self.vmult[j as usize] as usize] <= 9) obj.createblock(2, (i * 8) as i32, (j * 8)+1, 8, 6);
                        //if self.contents[i + self.vmult[j as usize] as usize] >= 49 && self.contents[i + self.vmult[j as usize] as usize] <= 79) obj.createblock(2, (i * 8) as i32 + 1, (j * 8) + 1, 6, 6);
                        if (
                            self.contents[i + self.vmult[j as usize] as usize] >= 63 &&
                            self.contents[i + self.vmult[j as usize] as usize] <= 74
                        ) || (
                            self.contents[i + self.vmult[j as usize] as usize] >= 6 &&
                            self.contents[i + self.vmult[j as usize] as usize] <= 9
                        ) {
                            //sticking up) {
                            if self.contents[i + self.vmult[j as usize] as usize] < 10 {
                                self.contents[i + self.vmult[j as usize] as usize] += 1;
                            }
                            //sticking up
                            if self.contents[i + self.vmult[j as usize] as usize] % 2 == 0 {
                                obj.createblock(2, (i * 8) as i32, (j * 8) as i32, 8, 4, None, None);
                            } else {
                                //Sticking down
                                obj.createblock(2, (i * 8) as i32, (j * 8) + 4, 8, 4, None, None);
                            }
                            if self.contents[i + self.vmult[j as usize] as usize] < 11 {
                                self.contents[i + self.vmult[j as usize] as usize] -= 1;
                            }
                        }
                        if self.contents[i + self.vmult[j as usize] as usize] >= 49 && self.contents[i + self.vmult[j as usize] as usize] <= 62 {
                            //left or right
                            obj.createblock(2, (i * 8) as i32, (j * 8)+3, 8, 2, None, None);
                        }
                    } else if self.tileset == 2 {
                        if self.contents[i + self.vmult[j as usize] as usize] == 6 || self.contents[i + self.vmult[j as usize] as usize] == 8 {
                            //sticking up
                            obj.createblock(2, (i * 8) as i32, (j * 8)+4, 8, 4, None, None);
                        }
                        if self.contents[i + self.vmult[j as usize] as usize] == 7 || self.contents[i + self.vmult[j as usize] as usize] == 9 {
                            //Sticking down
                            obj.createblock(2, (i * 8) as i32, (j * 8) as i32, 8, 4, None, None);
                        }
                    }
                    //Breakable blocks
                    if self.contents[i + self.vmult[j as usize] as usize] == 10 {
                        self.contents[i + self.vmult[j as usize] as usize] = 0;
                        obj.createentity(i as i32 * 8, j * 8, 4, None, None, None, None, None, None, game);
                    }
                    //Directional blocks
                    if self.contents[i + self.vmult[j as usize] as usize] >= 14 && self.contents[i + self.vmult[j as usize] as usize] <= 17 {
                        obj.createblock(3, i as i32 * 8, j * 8, 8, 8, Some(self.contents[i + self.vmult[j as usize] as usize] as i32 - 14), None);
                    }
                }
            }

            // for size_t i = 0; i < obj.entities.size(); i++ {
            for i in 0..obj.entities.len() {
                if obj.entities[i].r#type == 1 && obj.entities[i].behave >= 8 && obj.entities[i].behave < 10 {
                    //put a block underneath
                    let temp = obj.entities[i].xp / 8;
                    let temp2 = obj.entities[i].yp / 8;

                    self.settile(temp, temp2, 1);
                    self.settile(temp+1, temp2, 1);
                    self.settile(temp+2, temp2, 1);
                    self.settile(temp+3, temp2, 1);
                    if obj.entities[i].w == 64 {
                        self.settile(temp+4, temp2, 1);
                        self.settile(temp+5, temp2, 1);
                        self.settile(temp+6, temp2, 1);
                        self.settile(temp+7, temp2, 1);
                    }
                }
            }
        }

        //Special scripting: Create objects and triggers based on what crewmembers are rescued.
        if !self.finalmode && !self.custommode {
            //First up: the extra bits:
            //Vermilion's quest:
            if rx == 100 && ry == 105 {
                //On path to verdigris
                if game.crewstats[3] && !game.crewstats[4] {
                    obj.createentity(87, 105, 18, Some(15), Some(0), Some(18), None, None, None, game);
                    obj.createblock(5, 87-32, 0, 32+32+32, 240, Some(3), None);
                }
            } else if rx == 107 && ry == 100 {
                //victoria
                if game.crewstats[3] && !game.crewstats[5] {
                    obj.createentity(140, 137, 18, Some(15), Some(0), Some(18), None, None, None, game);
                    obj.createblock(5, 140-32, 0, 32+32+32, 240, Some(3), None);
                }
            } else if rx == 114 && ry == 109 {
                if game.crewstats[3] && !game.crewstats[2] {
                    obj.createentity(235, 81, 18, Some(15), Some(0), Some(18), None, None, None, game);
                    obj.createblock(5, 235-32, 0, 32+32+32, 240, Some(3), None);
                }
            }

            //Verdigris fixing the ship
            if rx == 101 && ry == 109 {
                if game.crewstats[4] {
                    if game.crewrescued() > 4 && game.crewrescued() != 6 {
                        obj.createentity(175, 121, 18, Some(13), Some(0), Some(18), None, None, None, game);
                        obj.createblock(5, 175-32, 0, 32+32+32, 240, Some(4), None);
                    }
                }
            } else if rx == 103 && ry == 109 {
                if game.crewstats[4] {
                    if game.crewrescued() <= 4 && game.crewrescued() != 6 {
                        obj.createentity(53, 161, 18, Some(13), Some(1), Some(18), None, None, None, game);
                        obj.createblock(5, 53-32, 0, 32+32+32, 240, Some(4), None);
                    }
                }
            }

            if rx == 104 && ry == 111 {
                //Red
                //First: is he rescued?
                if game.crewstats[3] {
                    //If so, red will always be at his post
                    obj.createentity(107, 121, 18, Some(15), Some(0), Some(18), None, None, None, game);
                    //What script do we use?
                    obj.createblock(5, 107-32, 0, 32+32+32, 240, Some(3), None);
                }
            } else if rx == 103 && ry == 111 {
                //Yellow
                //First: is he rescued?
                if game.crewstats[2] {
                    obj.createentity(198, 105, 18, Some(14), Some(0), Some(18), None, None, None, game);
                    //What script do we use?
                    obj.createblock(5, 198-32, 0, 32+32+32, 240, Some(2), None);
                }
            } else if rx == 103 && ry == 110 {
                //Green
                //First: is he rescued?
                if game.crewstats[4] {
                    obj.createentity(242, 177, 18, Some(13), Some(0), Some(18), None, None, None, game);
                    //What script do we use?
                    obj.createblock(5, 242-32, 177-20, 32+32+32, 40, Some(4), None);
                }
            } else if rx == 104 && ry == 110 {
                //Purple
                //First: is she rescued?
                if game.crewstats[1] {
                    obj.createentity(140, 177, 18, Some(20), Some(0), Some(18), None, None, None, game);
                    //What script do we use?
                    obj.createblock(5, 140-32, 0, 32+32+32, 240, Some(1), None);
                }
            } else if rx == 102 && ry == 110 {
                //Blue
                //First: is she rescued?
                if game.crewstats[5] {
                    //A slight varation - she's upside down
                    obj.createentity(249, 62, 18, Some(16), Some(0), Some(18), None, None, None, game);
                    let j = obj.getcrewman(5) as usize;
                    if INBOUNDS_VEC!(j, obj.entities) {
                        obj.entities[j as usize].rule = 7;
                        obj.entities[j as usize].tile +=6;
                    }
                    //What script do we use?
                    obj.createblock(5, 249-32, 0, 32+32+32, 240, Some(5), None);
                }
            }
        }
    }

    // void mapclass::twoframedelayfix(void)
    pub fn twoframedelayfix(&mut self, game: &mut game::Game, obj: &mut entity::EntityClass, script: &mut script::ScriptClass, help: &mut utility_class::UtilityClass) {
        // Fixes the two-frame delay in custom levels that use scripts to spawn an entity upon room load.
	    // Because when the room loads and newscript is set to run, newscript has already ran for that frame,
	    // and when the script gets loaded script.run() has already ran for that frame, too.
	    // A bit kludge-y, but it's the least we can do without changing the frame ordering.

	    if game.glitchrunnermode || !self.custommode || game.deathseq != -1 {
            return;
        }

        let mut block_idx: i32 = -1;
        // obj.checktrigger() sets block_idx
        let activetrigger = obj.checktrigger(&mut block_idx, help);
        if activetrigger <= -1 || !INBOUNDS_VEC!(block_idx, obj.blocks) || activetrigger < 300 {
            return;
        }

        game.newscript = obj.blocks[block_idx as usize].script.to_owned();
        obj.removetrigger(activetrigger);
        game.state = 0;
        game.statedelay = 0;
        scripts::load(script, &game.newscript);
    }
}
