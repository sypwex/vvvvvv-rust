use crate::{INBOUNDS_ARR, INBOUNDS_VEC, game, map, maths, music, screen::render::graphics, script, utility_class};

mod ent;
mod blockv;

#[derive(Debug,PartialEq)]
pub enum EntityEnum {
    BLOCK = 0,
    TRIGGER = 1,
    DAMAGE = 2,
    DIRECTIONAL = 3,
    SAFE = 4,
    ACTIVITY = 5
}

pub struct EntityClass {
    pub entities: Vec<ent::EntClass>,
    pub linecrosskludge: Vec<ent::EntClass>,

    k: i32,

    // std::vector<blockclass> blocks,
    pub blocks: Vec<blockv::BlockClass>,
    pub flags: [bool; 100],
    pub collect: [bool; 100],
    pub customcollect: [bool; 100],

    pub platformtile: i32,
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
    pub customplatformtile: i32,
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

    // void fatal_top(void)
    pub fn fatal_top(&mut self) {
        self.createblock(EntityEnum::DAMAGE as i32, -8, -8, 384, 16, None, None);
    }

    // void fatal_bottom(void)
    pub fn fatal_bottom(&mut self) {
        self.createblock(EntityEnum::DAMAGE as i32, -8, 224, 384, 16, None, None);
    }

    // void fatal_left(void)
    pub fn fatal_left(&mut self) {
        self.createblock(EntityEnum::DAMAGE as i32, -8, -8, 16, 260, None, None);
    }

    // void fatal_right(void)
    pub fn fatal_right(&mut self) {
        self.createblock(EntityEnum::DAMAGE as i32, 312, -8, 16, 260, None, None);
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
    #[allow(unused_assignments)]
    pub fn createblock(&mut self, t: i32, xp: i32, yp: i32, w: i32, h: i32, trig: Option<i32>, script: Option<String>) {
        // TODO: @sx: fix rustc(unused_assignments) someday
        let mut trig = trig.unwrap_or(0);
        let script = script.unwrap_or(String::new());

        self.k = self.blocks.len() as i32;
        let blockptr = self.get_blockptr();
        match t {
            t if t == EntityEnum::BLOCK as i32 => {
                self.blocks[blockptr].r#type = EntityEnum::BLOCK;
                self.blocks[blockptr].xp = xp;
                self.blocks[blockptr].yp = yp;
                self.blocks[blockptr].wp = w;
                self.blocks[blockptr].hp = h;
                self.blocks[blockptr].rectset(xp, yp, w, h);
            },
            t if t == EntityEnum::TRIGGER as i32 => {
                self.blocks[blockptr].r#type = EntityEnum::TRIGGER;
                self.blocks[blockptr].wp = w;
                self.blocks[blockptr].hp = h;
                self.blocks[blockptr].rectset(xp, yp, w, h);
                self.blocks[blockptr].trigger = trig;
                self.blocks[blockptr].script = script;
                info!("new trigger: {:?}", self.blocks[blockptr]);
            },
            t if t == EntityEnum::DAMAGE as i32 => {
                self.blocks[blockptr].r#type = EntityEnum::DAMAGE;
                self.blocks[blockptr].wp = w;
                self.blocks[blockptr].hp = h;
                self.blocks[blockptr].rectset(xp, yp, w, h);
                // info!("new damage: {:?}", self.blocks[blockptr]);
            },
            t if t == EntityEnum::DIRECTIONAL as i32 => {
                self.blocks[blockptr].r#type = EntityEnum::DIRECTIONAL;
                self.blocks[blockptr].wp = w;
                self.blocks[blockptr].hp = h;
                self.blocks[blockptr].rectset(xp, yp, w, h);
                self.blocks[blockptr].trigger = trig;
                info!("new directional: {:?}", self.blocks[blockptr]);
            },
            t if t == EntityEnum::SAFE as i32 => {
                self.blocks[blockptr].r#type = EntityEnum::SAFE;
                self.blocks[blockptr].xp = xp;
                self.blocks[blockptr].yp = yp;
                self.blocks[blockptr].wp = w;
                self.blocks[blockptr].hp = h;
                self.blocks[blockptr].rectset(xp, yp, w, h);
                info!("new safe: {:?}", self.blocks[blockptr]);
            },
            t if t == EntityEnum::ACTIVITY as i32 => {
                //Activity Zone
                self.blocks[blockptr].r#type = EntityEnum::ACTIVITY;
                self.blocks[blockptr].wp = w;
                self.blocks[blockptr].hp = h;
                self.blocks[blockptr].rectset(xp, yp, w, h);

                //Ok, each and every activity zone in the game is initilised here. "Trig" in this case is a variable that
                //assigns all the details.
                match trig {
                    0 => {
                        //testing zone
                        self.blocks[blockptr].prompt = "Press ENTER to explode".to_string();
                        self.blocks[blockptr].script = "intro".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 1;
                    },
                    1 => {
                        self.blocks[blockptr].prompt = "Press ENTER to talk to Violet".to_string();
                        self.blocks[blockptr].script = "talkpurple".to_string();
                        self.blocks[blockptr].setblockcolour("purple");
                        trig = 0;
                    },
                    2 => {
                        self.blocks[blockptr].prompt = "Press ENTER to talk to Vitellary".to_string();
                        self.blocks[blockptr].script = "talkyellow".to_string();
                        self.blocks[blockptr].setblockcolour("yellow");
                        trig = 0;
                    },
                    3 => {
                        self.blocks[blockptr].prompt = "Press ENTER to talk to Vermilion".to_string();
                        self.blocks[blockptr].script = "talkred".to_string();
                        self.blocks[blockptr].setblockcolour("red");
                        trig = 0;
                    },
                    4 => {
                        self.blocks[blockptr].prompt = "Press ENTER to talk to Verdigris".to_string();
                        self.blocks[blockptr].script = "talkgreen".to_string();
                        self.blocks[blockptr].setblockcolour("green");
                        trig = 0;
                    },
                    5 => {
                        self.blocks[blockptr].prompt = "Press ENTER to talk to Victoria".to_string();
                        self.blocks[blockptr].script = "talkblue".to_string();
                        self.blocks[blockptr].setblockcolour("blue");
                        trig = 0;
                    },
                    6 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_station_1".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    7 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_outside_1".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    8 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_outside_2".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    9 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_outside_3".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    10 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_outside_4".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    11 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_outside_5".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    12 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_outside_6".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    13 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_finallevel".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    14 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_station_2".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    15 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_station_3".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    16 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_station_4".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    17 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_warp_1".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    18 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_warp_2".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    19 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_lab_1".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    20 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_lab_2".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    21 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_secretlab".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    22 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_shipcomputer".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    23 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminals".to_string();
                        self.blocks[blockptr].script = "terminal_radio".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    24 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "terminal_jukebox".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    25 => {
                        self.blocks[blockptr].prompt = "Passion for Exploring".to_string();
                        self.blocks[blockptr].script = "terminal_juke1".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    26 => {
                        self.blocks[blockptr].prompt = "Pushing Onwards".to_string();
                        self.blocks[blockptr].script = "terminal_juke2".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    27 => {
                        self.blocks[blockptr].prompt = "Positive Force".to_string();
                        self.blocks[blockptr].script = "terminal_juke3".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    28 => {
                        self.blocks[blockptr].prompt = "Presenting VVVVVV".to_string();
                        self.blocks[blockptr].script = "terminal_juke4".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    29 => {
                        self.blocks[blockptr].prompt = "Potential for Anything".to_string();
                        self.blocks[blockptr].script = "terminal_juke5".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    30 => {
                        self.blocks[blockptr].prompt = "Predestined Fate".to_string();
                        self.blocks[blockptr].script = "terminal_juke6".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    31 => {
                        self.blocks[blockptr].prompt = "Pipe Dream".to_string();
                        self.blocks[blockptr].script = "terminal_juke7".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    32 => {
                        self.blocks[blockptr].prompt = "Popular Potpourri".to_string();
                        self.blocks[blockptr].script = "terminal_juke8".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    33 => {
                        self.blocks[blockptr].prompt = "Pressure Cooker".to_string();
                        self.blocks[blockptr].script = "terminal_juke9".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    34 => {
                        self.blocks[blockptr].prompt = "ecroF evitisoP".to_string();
                        self.blocks[blockptr].script = "terminal_juke10".to_string();
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    35 => {
                        self.blocks[blockptr].prompt = "Press ENTER to activate terminal".to_string();
                        self.blocks[blockptr].script = "custom_".to_owned() + &self.customscript;
                        self.blocks[blockptr].setblockcolour("orange");
                        trig = 0;
                    },
                    _ => println!("{}", 0),
                };

                info!("new activity: {:?}", self.blocks[blockptr]);
            },
            _ => println!("TODO: refactor to enum... unmatched type: {}", t),
        }
    }

    // TODO: @sx interesting point for borrow checker
    fn get_blockptr(&mut self) -> usize {
        /* Can we reuse the slot of a disabled block? */
        for i in 0..self.blocks.len() {
            if self.blocks[i].wp == 0 && self.blocks[i].hp == 0 && self.blocks[i].rect.w == 0 && self.blocks[i].rect.h == 0 {
                self.blocks[i].clear();
                return i;
            }
        }

        let block = blockv::BlockClass::new();
        self.blocks.push(block);
        return self.blocks.len() - 1;
    }

    // bool entityclass::disableentity(int t)
    pub fn disableentity(&mut self, t: usize) -> bool {
        if !INBOUNDS_VEC!(t, self.entities) {
            warn!("disableentity() out-of-bounds!");
            return true;
        }
        if self.entities[t].rule == 0 && t == self.getplayer() as usize {
            /* Don't disable the player entity! */
            return false
        }

        self.entities[t].invis = true;
        self.entities[t].size = -1;
        self.entities[t].r#type = -1;
        self.entities[t].rule = -1;

        true
    }

    // void entityclass::removeallblocks(void)
    pub fn removeallblocks(&mut self) {
        self.blocks.clear();
    }

    // void entityclass::disableblock( int t )
    pub fn disableblock(&mut self, t: usize) {
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
        // for size_t i = 0; i < blocks.size(); i++ {
        for i in 0..self.blocks.len() {
            if self.blocks[i].xp == x1 && self.blocks[i].yp == y1 {
                self.blocks[i].xp = x2;
                self.blocks[i].yp = y2;

                self.blocks[i].wp = w;
                self.blocks[i].hp = h;

                match self.blocks[i] {
                    blockv::BlockClass { xp, yp, wp, hp, .. } => self.blocks[i].rectset(xp, yp, wp, hp),
                };
                break;
            }
        }
    }

    // void entityclass::disableblockat(int x, int y)
    pub fn disableblockat(&mut self, x: i32, y: i32) {
        for i in 0..self.blocks.len() {
            if self.blocks[i].xp == x && self.blocks[i].yp == y {
                self.disableblock(i);
            }
        }
    }

    // void entityclass::removetrigger( int t )
    pub fn removetrigger(&mut self, t: i32) {
        for i in 0..self.blocks.len() {
            if self.blocks[i].r#type == EntityEnum::TRIGGER && self.blocks[i].trigger == t {
                self.disableblock(i);
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
        self.linecrosskludge.push(self.entities[t]);
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
        p1 == p11 && p2 == p21 && p3 == p31 && p4 == p41
    }

    // int entityclass::crewcolour( int t )
    pub fn crewcolour(&mut self, t: i32) -> i32 {
        //return the colour of the indexed crewmate
        match t {
            0 => 0,
            1 => 20,
            2 => 14,
            3 => 15,
            4 => 13,
            5 => 16,
            _ => 0,
        }
    }

    // void entityclass::createentity(int xp, int yp, int t, int meta1, int meta2, int p1, int p2, int p3, int p4)
    // void entityclass::createentity(int xp, int yp, int t, int meta1, int meta2, int p1, int p2)
    // void entityclass::createentity(int xp, int yp, int t, int meta1, int meta2, int p1)
    // void entityclass::createentity(int xp, int yp, int t, int meta1, int meta2)
    // void entityclass::createentity(int xp, int yp, int t, int meta1)
    // void entityclass::createentity(int xp, int yp, int t)
    pub fn createentity(&mut self, xp: i32, yp: i32, t: i32, meta1: Option<i32>, meta2: Option<i32>, p1: Option<i32>, p2: Option<i32>, p3: Option<i32>, p4: Option<i32>, game: &mut game::Game) {
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
                // if !INBOUNDS_ARR!(meta1, self.collect) || self.collect[meta1] {
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
                // if !INBOUNDS_ARR!(meta1, self.collect) || self.collect[meta1] {
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
                // if INBOUNDS_ARR!(meta1, collect) && !collect[meta1] {
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
                    53 | 54 => {
                        self.entities[entity_id].rule = 7;
                        self.entities[entity_id].size = 5;
                        self.entities[entity_id].w = meta1;
                        self.entities[entity_id].h = 1;
                    },
                    _ => (),
                };
                // @sx: TODO: only applicable to custom mode
                // if map.custommode {
                //     self.customwarpmode = true;
                //     map.warpx = false;
                //     map.warpy = false;
                // }
            },
            55 => {
                // Crew Member (custom, collectable)
                //1 - position in array
                //2 - colour
                self.entities[entity_id].rule = 3;
                self.entities[entity_id].r#type = 55;
                // if INBOUNDS_ARR!(meta2, self.customcrewmoods && self.customcrewmoods[meta2]==1) {
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
                // if !INBOUNDS_ARR!(meta1, self.customcollect) || self.customcollect[meta1] {
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
    pub fn updateentities(&mut self, i: usize, game: &mut game::Game, map: &mut map::Map, music: &mut music::Music, script: &mut script::ScriptClass) -> bool {
        if !INBOUNDS_VEC!(i, self.entities) {
            println!("updateentities() out-of-bounds!");
            return true;
        }

        if self.entities[i].statedelay <= 0 {
            match self.entities[i].r#type {
                0 => {
                    //Playerk
                },
                1 => {
                    //Movement behaviors
                    //Enemies can have a number of different behaviors:
                    match self.entities[i].behave {
                        0 => {
                            //Bounce, Start moving down
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].state = 3;
                                let entitygone = self.updateentities(i, game, map, music, script);
                                if entitygone {
                                    return true;
                                }
                            } else if self.entities[i].state == 1 {
                                if self.entities[i].outside() {
                                    self.entities[i].state = self.entities[i].onwall;
                                }
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vy = -self.entities[i].para;
                                self.entities[i].onwall = 3;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 3 {
                                self.entities[i].vy = self.entities[i].para;
                                self.entities[i].onwall = 2;
                                self.entities[i].state = 1;
                            }
                        },
                        1 => {
                            //Bounce, Start moving up
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].state = 2;
                                let entitygone = self.updateentities(i, game, map, music, script);
                                if entitygone {
                                    return true;
                                }
                            } else if self.entities[i].state == 1 {
                                if self.entities[i].outside() {
                                    self.entities[i].state = self.entities[i].onwall;
                                }
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vy = -self.entities[i].para;
                                self.entities[i].onwall = 3;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 3 {
                                self.entities[i].vy = self.entities[i].para;
                                self.entities[i].onwall = 2;
                                self.entities[i].state = 1;
                            }
                        },
                        2 => {
                            //Bounce, Start moving left
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].state = 3;
                                let entitygone = self.updateentities(i, game, map, music, script);
                                if entitygone {
                                    return true;
                                }
                            } else if self.entities[i].state == 1 {
                                if self.entities[i].outside() {
                                    self.entities[i].state = self.entities[i].onwall;
                                }
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vx = self.entities[i].para;
                                self.entities[i].onwall = 3;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 3 {
                                self.entities[i].vx = -self.entities[i].para;
                                self.entities[i].onwall = 2;
                                self.entities[i].state = 1;
                            }
                        },
                        3 => {
                            //Bounce, Start moving right
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].state = 3;
                                let entitygone = self.updateentities(i, game, map, music, script);
                                if entitygone {
                                    return true;
                                }
                            } else if self.entities[i].state == 1 {
                                if self.entities[i].outside() {
                                    self.entities[i].state = self.entities[i].onwall;
                                }
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vx = -self.entities[i].para;
                                self.entities[i].onwall = 3;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 3 {
                                self.entities[i].vx = self.entities[i].para;
                                self.entities[i].onwall = 2;
                                self.entities[i].state = 1;
                            }
                        },
                        4 => {
                            //Always move left
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vx = self.entities[i].para;
                            }
                        },
                        5 => {
                            //Always move right
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vx = self.entities[i].para.trunc();
                                self.entities[i].state = 1;
                                self.entities[i].onwall = 2;
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vx = 0.0;
                                self.entities[i].onwall = 0;
                                self.entities[i].xp -= self.entities[i].para.trunc() as i32;
                                self.entities[i].statedelay = 8;
                                self.entities[i].state = 0;
                            }
                        },
                        6 => {
                            //Always move up
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vy = self.entities[i].para.trunc();
                                self.entities[i].state = 1;
                                self.entities[i].onwall = 2;
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vy = -self.entities[i].para.trunc();
                                self.entities[i].onwall = 0;
                                self.entities[i].yp -= self.entities[i].para.trunc() as i32;
                                self.entities[i].statedelay = 8;
                                self.entities[i].state = 0;
                            }
                        },
                        7 => {
                            //Always move down
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vx = self.entities[i].para.trunc();
                            }
                        },
                        8 | 9 => {
                            //Threadmill: don't move, just impart velocity
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vx = 0.0;
                                self.entities[i].state = 1;
                                self.entities[i].onwall = 0;
                            }
                        },
                        10 => {
                            //Emitter: shoot an enemy every so often
                            if self.entities[i].state == 0 {
                                self.createentity(self.entities[i].xp+28, self.entities[i].yp, 1, Some(10), Some(1), None, None, None, None, game);
                                self.entities[i].state = 1;
                                self.entities[i].statedelay = 12;
                            } else if self.entities[i].state == 1 {
                                self.entities[i].state = 0;
                            }
                        },
                        11 => {
                            //Always move right, destroy when outside screen
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vx = self.entities[i].para;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 1 {
                                if self.entities[i].xp >= 335 {
                                    return self.disableentity(i);
                                }
                                if game.roomx == 117 {
                                    if self.entities[i].xp >= (33*8)-32 {
                                        return self.disableentity(i);
                                    }
                                    //collector for LIES
                                }
                            }
                        },
                        12 => {
                            //Emitter: shoot an enemy every so often (up)
                            if self.entities[i].state == 0 {
                                self.createentity(self.entities[i].xp, self.entities[i].yp, 1, Some(12), Some(1), None, None, None, None, game);
                                self.entities[i].state = 1;
                                self.entities[i].statedelay = 16;
                            } else if self.entities[i].state == 1 {
                                self.entities[i].state = 0;
                            }
                        },
                        13 => {
                            //Always move up, destroy when outside screen
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vy = self.entities[i].para;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 1 {
                                if self.entities[i].yp <= -60 {
                                    return self.disableentity(i);
                                }
                                if game.roomx == 113 && game.roomy == 108 {
                                    if self.entities[i].yp <= 60 {
                                        return self.disableentity(i);
                                    }
                                    //collector for factory
                                }
                            }
                        },
                        14 => {
                            //Very special hack: as two, but doesn't move in specific circumstances
                            if self.entities[i].state == 0 {
                                //Init
                                // for (size_t j = 0; j < self.entities.size(); j++ {
                                for j in 0..self.entities.len() {
                                    if self.entities[j].r#type == 2 && self.entities[j].state == 3 && self.entities[j].xp == (self.entities[i].xp-32)  {
                                        self.entities[i].state = 3;
                                        let entitygone = self.updateentities(i, game, map, music, script);
                                        if entitygone {
                                            return true;
                                        }
                                    }
                                }
                            } else if self.entities[i].state == 1 {
                                if self.entities[i].outside() {
                                    self.entities[i].state = self.entities[i].onwall;
                                }
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vx = self.entities[i].para;
                                self.entities[i].onwall = 3;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 3 {
                                self.entities[i].vx = -self.entities[i].para;
                                self.entities[i].onwall = 2;
                                self.entities[i].state = 1;
                            }
                        },
                        15 => {
                            //As above, but for 3!
                            if self.entities[i].state == 0 {
                                //Init
                                // for (size_t j = 0; j < self.entities.size(); j++ {
                                for j in 0..self.entities.len() {
                                    if self.entities[j].r#type == 2 && self.entities[j].state == 3 && self.entities[j].xp == self.entities[i].xp + 32 {
                                        self.entities[i].state = 3;
                                        let entitygone = self.updateentities(i, game, map, music, script);
                                        if entitygone {
                                            return true;
                                        }
                                    }
                                }
                            } else if self.entities[i].state == 1 {
                                if self.entities[i].outside() {
                                    self.entities[i].state = self.entities[i].onwall;
                                }
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vx = -self.entities[i].para;
                                self.entities[i].onwall = 3;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 3 {
                                self.entities[i].vx = self.entities[i].para;
                                self.entities[i].onwall = 2;
                                self.entities[i].state = 1;
                            }
                        },
                        16 => {
                            //MAVERICK BUS FOLLOWS HIS OWN RULES
                            if self.entities[i].state == 0 {
                                //Init
                                let player = self.getplayer() as usize;
                                //first, y position
                                if INBOUNDS_VEC!(player, self.entities) && self.entities[player].yp > 14 * 8 {
                                    self.entities[i].tile = 120;
                                    self.entities[i].yp = (28*8)-62;
                                    self.entities[i].lerpoldyp = (28*8)-62;
                                } else {
                                    self.entities[i].tile = 96;
                                    self.entities[i].yp = 24;
                                    self.entities[i].lerpoldyp = 24;
                                }
                                //now, x position
                                if INBOUNDS_VEC!(player, self.entities) && self.entities[player].xp > 20 * 8 {
                                    //approach from the left
                                    self.entities[i].xp = -64;
                                    self.entities[i].lerpoldxp = -64;
                                    self.entities[i].state = 2;
                                    let entitygone = self.updateentities(i, game, map, music, script); //right
                                    if entitygone {
                                        return true;
                                    }
                                } else {
                                    //approach from the left
                                    self.entities[i].xp = 320;
                                    self.entities[i].lerpoldxp = 320;
                                    self.entities[i].state = 3;
                                    let entitygone = self.updateentities(i, game, map, music, script); //left
                                    if entitygone {
                                        return true;
                                    }
                                }

                            } else if self.entities[i].state == 1 {
                                if self.entities[i].outside() {
                                    self.entities[i].state = self.entities[i].onwall;
                                }
                            } else if self.entities[i].state == 2 {
                                self.entities[i].vx = self.entities[i].para.trunc();
                                self.entities[i].onwall = 3;
                                self.entities[i].state = 1;
                            } else if self.entities[i].state == 3 {
                                self.entities[i].vx = -self.entities[i].para.trunc();
                                self.entities[i].onwall = 2;
                                self.entities[i].state = 1;
                            }
                        },
                        17 => {
                            //Special for ASCII Snake (left)
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].statedelay = 6;
                                self.entities[i].xp -= self.entities[i].para as i32;
                                self.entities[i].lerpoldxp -= self.entities[i].para as i32;
                            }
                        },
                        18 => {
                            //Special for ASCII Snake (right)
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].statedelay = 6;
                                self.entities[i].xp += self.entities[i].para as i32;
                                self.entities[i].lerpoldxp += self.entities[i].para as i32;
                            }
                        },
                        _ => println!("unknown movement behaviour ({})", self.entities[i].behave),
                    }
                },
                2 => {
                    //Disappearing platforms
                    //wait for collision
                    if self.entities[i].state == 1 {
                        self.entities[i].life = 12;
                        self.entities[i].state = 2;
                        self.entities[i].onentity = 0;

                        music.playef(7);
                    } else if self.entities[i].state == 2 {
                        self.entities[i].life -= 1;
                        if self.entities[i].life % 3 == 0 {
                            self.entities[i].walkingframe += 1;
                        }
                        if self.entities[i].life <= 0 {
                            self.disableblockat(self.entities[i].xp, self.entities[i].yp);
                            self.entities[i].state = 3;// = false;
                            self.entities[i].invis = true;
                        }
                    } else if self.entities[i].state == 3 {
                        //wait until recharged!
                    } else if self.entities[i].state == 4 {
                        //restart!
                        self.createblock(0, self.entities[i].xp, self.entities[i].yp, 32, 8, None, None);
                        self.entities[i].state = 4;
                        self.entities[i].invis = false;
                        self.entities[i].walkingframe -= 1;
                        self.entities[i].state += 1;
                        self.entities[i].onentity = 1;
                    } else if self.entities[i].state == 5 {
                        self.entities[i].life += 3;
                        if self.entities[i].life % 3 == 0 {
                            self.entities[i].walkingframe -= 1;
                        }
                        if self.entities[i].life >= 12 {
                            self.entities[i].life = 12;
                            self.entities[i].state = 0;
                            self.entities[i].walkingframe += 1;
                        }
                    }
                },
                3 => {
                    //Breakable blocks
                    //Only counts if vy of player entity is non zero
                    if self.entities[i].state == 1 {
                        self.entities[i].life = 4;
                        self.entities[i].state = 2;
                        self.entities[i].onentity = 0;
                        music.playef(6);
                    } else if self.entities[i].state == 2 {
                        self.entities[i].life -= 1;
                        self.entities[i].tile += 1;
                        if self.entities[i].life <= 0 {
                            self.disableblockat(self.entities[i].xp, self.entities[i].yp);
                            return self.disableentity(i);
                        }
                    }
                },
                4 => {
                    //Gravity token
                    //wait for collision
                    if self.entities[i].state == 1 {
                        game.gravitycontrol = (game.gravitycontrol + 1) % 2;
                        return self.disableentity(i);
                    }
                },
                5 => {
                    //Particle sprays
                    if self.entities[i].state == 0 {
                        self.entities[i].life -= 1;
                        if self.entities[i].life < 0 {
                            return self.disableentity(i);
                        }
                    }
                },
                6 => {
                    //Small pickup
                    //wait for collision
                    if self.entities[i].state == 1 {
                        music.playef(4);
                        if INBOUNDS_ARR!(self.entities[i].para as usize, self.collect) {
                            self.collect[self.entities[i].para as usize] = true;
                        }

                        return self.disableentity(i);
                    }
                },
                7 => {
                    //Found a trinket
                    //wait for collision
                    if self.entities[i].state == 1 {
                        if INBOUNDS_ARR!(self.entities[i].para as usize, self.collect) {
                            self.collect[self.entities[i].para as usize] = true;
                        }

                        if game.intimetrial {
                            music.playef(25);
                        } else {
                            game.state = 1000;
                            if music.currentsong != -1 {
                                music.silencedasmusik();
                            }
                            music.playef(3);
                            if game.trinkets(self) > game.stat_trinkets && !map.custommode {
                                game.stat_trinkets = game.trinkets(self);
                            }
                        }

                        return self.disableentity(i);
                    }
                },
                8 => {
                    //Savepoints
                    //wait for collision
                    if self.entities[i].state == 1 {
                        //First, deactivate all other savepoints
                        // for (size_t j = 0; j < self.entities.size(); j++ {
                        for j in 0..self.entities.len() {
                            if self.entities[j].r#type == 8 {
                                self.entities[j].colour = 4;
                                self.entities[j].onentity = 1;
                            }
                        }
                        self.entities[i].colour = 5;
                        self.entities[i].onentity = 0;
                        game.savepoint = self.entities[i].para as i32;
                        music.playef(5);

                        game.savex = self.entities[i].xp - 4;

                        if self.entities[i].tile == 20 {
                            game.savey = self.entities[i].yp - 2;
                            game.savegc = 1;
                        } else if self.entities[i].tile == 21 {
                            game.savey = self.entities[i].yp - 7;
                            game.savegc = 0;
                        }

                        game.saverx = game.roomx;
                        game.savery = game.roomy;
                        let player = self.getplayer() as usize;
                        if INBOUNDS_VEC!(player, self.entities) {
                            game.savedir = self.entities[player].dir;
                        }
                        self.entities[i].state = 0;
                    }
                },
                9 => {
                    //Gravity Lines
                    if self.entities[i].state == 1 {
                        self.entities[i].life -= 1;
                        self.entities[i].onentity = 0;

                        if self.entities[i].life <= 0 {
                            self.entities[i].state = 0;
                            self.entities[i].onentity = 1;
                        }
                    }
                },
                10 => {
                    //Vertical gravity Lines
                    if self.entities[i].state == 1 {
                        self.entities[i].onentity = 3;
                        self.entities[i].state = 2;

                        music.playef(8);
                        game.gravitycontrol = (game.gravitycontrol + 1) % 2;
                        game.totalflips += 1;
                        let temp = self.getplayer() as usize;
                        if game.gravitycontrol == 0 {
                            if INBOUNDS_VEC!(temp, self.entities) && self.entities[temp].vy < 3.0 {
                                self.entities[temp].vy = 3.0;
                            }
                        } else {
                            if INBOUNDS_VEC!(temp, self.entities) && self.entities[temp].vy > -3.0 {
                                self.entities[temp].vy = -3.0;
                            }
                        }
                    } else if self.entities[i].state == 2 {
                        self.entities[i].life -= 1;
                        if self.entities[i].life <= 0 {
                            self.entities[i].state = 0;
                            self.entities[i].onentity = 1;
                        }
                    } else if self.entities[i].state == 3 {
                        self.entities[i].state = 2;
                        self.entities[i].life = 4;
                        self.entities[i].onentity = 3;
                    } else if self.entities[i].state == 4 {
                        //Special case for room initilisations: As state one, except without the reversal
                        self.entities[i].onentity = 3;
                        self.entities[i].state = 2;
                    }
                },
                11 => {
                    //Warp point
                    //wait for collision
                    if self.entities[i].state == 1 {
                        //Depending on the room the warp point is in, teleport to a new location!
                        self.entities[i].onentity = 0;
                        //play a sound or somefink
                        music.playef(10);
                        game.teleport = true;

                        game.edteleportent = i as i32;
                        //for the multiple room:
                        if self.entities[i].xp == 12*8 { game.teleportxpos = 1; }
                        if self.entities[i].xp == 5*8 { game.teleportxpos = 2; }
                        if self.entities[i].xp == 28*8 { game.teleportxpos = 3; }
                        if self.entities[i].xp == 21*8 { game.teleportxpos = 4; }
                    }
                },
                12 => {
                    //Crew member
                    //Somewhat complex AI: exactly what they do depends on room, location, state etc
                    //At state 0, do nothing at all.
                    if self.entities[i].state == 1 {
                        //happy!
                        if INBOUNDS_VEC!(self.k as usize, self.entities) && self.entities[self.k as usize].rule == 6 {
                            self.entities[self.k as usize].tile = 0;
                        }
                        if INBOUNDS_VEC!(self.k as usize, self.entities) && self.entities[self.k as usize].rule == 7 {
                            self.entities[self.k as usize].tile = 6;
                        }
                        //Stay close to the hero!
                        let j = self.getplayer() as usize;
                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 5 {
                            self.entities[i].dir = 1;
                        } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 5 {
                            self.entities[i].dir = 0;
                        }

                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 45 {
                            self.entities[i].ax = 3.0;
                        } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 45 {
                            self.entities[i].ax = -3.0;
                        }

                        //Special rules:
                        if game.roomx == 110 && game.roomy == 105 && !map.custommode {
                            if self.entities[i].xp < 155 {
                                if self.entities[i].ax < 0.0 {
                                    self.entities[i].ax = 0.0;
                                }
                            }
                        }
                    } else if self.entities[i].state == 2 {
                        //Basic rules, don't change expression
                        let j = self.getplayer() as usize;
                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 5 {
                            self.entities[i].dir = 1;
                        } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 5 {
                            self.entities[i].dir = 0;
                        }

                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 45 {
                            self.entities[i].ax = 3.0;
                        } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 45 {
                            self.entities[i].ax = -3.0;
                        }
                    } else if self.entities[i].state == 10 {
                        //Everything from 10 on is for cutscenes
                        //Basic rules, don't change expression
                        let j = self.getplayer() as usize;
                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 5 {
                            self.entities[i].dir = 1;
                        } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 5 {
                            self.entities[i].dir = 0;
                        }

                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 45 {
                            self.entities[i].ax = 3.0;
                        } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 45 {
                            self.entities[i].ax = -3.0;
                        }
                    } else if self.entities[i].state == 11 {
                        //11-15 means to follow a specific character, in crew order (cyan, purple, yellow, red, green, blue)
                        let j = self.getcrewman(1) as usize; //purple
                        if INBOUNDS_VEC!(j, self.entities) {
                            if self.entities[j].xp > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if self.entities[j].xp < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            if self.entities[j].xp > self.entities[i].xp + 45 {
                                self.entities[i].ax = 3.0;
                            } else if self.entities[j].xp < self.entities[i].xp - 45 {
                                self.entities[i].ax = -3.0;
                            }
                        }
                    } else if self.entities[i].state == 12 {
                        //11-15 means to follow a specific character, in crew order (cyan, purple, yellow, red, green, blue)
                        let j = self.getcrewman(2) as usize; //yellow
                        if INBOUNDS_VEC!(j, self.entities) {
                            if self.entities[j].xp > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if self.entities[j].xp < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            if self.entities[j].xp > self.entities[i].xp + 45 {
                                self.entities[i].ax = 3.0;
                            } else if self.entities[j].xp < self.entities[i].xp - 45 {
                                self.entities[i].ax = -3.0;
                            }
                        }
                    } else if self.entities[i].state == 13 {
                        //11-15 means to follow a specific character, in crew order (cyan, purple, yellow, red, green, blue)
                        let j = self.getcrewman(3) as usize; //red
                        if INBOUNDS_VEC!(j, self.entities) {
                            if self.entities[j].xp > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if self.entities[j].xp < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            if self.entities[j].xp > self.entities[i].xp + 45 {
                                self.entities[i].ax = 3.0;
                            } else if self.entities[j].xp < self.entities[i].xp - 45 {
                                self.entities[i].ax = -3.0;
                            }
                        }
                    } else if self.entities[i].state == 14 {
                        //11-15 means to follow a specific character, in crew order (cyan, purple, yellow, red, green, blue)
                        let j = self.getcrewman(4) as usize; //green
                        if INBOUNDS_VEC!(j, self.entities) {
                            if self.entities[j].xp > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if self.entities[j].xp < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            if self.entities[j].xp > self.entities[i].xp + 45 {
                                self.entities[i].ax = 3.0;
                            } else if self.entities[j].xp < self.entities[i].xp - 45 {
                                self.entities[i].ax = -3.0;
                            }
                        }
                    } else if self.entities[i].state == 15 {
                        //11-15 means to follow a specific character, in crew order (cyan, purple, yellow, red, green, blue)
                        let j = self.getcrewman(5) as usize; //blue
                        if INBOUNDS_VEC!(j, self.entities) {
                            if self.entities[j].xp > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if self.entities[j].xp < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            if self.entities[j].xp > self.entities[i].xp + 45 {
                                self.entities[i].ax = 3.0;
                            } else if self.entities[j].xp < self.entities[i].xp - 45 {
                                self.entities[i].ax = -3.0;
                            }
                        }
                    } else if self.entities[i].state == 16 {
                        //Follow a position: given an x coordinate, seek it out.
                        if self.entities[i].para > self.entities[i].xp as f32 + 5.0 {
                            self.entities[i].dir = 1;
                        } else if self.entities[i].para < self.entities[i].xp as f32 - 5.0 {
                            self.entities[i].dir = 0;
                        }

                        if self.entities[i].para > self.entities[i].xp as f32 + 45.0 {
                            self.entities[i].ax = 3.0;
                        } else if self.entities[i].para < self.entities[i].xp as f32 - 45.0 {
                            self.entities[i].ax = -3.0;
                        }
                    } else if self.entities[i].state == 17 {
                        //stand still
                    } else if self.entities[i].state == 18 {
                        //Stand still and face the player
                        let j = self.getplayer() as usize;
                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 5 {
                            self.entities[i].dir = 1;
                        } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 5 {
                            self.entities[i].dir = 0;
                        }
                    } else if self.entities[i].state == 19 {
                        //Walk right off the screen after time t
                        if self.entities[i].para <= 0.0 {
                            self.entities[i].dir = 1;
                            self.entities[i].ax = 3.0;
                        } else {
                            self.entities[i].para -= 1.0;
                        }
                    } else if self.entities[i].state == 20 {
                        //Panic! For briefing script
                        if self.entities[i].life == 0 {
                            //walk left for a bit
                            self.entities[i].ax = 0.0;
                            if 40 > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if 40 < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            if 40 > self.entities[i].xp + 45 {
                                self.entities[i].ax = 3.0;
                            } else if 40 < self.entities[i].xp - 45 {
                                self.entities[i].ax = -3.0;
                            }
                            if self.entities[i].ax == 0.0 {
                                self.entities[i].life = 1;
                                self.entities[i].para = 30.0;
                            }
                        } else if self.entities[i].life == 1 {
                            //Stand around for a bit
                            self.entities[i].para -= 1.0;
                            if self.entities[i].para <= 0.0 {
                                self.entities[i].life += 1;
                            }
                        } else if self.entities[i].life == 2 {
                            //walk right for a bit
                            self.entities[i].ax = 0.0;
                            if 280 > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if 280 < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            if 280 > self.entities[i].xp + 45 {
                                self.entities[i].ax = 3.0;
                            } else if 280 < self.entities[i].xp - 45 {
                                self.entities[i].ax = -3.0;
                            }
                            if self.entities[i].ax == 0.0 {
                                self.entities[i].life = 3;
                                self.entities[i].para = 30.0;
                            }
                        } else if self.entities[i].life == 3 {
                            //Stand around for a bit
                            self.entities[i].para -= 1.0;
                            if self.entities[i].para <= 0.0 {
                                self.entities[i].life = 0;
                            }
                        }
                    }
                },
                13 => {
                    //Terminals (very similar to savepoints)
                    //wait for collision
                    if self.entities[i].state == 1 {
                        self.entities[i].colour = 5;
                        self.entities[i].onentity = 0;
                        music.playef(17);

                        self.entities[i].state = 0;
                    }
                },
                14 => {
                    //Super Crew member
                    //Actually needs less complex AI than the scripting crewmember
                    if self.entities[i].state == 0 {
                        //follow player, but only if he's on the floor!
                        let j = self.getplayer() as usize;
                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].onground>0 {
                            if self.entities[j].xp > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if self.entities[j].xp > 15 && self.entities[j].xp < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            if self.entities[j].xp > self.entities[i].xp + 45 {
                                self.entities[i].ax = 3.0;
                            } else if self.entities[j].xp < self.entities[i].xp - 45 {
                                self.entities[i].ax = -3.0;
                            }
                            if self.entities[i].ax < 0.0 && self.entities[i].xp < 60 {
                                self.entities[i].ax = 0.0;
                            }
                        } else {
                            if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 5 {
                                self.entities[i].dir = 1;
                            } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 5 {
                                self.entities[i].dir = 0;
                            }

                            self.entities[i].ax = 0.0;
                        }

                        if self.entities[i].xp > 240 {
                            self.entities[i].ax = 3.0;
                            self.entities[i].dir = 1;
                        }
                        if self.entities[i].xp >= 310 {
                            game.scmprogress += 1;
                            return self.disableentity(i);
                        }
                    }
                },
                15 => {
                    //Trophy
                    //wait for collision
                    if self.entities[i].state == 1 {
                        if !script.running {
                            self.trophytext += 2;
                        }
                        if self.trophytext > 30 {
                            self.trophytext = 30;
                        }
                        self.trophytype = self.entities[i].para as i32;

                        self.entities[i].state = 0;
                    }
                },
                23 => {
                    //swn game!
                    match self.entities[i].behave {
                        0 => {
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vx = 7.0;
                                if self.entities[i].xp > 320 {
                                    return self.disableentity(i);
                                }
                            }
                        },
                        1 => {
                            if self.entities[i].state == 0 {
                                //Init
                                self.entities[i].vx = -7.0;
                                if self.entities[i].xp < -20 {
                                    return self.disableentity(i);
                                }
                            }
                        },
                        _ => println!("unknown swn game behaviour ({})", self.entities[i].behave),
                    };
                },

                51 => {
                    //Vertical warp line
                    if self.entities[i].state == 2 {
                        let j = self.getplayer() as usize;
                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp <= 307 {
                            self.customwarpmodevon = false;
                            self.entities[i].state = 0;
                        }
                    } else if self.entities[i].state == 1 {
                        self.entities[i].state = 2;
                        self.entities[i].statedelay = 2;
                        self.entities[i].onentity = 1;
                        self.customwarpmodevon = true;
                    }
                },
                52 => {
                    //Vertical warp line
                    if self.entities[i].state == 2 {
                        let j = self.getplayer() as usize;
                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp <= 307 {
                            self.customwarpmodevon = false;
                            self.entities[i].state = 0;
                        }
                    } else if self.entities[i].state == 1 {
                        self.entities[i].state = 2;
                        self.entities[i].statedelay = 2;
                        self.entities[i].onentity = 1;
                        self.customwarpmodevon = true;
                    }
                },
                53 => {
                    //Warp lines Horizonal
                    if self.entities[i].state == 2 {
                        self.customwarpmodehon = false;
                        self.entities[i].state = 0;
                    } else if self.entities[i].state == 1 {
                        self.entities[i].state = 2;
                        self.entities[i].statedelay = 2;
                        self.entities[i].onentity = 1;
                        self.customwarpmodehon = true;
                    }
                },
                54 => {
                    //Warp lines Horizonal
                    if self.entities[i].state == 2 {
                        self.customwarpmodehon = false;
                        self.entities[i].state = 0;
                    } else if self.entities[i].state == 1 {
                        self.entities[i].state = 2;
                        self.entities[i].statedelay = 2;
                        self.entities[i].onentity = 1;
                        self.customwarpmodehon = true;
                    }
                },
                55 => {
                    //Collectable crewmate
                    //wait for collision
                    if self.entities[i].state == 0 {
                        //Basic rules, don't change expression
                        let j = self.getplayer() as usize;
                        if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp > self.entities[i].xp + 5 {
                            self.entities[i].dir = 1;
                        } else if INBOUNDS_VEC!(j, self.entities) && self.entities[j].xp < self.entities[i].xp - 5 {
                            self.entities[i].dir = 0;
                        }
                    } else if self.entities[i].state == 1 {
                        if INBOUNDS_ARR!(self.entities[i].para as usize, self.customcollect) {
                            self.customcollect[self.entities[i].para as usize] = true;
                        }

                        if game.intimetrial {
                            music.playef(27);
                        } else {
                            game.state = 1010;
                            //music.haltdasmusik();
                            if music.currentsong != -1 {
                                music.silencedasmusik();
                            }
                            music.playef(27);
                        }

                        return self.disableentity(i);
                    }
                },
                100 => {
                    //The teleporter
                    if self.entities[i].state == 1 {
                        //if inactive, activate!
                        if self.entities[i].tile == 1 {
                            music.playef(18);
                            self.entities[i].tile = 2;
                            self.entities[i].colour = 101;
                            if !game.intimetrial && !game.nodeathmode {
                                game.state = 2000;
                                game.statedelay = 0;
                            }

                            game.activetele = true;
                            game.teleblock.x = self.entities[i].xp - 32;
                            game.teleblock.y = self.entities[i].yp - 32;
                            game.teleblock.w = 160;
                            game.teleblock.h = 160;


                            //Alright, let's set this as our savepoint too
                            //First, deactivate all other savepoints
                            // for (size_t j = 0; j < self.entities.size(); j++ {
                            for j in 0..self.entities.len() {
                                if self.entities[j].r#type == 8 {
                                    self.entities[j].colour = 4;
                                    self.entities[j].onentity = 1;
                                }
                            }
                            game.savepoint = self.entities[i].para as i32;
                            game.savex = self.entities[i].xp + 44;
                            game.savey = self.entities[i].yp + 44;
                            game.savegc = 0;

                            game.saverx = game.roomx;
                            game.savery = game.roomy;
                            let player = self.getplayer() as usize;
                            if INBOUNDS_VEC!(player, self.entities) {
                                game.savedir = self.entities[player].dir;
                            }
                        }

                        self.entities[i].onentity = 0;
                        self.entities[i].state = 0;
                    } else if self.entities[i].state == 2 {
                        //Initilise the teleporter without changing the game state or playing sound
                        self.entities[i].onentity = 0;
                        self.entities[i].tile = 6;
                        self.entities[i].colour = 102;

                        game.activetele = true;
                        game.teleblock.x = self.entities[i].xp - 32;
                        game.teleblock.y = self.entities[i].yp - 32;
                        game.teleblock.w = 160;
                        game.teleblock.h = 160;

                        self.entities[i].state = 0;
                    }
                },
                _ => println!("no idea how to update entity type ({})", self.entities[i].r#type),
            }
        } else {
            self.entities[i].statedelay -= 1;
            if self.entities[i].statedelay < 0 {
                self.entities[i].statedelay = 0;
            }
        }

        return false;
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

                        // @sx: this code disabled in original codebase
                        //if self.entities[_i].visualonroof > 0 { self.entities[_i].drawframe += 6; }
                    } else {
                        self.entities[_i].drawframe  += 1;
                        // @sx: this code disabled in original codebase
                        //if game.gravitycontrol == 1 {
                        //    self.entities[_i].drawframe += 6;
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
                        // @sx: this code disabled in original codebase
                        //if game.gravitycontrol == 1 { self.entities[_i].drawframe += 2; }
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
                        // TODO: @sx: buffer overflow
                        // self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                        self.entities[_i].drawframe.checked_add(self.entities[_i].walkingframe as usize).unwrap_or(0);
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
                        // TODO: @sx: buffer overflow
                        // self.entities[_i].drawframe += self.entities[_i].walkingframe as usize;
                        self.entities[_i].drawframe.checked_add(self.entities[_i].walkingframe as usize).unwrap_or(0);
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
        // for (size_t i = 0; i < entities.size(); i++)
        for i in 0..self.entities.len() {
            if self.entities[i].r#type == 100 {
                return i as i32;
            }
        }

        return -1;
    }

    // bool entityclass::entitycollide( int a, int b )
    pub fn entitycollide(&self, a: usize, b: usize, help: &mut utility_class::UtilityClass) -> bool {
        if !INBOUNDS_VEC!(a, self.entities) || !INBOUNDS_VEC!(b, self.entities) {
            println!("entitycollide() out-of-bounds!");
            return false;
        }

        //Do self.entities a and b collide?
        let temprect = sdl2::rect::Rect::new(self.entities[a].xp + self.entities[a].cx, self.entities[a].yp + self.entities[a].cy, self.entities[a].w as u32, self.entities[a].h as u32);
        let temprect2 = sdl2::rect::Rect::new(self.entities[b].xp + self.entities[b].cx, self.entities[b].yp + self.entities[b].cy, self.entities[b].w as u32, self.entities[b].h as u32);

        if help.intersects(temprect, temprect2) {
            return true;
        }
        return false;
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
    fn checkplatform(&self, temprect: sdl2::rect::Rect, help: &mut utility_class::UtilityClass) -> Option<(i32, i32)> {
        //Return true if rectset intersects a moving platform, setups px & py to the platform x & y
        for i in 0..self.blocks.len() {
            if self.blocks[i].r#type == EntityEnum::BLOCK && help.intersects(self.blocks[i].rect, temprect) {
                return Some((self.blocks[i].xp, self.blocks[i].yp))
            }
        }

        None
    }

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
        //Returns first entity of horizontal platform at (px, py), -1000 otherwise.
        for i in 0..self.entities.len() {
            if self.entities[i].rule == 2 && self.entities[i].behave >= 2 && self.entities[i].xp == px && self.entities[i].yp == py {
                return if self.entities[i].behave == 8 {
                    //threadmill!
                    self.entities[i].para
                } else if self.entities[i].behave == 9 {
                    //threadmill!
                    -self.entities[i].para
                } else {
                    self.entities[i].vx
                };
            }
        }

        return -1000.0
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
    pub fn entitycollideplatformroof(&self, t: usize, help: &mut utility_class::UtilityClass) -> f32 {
        if !INBOUNDS_VEC!(t, self.entities) {
            warn!("entitycollideplatformroof() out-of-bounds!");
            return -1000.0;
        }

        let temprect = sdl2::rect::Rect::new(self.entities[t].xp + self.entities[t].cx, self.entities[t].yp + self.entities[t].cy - 1, self.entities[t].w as u32, self.entities[t].h as u32);

        return match self.checkplatform(temprect, help) {
            //px and py now contain an x y coordinate for a platform, find it
            Some((px, py)) => self.hplatformat(px, py),
            _ => -1000.0,
        }
    }

    // float entityclass::entitycollideplatformfloor( int t )
    pub fn entitycollideplatformfloor(&self, t: usize, help: &mut utility_class::UtilityClass) -> f32 {
        if !INBOUNDS_VEC!(t, self.entities) {
            warn!("entitycollideplatformfloor() out-of-bounds!");
            return -1000.0;
        }

        let temprect = sdl2::rect::Rect::new(self.entities[t].xp + self.entities[t].cx, self.entities[t].yp + self.entities[t].cy + 1, self.entities[t].w as u32, self.entities[t].h as u32);

        return match self.checkplatform(temprect, help) {
            //px and py now contain an x y coordinate for a platform, find it
            Some((px, py)) => self.hplatformat(px, py),
            _ => -1000.0,
        }
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
    fn testwallsx(&mut self, t: usize, tx: i32, ty: i32, skipdirblocks: bool, map: &mut map::Map, help: &mut utility_class::UtilityClass) -> bool {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("testwallsx() out-of-bounds!");
            return false
        }

        let temprect = sdl2::rect::Rect::new(tx as i32 + self.entities[t].cx, ty as i32 + self.entities[t].cy, self.entities[t].w as u32, self.entities[t].h as u32);

        let skipblocks = self.entities[t].rule < 2 || self.entities[t].r#type == 14;
        let mut dx = 0f32;
        let dy = 0f32;
        if self.entities[t].rule == 0 {
            dx = self.entities[t].vx;
        }
        let dr = self.entities[t].rule;

        //Ok, now we check walls
        if self.checkwall(temprect, Some(dx), Some(dy), Some(dr as f32), Some(skipblocks), Some(skipdirblocks), map, help) {
            if self.entities[t].vx > 1.0 {
                self.entities[t].vx -= 1.0;
                self.entities[t].newxp = self.entities[t].xp as f32 + self.entities[t].vx;
                return self.testwallsx(t, self.entities[t].newxp as i32, self.entities[t].yp, skipdirblocks, map, help);
            } else if self.entities[t].vx < -1.0 {
                self.entities[t].vx += 1.0;
                self.entities[t].newxp = self.entities[t].xp as f32 + self.entities[t].vx;
                return self.testwallsx(t, self.entities[t].newxp as i32, self.entities[t].yp, skipdirblocks, map, help);
            } else {
                self.entities[t].vx = 0.0;
                return false
            }
        }

        true
    }

    // bool entityclass::testwallsy( int t, float tx, float ty )
    fn testwallsy(&mut self, t: usize, tx: f32, ty: f32, map: &mut map::Map, help: &mut utility_class::UtilityClass) -> bool {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("testwallsy() out-of-bounds!");
            return false;
        }

        let temprect = sdl2::rect::Rect::new(tx as i32 + self.entities[t].cx, ty as i32 + self.entities[t].cy, self.entities[t].w as u32, self.entities[t].h as u32);
        let skipblocks = self.entities[t].rule < 2 || self.entities[t].r#type == 14;

        let dx = 0.0f32;
        let mut dy = 0.0f32;
        if self.entities[t].rule == 0 {
            dy = self.entities[t].vy;
        }
        let dr = self.entities[t].rule;

        //Ok, now we check walls
        if self.checkwall(temprect, Some(dx), Some(dy), Some(dr as f32), Some(skipblocks), Some(false), map, help) {
            if self.entities[t].vy > 1.0 {
                self.entities[t].vy -= 1.0;
                self.entities[t].newyp = self.entities[t].yp as f32 + self.entities[t].vy;
                return self.testwallsy(t, self.entities[t].xp as f32, self.entities[t].newyp, map, help);
            } else if self.entities[t].vy < -1.0 {
                self.entities[t].vy += 1.0;
                self.entities[t].newyp = self.entities[t].yp as f32 + self.entities[t].vy;
                return self.testwallsy(t, self.entities[t].xp as f32, self.entities[t].newyp, map, help);
            } else {
                self.entities[t].vy = 0.0;
                return false;
            }
        }

        true
    }

    // void entityclass::applyfriction( int t, float xrate, float yrate )
    fn applyfriction(&mut self, t: usize, xrate: f32, yrate: f32) {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("applyfriction() out-of-bounds!");
            return;
        }

        if self.entities[t].vx > 0.0 { self.entities[t].vx -= xrate; }
        if self.entities[t].vx < 0.0 { self.entities[t].vx += xrate; }
        if self.entities[t].vy > 0.0 { self.entities[t].vy -= yrate; }
        if self.entities[t].vy < 0.0 { self.entities[t].vy += yrate; }
        if self.entities[t].vy > 10.0 { self.entities[t].vy = 10.0; }
        if self.entities[t].vy < -10.0 { self.entities[t].vy = -10.0; }
        if self.entities[t].vx > 6.0 { self.entities[t].vx = 6.0; }
        if self.entities[t].vx < -6.0 { self.entities[t].vx = -6.0; }

        if self.entities[t].vx.abs() < xrate { self.entities[t].vx = 0.0; }
        if self.entities[t].vy.abs() < yrate { self.entities[t].vy = 0.0; }
    }

    // void entityclass::updateentitylogic( int t )
    pub fn updateentitylogic(&mut self, t: usize, game: &mut game::Game) {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("updateentitylogic() out-of-bounds!");
            return;
        }

        self.entities[t].oldxp = self.entities[t].xp;
        self.entities[t].oldyp = self.entities[t].yp;

        self.entities[t].vx = self.entities[t].vx + self.entities[t].ax;
        self.entities[t].vy = self.entities[t].vy + self.entities[t].ay;
        self.entities[t].ax = 0.0;

        if self.entities[t].gravity {
            if self.entities[t].rule == 0 {
                if game.gravitycontrol == 0 {
                    self.entities[t].ay = 3.0;
                } else {
                    self.entities[t].ay = -3.0;
                }
            } else if self.entities[t].rule == 7 {
                self.entities[t].ay = -3.0;
            } else {
                self.entities[t].ay = 3.0;
            }
            self.applyfriction(t, game.inertia, 0.25);
        }

        self.entities[t].newxp = self.entities[t].xp as f32 + self.entities[t].vx;
        self.entities[t].newyp = self.entities[t].yp as f32 + self.entities[t].vy;
    }

    // void entityclass::entitymapcollision( int t )
    pub fn entitymapcollision(&mut self, t: usize, map: &mut map::Map, help: &mut utility_class::UtilityClass) {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("entitymapcollision() out-of-bounds!");
            return;
        }

        if self.testwallsx(t, self.entities[t].newxp as i32, self.entities[t].yp, false, map, help) {
            self.entities[t].xp = self.entities[t].newxp as i32;
        } else {
            if self.entities[t].onwall > 0 {
                self.entities[t].state = self.entities[t].onwall;
            }
            if self.entities[t].onxwall > 0 {
                self.entities[t].state = self.entities[t].onxwall;
            }
        }
        if self.testwallsy(t, self.entities[t].xp as f32, self.entities[t].newyp, map, help) {
            self.entities[t].yp = self.entities[t].newyp as i32;
        } else {
            if self.entities[t].onwall > 0 {
                self.entities[t].state = self.entities[t].onwall;
            }
            if self.entities[t].onywall > 0 {
                self.entities[t].state = self.entities[t].onywall;
            }
        }
    }

    // void entityclass::movingplatformfix( int t, int j )
    pub fn movingplatformfix(&mut self, t: usize, j: usize, help: &mut utility_class::UtilityClass, map: &mut map::Map) {
        if !INBOUNDS_VEC!(t, self.entities) || !INBOUNDS_VEC!(j, self.entities) {
            warn!("movingplatformfix() out-of-bounds!");
            return;
        }

        //If this intersects the entity, then we move them along it
        if self.entitycollide(t, j, help) {
            //ok, bollox, let's make sure
            self.entities[j].yp = self.entities[j].yp + self.entities[j].vy as i32;
            if self.entitycollide(t, j, help) {
                self.entities[j].yp = self.entities[j].yp - self.entities[j].vy as i32;
                self.entities[j].vy = self.entities[t].vy;
                self.entities[j].newyp = self.entities[j].yp as f32 + self.entities[j].vy;
                if self.testwallsy(j, self.entities[j].xp as f32, self.entities[j].newyp, map, help) {
                    if self.entities[t].vy > 0.0 {
                        self.entities[j].yp = self.entities[t].yp + self.entities[t].h;
                        self.entities[j].vy = 0.0;
                        self.entities[j].onroof = 2;
                        self.entities[j].visualonroof = self.entities[j].onroof;
                    } else {
                        self.entities[j].yp = self.entities[t].yp - self.entities[j].h-self.entities[j].cy;
                        self.entities[j].vy = 0.0;
                        self.entities[j].onground = 2;
                        self.entities[j].visualonground = self.entities[j].onground;
                    }
                } else {
                    self.entities[t].state = self.entities[t].onwall;
                }
            }
        }
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

                self.collisioncheck(i, j, Some(scm), game, graphics, map, music, help);
            }
        }

        //can't have the player being stuck...
        self.stuckprevention(self.getplayer(), game, map, help);

        //Can't have the supercrewmate getting stuck either!
        if game.supercrewmate {
            self.stuckprevention(self.getscm(), game, map, help);
        }

        //Is the player colliding with any damageblocks?
        if self.checkdamage(None, help) && !map.invincibility {
            //usual player dead stuff
            println!("TODO: @sx: oopse, we're dead, uncomment next line");
            // game.deathseq = 30;
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
    fn collisioncheck(&mut self, i: usize, j: usize, scm: Option<bool>, game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, music: &mut music::Music, help: &mut utility_class::UtilityClass) {
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
                if self.entitycollide(i, j, help) && !map.invincibility {
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

                        // TODO: @sx borrow checker interesting point
                        if graphics.Hitest(drawframe1, colpoint1, drawframe2, colpoint2, help) {
                            println!("TODO: @sx: oopse, we're dead, uncomment next lines");
                            // game.deathseq = 30;
                            // game.scmhurt = scm;
                        }
                    } else {
                        //Ok, then we just assume a normal bounding box collision
                        println!("TODO: @sx: oopse, we're dead, uncomment next lines");
                        // game.deathseq = 30;
                        // game.scmhurt = scm;
                    }
                }
            },
            2 => {
                //Moving platforms
                if self.entities[j].behave >= 8 && self.entities[j].behave < 10 {
                    //We don't want conveyors, moving platforms only
                    return
                }
                if self.entitycollide(i, j, help) {
                    //Disable collision temporarily so we don't push the person out!
                    //Collision will be restored at end of platform update loop in gamelogic
                    self.disableblockat(self.entities[j].xp, self.entities[j].yp);
                }
            },
            3 => {
                //Entity to entity
                if self.entities[j].onentity > 0 {
                    if self.entitycollide(i, j, help) {
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
                            if self.entitycollide(i, j, help) {
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
    fn stuckprevention(&mut self, t: i32, game: &mut game::Game, map: &mut map::Map, help: &mut utility_class::UtilityClass) {
        if !INBOUNDS_VEC!(t, self.entities) {
            println!("stuckprevention() out-of-bounds!");
            return;
        }
        let t = t as usize;

        // Can't have this entity (player or supercrewmate) being stuck...
        if !self.testwallsx(t, self.entities[t].xp, self.entities[t].yp, true, map, help) {
            // Let's try to get out...
            if game.gravitycontrol == 0 {
                self.entities[t].yp -= 3;
            } else {
                self.entities[t].yp += 3;
            }
        }
    }

}
