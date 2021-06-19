pub struct TowerBG {
    // pub buffer: &sdl2::surface::SurfaceRef,
    // pub buffer_lerp: &sdl2::surface::SurfaceRef,
    pub tdrawback: bool,
    pub bypos: i32,
    pub bscroll: i32,
    pub colstate: i32,
    pub scrolldir: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl TowerBG {
    pub fn new (colstate: i32) -> Self {
        TowerBG {
            // pub buffer: &sdl2::surface::SurfaceRef,
            // pub buffer_lerp: &sdl2::surface::SurfaceRef,
            tdrawback: false,
            bypos: 0,
            bscroll: 0,
            colstate,
            scrolldir: 0,
            r: 0,
            g: 0,
            b: 0,
        }
    }
}
