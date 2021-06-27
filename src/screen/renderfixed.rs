use crate::map;
use crate::{game, maths, scenes::RenderResult};
use crate::screen::render::graphics;

use super::render::BackGround;

pub struct RenderFixed {
    glow: i32,
    slowsine: i32,
    glowdir: i32,
}

impl RenderFixed {
    pub fn new() -> RenderFixed {
        RenderFixed {
            glow: 0,
            slowsine: 0,
            glowdir: 0,
        }
    }

    // void UtilityClass::updateglow(void)
    pub fn update_glow(&mut self) {
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

    // static inline void titleupdatetextcol(void)
    pub fn titleupdatetextcol(&mut self, graphics: &mut graphics::Graphics) {
        graphics.col_tr = graphics.buffers.titlebg.r - (self.glow / 4) - (maths::fRandom() as i32 * 4);
        graphics.col_tg = graphics.buffers.titlebg.g - (self.glow / 4) - (maths::fRandom() as i32 * 4);
        graphics.col_tb = graphics.buffers.titlebg.b - (self.glow / 4) - (maths::fRandom() as i32 * 4);

        if graphics.col_tr < 0 {
            graphics.col_tr = 0;
        }
        if graphics.col_tr > 255 {
            graphics.col_tr = 255;
        }
        if graphics.col_tg < 0 {
            graphics.col_tg = 0;
        }
        if graphics.col_tg > 255 {
            graphics.col_tg = 255;
        }
        if graphics.col_tb < 0 {
            graphics.col_tb = 0;
        }
        if graphics.col_tb > 255 {
            graphics.col_tb = 255;
        }
    }

    // void gamerenderfixed(void)
    pub fn gamerenderfixed(&mut self) -> Option<RenderResult> {
        println!("DEADBEEF: gamerenderfixed method not implemented yet");
        None
    }

    // void titlerenderfixed(void)
    pub fn titlerenderfixed(&mut self, map: &mut map::Map, game: &mut game::Game, graphics: &mut graphics::Graphics) -> Option<RenderResult> {
        if !game.colourblindmode {
            graphics.updatetowerbackground(BackGround::Title, map);
        }

        if !game.menustart {
            graphics.col_tr = 164 - (self.glow / 2) - (maths::fRandom() as i32 * 4);
            graphics.col_tg = 164 - (self.glow / 2) - (maths::fRandom() as i32 * 4);
            graphics.col_tb = 164 - (self.glow / 2) - (maths::fRandom() as i32 * 4);
        } else {
            self.titleupdatetextcol(graphics);

            graphics.updatetitlecolours(self.glow);
        }

        graphics.crewframedelay -= 1;
        if graphics.crewframedelay <= 0 {
            graphics.crewframedelay = 8;
            graphics.crewframe = (graphics.crewframe + 1) % 2;
        }

        None
    }

    // void maprenderfixed(void)
    pub fn maprenderfixed(&mut self) {
        println!("DEADBEEF: maprenderfixed method not implemented yet");
    }

    // void teleporterrenderfixed(void)
    pub fn teleporterrenderfixed(&mut self) {
        println!("DEADBEEF: teleporterrenderfixed method not implemented yet");
    }

    // void gamecompleterenderfixed(void)
    pub fn gamecompleterenderfixed(&mut self) {
        println!("DEADBEEF: gamecompleterenderfixed method not implemented yet");
    }

}
