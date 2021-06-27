use crate::screen::render::graphics::{self, towerbg};
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

pub struct Map {
    pub roomdeaths: [i32; 20*20],
    pub roomdeathsfinal: [i32; 20*20],
    contents: [i16; 40*30],
    explored: [bool; 20*20],
    vmult: [i32; 30],

	//Start here!
	colstatedelay: i32,
	colsuperstate: i32,
	spikeleveltop: i32,
	spikelevelbottom: i32,
	oldspikeleveltop: i32,
	oldspikelevelbottom: i32,
	pub warpx: bool,
	pub warpy: bool,
	extrarow: i32,

	pub showteleporters: bool,
	pub showtargets: bool,
	pub showtrinkets: bool,

	pub finalmode: bool,
	pub finalstretch: bool,

	cursorstate: i32,
	cursordelay: i32,

	pub towermode: bool,
	pub cameraseekframe: i32,
	pub resumedelay: i32,

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

	// This needs to be in map instead!
	pub invincibility: bool,

	tileset: i32,

	pub ypos: i32,
	oldypos: i32,

	background: i32,
	cameramode: i32,
	cameraseek: i32,
	minitowermode: bool,
	roomtexton: bool,

    //Levels
    // otherlevel: otherlevelclass,
    // spacestation2: spacestation2class,
    // lablevel: labclass,
    // finallevel: finalclass,
    // warplevel: warpclass,
    pub tower: towerclass::TowerClass,
    // extrarow: int,
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

            cursorstate: 0,
            cursordelay: 0,

            towermode: false,
            cameraseekframe: 0,
            resumedelay: 0,

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

            //This needs to be in map instead!
            invincibility: false,

            tileset: 0,

            ypos: 0,
            oldypos: 0,

            background: 0,
            cameramode: 0,
            cameraseek: 0,
            minitowermode: false,
            roomtexton: false,

            //Levels
            // otherlevel: otherlevelclass,
            // spacestation2: spacestation2class,
            // lablevel: labclass,
            // finallevel: finalclass,
            // warplevel: warpclass,
            tower: towerclass::TowerClass::new(),
            // extrarow: int,
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
    // void mapclass::settrinket(int x, int y)

    // void mapclass::resetmap(void)
    pub fn resetmap(&mut self) {
        self.explored = [false; 20*20];
    }

    // void mapclass::resetnames(void)
    pub fn resetnames (&mut self) {

    }

    // void mapclass::transformname(int t)
    // std::string mapclass::getglitchname(int x, int y)

    // void mapclass::initmapdata(void)
    pub fn initmapdata (&mut self) {

    }

    // void mapclass::initcustommapdata(void)
    // int mapclass::finalat(int x, int y)
    // int mapclass::maptiletoenemycol(int t)
    // void mapclass::changefinalcol(int t)

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
    // bool mapclass::collide(int x, int y)
    // void mapclass::settile(int xp, int yp, int t)
    // int mapclass::area(int _rx, int _ry)
    pub fn area(&self, _rx: i32, _ry: i32) -> i32 {
        println!("DEADBEEF: mapclass::area method not implemented yet");
        0
    }

    // bool mapclass::isexplored(const int rx, const int ry)
    // void mapclass::setexplored(const int rx, const int ry, const bool status)
    // void mapclass::exploretower(void)
    // void mapclass::hideship(void)
    // void mapclass::showship(void)

    // void mapclass::resetplayer(void)
    // void mapclass::resetplayer(const bool player_died)
    pub fn resetplayer(&mut self, player_died: Option<bool>) {
        println!("DEADBEEF: mapclass::resetplayer method not implemented yet");
    }

    // void mapclass::warpto(int rx, int ry , int t, int tx, int ty)
    // void mapclass::gotoroom(int rx, int ry)
    pub fn gotoroom(&mut self, rx: i32, ry: i32) {
        println!("DEADBEEF: mapclass::gotoroom method not implemented yet");
    }

    // std::string mapclass::currentarea(int t)
    // void mapclass::loadlevel(int rx, int ry)
    // void mapclass::twoframedelayfix(void)
}
