use crate::{game, screen::render::graphics, utility_class};

// #define rn( rx,  ry) ((rx) + ((ry) * 100))
#[macro_export]
macro_rules! rn {
    ($rx:expr, $ry:expr) => {
        $rx + ($ry * 100)
    };
}

#[derive(Copy,Clone)]
pub struct EntClass {
    //Fundamentals
    pub r#type: i32,
    pub size: i32,
    pub tile: i32,
    pub rule: i32,
    pub state: i32,
    pub statedelay: i32,
    pub behave: i32,
    pub animate: i32,
    pub para: f32,
    pub life: i32,
    pub colour: i32,
    pub invis: bool,

    //Position and velocity
    pub oldxp: i32,
    pub oldyp: i32,
    pub ax: f32,
    pub ay: f32,
    pub vx: f32,
    pub vy: f32,
    pub cx: i32,
    pub cy: i32,
    pub w: i32,
    pub h: i32,
    pub newxp: f32,
    pub newyp: f32,
    pub isplatform: bool,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,

    //Collision Rules
    pub onentity: i32,
    pub harmful: bool,
    pub onwall: i32,
    pub onxwall: i32,
    pub onywall: i32,

    //Platforming specific
    pub gravity: bool,
    pub onground: i32,
    pub onroof: i32,

    //Animation
    pub framedelay: i32,
    pub drawframe: usize,
    pub walkingframe: i32,
    pub dir: i32,
    pub actionframe: i32,
    pub visualonground: i32,
    pub visualonroof: i32,
    pub yp: i32,
    pub xp: i32,

    pub realcol: u32,
    pub lerpoldxp: i32,
    pub lerpoldyp: i32,
}

impl std::fmt::Debug for EntClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("EntClass")
            // Fundamentals
            .field("type", match &self.r#type {
                // see entityclass::updateentities() for brief description
                0 => &"Player",
                1 => &"A moving platform",
                2 => &"Disappearing platforms",
                3 => &"Breakable blocks",
                4 => &"Gravity Tokens",
                5 => &"Decorative particles",
                6 => &"Small collectibles/pickups",
                7 => &"Something Shiny (true or fake trinket ar smthn else?)",
                8 => &"Savepoint",
                9 => &"Horizontal Gravity Line",
                10 => &"Vertical Gravity Line",
                11 => &"Warp token",
                12 => &"A special case! (e.g. Crew member)",
                13 => &"Terminal",
                14 => &"A special case! (e.g. super crew member)",
                15 => &"Trophies",
                23 => &"SWN Enemies",
                51..=54 => &"Warp lines",
                55 => &"crew member (custom, collectable)",
                100 => &"Teleporter",
                _ => &self.r#type,

                // input t argument from entityclass::createentity()
                // 0 => &"Player",
                // 1 => &"Simple enemy, bouncing off the walls",
                // 2 => &"A moving platform",
                // 3 => &"Disappearing platforms",
                // 4 => &"Breakable blocks",
                // 5 => &"Gravity Tokens",
                // 6 | 7 => &"Decorative particles",
                // 8 => &"Small collectibles",
                // 9 => &"Something Shiny",
                // 10 => &"Savepoint",
                // 11 => &"Horizontal Gravity Line",
                // 12 => &"Vertical Gravity Line",
                // 13 => &"Warp token",
                // 14 => &"Teleporter",
                // 15 => &"Crew Member (warp zone)",
                // 16 => &"Crew Member, upside down (space station)",
                // 17 => &"Crew Member (Lab)",
                // 18 => &"Crew Member (Ship)",
                // 19 => &"Crew Member (Ship) More tests!",
                // 20 => &"Terminal",
                // 21 => &"as above, except doesn't highlight",
                // 22 => &"Fake trinkets, only appear if you've collected them",
                // 23 => &"SWN Enemies",
                // 24 => &"Super Crew Member",
                // 25 => &"Trophies",
                // 26 => &"Epilogue super warp token",
                // 51 => &"Vertical",
                // 52 => &"Vertical",
                // 53 => &"Horizontal",
                // 54 => &"Horizontal",
                // 55 => &"Crew Member (custom, collectable)",
                // 56 => &"Custom enemy",
            })
            .field("size", match &self.size {
                0 => &"sprite",
                1 => &"tile",
                2 => &"moving platform of width 4 (32)",
                3 => &"apparently a \"bug chunky pixel\"",
                4 => &"coin/small pickup",
                5 => &"horizontal line",
                6 => &"is vertical",
                7 => &"teleporter",
                2 => &"moving platform of width 8",
                9 => &"really big sprite 2x2",
                10 => &"2x1 sprite",
                11 => &"the fucking elephant",
                12 => &"Regular sprites that don't wrap",
                13 => &"epilogue huge hero",
                _ => &self.size,
            })
            .field("tile", &self.tile)
            .field("rule", &self.rule)
            .field("state", &self.state)
            .field("statedelay", &self.statedelay)
            .field("behave", &self.behave)
            .field("animate", &self.animate)
            .field("para", &self.para)
            .field("life", &self.life)
            .field("invis", &self.invis)

            .field("xp", &self.xp)
            .field("yp", &self.yp)
            .field("oldxp", &self.oldxp)
            .field("oldyp", &self.oldyp)
            .field("ax", &self.ax)
            .field("ay", &self.ay)
            .field("vx", &self.vx)
            .field("vy", &self.vy)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("w", &self.w)
            .field("h", &self.h)
            .field("newxp", &self.newxp)
            .field("newyp", &self.newyp)
            .field("isplatform", &self.isplatform)
            .field("x1", &self.x1)
            .field("y1", &self.y1)
            .field("x2", &self.x2)
            .field("y2", &self.y2)
            .field("lerpoldxp", &self.lerpoldxp)
            .field("lerpoldyp", &self.lerpoldyp)

            .field("onentity", &self.onentity)
            .field("harmful", &self.harmful)
            .field("onwall", &self.onwall)
            .field("onxwall", &self.onxwall)
            .field("onywall", &self.onywall)

            .field("gravity", &self.gravity)
            .field("onground", &self.onground)
            .field("onroof", &self.onroof)

            .field("framedelay", &self.framedelay)
            .field("drawframe", &self.drawframe)
            .field("walkingframe", &self.walkingframe)
            .field("dir", &self.dir)
            .field("actionframe", &self.actionframe)
            .field("visualonground", &self.visualonground)
            .field("visualonroof", &self.visualonroof)
            .field("colour", &self.colour)
            .field("realcol", &self.realcol)
            .finish()
    }
}

impl EntClass {
    // entclass::entclass(void)
    pub fn new() -> Self {
        Self {
            //Fundamentals
            invis: false,
            r#type: 0,
            size: 0,
            tile: 0,
            rule: 0,
            state: 0,
            statedelay: 0,
            behave: 0,
            animate: 0,
            para: 0f32,
            life: 0,
            colour: 0,

            //Position and velocity
            oldxp: 0,
            oldyp: 0,
            ax: 0f32,
            ay: 0f32,
            vx: 0f32,
            vy: 0f32,
            cx: 0,
            cy: 0,
            w: 16,
            h: 16,
            newxp: 0f32,
            newyp: 0f32,
            isplatform: false,
            x1: 0,
            y1: 0,
            x2: 320,
            y2: 240,

            //Collision Rules
            onentity: 0,
            harmful: false,
            onwall: 0,
            onxwall: 0,
            onywall: 0,

            //Platforming specific
            gravity: false,
            onground: 0,
            onroof: 0,

            //Animation
            framedelay: 0,
            drawframe: 0,
            walkingframe: 0,
            dir: 0,
            actionframe: 0,
            visualonground: 0,
            visualonroof: 0,
            yp: 0,
            xp: 0,

            realcol: 0,
            lerpoldxp: 0,
            lerpoldyp: 0,
        }
    }

    // void entclass::clear(void)
    pub fn clear(&mut self) {
        self.invis = false;
        self.r#type = 0;
        self.size = 0;
        self.tile = 0;
        self.rule = 0;
        self.state = 0;
        self.statedelay = 0;
        self.life = 0;
        self.colour = 0;
        self.para = 0.0;
        self.behave = 0;
        self.animate = 0;

        self.xp = 0;
        self.yp = 0;
        self.ax = 0.0;
        self.ay = 0.0;
        self.vx = 0.0;
        self.vy = 0.0;
        self.w = 16;
        self.h = 16;
        self.cx = 0;
        self.cy = 0;
        self.newxp = 0.0;
        self.newyp = 0.0;
        self.oldxp = 0;
        self.oldyp = 0;

        self.x1 = 0;
        self.y1 = 0;
        self.x2 = 320;
        self.y2 = 240;

        self.gravity = false;
        self.onground = 0;
        self.onroof = 0;
        self.visualonground = 0;
        self.visualonroof = 0;

        self.onentity = 0;
        self.harmful = false;
        self.onwall = 0;
        self.onxwall = 0;
        self.onywall = 0;
        self.isplatform = false;

        self.framedelay = 0;
        self.drawframe = 0;
        self.walkingframe = 0;
        self.dir = 0;
        self.actionframe = 0;

        self.realcol = 0;
        self.lerpoldxp = 0;
        self.lerpoldyp = 0;
    }

    // bool entclass::outside(void)
    pub fn outside(&mut self) -> bool {
        // Returns true if any point of the entity is outside the map.
        // Adjusts velocity for a clean collision.
        if self.xp < self.x1 {
            self.xp = self.x1;
            return true;
        }
        if self.yp < self.y1 {
            self.yp = self.y1;
            return true;
        }
        if self.xp + self.w > self.x2 {
            self.xp = self.x2 - self.w;
            return true;
        }
        if self.yp + self.h > self.y2 {
            self.yp = self.y2 - self.h;
            return true;
        }

        return false;
    }

    // void entclass::setenemy( int t )
    pub fn setenemy(&mut self, t: i32) {
        println!("DEADBEEF: entclass::setenemy() method not implemented yet");
    }

    // void entclass::setenemyroom( int rx, int ry )
    pub fn setenemyroom(&mut self, rx: i32, ry: i32) {
        let rx = rx - 100;
        let ry = ry - 100;

        //Simple function to initilise simple enemies
        match rn!(rx, ry) {
            //Space Station 1
            t if t == rn!(12, 3) => {
                //Security Drone
                self.tile = 36;
                self.colour = 8;
                self.animate = 1;
            },
            t if t == rn!(13, 3) => {
                //Wavelengths
                self.tile = 32;
                self.colour = 7;
                self.animate = 1;
                self.w = 32;
            },
            t if t == rn!(15, 3) => {
                //Traffic
                self.tile = 28;
                self.colour = 6;
                self.animate = 1;
                self.w = 22;
                self.h = 32;
            },
            t if t == rn!(12, 5) => {
                //The Yes Men
                self.tile = 40;
                self.colour = 9;
                self.animate = 1;
                self.w = 20;
                self.h = 20;
            },
            t if t == rn!(13, 6) => {
                //Hunchbacked Guards
                self.tile = 44;
                self.colour = 8;
                self.animate = 1;
                self.w = 16;
                self.h = 20;
            },
            t if t == rn!(13, 4) => {
                //Communication Station
                self.harmful = false;
                if self.xp == 256 {
                    //transmittor
                    self.tile = 104;
                    self.colour = 4;
                    self.animate = 7;
                    self.w = 16;
                    self.h = 16;
                    self.xp -= 24;
                    self.lerpoldxp -= 24;
                    self.yp -= 16;
                    self.lerpoldyp -= 16;
                } else {
                    //radar dish
                    self.tile = 124;
                    self.colour = 4;
                    self.animate = 6;
                    self.w = 32;
                    self.h = 32;
                    self.cx = 4;
                    self.size = 9;
                    self.xp -= 4;
                    self.lerpoldxp -= 4;
                    self.yp -= 32;
                    self.lerpoldyp -= 32;
                }
            },
            t if t == rn!(4, 0) => {
                //The Lab
                self.tile = 78;
                self.colour = 7;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(2, 0) => {
                self.tile = 88;
                self.colour = 11;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            //Space Station 2
            t if t == rn!(14, 11) => {
                //Lies
                self.colour = 17;
            },
            t if t == rn!(16, 11) => {
                //Lies
                self.colour = 8;
            },
            t if t == rn!(13, 10) => {
                //Factory
                self.colour = 11;
            },
            t if t == rn!(13, 9) => {
                //Factory
                self.colour = 9;
            },
            t if t == rn!(13, 8) => {
                //Factory
                self.colour = 8;
            },
            t if t == rn!(11, 13) => {
                //Truth
                self.tile = 64;
                self.colour = 7;
                self.animate = 100;
                self.w = 44;
                self.h = 10;
                self.size = 10;
            },
            t if t == rn!(17, 7) => {
                //Brass sent us under the top
                self.tile = 82;
                self.colour = 8;
                self.animate = 5;
                self.w = 28;
                self.h = 32;
                self.cx = 4;
            },
            t if t == rn!(10, 7) => {
                // (deception)
                self.tile = 92;
                self.colour = 6;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(14, 13) => {
                // (chose poorly)
                self.tile = 56;
                self.colour = 6;
                self.animate = 1;
                self.w = 15;
                self.h = 24;
            },
            t if t == rn!(13, 12) => {
                // (backsliders)
                self.tile = 164;
                self.colour = 7;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(14, 8) => {
                // (wheel of fortune room)
                self.tile = 116;
                self.colour = 12;
                self.animate = 1;
                self.w = 32;
                self.h = 32;
            },
            t if t == rn!(16, 9) => {
                // (seeing dollar signs)
                self.tile = 68;
                self.colour = 7;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(16, 7) => {
                // (tomb of mad carew)
                self.tile = 106;
                self.colour = 7;
                self.animate = 2;
                self.w = 24;
                self.h = 25;
            },
            //Warp Zone
            t if t == rn!(15, 2) => {
                // (numbers)
                self.tile = 100;
                self.colour = 6;
                self.animate = 1;
                self.w = 32;
                self.h = 14;
                self.yp += 1;
                self.lerpoldyp += 1;
            },
            t if t == rn!(16, 2) => {
                // (Manequins)
                self.tile = 52;
                self.colour = 7;
                self.animate = 5;
                self.w = 16;
                self.h = 25;
                self.yp -= 4;
                self.lerpoldyp -= 4;
            },
            t if t == rn!(18, 0) => {
                // (Obey)
                self.tile = 51;
                self.colour = 11;
                self.animate = 100;
                self.w = 30;
                self.h = 14;
            },
            t if t == rn!(19, 1) => {
                // Ascending and Descending
                self.tile = 48;
                self.colour = 9;
                self.animate = 5;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(19, 2) => {
                // Shockwave Rider
                self.tile = 176;
                self.colour = 6;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(18, 3) => {
                // Mind the gap
                self.tile = 168;
                self.colour = 7;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(17, 3) => {
                // Edge Games
                if self.yp == 96 {
                    self.tile = 160;
                    self.colour = 8;
                    self.animate = 1;
                    self.w = 16;
                    self.h = 16;
                } else {
                    self.tile = 156;
                    self.colour = 8;
                    self.animate = 1;
                    self.w = 16;
                    self.h = 16;
                }
            },
            t if t == rn!(16, 0) => {
                // I love you
                self.tile = 112;
                self.colour = 8;
                self.animate = 5;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(14, 2) => {
                // That's why I have to kill you
                self.tile = 114;
                self.colour = 6;
                self.animate = 5;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(18, 2) => {
                // Thinking with Portals
                //depends on direction
                if self.xp == 88 {
                    self.tile = 54+12;
                    self.colour = 12;
                    self.animate = 100;
                    self.w = 60;
                    self.h = 16;
                    self.size = 10;
                } else {
                    self.tile = 54;
                    self.colour = 12;
                    self.animate = 100;
                    self.w = 60;
                    self.h = 16;
                    self.size = 10;
                }
            },
            //Final level
            t if t == rn!(50-100, 53-100) => {
                //The Yes Men
                self.tile = 40;
                self.colour = 9;
                self.animate = 1;
                self.w = 20;
                self.h = 20;
            },
            t if t == rn!(48-100, 51-100) => {
                //Wavelengths
                self.tile = 32;
                self.colour = 7;
                self.animate = 1;
                self.w = 32;
            },
            t if t == rn!(43-100, 52-100) => {
                // Ascending and Descending
                self.tile = 48;
                self.colour = 9;
                self.animate = 5;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(46-100, 51-100) => {
                //kids his age
                self.tile = 88;
                self.colour = 11;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(43-100, 51-100) => {
                // Mind the gap
                self.tile = 168;
                self.colour = 7;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(44-100, 51-100) => {
                // vertigo?
                self.tile = 172;
                self.colour = 7;
                self.animate = 100;
                self.w = 32;
                self.h = 32;
            },
            t if t == rn!(44-100, 52-100) => {
                // (backsliders)
                self.tile = 164;
                self.colour = 7;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(43-100, 56-100) => {
                //Intermission 1
                self.tile = 88;
                self.colour = 21;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            t if t == rn!(45-100, 56-100) => {
                //Intermission 1
                self.tile = 88;
                self.colour = 21;
                self.animate = 1;
                self.w = 16;
                self.h = 16;
            },
            //The elephant
            t if (t == rn!(11, 9)) | (t == rn!(12, 9)) | (t == rn!(11, 8)) | (t == rn!(12, 8)) => {
                self.tile = 0;
                self.colour = 102;
                self.animate = 0;
                self.w = 464;
                self.h = 320;
                self.size = 11;
                self.harmful = false;
            },
            _ => println!("unknown room rx=({}),ry=({}) === rn!({})", rx, ry, rn!(rx, ry)),
        };
    }

    // void entclass::settreadmillcolour( int rx, int ry )
    pub fn settreadmillcolour(&mut self, rx: i32, ry: i32) {
        let rx = ry - 100;
        let ry = rx - 100;
        let rx = ry + 50 - 12;
        let ry = rx + 50 - 14;   //Space Station

        self.tile = match rn!(rx, ry) {
            t if t == rn!(52, 48) => 791, //Cyan

            t if t == rn!(49, 47) => 24, //Yellow
            t if t == rn!(56, 44) => 24, //Yellow
            t if t == rn!(54, 49) => 24, //Yellow

            t if t == rn!(49, 49) => 36, //Green
            t if t == rn!(55, 44) => 36, //Green
            t if t == rn!(54, 43) => 36, //Green
            t if t == rn!(53, 49) => 36, //Green
            t if t == rn!(54, 45) => 711, //Green (special)
            t if t == rn!(51, 48) => 711, //Green (special)

            t if t == rn!(50, 49) => 28, //Purple
            t if t == rn!(54, 44) => 28, //Purple
            t if t == rn!(49, 42) => 28, //Purple
            t if t == rn!(55, 43) => 28, //Purple
            t if t == rn!(54, 47) => 28, //Purple
            t if t == rn!(53, 48) => 28, //Purple

            t if t == rn!(51, 47) => 32, //Red
            t if t == rn!(52, 49) => 32, //Red
            t if t == rn!(48, 43) => 32, //Red
            t if t == rn!(55, 47) => 32, //Red
            t if t == rn!(54, 48) => 32, //Red

            _ => 20, //default as blue
        };
    }

    // void entclass::updatecolour(void)
    pub fn updatecolour(&mut self, game: &mut game::Game, graphics: &mut graphics::Graphics, help: &mut utility_class::UtilityClass) {
        match self.size {
            0 | 7 | 9 | 10 | 13 => {
                // Sprites | Teleporter | Really Big Sprite! (2x2) | 2x1 Sprite | Special for epilogue: huge hero!
                graphics.setcol(self.colour, help.glow);
                self.realcol = graphics.ct.colour;
            },
            3 => {
                // Big chunky pixels!
                self.realcol = graphics.bigchunkygetcol(self.colour);
            },
            4 => {
                // Small pickups
                graphics.huetilesetcol(self.colour);
                self.realcol = graphics.ct.colour;
            },
            11 => {
                // The fucking elephant
                if game.noflashingmode {
                    graphics.setcol(22, help.glow);
                } else {
                    graphics.setcol(self.colour, help.glow);
                }
                self.realcol = graphics.ct.colour;
            },
            12 => {
                // Regular sprites that don't wrap
                // if we're outside the screen, we need to draw indicators
                if (self.xp < -20 && self.vx > 0.0) || (self.xp > 340 && self.vx < 0.0) {
                    graphics.setcol(23, help.glow);
                } else {
                    graphics.setcol(self.colour, help.glow);
                }

                self.realcol = graphics.ct.colour;
            },
            _ => (),
        };
    }

    // bool entclass::ishumanoid(void)
    pub fn ishumanoid(&mut self) -> bool {
        false
    }

}
