#[derive(Debug)]
pub struct TextBoxClass {
    //Fundamentals
    pub line: Vec<String>,
    pub xp: i32,
    pub yp: i32,
    pub lw: i32,
    pub w: i32,
    pub h: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub tr: i32,
    pub tg: i32,
    pub tb: i32,
    pub timer: i32,

    pub tl: f32,
    pub prev_tl: f32,
    pub tm: i32,

    max: i32,

    /* Whether to flip text box y-position in Flip Mode. */
    pub flipme: bool,
}

impl TextBoxClass {
    pub fn new() -> Self {
        Self {
            //Fundamentals
            line: Vec::new(),
            xp: 0,
            yp: 0,
            lw: 0,
            w: 0,
            h: 0,
            r: 0,
            g: 0,
            b: 0,
            tr: 0,
            tg: 0,
            tb: 0,
            timer: 0,

            tl: 0f32,
            prev_tl: 0f32,
            tm: 0,

            max: 0,

            /* Whether to flip text box y-position in Flip Mode. */
            flipme: false,
        }
    }

    // void centerx(void);
    pub fn centerx(&mut self) {
        self.resize();
        self.xp = 160 - (self.w / 2);
        self.resize();
    }

    // void centery(void);
    pub fn centery(&mut self) {
        self.resize();
        self.yp = 120 - (self.h / 2);
        self.resize();
    }

    // void adjust(void);
    pub fn adjust(&mut self) {
        self.resize();
        if self.xp < 10 { self.xp = 10; }
        if self.yp < 10 { self.yp = 10; }
        if self.xp + self.w > 310 { self.xp = 310 - self.w; }
        if self.yp + self.h > 230 { self.yp = 230 - self.h; }
        self.resize();
    }

    // void initcol(int rr, int gg, int bb);
    pub fn initcol(&mut self, rr: i32, gg: i32, bb: i32) {
        self.tr = rr;
        self.tg = gg;
        self.tb = bb;
        self.r = 0;
        self.g = 0;
        self.b = 0;
        self.tl = 0.5;
    }

    // void setcol(int rr, int gg, int bb);
    pub fn setcol(&mut self, rr: i32, gg: i32, bb: i32) {
        self.r = rr;
        self.g = gg;
        self.b = bb;
    }
    pub fn setcol_tl_lerp(&mut self, lerp: f32) {
        self.r = self.tr * lerp as i32;
        self.g = self.tg * lerp as i32;
        self.b = self.tb * lerp as i32;
    }

    // void update(void);
    pub fn update(&mut self) {
        self.prev_tl = self.tl;

        if self.tm == 0 {
            self.tl += 0.1;
            if self.tl >= 1.0 {
                self.tl = 1.0;
                self.tm = 1;
            }
        } else if self.tm == 2 {
            self.tl -= 0.1;
            if self.tl <= 0.5 {
                self.tl = 0.5;
                //this textbox will be removed by updatetextboxes() later
            }
        }

        if self.timer > 0 {
            self.timer -= 1;
            if self.timer == 0 {
                self.tm = 2;
            }
        }
    }

    // void remove(void);
    pub fn remove(&mut self) {
        self.tm = 2;
        self.tl = 1.0; //Remove mode
    }

    // void removefast(void);
    pub fn removefast(&mut self) {
        self.tm = 2;
        self.tl = 0.4; //Remove mode
    }

    // void resize(void);
    pub fn resize(&mut self) {
        //Set the width and height to the correct sizes
        self.max = 0;
        // for (size_t iter = 0; iter < line.size(); iter++) {
        for iter in 0..self.line.len() {
            // let len = utf8::unchecked::distance(self.line[iter].begin(), self.line[iter].end());
            let len = self.line[iter].len() as i32;

            if len > self.max {
                self.max = len;
            }
        }

        self.lw = self.max;
        self.w = (self.max + 2) * 8;
        self.h = (self.line.len() as i32 + 2) * 8;
    }

    // void addline(std::string t);
    pub fn addline(&mut self, t: &str) {
        self.line.push(t.to_string());
        self.resize();
        if self.line.len() >= 12 {
            self.line.clear();
        }
    }

}
