use rand::Rng;

use crate::{game, scenes::RenderResult};
use crate::screen::render::graphics;

pub struct RenderFixed {
  glow: i32,
  slowsine: i32,
  glowdir: i32,
}

impl RenderFixed {
  pub fn new () -> RenderFixed {
    RenderFixed {
      glow: 0,
      slowsine: 0,
      glowdir: 0,
    }
  }

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

  pub fn titleupdatetextcol (&mut self, graphics: &mut graphics::Graphics) {
    graphics.col_tr = graphics.titlebg.r - (self.glow / 4) - (fRandom() as i32 * 4);
    graphics.col_tg = graphics.titlebg.g - (self.glow / 4) - (fRandom() as i32 * 4);
    graphics.col_tb = graphics.titlebg.b - (self.glow / 4) - (fRandom() as i32 * 4);

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

  pub fn title_render_fixed (&mut self, game: &mut game::Game, graphics: &mut graphics::Graphics) -> Option<RenderResult> {
    // if !game.colourblindmode {
    //   graphics.updatetowerbackground(graphics.titlebg);
    // }

    if !game.menustart {
      graphics.col_tr = 164 - (self.glow / 2) - (fRandom() as i32 * 4);
      graphics.col_tg = 164 - (self.glow / 2) - (fRandom() as i32 * 4);
      graphics.col_tb = 164 - (self.glow / 2) - (fRandom() as i32 * 4);
    } else {
      self.titleupdatetextcol(graphics);

      // graphics.updatetitlecolours();
    }

    graphics.crewframedelay -= 1;
    if graphics.crewframedelay <= 0 {
      graphics.crewframedelay = 8;
      graphics.crewframe = (graphics.crewframe + 1) % 2;
    }

    None
  }

}

pub fn fRandom () -> f32 {
  // return ( float(rand()) / float(RAND_MAX)) ;
  rand::thread_rng().gen::<f32>() / i32::MAX as f32
}
