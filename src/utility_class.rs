// #define INBOUNDS_VEC(index, vector) ((int) index >= 0 && (int) index < (int) vector.size())
#[macro_export]
macro_rules! INBOUNDS_VEC {
    ($index:expr, $vector:expr) => {
        $index >= 0 && ($index as usize) < $vector.len()
    }
}

// #define INBOUNDS_ARR(index, array) ((int) index >= 0 && (int) index < (int) SDL_arraysize(array))
#[macro_export]
macro_rules! INBOUNDS_ARR {
    ($index:expr, $array:expr) => {
        $index >= 0 && ($index as usize) < $array.len()
    }
}

// #define WHINE_ONCE(message) static bool whine = true; if (whine) {  whine = false;  puts(message); }
#[macro_export]
macro_rules! WHINE_ONCE {
    ($message:tt) => {
        static mut WHINE: bool = true;
        unsafe {
            if WHINE == true {
                WHINE = false;
                println!($message);
            }
        }
    }
}

// static const char* GCChar(const SDL_GameControllerButton button)
pub fn GCChar(button: sdl2_sys::SDL_GameControllerButton) -> Option<&'static str> {
    println!("DEADBEEF: GCChar not implemented yet");
    // switch (button)
    // {
    // case SDL_CONTROLLER_BUTTON_A:
    //     return "A";
    // case SDL_CONTROLLER_BUTTON_B:
    //     return "B";
    // case SDL_CONTROLLER_BUTTON_X:
    //     return "X";
    // case SDL_CONTROLLER_BUTTON_Y:
    //     return "Y";
    // case SDL_CONTROLLER_BUTTON_BACK:
    //     return "BACK";
    // case SDL_CONTROLLER_BUTTON_GUIDE:
    //     return "GUIDE";
    // case SDL_CONTROLLER_BUTTON_START:
    //     return "START";
    // case SDL_CONTROLLER_BUTTON_LEFTSTICK:
    //     return "L3";
    // case SDL_CONTROLLER_BUTTON_RIGHTSTICK:
    //     return "R3";
    // case SDL_CONTROLLER_BUTTON_LEFTSHOULDER:
    //     return "LB";
    // case SDL_CONTROLLER_BUTTON_RIGHTSHOULDER:
    //     return "RB";
    // default:
    //     SDL_assert(0 && "Unhandled button!");
    //     return NULL;
    // }

    None
}

// int ss_toi(const std::string& str)
pub fn ss_toi(str: &str) -> i32 {
    let mut retval = 0;
    let mut negative = false;
    static radix: i32 = 10;

    // for (size_t i = 0; i < str.size(); ++i) {
    for (i, chr) in str.chars().enumerate() {
        if i == 0 && chr == '-' {
            negative = true;
            continue;
        }

        if chr.is_alphanumeric() {
            // retval *= radix;
            // retval += chr - '0';
            retval = (retval*radix) + chr.to_digit(10).unwrap() as i32;
        } else {
            break;
        }
    }

    return if negative {
        -retval
    } else {
        retval
    }
}

// bool next_split(size_t* start, size_t* len, const char* str, const char delim)
pub fn next_split(start: usize, len: usize, r#str: &str, delim: char) -> bool {
    println!("DEADBEEF: next_split method not implemented yet");
    // *len = 0;

    // if str[idx] == '\0' {
    //     return false;
    // }

    // while (true)
    // {
    //     if (str[idx] == delim)
    //     {
    //         *start += 1;
    //         return true;
    //     }
    //     else if (str[idx] == '\0')
    //     {
    //         return true;
    //     }

    //     idx += 1;
    //     *start += 1;
    //     *len += 1;
    // }
    false
}

// bool next_split_s( char buffer[], const size_t buffer_size, size_t* start, const char* str, const char delim)
pub fn next_split_s() {
    println!("DEADBEEF: next_split_s method not implemented yet");
    // size_t len = 0;
    // const size_t prev_start = *start;

    // const bool retval = next_split(start, &len, &str[*start], delim);

    // if (retval)
    // {
    //     /* Using SDL_strlcpy() here results in calling SDL_strlen() */
    //     /* on the whole string, which results in a visible freeze */
    //     /* if it's a very large string */
    //     const size_t length = VVV_min(buffer_size - 1, len);
    //     SDL_memcpy(buffer, &str[prev_start], length);
    //     buffer[length] = '\0';
    // }

    // return retval;
}

pub struct UtilityClass {
    pub glow: i32,
    pub slowsine: i32,
    glowdir: i32,
    splitseconds: [i32;30],
}

impl UtilityClass {
    // UtilityClass::UtilityClass(void)
    pub fn new() -> UtilityClass {
        let mut splitseconds: [i32;30] = Default::default();
        // for (size_t i = 0; i < SDL_arraysize(splitseconds); i++)
        for i in 0..30 {
            splitseconds[i] = (i as i32 * 100) / 30;
        }

        UtilityClass {
            glow: 0,
            slowsine: 0,
            glowdir: 0,
            splitseconds,
        }
    }

    // std::string UtilityClass::String( int _v )
    pub fn String(&self, _v: i32) -> String {
        _v.to_string()
    }

    // int UtilityClass::Int(const char* str, int fallback /*= 0*/)
    // std::string UtilityClass::GCString(const std::vector<SDL_GameControllerButton>& buttons)
    pub fn GCString(&self, buttons: &Vec<sdl2::controller::Button>) -> &str {
        println!("DEADBEEF: UtilityClass::GCString not implemented yet");
        &""
    }

    // std::string UtilityClass::twodigits( int t )
    pub fn twodigits(&self, t: i32) -> String {
        match t {
            t if t < 10 => ["0", &t.to_string()].concat(),
            t if t >= 100 => "??".to_string(),
            _ => t.to_string(),
        }
    }

    // std::string UtilityClass::timestring( int t )
    pub fn timestring(&self, t: i32) -> String {
        //given a time t in frames, return a time in seconds
        let temp = (t - (t % 30)) / 30;
        if temp < 60 {
            //less than one minute
            let t = t as usize % 30;
            [temp.to_string(), ":".to_string(), self.twodigits(self.splitseconds[t])].concat()
        } else {
            let temp2 = (temp - (temp % 60)) / 60;
            let temp = temp % 60;
            let t = t as usize % 30;
            [temp2.to_string(), ":".to_string(), self.twodigits(temp), ":".to_string(), self.twodigits(self.splitseconds[t])].concat()
        }
    }

    // std::string UtilityClass::number( int _t )
    pub fn number(&self, t: i32) -> String {
        static ones_place: [&str;9] = ["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
        static tens_place: [&str;9] = ["Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];
        static teens: [&str;9] = ["Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];

        match t {
            t if t < 0 => "???".to_string(),
            t if t > 100 => "Lots".to_string(),
            t if t == 0 => "Zero".to_string(),
            t if t == 100 => "One Hundred".to_string(),
            t if t >= 1 && t <= 9 => ones_place[t as usize - 1].to_string(),
            t if t >= 11 && t <= 19 => teens[t as usize - 11].to_string(),
            t if t % 10 == 0 => tens_place[(t as usize / 10) - 1].to_string(),
            _ => [tens_place[(t as usize / 10) - 1], " ", ones_place[(t as usize % 10) - 1]].concat(),
        }
    }

    // bool UtilityClass::intersects( SDL_Rect A, SDL_Rect B )
    pub fn intersects(&self, A: sdl2::rect::Rect, B: sdl2::rect::Rect) -> bool {
        // return (SDL_HasIntersection(&A, &B) == SDL_TRUE);
        A.has_intersection(B)
    }

    // void UtilityClass::updateglow(void)
    pub fn updateglow(&mut self) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let help = UtilityClass::new();

        assert_eq!("Zero", help.number(0));
        assert_eq!("One", help.number(1));
        assert_eq!("Nine", help.number(9));

        assert_eq!("Eleven", help.number(11));
        assert_eq!("Nineteen", help.number(19));

        assert_eq!("Ten", help.number(10));
        assert_eq!("Ninety", help.number(90));

        assert_eq!("Twenty One", help.number(21));
        assert_eq!("Thirty Three", help.number(33));

        assert_eq!("One Hundred", help.number(100));

        assert_eq!("???", help.number(-1));
        assert_eq!("Lots", help.number(101));
    }
}
