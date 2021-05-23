use crate::{game, scenes::RenderResult};
pub mod graphics;

pub struct Render {
    pub graphics: Box<graphics::Graphics>,
    tr: i32,
    tg: i32,
    tb: i32,
}

impl Render {
    pub fn new (pf: sdl2::pixels::PixelFormatEnum) -> Render {
        let graphics = Box::new(graphics::Graphics::new(pf));

        Render {
            graphics,
            tr: 0,
            tg: 0,
            tb: 0,
        }
    }

    pub fn title_render(&mut self, game: &mut game::Game) -> RenderResult {
        // FillRect(graphics.backBuffer, 0,0,graphics.backBuffer->w, graphics.backBuffer->h, 0x00000000 );
        self.graphics.buffers.fill_back_buffer_with_color(sdl2::pixels::Color::BLACK);

        if !game.menustart {
            let tr = self.graphics.col_tr;
            let tg = self.graphics.col_tg;
            let tb = self.graphics.col_tb;

            let temp = 50;
            let vsprite = 27; // 23
            self.graphics.draw_sprite((160 - 96) + 0 * 32, temp, vsprite, tr, tg, tb);
            self.graphics.draw_sprite((160 - 96) + 1 * 32, temp, vsprite, tr, tg, tb);
            self.graphics.draw_sprite((160 - 96) + 2 * 32, temp, vsprite, tr, tg, tb);
            self.graphics.draw_sprite((160 - 96) + 3 * 32, temp, vsprite, tr, tg, tb);
            self.graphics.draw_sprite((160 - 96) + 4 * 32, temp, vsprite, tr, tg, tb);
            self.graphics.draw_sprite((160 - 96) + 5 * 32, temp, vsprite, tr, tg, tb);
            // #if defined(MAKEANDPLAY)
            // graphics.Print(-1,temp+35,"     MAKE AND PLAY EDITION",tr, tg, tb, true);
            // #endif

            self.graphics.print(5, 175, "[ Press ACTION to Start ]".to_string(), tr, tg, tb, true);
            self.graphics.print(5, 195, "ACTION = Space, Z, or V".to_string(), tr/2, tg/2, tb/2, true);
        } else {
            // if(!game.colourblindmode) graphics.drawtowerbackground(graphics.titlebg);

            self.menu_render();

            let mut tr = ((self.graphics.col_tr as f32) * 0.8) as i32;
            let mut tg = ((self.graphics.col_tg as f32) * 0.8) as i32;
            let mut tb = ((self.graphics.col_tb as f32) * 0.8) as i32;
            if tr <   0 { tr = 0   };
            if tr > 255 { tr = 255 };
            if tg <   0 { tg = 0   };
            if tg > 255 { tg = 255 };
            if tb <   0 { tb = 0   };
            if tb > 255 { tb = 255 };

            // graphics.drawmenu(tr, tg, tb, game.currentmenuname == Menu::levellist);
        }

        // self.graphics.drawfade();

        RenderResult::WithScreenEffects
    }

    pub fn menu_render (&mut self) {
        let temp = 50;

        // switch (game.currentmenuname)
        // case Menu::mainmenu:
        let vsprite = 27; // 23
        self.graphics.draw_sprite((160 - 96) + 0 * 32, temp, vsprite, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 1 * 32, temp, vsprite, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 2 * 32, temp, vsprite, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 3 * 32, temp, vsprite, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 4 * 32, temp, vsprite, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 5 * 32, temp, vsprite, self.tr, self.tg, self.tb);
        self.graphics.print( 310 - (6*8), 230, "v2.3-rust".to_string(), self.tr/2, self.tg/2, self.tb/2, false);

        // if(music.mmmmmm){
        //     graphics.Print( 10, 230, "[MMMMMM Mod Installed]", tr/2, tg/2, tb/2);
        // }
    }

    pub fn get_render_rect (&mut self) -> sdl2::rect::Rect {
        self.graphics.buffers.get_back_buffer_rect()
    }

}
