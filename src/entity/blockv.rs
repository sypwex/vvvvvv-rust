pub struct BlockClass {
    //Fundamentals
    pub rect: sdl2::rect::Rect,
    pub r#type: super::EntityEnum,
    pub trigger: i32,
    pub xp: i32,
    pub yp: i32,
    pub wp: i32,
    pub hp: i32,
    pub script: String,
    pub prompt: String,
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl BlockClass {
    // blockclass::blockclass(void)
    pub fn new() -> Self {
        Self {
            r#type: super::EntityEnum::BLOCK,
            trigger: 0,

            xp: 0,
            yp: 0,
            wp: 0,
            hp: 0,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),

            r: 0,
            g: 0,
            b: 0,

            /* std::strings get initialized automatically, but this is */
            /* in case this function gets called again after construction */
            script: String::new(),
            prompt: String::new(),
        }
    }

    // void blockclass::clear(void)
    pub fn clear(&mut self) {
        self.r#type = super::EntityEnum::BLOCK;
        self.trigger = 0;
        self.xp = 0;
        self.yp = 0;
        self.wp = 0;
        self.hp = 0;
        self.rect = sdl2::rect::Rect::new(0, 0, 0, 0);
        self.r = 0;
        self.g = 0;
        self.b = 0;
        self.script = String::new();
        self.prompt = String::new();
    }

    // void blockclass::rectset(const int xi, const int yi, const int wi, const int hi)
    pub fn rectset(&mut self, xi: i32, yi: i32, wi: i32, hi: i32) {
        self.rect.x = xi;
        self.rect.y = yi;
        self.rect.w = wi;
        self.rect.h = hi;
    }

    // void blockclass::setblockcolour( std::string col )
    pub fn setblockcolour(&mut self, col: &str) {
        match col {
            "cyan" => {
                self.r = 164;
                self.g = 164;
                self.b = 255;
            }
            "red" => {
                self.r = 255;
                self.g = 60;
                self.b = 60;
            }
            "green" => {
                self.r = 144;
                self.g = 255;
                self.b = 144;
            }
            "yellow" => {
                self.r = 255;
                self.g = 255;
                self.b = 134;
            }
            "blue" => {
                self.r = 95;
                self.g = 95;
                self.b = 255;
            }
            "purple" => {
                self.r = 255;
                self.g = 134;
                self.b = 255;
            }
            "white" => {
                self.r = 244;
                self.g = 244;
                self.b = 244;
            }
            "gray" => {
                self.r = 174;
                self.g = 174;
                self.b = 174;
            }
            "orange" => {
                self.r = 255;
                self.g = 130;
                self.b = 20;
            },
            _ => {
                //use a gray
                self.r = 174;
                self.g = 174;
                self.b = 174;
            }
        };
    }
}
