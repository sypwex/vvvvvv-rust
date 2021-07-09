use crate::{game, screen::render::graphics, utility_class};

pub struct EntClass {
    //Fundamentals
    pub invis: bool,
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

    //Position and velocity
    pub oldxp: i32,
    pub oldyp: i32,
    pub ax: f32,
    ay: f32,
    pub vx: f32,
    pub vy: f32,
    pub cx: i32,
    pub cy: i32,
    pub w: i32,
    pub h: i32,
    pub newxp: f32,
    newyp: f32,
    pub isplatform: bool,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,

    //Collision Rules
    pub onentity: i32,
    pub harmful: bool,
    onwall: i32,
    onxwall: i32,
    onywall: i32,

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
    fn outside(&mut self) -> bool {
        false
    }

    // void entclass::setenemy( int t )
    pub fn setenemy(&mut self, t: i32) {
        println!("DEADBEEF: entclass::setenemy() method not implemented yet");
    }

    // void entclass::setenemyroom( int rx, int ry )
    pub fn setenemyroom(&mut self, rx: i32, ry: i32)  {
        println!("DEADBEEF: entclass::setenemyroom() method not implemented yet");
    }

    // void entclass::settreadmillcolour( int rx, int ry )
    pub fn settreadmillcolour(&mut self, rx: i32, ry: i32) {
        println!("DEADBEEF: entclass::settreadmillcolour() method not implemented yet");
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
