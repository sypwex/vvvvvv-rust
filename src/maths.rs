use rand::Rng;

// @vvv: This header holds Maths functions that emulate the functionality of flash's

// @vvv: random Returns 0..1
// float inline fRandom(void)
pub fn fRandom() -> f32 {
    // return ( float(rand()) / float(RAND_MAX)) ;
    rand::thread_rng().gen::<f32>() / i32::MAX as f32
}

// inline int clamp(int x, int a, int b)
pub fn clamp(x: i32, a: i32, b: i32) -> i32 {
    // x < a ? a : (x > b ? b : x)
    if x < a {
        a
    } else {
        if x > b {
            b
        } else {
            x
        }
    }
}

pub struct point {
    pub x: i32,
    pub y: i32,
}

// inline int VVV_min(const int a, const int b)
pub fn VVV_min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

// inline int VVV_max(const int a, const int b)
pub fn VVV_max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// newly added rusty methods

// should behave like stdlib::rand()
pub fn c_rand() -> i32 {
    rand::thread_rng().gen::<i32>()
}
