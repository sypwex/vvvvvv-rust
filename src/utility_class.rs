pub struct UtilityClass {
    pub glow: i32,
    slowsine: i32,
    glowdir: i32,
}

impl UtilityClass {
    pub fn new () -> UtilityClass {
        UtilityClass {
            glow: 0,
            slowsine: 0,
            glowdir: 0,
        }
    }

    // UtilityClass::UtilityClass(void)
    // std::string UtilityClass::String( int _v )
    // int UtilityClass::Int(const char* str, int fallback /*= 0*/)
    // std::string UtilityClass::GCString(const std::vector<SDL_GameControllerButton>& buttons)
    pub fn GCString(&self, buttons: &Vec<sdl2::controller::Button>) -> &str {
        println!("DEADBEEF: GCString not implemented yet");
        &""
    }

    // std::string UtilityClass::twodigits( int t )
    // std::string UtilityClass::timestring( int t )
    // std::string UtilityClass::number( int _t )
    pub fn number(&self, _t: i32) -> &str {
        println!("DEADBEEF: UtilityClass::number() method not implemented yet");
        &""
    }

    // bool UtilityClass::intersects( SDL_Rect A, SDL_Rect B )
    // void UtilityClass::updateglow(void)
    pub fn update_glow (&mut self) {
        self.slowsine += 1;
        if self.slowsine >= 64 {
            self.slowsine = 0;
        }

        if self.glowdir == 0 {
            self.glow += 2;
            if self.glow >= 62 {
                self.glowdir = 1;
            }
        } else {
            self.glow -= 2;
            if self.glow < 2 {
                self.glowdir = 0;
            }
        }
    }

}
