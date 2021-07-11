use crate::{INBOUNDS_VEC, game, map, maths, music, screen::render::graphics, utility_class::{self, UtilityClass}};
mod ent;
mod blockv;

pub struct EntityClass {
    pub entities: Vec<ent::EntClass>,
    pub linecrosskludge: Vec<ent::EntClass>,

    k: i32,

    // std::vector<blockclass> blocks,
    pub blocks: Vec<blockv::BlockClass>,
    pub flags: [bool; 100],
    pub collect: [bool; 100],
    pub customcollect: [bool; 100],

    platformtile: i32,
    pub vertplatforms: bool,
    pub horplatforms: bool,

    // :(
    pub nearelephant: bool,
    pub upsetmode: bool,
    pub upset: i32,

    //Trophy Text
    pub trophytext: i32,
    pub trophytype: i32,
    pub oldtrophytext: i32,

    //Secret lab scripts
    pub altstates: i32,

    //Custom stuff
    customenemy: i32,
    customplatformtile: i32,
    pub customwarpmode: bool,
    pub customwarpmodevon: bool,
    pub customwarpmodehon: bool,
    customscript: String,
    pub customcrewmoods: [bool; game::numcrew],
}

impl<'a> EntityClass {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            linecrosskludge: Vec::new(),

            k: 0,

            blocks: Vec::new(),
            flags: [false; 100],
            collect: [false; 100],
            customcollect: [false; 100],

            platformtile: 0,
            vertplatforms: false,
            horplatforms: false,

            // :(
            nearelephant: false,
            upsetmode: false,
            upset: 0,

            //Trophy Text
            trophytext: 0,
            trophytype: 0,
            oldtrophytext: 0,

            //Secret lab scripts
            altstates: 0,

            //Custom stuff
            customenemy: 0,
            customplatformtile: 0,
            customwarpmode: false,
            customwarpmodevon: false,
            customwarpmodehon: false,
            customscript: String::new(),
            customcrewmoods: [true; game::numcrew],
        }
    }

    // bool entityclass::checktowerspikes(int t)
    pub fn checktowerspikes(&mut self, t: i32) -> bool {
        println!("DEADBEEF: entityclass::checktowerspikes() method not implemented yet");
        false
    }

    // void entityclass::init(void)

    // void entityclass::resetallflags(void)
    pub fn resetallflags(&mut self) {
        self.flags = [false; 100];
    }

    // int entityclass::swncolour( int t )
    fn swncolour(&mut self, t: i32) -> i32 {
        println!("DEADBEEF: entityclass::swncolour() method not implemented yet");
        0
    }

    // void entityclass::swnenemiescol( int t )
    pub fn swnenemiescol(&mut self, t: i32) {
        println!("DEADBEEF: entityclass::swnenemiescol() method not implemented yet");
    }

    // void entityclass::gravcreate( int ypos, int dir, int xoff /*= 0*/, int yoff /*= 0*/ )
    fn gravcreate(&mut self, ypos: i32, dir: i32, xoff: Option<i32>, yoff: Option<i32>) {
        let xoff = xoff.unwrap_or(0);
        let yoff = yoff.unwrap_or(0);
        println!("DEADBEEF: entityclass::gravcreate() method not implemented yet");
    }

    // void entityclass::generateswnwave( int t )
    pub fn generateswnwave(&mut self, t: i32) {
        println!("DEADBEEF: entityclass::generateswnwave() method not implemented yet");
    }

    // void entityclass::createblock( int t, int xp, int yp, int w, int h, int trig /*= 0*/, const std::string& script /*= ""*/ )
    pub fn createblock(&mut self, t: i32, xp: i32, yp: i32, w: i32, h: i32, trig: Option<i32>, script: Option<String>) -> bool {
        let trig = trig.unwrap_or(0);
        let script = script.unwrap_or(String::new());

        println!("DEADBEEF: entityclass::createblock() method not implemented yet");
        false
    }

    // bool entityclass::disableentity(int t)
    pub fn disableentity(&mut self, t: usize) -> bool {
        println!("DEADBEEF: entityclass::disableentity() method not implemented yet");
        false
    }

    // void entityclass::removeallblocks(void)
    pub fn removeallblocks(&mut self) {
        self.blocks.clear();
    }

    // void entityclass::disableblock( int t )
    fn disableblock(&mut self, t: i32) {
        let t = t as usize;
        if !INBOUNDS_VEC!(t, self.blocks) {
            println!("disableblock() out-of-bounds!");
            return;
        }

        self.blocks[t].wp = 0;
        self.blocks[t].hp = 0;

        self.blocks[t].rect.w = self.blocks[t].wp;
        self.blocks[t].rect.h = self.blocks[t].hp;
    }

    // void entityclass::moveblockto(int x1, int y1, int x2, int y2, int w, int h)
    pub fn moveblockto(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32) {
        println!("DEADBEEF: entityclass::moveblockto() method not implemented yet");
    }

    // void entityclass::disableblockat(int x, int y)
    pub fn disableblockat(&mut self, x: i32, y: i32) {
        println!("DEADBEEF: entityclass::disableblockat() method not implemented yet");
    }

    // void entityclass::removetrigger( int t )
    pub fn removetrigger(&mut self, t: i32) {
        // for size_t i=0; i<blocks.size(); i++ {
        for i in 0..self.blocks.len() {
            if self.blocks[i].r#type == EntityEnum::TRIGGER && self.blocks[i].trigger == t {
                self.disableblock(i as i32);
            }
        }
    }

    // void entityclass::copylinecross( int t )
    pub fn copylinecross(&mut self, t: usize) {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("copylinecross() out-of-bounds!");
            return;
        }

        //Copy entity t into the first free linecrosskludge entity
        println!("copylinecross TBI");
        // self.linecrosskludge.push(self.entities[t]);
    }

    // void entityclass::revertlinecross( int t, int s )
    pub fn revertlinecross(&mut self, t: usize, s: usize) {
        if !INBOUNDS_VEC!(t, self.entities) | !INBOUNDS_VEC!(s, self.linecrosskludge) {
            println!("revertlinecross() out-of-bounds!");
            return;
        }

        //Restore entity t info from linecrossing s
        self.entities[t].onentity = self.linecrosskludge[s].onentity;
        self.entities[t].state = self.linecrosskludge[s].state;
        self.entities[t].life = self.linecrosskludge[s].life;
    }

    // bool entityclass::gridmatch( int p1, int p2, int p3, int p4, int p11, int p21, int p31, int p41 )
    fn gridmatch(&mut self, p1: i32, p2: i32, p3: i32, p4: i32, p11: i32, p21: i32, p31: i32, p41: i32) -> bool {
        println!("DEADBEEF: entityclass::gridmatch() method not implemented yet");
        false
    }

    // int entityclass::crewcolour( int t )
    fn crewcolour(&mut self, t: i32) -> i32 {
        println!("DEADBEEF: entityclass::crewcolour() method not implemented yet");
        0
    }

    // void entityclass::createentity(int xp, int yp, int t, int meta1, int meta2, int p1, int p2, int p3, int p4)
    // void entityclass::createentity(int xp, int yp, int t, int meta1, int meta2, int p1, int p2)
    // void entityclass::createentity(int xp, int yp, int t, int meta1, int meta2, int p1)
    // void entityclass::createentity(int xp, int yp, int t, int meta1, int meta2)
    // void entityclass::createentity(int xp, int yp, int t, int meta1)
    // void entityclass::createentity(int xp, int yp, int t)
    pub fn createentity(&mut self, xp: i32, yp: i32, t: i32, meta1: Option<i32>, meta2: Option<i32>, p1: Option<i32>, p2: Option<i32>, p3: Option<i32>, p4: Option<i32>, map: &mut map::Map, game: &mut game::Game) {
        let mut meta1 = meta1.unwrap_or(0);
        let mut meta2 = meta2.unwrap_or(0);
        let p1 = p1.unwrap_or(0);
        let p2 = p2.unwrap_or(0);
        let p3 = p3.unwrap_or(0);
        let p4 = p4.unwrap_or(0);

        self.k = self.entities.len() as i32;

        /* Can we reuse the slot of a disabled entity? */
        let mut j = None;
        for i in 0..self.entities.len() {
            if self.entities[i].invis && self.entities[i].size == -1 && self.entities[i].r#type == -1 && self.entities[i].rule == -1 {
                j = Some(i);
            }
        }

        let entity_id = match j {
            Some(j) => {
                self.entities[j].clear();
                j
            },
            None => {
                let v = ent::EntClass::new();
                self.entities.push(v);
                (self.entities.len()-1) as usize
            },
        };

        //Size 0 is a sprite
        //Size 1 is a tile
        //Beyond that are special cases (to do)
        //Size 2 is a moving platform of width 4 (32)
        //Size 3 is apparently a "bug chunky pixel"
        //Size 4 is a coin/small pickup
        //Size 5 is a horizontal line, 6 is vertical

        //Rule 0 is the playable character
        //Rule 1 is anything harmful
        //Rule 2 is anything decorative (no collisions)
        //Rule 3 is anything that results in an entity to entity collision and state change
        //Rule 4 is a horizontal line, 5 is vertical
        //Rule 6 is a crew member

        // #if !defined(NO_CUSTOM_LEVELS)
        // Special case for gray Warp Zone tileset!
        // const edlevelclass* const room = ed.getroomprop(game.roomx - 100, game.roomy - 100);
        // let custom_gray = room.tileset == 3 && room.tilecol == 6;
        // #else
        let custom_gray = false;
        // #endif

        self.entities[entity_id].xp = xp;
        self.entities[entity_id].yp = yp;
        self.entities[entity_id].lerpoldxp = xp;
        self.entities[entity_id].lerpoldyp = yp;
        self.entities[entity_id].r#type = t;
        match t {
            0 => {
                //Player
                self.entities[entity_id].rule = 0; //Playable character
                self.entities[entity_id].tile = 0;
                self.entities[entity_id].colour = 0;
                self.entities[entity_id].cx = 6;
                self.entities[entity_id].cy = 2;
                self.entities[entity_id].w = 12;
                self.entities[entity_id].h = 21;
                self.entities[entity_id].dir = 1;

                if meta1 == 1 { self.entities[entity_id].invis = true; }

                self.entities[entity_id].gravity = true;
            },
            1 => {
                //Simple enemy, bouncing off the walls
                self.entities[entity_id].rule = 1;
                self.entities[entity_id].behave = meta1;
                self.entities[entity_id].para = meta2 as f32;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].cx = 0;
                self.entities[entity_id].cy = 0;

                self.entities[entity_id].x1 = p1;
                self.entities[entity_id].y1 = p2;
                self.entities[entity_id].x2 = p3;
                self.entities[entity_id].y2 = p4;

                self.entities[entity_id].harmful = true;
                self.entities[entity_id].tile = 24;
                self.entities[entity_id].animate = 0;
                self.entities[entity_id].colour = 8;

                if game.roomy == 111 && (game.roomx >= 113 && game.roomx <= 117) {
                    self.entities[entity_id].setenemy(0);
                    self.entities[entity_id].setenemyroom(game.roomx, game.roomy); //For colour
                } else if game.roomx == 113 && (game.roomy <= 110 && game.roomy >= 108) {
                    self.entities[entity_id].setenemy(1);
                    self.entities[entity_id].setenemyroom(game.roomx, game.roomy); //For colour
                } else if game.roomx == 113 && game.roomy == 107 {
                    //MAVVERRRICK
                    self.entities[entity_id].tile = 96;
                    self.entities[entity_id].colour = 6;
                    self.entities[entity_id].size = 9;
                    self.entities[entity_id].w = 64;
                    self.entities[entity_id].h = 44;
                    self.entities[entity_id].animate = 4;
                } else {
                    self.entities[entity_id].setenemyroom(game.roomx, game.roomy);
                }
            },
            2 => {
                //A moving platform
                self.entities[entity_id].rule = 2;
                self.entities[entity_id].r#type = 1;
                self.entities[entity_id].size = 2;
                self.entities[entity_id].tile = 1;

                if self.customplatformtile > 0 {
                    self.entities[entity_id].tile = self.customplatformtile;
                } else if self.platformtile > 0 {
                    self.entities[entity_id].tile = self.platformtile;
                } else {
                    //appearance again depends on location
                    if self.gridmatch(p1, p2, p3, p4, 100, 70, 320, 160) { self.entities[entity_id].tile = 616; }
                    if self.gridmatch(p1, p2, p3, p4, 72, 0, 248, 240) { self.entities[entity_id].tile = 610; }
                    if self.gridmatch(p1, p2, p3, p4, -20, 0, 320, 240) { self.entities[entity_id].tile = 413; }

                    if self.gridmatch(p1, p2, p3, p4, -96, -72, 400, 312) { self.entities[entity_id].tile = 26; }
                    if self.gridmatch(p1, p2, p3, p4, -32, -40, 352, 264) { self.entities[entity_id].tile = 27; }
                }

                self.entities[entity_id].w = 32;
                self.entities[entity_id].h = 8;

                if meta1 <= 1 { self.vertplatforms = true; }
                if meta1 >= 2  && meta1 <= 5 { self.horplatforms = true; }
                if meta1 == 14 || meta1 == 15 { self.horplatforms = true; } //special case for last part of Space Station
                if meta1 >= 6  && meta1 <= 7 { self.vertplatforms = true; }

                if meta1 >= 10  && meta1 <= 11 {
                    //Double sized threadmills
                    self.entities[entity_id].w = 64;
                    self.entities[entity_id].h = 8;
                    meta1 -= 2;
                    self.entities[entity_id].size = 8;
                }

                self.entities[entity_id].behave = meta1;
                self.entities[entity_id].para = meta2 as f32;

                if meta1 >= 8 && meta1 <= 9 {
                    self.horplatforms = true; //threadmill!
                    self.entities[entity_id].animate = 10;
                    if self.customplatformtile > 0 {
                        self.entities[entity_id].tile = self.customplatformtile+4;
                        if meta1 == 8 { self.entities[entity_id].tile += 4; }
                        if meta1 == 9 { self.entities[entity_id].animate = 11; }
                    } else {
                        self.entities[entity_id].settreadmillcolour(game.roomx, game.roomy);
                        if meta1 == 8 { self.entities[entity_id].tile += 40; }
                        if meta1 == 9 { self.entities[entity_id].animate = 11; }
                    }
                } else {
                    self.entities[entity_id].animate = 100;
                }

                self.entities[entity_id].x1 = p1;
                self.entities[entity_id].y1 = p2;
                self.entities[entity_id].x2 = p3;
                self.entities[entity_id].y2 = p4;

                self.entities[entity_id].isplatform = true;

                self.createblock(0, xp, yp, 32, 8, None, None);
            },
            3 => {
                //Disappearing platforms
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 2;
                self.entities[entity_id].size = 2;
                self.entities[entity_id].tile = 2;
                //appearance again depends on location
                if self.customplatformtile>0 {
                    self.entities[entity_id].tile=self.customplatformtile;
                } else if meta1 > 0 {
                    self.entities[entity_id].tile = meta1;
                } else {
                    if game.roomx==49 && game.roomy==52 { self.entities[entity_id].tile = 18; }
                    if game.roomx == 50 && game.roomy == 52 { self.entities[entity_id].tile = 22; }
                }

                self.entities[entity_id].cy = -1;
                self.entities[entity_id].w = 32;
                self.entities[entity_id].h = 10;
                self.entities[entity_id].behave = meta1;
                self.entities[entity_id].para = meta2 as f32;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;

                self.createblock(0, xp, yp, 32, 8, None, None);
            },
            4 => {
                //Breakable blocks
                self.entities[entity_id].rule = 6;
                self.entities[entity_id].r#type = 3;
                self.entities[entity_id].size = 1;
                self.entities[entity_id].tile = 10;
                self.entities[entity_id].cy = -1;
                self.entities[entity_id].w = 8;
                self.entities[entity_id].h = 10;
                self.entities[entity_id].behave = meta1;
                self.entities[entity_id].para = meta2 as f32;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;

                self.createblock(0, xp, yp, 8, 8, None, None);
            },
            5 => {
                //Gravity Tokens
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 4;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].tile = 11;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].behave = meta1;
                self.entities[entity_id].para = meta2 as f32;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;
            },
            6 => {
                //Decorative particles
                self.entities[entity_id].rule = 2;
                self.entities[entity_id].r#type = 5;  //Particles
                self.entities[entity_id].colour = 1;
                self.entities[entity_id].size = 3;
                self.entities[entity_id].vx = meta1 as f32;
                self.entities[entity_id].vy = meta2 as f32;

                self.entities[entity_id].life = 12;
            },
            7 => {
                //Decorative particles
                self.entities[entity_id].rule = 2;
                self.entities[entity_id].r#type = 5;  //Particles
                self.entities[entity_id].colour = 2;
                self.entities[entity_id].size = 3;
                self.entities[entity_id].vx = meta1 as f32;
                self.entities[entity_id].vy = meta2 as f32;

                self.entities[entity_id].life = 12;
            },
            8 => {
                //Small collectibles
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 6;
                self.entities[entity_id].size = 4;
                self.entities[entity_id].tile = 48;
                self.entities[entity_id].w = 8;
                self.entities[entity_id].h = 8;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;

                //Check if it's already been collected
                self.entities[entity_id].para = meta1 as f32;
                // if !INBOUNDS_ARR(meta1, self.collect) || self.collect[meta1] {
                //     return;
                // }
            },
            9 => {
                //Something Shiny
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 7;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].tile = 22;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].colour = 3;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;

                //Check if it's already been collected
                self.entities[entity_id].para = meta1 as f32;
                // @sx: TODO
                // if !INBOUNDS_ARR(meta1, self.collect) || self.collect[meta1] {
                //     return;
                // }
            },
            10 => {
                //Savepoint
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 8;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].tile = 20 + meta1;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].colour = 4;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;
                self.entities[entity_id].para = meta2 as f32;

                if game.savepoint == meta2 {
                    self.entities[entity_id].colour = 5;
                    self.entities[entity_id].onentity = 0;
                }

                if game.nodeathmode {
                    return;
                }
            },
            11 => {
                //Horizontal Gravity Line
                self.entities[entity_id].rule = 4;
                self.entities[entity_id].r#type = 9;
                self.entities[entity_id].size = 5;
                self.entities[entity_id].life = 0;
                self.entities[entity_id].w = meta1;
                self.entities[entity_id].h = 1;
                self.entities[entity_id].onentity = 1;
            },
            12 => {
                //Vertical Gravity Line
                self.entities[entity_id].rule = 5;
                self.entities[entity_id].r#type = 10;
                self.entities[entity_id].size = 6;
                self.entities[entity_id].life = 0;
                self.entities[entity_id].w = 1;
                self.entities[entity_id].h = meta1;
                //self.entities[entity_id].colour = 0;
                self.entities[entity_id].onentity = 1;
            },
            13 => {
                //Warp token
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 11;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].tile = 18;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].colour = 10;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 2;
                //Added in port, hope it doesn't break anything
                self.entities[entity_id].behave = meta1;
                self.entities[entity_id].para = meta2 as f32;
            },
            14 => {
                // Teleporter
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 100;
                self.entities[entity_id].size = 7;
                self.entities[entity_id].tile = 1; //inactive
                self.entities[entity_id].w = 96;
                self.entities[entity_id].h = 96;
                self.entities[entity_id].colour = 100;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;
                self.entities[entity_id].para = meta2 as f32;
            },
            15 => {
                // Crew Member (warp zone)
                self.entities[entity_id].rule = 6;
                self.entities[entity_id].r#type = 12; //A special case!
                self.entities[entity_id].tile = 144;
                self.entities[entity_id].colour = 13; //144 for sad :(
                self.entities[entity_id].cx = 6;
                self.entities[entity_id].cy = 2;
                self.entities[entity_id].w = 12;
                self.entities[entity_id].h = 21;
                self.entities[entity_id].dir = 0;

                self.entities[entity_id].state = meta1;

                self.entities[entity_id].gravity = true;
            },
            16 => {
                // Crew Member, upside down (space station)
                self.entities[entity_id].rule = 7;
                self.entities[entity_id].r#type = 12; //A special case!
                self.entities[entity_id].tile = 144+6;
                self.entities[entity_id].colour = 14; //144 for sad (upside down+12):(
                self.entities[entity_id].cx = 6;
                self.entities[entity_id].cy = 2;
                self.entities[entity_id].w = 12;
                self.entities[entity_id].h = 21;
                self.entities[entity_id].dir = 1;

                self.entities[entity_id].state = meta1;

                self.entities[entity_id].gravity = true;
            },
            17 => {
                // Crew Member (Lab)
                self.entities[entity_id].rule = 6;
                self.entities[entity_id].r#type = 12; //A special case!
                self.entities[entity_id].tile = 144;
                self.entities[entity_id].colour = 16; //144 for sad :(
                self.entities[entity_id].cx = 6;
                self.entities[entity_id].cy = 2;
                self.entities[entity_id].w = 12;
                self.entities[entity_id].h = 21;
                self.entities[entity_id].dir = 1;

                self.entities[entity_id].state = meta1;

                self.entities[entity_id].gravity = true;
            },
            18 => {
                // Crew Member (Ship)
                //This is the scriping crewmember
                self.entities[entity_id].rule = 6;
                self.entities[entity_id].r#type = 12; //A special case!
                self.entities[entity_id].colour = meta1;
                if meta2 == 0 {
                    self.entities[entity_id].tile = 0;
                } else {
                    self.entities[entity_id].tile = 144;
                }
                self.entities[entity_id].cx = 6;
                self.entities[entity_id].cy = 2;
                self.entities[entity_id].w = 12;
                self.entities[entity_id].h = 21;
                self.entities[entity_id].dir = 0;

                self.entities[entity_id].state = p1;
                self.entities[entity_id].para = p2 as f32;

                if p1 == 17 {
                    self.entities[entity_id].dir = p2;
                }

                self.entities[entity_id].gravity = true;
            },
            19 => {
                // Crew Member (Ship) More tests!
                self.entities[entity_id].rule = 6;
                self.entities[entity_id].r#type = 12; //A special case!
                self.entities[entity_id].tile = 0;
                self.entities[entity_id].colour = 6; //54 for sad :(
                self.entities[entity_id].cx = 6;
                self.entities[entity_id].cy = 2;
                self.entities[entity_id].w = 12;
                self.entities[entity_id].h = 21;
                self.entities[entity_id].dir = 1;

                self.entities[entity_id].state = meta1;

                self.entities[entity_id].gravity = true;
            },
            20 => {
                //Terminal
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 13;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].tile = 16 + meta1;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].colour = 4;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;
                self.entities[entity_id].para = meta2 as f32;
            },
            21 => {
                //as above, except doesn't highlight
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 13;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].tile = 16 + meta1;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].colour = 4;
                self.entities[entity_id].onentity = 0;
                self.entities[entity_id].animate = 100;
                self.entities[entity_id].para = meta2 as f32;
            },
            22 => {
                //Fake trinkets, only appear if you've collected them
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 7;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].tile = 22;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].colour = 3;
                self.entities[entity_id].onentity = 0;
                self.entities[entity_id].animate = 100;

                //Check if it's already been collected
                self.entities[entity_id].para = meta1 as f32;
                // @sx: TODO
                // if INBOUNDS_ARR(meta1, collect) && !collect[meta1] {
                //     return;
                // }
            },
            23 => {
                //SWN Enemies
                //Given a different behavior, these enemies are especially for SWN mode and disappear outside the screen.
                self.entities[entity_id].rule = 1;
                self.entities[entity_id].r#type = 23;
                self.entities[entity_id].behave = meta1;
                self.entities[entity_id].para = meta2 as f32;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].cx = 0;
                self.entities[entity_id].cy = 0;

                self.entities[entity_id].x1 = -2000;
                self.entities[entity_id].y1 = -100;
                self.entities[entity_id].x2 = 5200;
                self.entities[entity_id].y2 = 340;

                self.entities[entity_id].harmful = true;

                //initilise tiles here based on behavior
                self.entities[entity_id].size = 12; //don't wrap around
                self.entities[entity_id].colour = 21;
                self.entities[entity_id].tile = 78; //default case
                self.entities[entity_id].animate = 1;
                if game.swngame == 1 {
                    //set colour based on current state
                    self.entities[entity_id].colour = self.swncolour(game.swncolstate);
                }
            },
            24 => {
                // Super Crew Member
                //This special crewmember is way more advanced than the usual kind, and can interact with game objects
                self.entities[entity_id].rule = 6;
                self.entities[entity_id].r#type = 14; //A special case!
                self.entities[entity_id].colour = meta1;
                if meta1 == 16 {
                    //victoria is sad!
                    if meta2 == 2 { meta2 = 1; }
                } else {
                    if meta2 == 2 { meta2 = 0; }
                }
                if meta2 == 0 {
                    self.entities[entity_id].tile = 0;
                } else {
                    self.entities[entity_id].tile = 144;
                }
                self.entities[entity_id].cx = 6;
                self.entities[entity_id].cy = 2;
                self.entities[entity_id].w = 12;
                self.entities[entity_id].h = 21;
                self.entities[entity_id].dir = 1;

                self.entities[entity_id].x1 = -2000;
                self.entities[entity_id].y1 = -100;
                self.entities[entity_id].x2 = 5200;
                self.entities[entity_id].y2 = 340;

                self.entities[entity_id].state = p1;
                self.entities[entity_id].para = p2 as f32;

                if p1 == 17 {
                    self.entities[entity_id].dir = p2;
                }

                self.entities[entity_id].gravity = true;
            },
            25 => {
                //Trophies
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 15;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].colour = 4;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].animate = 100;
                self.entities[entity_id].para = meta2 as f32;

                //Decide tile here based on given achievement: both whether you have them and what they are
                //default is just a trophy base:
                self.entities[entity_id].tile = 180 + meta1;
                match meta2 {
                    1 => {
                        if game.bestrank[0] >= 3 {
                            self.entities[entity_id].tile = 184 + meta1;
                            self.entities[entity_id].colour = 31;
                        }
                    },
                    2 => {
                        if game.bestrank[1] >= 3 {
                            self.entities[entity_id].tile = 186 + meta1;
                            self.entities[entity_id].colour = 33;
                        }
                    },
                    3 => {
                        if game.bestrank[2] >= 3 {
                            self.entities[entity_id].tile = 184 + meta1;
                            self.entities[entity_id].colour = 35;
                        }
                    },
                    4 => {
                        if game.bestrank[3] >= 3 {
                            self.entities[entity_id].tile = 184 + meta1;
                            self.entities[entity_id].colour = 30;
                        }
                    },
                    5 => {
                        if game.bestrank[4] >= 3 {
                            self.entities[entity_id].tile = 184 + meta1;
                            self.entities[entity_id].colour = 34;
                        }
                    },
                    6 => {
                        if game.bestrank[5] >= 3 {
                            self.entities[entity_id].tile = 184 + meta1;
                            self.entities[entity_id].colour = 32;
                        }
                    },

                    7 => {
                        if game.unlock[5] {
                            self.entities[entity_id].tile = 188 + meta1;
                            self.entities[entity_id].colour = 37;
                            self.entities[entity_id].h += 3;
                            self.entities[entity_id].lerpoldyp -= 3;
                            self.entities[entity_id].yp -= 3;
                        }
                    },
                    8 => {
                        if game.unlock[19] {
                            self.entities[entity_id].tile = 188 + meta1;
                            self.entities[entity_id].colour = 37;
                            self.entities[entity_id].h += 3;
                        }
                    },

                    9 => {
                        if game.bestgamedeaths > -1 {
                            if game.bestgamedeaths <= 50 {
                                self.entities[entity_id].tile = 182 + meta1;
                                self.entities[entity_id].colour = 40;
                            }
                        }
                    },
                    10 => {
                        if game.bestgamedeaths > -1 {
                            if game.bestgamedeaths <= 100 {
                                self.entities[entity_id].tile = 182 + meta1;
                                self.entities[entity_id].colour = 36;
                            }
                        }
                    },
                    11 => {
                        if game.bestgamedeaths > -1 {
                            if game.bestgamedeaths <= 250 {
                                self.entities[entity_id].tile = 182 + meta1;
                                self.entities[entity_id].colour = 38;
                            }
                        }
                    },
                    12 => {
                        if game.bestgamedeaths > -1 {
                            if game.bestgamedeaths <= 500 {
                                self.entities[entity_id].tile = 182 + meta1;
                                self.entities[entity_id].colour = 39;
                            }
                        }
                    },

                    13 => {
                        if game.swnbestrank >= 1 {
                            self.entities[entity_id].tile = 182 + meta1;
                            self.entities[entity_id].colour = 39;
                        }
                    },
                    14 => {
                        if game.swnbestrank >= 2 {
                            self.entities[entity_id].tile = 182 + meta1;
                            self.entities[entity_id].colour = 39;
                        }
                    },
                    15 => {
                        if game.swnbestrank >= 3 {
                            self.entities[entity_id].tile = 182 + meta1;
                            self.entities[entity_id].colour = 39;
                        }
                    },
                    16 => {
                        if game.swnbestrank >= 4 {
                            self.entities[entity_id].tile = 182 + meta1;
                            self.entities[entity_id].colour = 38;
                        }
                    },
                    17 => {
                        if game.swnbestrank >= 5 {
                            self.entities[entity_id].tile = 182 + meta1;
                            self.entities[entity_id].colour = 36;
                        }
                    },
                    18 => {
                        if game.swnbestrank >= 6 {
                            self.entities[entity_id].tile = 182 + meta1;
                            self.entities[entity_id].colour = 40;
                        }
                    },

                    19 => {
                        if game.unlock[20] {
                            self.entities[entity_id].tile = 3;
                            self.entities[entity_id].colour = 102;
                            self.entities[entity_id].size = 13;
                            self.entities[entity_id].xp -= 64;
                            self.entities[entity_id].yp -= 128;
                        }
                    },

                    _ => println!("unkown meta2 value {}", meta2),
                }
            },
            26 => {
                //Epilogue super warp token
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 11;
                self.entities[entity_id].size = 0;
                self.entities[entity_id].tile = 18;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].colour = 3;
                self.entities[entity_id].onentity = 0;
                self.entities[entity_id].animate = 100;
                self.entities[entity_id].para = meta2 as f32;
                self.entities[entity_id].size = 13;
            },

            /* Warp lines */
            // case 51: /* Vertical */
            // case 52: /* Vertical */
            // case 53: /* Horizontal */
            // case 54: /* Horizontal */
            51 | 52 | 53 | 54 => {
                self.entities[entity_id].r#type = t;
                self.entities[entity_id].onentity = 1;
                self.entities[entity_id].invis = true;
                self.entities[entity_id].life = 0;
                match t {
                    51 | 52 => {
                        self.entities[entity_id].rule = 5;
                        self.entities[entity_id].size = 6;
                        self.entities[entity_id].w = 1;
                        self.entities[entity_id].h = meta1;
                    },
                    53 | 53 => {
                        self.entities[entity_id].rule = 7;
                        self.entities[entity_id].size = 5;
                        self.entities[entity_id].w = meta1;
                        self.entities[entity_id].h = 1;
                    },
                    _ => (),
                };
                if map.custommode {
                    self.customwarpmode = true;
                    map.warpx = false;
                    map.warpy = false;
                }
            },
            55 => {
                // Crew Member (custom, collectable)
                //1 - position in array
                //2 - colour
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 55;
                // if INBOUNDS_ARR(meta2, self.customcrewmoods && self.customcrewmoods[meta2]==1) {
                // @sx: TODO
                    self.entities[entity_id].tile = 144;
                // } else {
                //     self.entities[entity_id].tile = 0;
                // }
                self.entities[entity_id].colour = self.crewcolour(meta2);
                self.entities[entity_id].cx = 6;
                self.entities[entity_id].cy = 2;
                self.entities[entity_id].w = 12;
                self.entities[entity_id].h = 21;
                self.entities[entity_id].dir = 0;

                self.entities[entity_id].state = 0;
                self.entities[entity_id].onentity = 1;
                //self.entities[entity_id].state = meta1;

                self.entities[entity_id].gravity = true;

                //Check if it's already been collected
                self.entities[entity_id].para = meta1 as f32;
                // @sx: TODO
                // if !INBOUNDS_ARR(meta1, self.customcollect) || self.customcollect[meta1] {
                //     return;
                // }
            },
            56 => {
                //Custom enemy
                self.entities[entity_id].rule = 1;
                self.entities[entity_id].r#type = 1;
                self.entities[entity_id].behave = meta1;
                self.entities[entity_id].para = meta2 as f32;
                self.entities[entity_id].w = 16;
                self.entities[entity_id].h = 16;
                self.entities[entity_id].cx = 0;
                self.entities[entity_id].cy = 0;

                self.entities[entity_id].x1 = p1;
                self.entities[entity_id].y1 = p2;
                self.entities[entity_id].x2 = p3;
                self.entities[entity_id].y2 = p4;

                self.entities[entity_id].harmful = true;

                match self.customenemy {
                    0 => self.entities[entity_id].setenemyroom(4+100, 0+100),
                    1 => self.entities[entity_id].setenemyroom(2+100, 0+100),
                    2 => self.entities[entity_id].setenemyroom(12+100, 3+100),
                    3 => self.entities[entity_id].setenemyroom(13+100, 12+100),
                    4 => self.entities[entity_id].setenemyroom(16+100, 9+100),
                    5 => self.entities[entity_id].setenemyroom(19+100, 1+100),
                    6 => self.entities[entity_id].setenemyroom(19+100, 2+100),
                    7 => self.entities[entity_id].setenemyroom(18+100, 3+100),
                    8 => self.entities[entity_id].setenemyroom(16+100, 0+100),
                    9 => self.entities[entity_id].setenemyroom(14+100, 2+100),
                    _ => self.entities[entity_id].setenemyroom(4+100, 0+100),
                }

                //Set colour based on room tile
                //Set custom colours
                if self.customplatformtile > 0 {
                    let entcol = self.customplatformtile/12;
                    match entcol {
                        //RED
                        3 | 7 | 12 | 23 | 28 | 34 | 42 | 48 | 58 => self.entities[entity_id].colour = 6,
                        //GREEN
                        5 | 9 | 22 | 25 | 29 | 31 | 38 | 46 | 52 | 53 => self.entities[entity_id].colour = 7,
                        //BLUE
                        1 | 6 | 14 | 27 | 33 | 44 | 50 | 57 => self.entities[entity_id].colour = 12,
                        //YELLOW
                        4 | 17 | 24 | 30 | 37 | 45 | 51 | 55 => self.entities[entity_id].colour = 9,
                        //PURPLE
                        2 | 11 | 15 | 19 | 32 | 36 | 49 => self.entities[entity_id].colour = 20,
                        //CYAN
                        8 | 10 | 13 | 18 | 26 | 35 | 41 | 47 | 54 => self.entities[entity_id].colour = 11,
                        //PINK
                        16 | 20 | 39 | 43 | 56 => self.entities[entity_id].colour = 8,
                        //ORANGE
                        21 | 40 => self.entities[entity_id].colour = 17,
                        _ => self.entities[entity_id].colour = 6,
                    }
                }

                if custom_gray {
                    self.entities[entity_id].colour = 18;
                }
            },
            _ => println!("unknown entity type {}", t),

        };

        self.entities[entity_id].drawframe = self.entities[entity_id].tile as usize;

        // @sx: WTF????
        // /* Fix crewmate facing directions
        //  * This is a bit kludge-y but it's better than copy-pasting
        //  * and is okay to do because entity 12 does not change state on its own
        //  */
        // if self.entities[entity_id].r#type == 12 {
        //     let indice = match reuse {
        //         true => entptr - self.entities.data(),
        //         false => self.entities.size() - 1,
        //     };
        //     self.updateentities(indice);
        // }
    }

    // bool entityclass::updateentities( int i )
    pub fn updateentities(&mut self, i: i32) -> bool {
        println!("DEADBEEF: entityclass::updateentities() method not implemented yet");
        false
    }

    // void entityclass::animateentities( int _i )
    pub fn animateentities(&mut self, _i: i32, game: &mut game::Game) {
        if !INBOUNDS_VEC!(_i, self.entities) {
            println!("animateentities() out-of-bounds!");
            return;
        }
        let _i = _i as usize;

        if self.entities[_i].statedelay < 1 {
            match self.entities[_i].r#type {
                0 => {
                    self.entities[_i].framedelay -= 1;
                    self.entities[_i].drawframe = if self.entities[_i].dir == 1 {
                        self.entities[_i].tile
                    } else {
                        self.entities[_i].tile + 3
                    } as usize;

                    if self.entities[_i].visualonground > 0 || self.entities[_i].visualonroof > 0 {
                        if self.entities[_i].vx > 0.0 || self.entities[_i].vx < -0.0 {
                            //Walking
                            if self.entities[_i].framedelay <= 1 {
                                self.entities[_i].framedelay = 4;
                                self.entities[_i].walkingframe += 1;
                            }
                            if self.entities[_i].walkingframe >=2 {
                                self.entities[_i].walkingframe = 0;
                            }
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize + 1;
                        }

                        if self.entities[_i].visualonroof > 0 {
                            self.entities[_i].drawframe += 6;
                        }

                        // Stuck in a wall? Then default to gravitycontrol
                        if self.entities[_i].visualonground > 0 && self.entities[_i].visualonroof > 0 && game.gravitycontrol == 0 {
                            self.entities[_i].drawframe -= 6;
                        }
                    } else {
                        self.entities[_i].drawframe += 1;
                        if game.gravitycontrol == 1 {
                            self.entities[_i].drawframe += 6;
                        }
                    }

                    if game.deathseq > -1 {
                        self.entities[_i].drawframe = 13;
                        if self.entities[_i].dir == 1 {
                            self.entities[_i].drawframe = 12;
                        }
                        if game.gravitycontrol == 1 {
                            self.entities[_i].drawframe += 2;
                        }
                    }
                },
                1 | 23 => {
                    //Variable animation
                    match self.entities[_i].animate {
                        0 => {
                            //Simple oscilation
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 8;
                                if self.entities[_i].actionframe == 0 {
                                    self.entities[_i].walkingframe += 1;
                                    if self.entities[_i].walkingframe == 4 {
                                        self.entities[_i].walkingframe = 2;
                                        self.entities[_i].actionframe = 1;
                                    }
                                } else {
                                    self.entities[_i].walkingframe -= 1;
                                    if self.entities[_i].walkingframe == -1 {
                                        self.entities[_i].walkingframe = 1;
                                        self.entities[_i].actionframe = 0;
                                    }
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                        },
                        1 => {
                            //Simple Loop
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 8;
                                self.entities[_i].walkingframe += 1;
                                if self.entities[_i].walkingframe == 4 {
                                    self.entities[_i].walkingframe = 0;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                        },
                        2 => {
                            //Simpler Loop (just two frames)
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 2;
                                self.entities[_i].walkingframe += 1;
                                if self.entities[_i].walkingframe == 2 {
                                    self.entities[_i].walkingframe = 0;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                        },
                        3 => {
                            //Simpler Loop (just two frames, but double sized)
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 2;
                                self.entities[_i].walkingframe += 1;
                                if self.entities[_i].walkingframe == 2 {
                                    self.entities[_i].walkingframe = 0;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize * 2;
                        },
                        4 => {
                            //Simpler Loop (just two frames, but double sized) (as above but slower)
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 6;
                                self.entities[_i].walkingframe += 1;
                                if self.entities[_i].walkingframe == 2 {
                                    self.entities[_i].walkingframe = 0;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize * 2;
                        },
                        5 => {
                            //Simpler Loop (just two frames) (slower)
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 6;
                                self.entities[_i].walkingframe += 1;
                                if self.entities[_i].walkingframe == 2 {
                                    self.entities[_i].walkingframe = 0;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                        },
                        6 => {
                            //Normal Loop (four frames, double sized)
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 4;
                                self.entities[_i].walkingframe += 1;
                                if self.entities[_i].walkingframe == 4 {
                                    self.entities[_i].walkingframe = 0;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize * 2;
                        },
                        7 => {
                            //Simpler Loop (just two frames) (slower) (with directions!)
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 6;
                                self.entities[_i].walkingframe += 1;
                                if self.entities[_i].walkingframe == 2 {
                                    self.entities[_i].walkingframe = 0;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;

                            if self.entities[_i].vx > 0.0 {
                                self.entities[_i].drawframe += 2;
                            }
                        },
                        10 => {
                            //Threadmill left
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 3; //(6-self.entities[_i].para);
                                self.entities[_i].walkingframe -= 1;
                                if self.entities[_i].walkingframe == -1 {
                                    self.entities[_i].walkingframe = 3;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                        },
                        11 => {
                            //Threadmill right
                            self.entities[_i].framedelay -= 1;
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 3; //(6-self.entities[_i].para);
                                self.entities[_i].walkingframe += 1;
                                if self.entities[_i].walkingframe == 4 {
                                    self.entities[_i].walkingframe = 0;
                                }
                            }

                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                        },
                        100 => {
                            //Simple case for no animation (platforms, etc)
                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                        },
                        _ => {
                            self.entities[_i].drawframe = self.entities[_i].tile as usize;
                        },
                    };
                },
                2 => { //Disappearing platforms
                    self.entities[_i].drawframe = self.entities[_i].tile as usize + self.entities[_i].walkingframe as usize;
                },
                11 => {
                    self.entities[_i].drawframe = self.entities[_i].tile as usize;
                    if self.entities[_i].animate == 2 {
                        //Simpler Loop (just two frames)
                        self.entities[_i].framedelay -= 1;
                        if self.entities[_i].framedelay<=0 {
                            self.entities[_i].framedelay = 10;
                            self.entities[_i].walkingframe += 1;
                            if self.entities[_i].walkingframe == 2 {
                                self.entities[_i].walkingframe = 0;
                            }
                        }

                        self.entities[_i].drawframe = self.entities[_i].tile as usize;
                        self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                    }
                },
                12 | 55 | 14 => {
                    //Crew member! Very similar to hero
                    self.entities[_i].framedelay -= 1;
                    self.entities[_i].drawframe = if self.entities[_i].dir == 1 {
                        self.entities[_i].tile
                    } else {
                        self.entities[_i].tile + 3
                    } as usize;

                    if self.entities[_i].visualonground > 0 || self.entities[_i].visualonroof > 0 {
                        if self.entities[_i].vx > 0.0 || self.entities[_i].vx < -0.0 {
                            //Walking
                            if self.entities[_i].framedelay <= 0 {
                                self.entities[_i].framedelay = 4;
                                self.entities[_i].walkingframe += 1;
                            }
                            if self.entities[_i].walkingframe >= 2 {
                                self.entities[_i].walkingframe = 0;
                            }
                            self.entities[_i].drawframe += self.entities[_i].walkingframe as usize + 1;
                        }

                        //if self.entities[_i].visualonroof > 0) self.entities[_i].drawframe += 6;
                    } else {
                        self.entities[_i].drawframe  += 1;
                        //if game.gravitycontrol == 1) {
                        //	self.entities[_i].drawframe += 6;
                        //}
                    }

                    if game.deathseq > -1 {
                        self.entities[_i].drawframe = 13;
                        if self.entities[_i].dir == 1 {
                            self.entities[_i].drawframe = 12;
                        }

                        if self.entities[_i].rule == 7 {
                            self.entities[_i].drawframe += 2;
                        }
                        //if game.gravitycontrol == 1) self.entities[_i].drawframe += 2;
                    }
                },
                100 => { //the teleporter!
                    if self.entities[_i].tile == 1 {
                        //it's inactive
                        self.entities[_i].drawframe = self.entities[_i].tile as usize;
                    } else if self.entities[_i].tile == 2 {
                        self.entities[_i].drawframe = self.entities[_i].tile as usize;

                        self.entities[_i].framedelay -= 1;
                        if self.entities[_i].framedelay <= 0 {
                            self.entities[_i].framedelay = 1;
                            self.entities[_i].walkingframe = (maths::fRandom() * 6.0) as i32;
                            if self.entities[_i].walkingframe >= 4 {
                                self.entities[_i].walkingframe = -1;
                                self.entities[_i].framedelay = 4;
                            }
                        }

                        self.entities[_i].drawframe = self.entities[_i].tile as usize;
                        self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                    } else if self.entities[_i].tile == 6 {
                        //faster!
                        self.entities[_i].drawframe = self.entities[_i].tile as usize;

                        self.entities[_i].framedelay -= 1;
                        if self.entities[_i].framedelay <= 0 {
                            self.entities[_i].framedelay = 2;
                            self.entities[_i].walkingframe = (maths::fRandom() * 6.0) as i32;
                            if self.entities[_i].walkingframe >= 4 {
                                self.entities[_i].walkingframe = -5;
                                self.entities[_i].framedelay = 4;
                            }
                        }

                        self.entities[_i].drawframe = self.entities[_i].tile as usize;
                        self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                    }
                },
                _ => {
                    self.entities[_i].drawframe = self.entities[_i].tile as usize;
                },
            };
        } else {
            // @sx: disabled in original
            // self.entities[_i].statedelay -= 1;
            if self.entities[_i].statedelay < 0 {
                self.entities[_i].statedelay = 0;
            }
        }
    }

    // int entityclass::getcompanion(void)
    pub fn getcompanion(&mut self) -> i32 {
        //Returns the index of the companion with rule t
        for i in 0..self.entities.len() {
            if self.entities[i].rule == 6 || self.entities[i].rule == 7 {
                return i as i32;
            }
        }

        -1
    }

    // int entityclass::getplayer(void)
    pub fn getplayer(&self) -> i32 {
        //Returns the index of the first player entity
        for i in 0..self.entities.len() {
            if self.entities[i].r#type == 0 {
                return i as i32
            }
        }

        -1
    }

    // int entityclass::getscm(void)
    pub fn getscm(&self) -> i32 {
        //Returns the supercrewmate
        // for size_t i = 0; i < entities.size(); i++ {
        for i in 0..self.entities.len() {
            if self.entities[i].r#type == 14 {
                return i as i32
            }
        }

        0
    }

    // int entityclass::getlineat( int t )
    pub fn getlineat(&mut self, t: i32) -> usize {
        println!("DEADBEEF: entityclass::getlineat method not implemented yet");
        0
    }

    // int entityclass::getcrewman( int t )
    pub fn getcrewman(&self, t: i32) -> i32 {
        //Returns the index of the crewman with colour index given by t
        let t = match t {
            t if t == 0 => 0,
            t if t == 1 => 20,
            t if t == 2 => 14,
            t if t == 3 => 15,
            t if t == 4 => 13,
            t if t == 5 => 16,
            _ => t,
        };

        // for (size_t i = 0; i < entities.size(); i++)
        for i in 0..self.entities.len() {
            if ((self.entities[i].r#type == 12) | (self.entities[i].r#type == 14)) &&
               ((self.entities[i].rule   == 6)  | (self.entities[i].rule   == 7)) {
                if self.entities[i].colour == t {
                    return i as i32
                }
            }
        }

        0
    }

    // int entityclass::getcustomcrewman( int t )
    pub fn getcustomcrewman(&self, t: i32) -> i32 {
        println!("DEADBEEF: entityclass::getcustomcrewman() method not implemented yet");
        0
    }

    // int entityclass::getteleporter(void)
    pub fn getteleporter(&self) -> i32 {
        println!("DEADBEEF: entityclass::getteleporter() method not implemented yet");
        0
    }

    // bool entityclass::entitycollide( int a, int b )
    pub fn entitycollide(&self, a: usize, b: usize) -> bool {
        println!("DEADBEEF: entityclass::entitycollide() method not implemented yet");
        false
    }

    // bool entityclass::checkdamage(bool scm /*= false*/)
    pub fn checkdamage(&self, scm: Option<bool>, help: &mut utility_class::UtilityClass) -> bool {
        let scm = scm.unwrap_or(false);

        //Returns true if player (or supercrewmate) collides with a damagepoint
        // for size_t i=0; i < entities.size(); i++ {
        for i in 0..self.entities.len() {
            if (scm && self.entities[i].r#type == 14) || (!scm && self.entities[i].rule == 0) {
                let temprect = sdl2::rect::Rect::new(
                    self.entities[i].xp + self.entities[i].cx,
                    self.entities[i].yp + self.entities[i].cy,
                    self.entities[i].w as u32,
                    self.entities[i].h as u32,
                );

                // for size_t j=0; j<blocks.size(); j++ {
                for j in 0..self.blocks.len() {
                    if self.blocks[j].r#type == EntityEnum::DAMAGE && help.intersects(self.blocks[j].rect, temprect) {
                        return true;
                    }
                }
            }
        }

        false
    }

    // int entityclass::checktrigger(int* block_idx)
    pub fn checktrigger(&self, block_idx: &mut i32, help: &mut utility_class::UtilityClass) -> i32 {
        //Returns an int player entity (rule 0) collides with a trigger
        //Also returns the index of the block
        *block_idx = -1;
        // for size_t i=0; i < entities.size(); i++
        for i in 0..self.entities.len() {
            if self.entities[i].rule == 0 {
                let temprect = sdl2::rect::Rect::new(
                    self.entities[i].xp + self.entities[i].cx,
                    self.entities[i].yp + self.entities[i].cy,
                    self.entities[i].w as u32,
                    self.entities[i].h as u32
                );

                // for size_t j=0; j<blocks.size(); j++ {
                for j in 0..self.blocks.len() {
                    if self.blocks[j].r#type == EntityEnum::TRIGGER && help.intersects(self.blocks[j].rect, temprect) {
                        *block_idx = j as i32;
                        return self.blocks[j].trigger
                    }
                }
            }
        }

        -1
    }

    // int entityclass::checkactivity(void)
    pub fn checkactivity(&self, help: &mut utility_class::UtilityClass) -> i32 {
        //Returns an int player entity (rule 0) collides with an activity
        // for size_t i=0; i < entities.size(); i++ {
        for i in 0..self.entities.len() {
            if self.entities[i].rule == 0 {
                let temprect = sdl2::rect::Rect::new(
                    self.entities[i].xp + self.entities[i].cx,
                    self.entities[i].yp + self.entities[i].cy,
                    self.entities[i].w as u32,
                    self.entities[i].h as u32,
                );

                // for (size_t j=0; j<blocks.size(); j++ {
                for j in 0..self.blocks.len() {
                    if self.blocks[j].r#type == EntityEnum::ACTIVITY && help.intersects(self.blocks[j].rect, temprect) {
                        return j as i32
                    }
                }
            }
        }

        -1
    }

    // int entityclass::getgridpoint( int t )
    fn getgridpoint(&self, t: i32) -> i32 {
        (t - (t % 8)) / 8
    }

    // bool entityclass::checkplatform(const SDL_Rect& temprect, int* px, int* py)

    // bool entityclass::checkblocks(const SDL_Rect& temprect, const float dx, const float dy, const float dr, const bool skipdirblocks)
    fn checkblocks(&self, temprect: sdl2::rect::Rect, dx: f32, dy: f32, dr: f32, skipdirblocks: bool, help: &mut utility_class::UtilityClass) -> bool {
        // for size_t i = 0; i < blocks.size(); i++ {
            for block in self.blocks.iter() {
            if !skipdirblocks && block.r#type == EntityEnum::DIRECTIONAL {
                if (dy >  0.0 && block.trigger == 0 && help.intersects(block.rect, temprect)) |
                   (dy <= 0.0 && block.trigger == 1 && help.intersects(block.rect, temprect)) |
                   (dx >  0.0 && block.trigger == 2 && help.intersects(block.rect, temprect)) |
                   (dx <= 0.0 && block.trigger == 3 && help.intersects(block.rect, temprect)) {
                    return true
                }
            }
            if block.r#type == EntityEnum::BLOCK && help.intersects(block.rect, temprect) {
                return true
            }
            if block.r#type == EntityEnum::SAFE && dr == 1.0 && help.intersects(block.rect, temprect) {
                return true
            }
        }

        false
    }

    // bool entityclass::checkwall(const SDL_Rect& temprect)
    // bool entityclass::checkwall(const SDL_Rect& temprect, const float dx, const float dy, const float dr, const bool skipblocks, const bool skipdirblocks)
    fn checkwall(&self, temprect: sdl2::rect::Rect, dx: Option<f32>, dy: Option<f32>, dr: Option<f32>, skipblocks: Option<bool>, skipdirblocks: Option<bool>, map: &mut map::Map, help: &mut utility_class::UtilityClass) -> bool {
        // self.checkwall(temprect, 0, 0, 0, true, false)
        let dx = dx.unwrap_or(0.0);
        let dy = dy.unwrap_or(0.0);
        let dr = dr.unwrap_or(0.0);
        let skipblocks = skipblocks.unwrap_or(true);
        let skipdirblocks = skipdirblocks.unwrap_or(false);

        //Returns true if entity setup in temprect collides with a wall
        if skipblocks {
            if self.checkblocks(temprect, dx, dy, dr, skipdirblocks, help) {
                return true
            }
        }

        let tempx = self.getgridpoint(temprect.x);
        let tempy = self.getgridpoint(temprect.y);
        let tempw = self.getgridpoint(temprect.x + temprect.w - 1);
        let temph = self.getgridpoint(temprect.y + temprect.h - 1);
        if map.collide(tempx, tempy) | map.collide(tempw, tempy) | map.collide(tempx, temph) | map.collide(tempw, temph) {
            return true;
        }

        if temprect.h >= 12 {
            let tpy1 = self.getgridpoint(temprect.y + 6);
            if map.collide(tempx, tpy1) | map.collide(tempw, tpy1) {
                return true;
            }
            if temprect.h >= 18 {
                let tpy1 = self.getgridpoint(temprect.y + 12);
                if map.collide(tempx, tpy1) | map.collide(tempw, tpy1) {
                    return true;
                }
                if temprect.h >= 24 {
                    let tpy1 = self.getgridpoint(temprect.y + 18);
                    if map.collide(tempx, tpy1) | map.collide(tempw, tpy1) {
                        return true;
                    }
                }
            }
        }

        if temprect.w >= 12 {
            let tpx1 = self.getgridpoint(temprect.x + 6);
            if map.collide(tpx1, tempy) | map.collide(tpx1, temph) {
                return true;
            }
        }

        return false
    }

    // float entityclass::hplatformat(const int px, const int py)
    fn hplatformat(&self, px: i32, py: i32) -> f32 {
        println!("DEADBEEF: testwallsx::hplatformat() method not implemented yet");
        0.0
    }

    // int entityclass::yline( int a, int b )
    fn yline(&self, a: i32, b: i32) -> i32 {
        println!("DEADBEEF: testwallsx::yline() method not implemented yet");
        0
    }

    // bool entityclass::entityhlinecollide( int t, int l )
    fn entityhlinecollide(&self, t: usize, l: usize) -> bool {
        println!("DEADBEEF: testwallsx::entityhlinecollide() method not implemented yet");
        false
    }

    // bool entityclass::entityvlinecollide( int t, int l )
    fn entityvlinecollide(&self, t: usize, l: usize) -> bool {
        println!("DEADBEEF: testwallsx::entityvlinecollide() method not implemented yet");
        false
    }

    // bool entityclass::entitywarphlinecollide(int t, int l) {
    fn entitywarphlinecollide(&self, t: i32, l: i32) -> bool {
        println!("DEADBEEF: testwallsx::entitywarphlinecollide() method not implemented yet");
        false
    }

    // bool entityclass::entitywarpvlinecollide(int t, int l) {
    fn entitywarpvlinecollide(&self, t: i32, l: i32) -> bool {
        println!("DEADBEEF: testwallsx::entitywarpvlinecollide() method not implemented yet");
        false
    }

    // float entityclass::entitycollideplatformroof( int t )
    pub fn entitycollideplatformroof(&self, t: i32) -> f32 {
        println!("DEADBEEF: testwallsx::entitycollideplatformroof() method not implemented yet");
        0.0
    }

    // float entityclass::entitycollideplatformfloor( int t )
    pub fn entitycollideplatformfloor(&self, t: i32) -> f32 {
        println!("DEADBEEF: testwallsx::entitycollideplatformfloor() method not implemented yet");
        0.0
    }

    // bool entityclass::entitycollidefloor( int t )
    pub fn entitycollidefloor(&mut self, t: usize, map: &mut map::Map, help: &mut utility_class::UtilityClass) -> bool {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("entitycollidefloor() out-of-bounds!");
            return false
        }

        let temprect = sdl2::rect::Rect::new(
            self.entities[t].xp + self.entities[t].cx,
            self.entities[t].yp + self.entities[t].cy + 1,
            self.entities[t].w as u32,
            self.entities[t].h as u32,
        );

        return self.checkwall(temprect, None, None, None, None, None, map, help)
    }

    // bool entityclass::entitycollideroof( int t )
    pub fn entitycollideroof(&mut self, t: usize, map: &mut map::Map, help: &mut utility_class::UtilityClass) -> bool {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("entitycollidefloor() out-of-bounds!");
            return false
        }

        let temprect = sdl2::rect::Rect::new(
            self.entities[t].xp + self.entities[t].cx,
            self.entities[t].yp + self.entities[t].cy - 1,
            self.entities[t].w as u32,
            self.entities[t].h as u32,
        );

        return self.checkwall(temprect, None, None, None, None, None, map, help)
    }

    // bool entityclass::testwallsx( int t, int tx, int ty, const bool skipdirblocks )
    fn testwallsx(&self, t: usize, tx: i32, ty: i32, skipdirblocks: bool) -> bool {
        println!("DEADBEEF: testwallsx::checkactivity() method not implemented yet");
        false
    }

    // bool entityclass::testwallsy( int t, float tx, float ty )
    fn testwallsy(&self, t: i32, tx: f32, ty: f32) -> bool {
        println!("DEADBEEF: testwallsy::checkactivity() method not implemented yet");
        false
    }

    // void entityclass::applyfriction( int t, float xrate, float yrate )
    fn applyfriction(&self, t: i32, xrate: f32, yrate: f32) {
        println!("DEADBEEF: applyfriction::checkactivity() method not implemented yet");
    }

    // void entityclass::updateentitylogic( int t )
    pub fn updateentitylogic(&self, t: i32) {
        println!("DEADBEEF: entityclass::updateentitylogic() method not implemented yet");
    }

    // void entityclass::entitymapcollision( int t )
    pub fn entitymapcollision(&self, t: i32) {
        println!("DEADBEEF: entityclass::entitymapcollision() method not implemented yet");
    }

    // void entityclass::movingplatformfix( int t, int j )
    pub fn movingplatformfix(&self, t: i32, j: i32) {
        println!("DEADBEEF: entityclass::movingplatformfix() method not implemented yet");
    }

    // void entityclass::customwarplinecheck(int i) {
    pub fn customwarplinecheck(&self, i: i32) {
        println!("DEADBEEF: entityclass::customwarplinecheck() method not implemented yet");
    }

    // void entityclass::entitycollisioncheck(void)
    pub fn entitycollisioncheck(&mut self, game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, music: &mut music::Music, help: &mut utility_class::UtilityClass) {
        // for size_t i = 0; i < entities.size(); i++ {
        for i in 0..self.entities.len() {
            let player = self.entities[i].rule == 0;
            let scm = game.supercrewmate && self.entities[i].r#type == 14;
            if !player && !scm {
                continue;
            }

            //We test entity to entity
            // for (size_t j = 0; j < entities.size(); j++ {
            for j in 0..self.entities.len() {
                if i == j {
                    continue;
                }

                self.collisioncheck(i, j, Some(scm), game, graphics, map, music);
            }
        }

        //can't have the player being stuck...
        self.stuckprevention(self.getplayer(), game);

        //Can't have the supercrewmate getting stuck either!
        if game.supercrewmate {
            self.stuckprevention(self.getscm(), game);
        }

        //Is the player colliding with any damageblocks?
        if self.checkdamage(None, help) && !map.invincibility {
            //usual player dead stuff
            game.deathseq = 30;
        }

        //how about the supercrewmate?
        if game.supercrewmate {
            if self.checkdamage(Some(true), help) && !map.invincibility {
                //usual player dead stuff
                game.scmhurt = true;
                game.deathseq = 30;
            }
        }

        // WARNING: If updating this code, don't forget to update Map.cpp mapclass::twoframedelayfix()
        let mut block_idx: i32 = -1;
        let activetrigger = self.checktrigger(&mut block_idx, help);
        if activetrigger > -1 && INBOUNDS_VEC!(block_idx, self.blocks) {
            // Load the block's script if its gamestate is out of range
            if self.blocks[block_idx as usize].script != "" && (activetrigger < 300 || activetrigger > 336) {
                game.startscript = true;
                game.newscript = self.blocks[block_idx as usize].script.to_owned();
                self.removetrigger(activetrigger);
                game.state = 0;
            } else {
                game.state = activetrigger;
            }
            game.statedelay = 0;
        }
    }

    // void entityclass::collisioncheck(int i, int j, bool scm /*= false*/)
    fn collisioncheck(&mut self, i: usize, j: usize, scm: Option<bool>, game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, music: &mut music::Music) {
        let scm = scm.unwrap_or(false);

        if !INBOUNDS_VEC!(i, self.entities) || !INBOUNDS_VEC!(j, self.entities) {
            println!("collisioncheck() out-of-bounds!");
            return;
        }

        match self.entities[j].rule {
            1 => {
                if !self.entities[j].harmful {
                    return
                }

                //person i hits enemy or enemy bullet j
                if self.entitycollide(i, j) && !map.invincibility {
                    if self.entities[i].size == 0 && (self.entities[j].size == 0 || self.entities[j].size == 12) {
                        //They're both sprites, so do a per pixel collision
                        let colpoint1 = maths::point {
                            x: self.entities[i].xp,
                            y: self.entities[i].yp,
                        };
                        let colpoint2 = maths::point {
                            x: self.entities[j].xp,
                            y: self.entities[j].yp,
                        };
                        let drawframe1 = self.entities[i].drawframe;
                        let drawframe2 = self.entities[j].drawframe;

                        graphics.Hitest();
                        // std::vector<SDL_Surface*>& spritesvec = graphics.flipmode ? graphics.flipsprites : graphics.sprites;
                        // if INBOUNDS_VEC!(drawframe1, spritesvec) &&
                        //     INBOUNDS_VEC!(drawframe2, spritesvec) &&
                        //     graphics.Hitest(spritesvec[drawframe1], colpoint1, spritesvec[drawframe2], colpoint2)
                        // {
                        //     //Do the collision stuff
                        //     game.deathseq = 30;
                        //     game.scmhurt = scm;
                        // }
                    } else {
                        //Ok, then we just assume a normal bounding box collision
                        game.deathseq = 30;
                        game.scmhurt = scm;
                    }
                }
            },
            2 => {
                //Moving platforms
                if self.entities[j].behave >= 8 && self.entities[j].behave < 10 {
                    //We don't want conveyors, moving platforms only
                    return
                }
                if self.entitycollide(i, j) {
                    //Disable collision temporarily so we don't push the person out!
                    //Collision will be restored at end of platform update loop in gamelogic
                    self.disableblockat(self.entities[j].xp, self.entities[j].yp);
                }
            },
            3 => {
                //Entity to entity
                if self.entities[j].onentity > 0 {
                    if self.entitycollide(i, j) {
                        self.entities[j].state = self.entities[j].onentity;
                    }
                }
            },
            4 => {
                //Person vs horizontal line!
                if game.deathseq == -1 {
                    //Here we compare the person's old position versus his new one versus the line.
                    //All points either be above or below it. Otherwise, there was a collision this frame.
                    if self.entities[j].onentity > 0 {
                        if self.entityhlinecollide(i, j) {
                            music.playef(8);
                            game.gravitycontrol = (game.gravitycontrol + 1) % 2;
                            game.totalflips += 1;
                            if game.gravitycontrol == 0 {
                                if self.entities[i].vy < 1.0 {
                                    self.entities[i].vy = 1.0;
                                }
                            } else {
                                if self.entities[i].vy > -1.0 {
                                    self.entities[i].vy = -1.0;
                                }
                            }

                            self.entities[j].state = self.entities[j].onentity;
                            self.entities[j].life = 6;
                        }
                    }
                }
            },
            5 => {
                //Person vs vertical gravity/warp line!
                if game.deathseq == -1 {
                    if self.entities[j].onentity > 0 {
                        if self.entityvlinecollide(i, j) {
                            self.entities[j].state = self.entities[j].onentity;
                            self.entities[j].life = 4;
                        }
                    }
                }
            },
            6 => {
                //Person versus crumbly blocks! Special case
                if self.entities[j].onentity > 0 {
                    //ok; only check the actual collision if they're in a close proximity
                    let mut temp = self.entities[i].yp - self.entities[j].yp;
                    if temp > -30 && temp < 30 {
                        temp = self.entities[i].xp - self.entities[j].xp;
                        if temp > -30 && temp < 30 {
                            if self.entitycollide(i, j) {
                                self.entities[j].state = self.entities[j].onentity;
                            }
                        }
                    }
                }
            },
            7 => {
                // Person versus horizontal warp line, pre-2.1
                if game.glitchrunnermode && game.deathseq == -1 && self.entities[j].onentity > 0 && self.entityhlinecollide(i, j) {
                    self.entities[j].state = self.entities[j].onentity;
                }
            },
            _ => (),
        };
    }

    // void entityclass::stuckprevention(int t)
    fn stuckprevention(&mut self, t: i32, game: &mut game::Game) {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("stuckprevention() out-of-bounds!");
            return;
        }
        let t = t as usize;

        // Can't have this entity (player or supercrewmate) being stuck...
        if !self.testwallsx(t, self.entities[t].xp, self.entities[t].yp, true) {
            // Let's try to get out...
            if game.gravitycontrol == 0 {
                self.entities[t].yp -= 3;
            } else {
                self.entities[t].yp += 3;
            }
        }
    }

}

#[derive(PartialEq)]
pub enum EntityEnum {
    BLOCK = 0,
    TRIGGER = 1,
    DAMAGE = 2,
    DIRECTIONAL = 3,
    SAFE = 4,
    ACTIVITY = 5
}
