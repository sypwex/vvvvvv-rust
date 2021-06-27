use crate::{game, map};
mod ent;

pub struct EntityClass {
    pub entities: Vec<ent::EntClass>,
    linecrosskludge: Vec<ent::EntClass>,

    k: i32,

    // std::vector<blockclass> blocks,
    pub flags: [bool; 100],
    pub collect: [bool; 100],
    pub customcollect: [bool; 100],

    platformtile: i32,
    vertplatforms: bool,
    horplatforms: bool,

    // :(
    pub nearelephant: bool,
    pub upsetmode: bool,
    pub upset: i32,

    //Trophy Text
    pub trophytext: i32,
    pub trophytype: i32,
    oldtrophytext: i32,

    //Secret lab scripts
    pub altstates: i32,

    //Custom stuff
    customenemy: i32,
    customplatformtile: i32,
    customwarpmode: bool,
    customwarpmodevon: bool,
    customwarpmodehon: bool,
    customscript: String,
    pub customcrewmoods: [bool; game::numcrew],
}

impl<'a> EntityClass {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            linecrosskludge: Vec::new(),

            k: 0,

            // std::vector<blockclass> blocks,
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
    // void entityclass::gravcreate( int ypos, int dir, int xoff /*= 0*/, int yoff /*= 0*/ )
    // void entityclass::generateswnwave( int t )

    // void entityclass::createblock( int t, int xp, int yp, int w, int h, int trig /*= 0*/, const std::string& script /*= ""*/ )
    fn createblock(&mut self, t: i32, xp: i32, yp: i32, w: i32, h: i32, trig: Option<i32>, script: Option<String>) -> bool {
        let trig = trig.unwrap_or(0);
        let script = script.unwrap_or(String::new());

        println!("DEADBEEF: entityclass::createblock() method not implemented yet");
        false
    }

    // bool entityclass::disableentity(int t)
    fn disableentity(&mut self, t: i32) -> bool {
        println!("DEADBEEF: entityclass::disableentity() method not implemented yet");
        false
    }

    // void entityclass::removeallblocks(void)
    // void entityclass::disableblock( int t )
    // void entityclass::moveblockto(int x1, int y1, int x2, int y2, int w, int h)
    // void entityclass::disableblockat(int x, int y)
    // void entityclass::removetrigger( int t )
    // void entityclass::copylinecross( int t )
    // void entityclass::revertlinecross( int t, int s )

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

        self.entities[entity_id].drawframe = self.entities[entity_id].tile;

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
    fn updateentities(&mut self, i: i32) -> bool {
        println!("DEADBEEF: entityclass::updateentities() method not implemented yet");
        false
    }

    // void entityclass::animateentities( int _i )
    // int entityclass::getcompanion(void)

    // int entityclass::getplayer(void)
    pub fn getplayer(&mut self) -> Option<usize> {
        //Returns the index of the first player entity
        for i in 0..self.entities.len() {
            if self.entities[i].r#type == 0 {
                return Some(i)
            }
        }

        None
    }

    // int entityclass::getscm(void)
    // int entityclass::getlineat( int t )
    // int entityclass::getcrewman( int t )
    // int entityclass::getcustomcrewman( int t )
    // int entityclass::getteleporter(void)
    // bool entityclass::entitycollide( int a, int b )
    // bool entityclass::checkdamage(bool scm /*= false*/)
    // int entityclass::checktrigger(int* block_idx)
    // int entityclass::checkactivity(void)
    // int entityclass::getgridpoint( int t )
    // bool entityclass::checkplatform(const SDL_Rect& temprect, int* px, int* py)
    // bool entityclass::checkblocks(const SDL_Rect& temprect, const float dx, const float dy, const float dr, const bool skipdirblocks)
    // bool entityclass::checkwall(const SDL_Rect& temprect, const float dx, const float dy, const float dr, const bool skipblocks, const bool skipdirblocks)
    // bool entityclass::checkwall(const SDL_Rect& temprect)
    // float entityclass::hplatformat(const int px, const int py)
    // int entityclass::yline( int a, int b )
    // bool entityclass::entityhlinecollide( int t, int l )
    // bool entityclass::entityvlinecollide( int t, int l )
    // bool entityclass::entitywarphlinecollide(int t, int l) {
    // bool entityclass::entitywarpvlinecollide(int t, int l) {
    // float entityclass::entitycollideplatformroof( int t )
    // float entityclass::entitycollideplatformfloor( int t )
    // bool entityclass::entitycollidefloor( int t )
    // bool entityclass::entitycollideroof( int t )
    // bool entityclass::testwallsx( int t, int tx, int ty, const bool skipdirblocks )
    // bool entityclass::testwallsy( int t, float tx, float ty )
    // void entityclass::applyfriction( int t, float xrate, float yrate )
    // void entityclass::updateentitylogic( int t )
    // void entityclass::entitymapcollision( int t )
    // void entityclass::movingplatformfix( int t, int j )
    // void entityclass::customwarplinecheck(int i) {
    // void entityclass::entitycollisioncheck(void)
    // void entityclass::collisioncheck(int i, int j, bool scm /*= false*/)
    // void entityclass::stuckprevention(int t)
}

enum EntityEnum {
    BLOCK = 0,
    TRIGGER = 1,
    DAMAGE = 2,
    DIRECTIONAL = 3,
    SAFE = 4,
    ACTIVITY = 5
}
