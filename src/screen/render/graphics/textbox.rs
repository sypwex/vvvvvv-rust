pub struct TextBoxClass {
    //Fundamentals
    line: Vec<String>,
    xp: i32,
    yp: i32,
    lw: i32,
    w: i32,
    h: i32,
    r: i32,
    g: i32,
    b: i32,
    tr: i32,
    tg: i32,
    tb: i32,
    timer: i32,

    tl: f32,
    prev_tl: f32,
    tm: i32,

    max: i32,

    /* Whether to flip text box y-position in Flip Mode. */
    flipme: bool,
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
    // void centery(void);
    // void adjust(void);
    // void initcol(int rr, int gg, int bb);
    // void setcol(int rr, int gg, int bb);
    // void update(void);
    // void remove(void);
    // void removefast(void);
    // void resize(void);
    // void addline(std::string t);
}
