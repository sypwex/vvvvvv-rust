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
    timer: i32,

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
    fn centerx(&mut self) {
        println!("DEADBEEF: TextBox::centerx() method not implemented yet");
    }

    // void centery(void);
    fn centery(&mut self) {
        println!("DEADBEEF: TextBox::centery() method not implemented yet");
    }

    // void adjust(void);
    fn adjust(&mut self) {
        println!("DEADBEEF: TextBox::adjust() method not implemented yet");
    }

    // void initcol(int rr, int gg, int bb);
    fn initcol(&mut self, rr: i32, gg: i32, bb: i32) {
        println!("DEADBEEF: TextBox::initcol() method not implemented yet");
    }

    // void setcol(int rr, int gg, int bb);
    pub fn setcol(&mut self, rr: i32, gg: i32, bb: i32) {
        println!("DEADBEEF: TextBox::setcol() method not implemented yet");
    }

    pub fn setcol_tl_lerp(&mut self, lerp: f32) {
        let rr = self.tr * lerp as i32;
        let gg = self.tg * lerp as i32;
        let bb = self.tb * lerp as i32;

        println!("DEADBEEF: TextBox::setcol() method not implemented yet");
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
    fn remove(&mut self) {
        println!("DEADBEEF: TextBox::remove() method not implemented yet");
    }

    // void removefast(void);
    fn removefast(&mut self) {
        println!("DEADBEEF: TextBox::removefast() method not implemented yet");
    }

    // void resize(void);
    fn resize(&mut self) {
        println!("DEADBEEF: TextBox::resize() method not implemented yet");
    }

    // void addline(std::string t);
    fn addline(&mut self, t: &str) {
        println!("DEADBEEF: TextBox::addline() method not implemented yet");
    }

}
