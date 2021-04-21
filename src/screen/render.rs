mod graphics;
mod graphics_util;

pub struct Render {
    pub graphics: Box<graphics::Graphics>,
    tr: i32,
    tg: i32,
    tb: i32,
}

impl Render {
    pub fn new () -> Render {
        Render {
            graphics: Box::new(graphics::Graphics::new()),
            tr: 0,
            tg: 0,
            tb: 0,
        }
    }

    pub fn title_render(&mut self) {
        self.graphics.fill_back_buffer_with_color(sdl2::pixels::Color::BLACK);
        // FillRect(graphics.backBuffer, 0,0,graphics.backBuffer->w, graphics.backBuffer->h, 0x00000000 );

        // if (!game.menustart)
        // {
        //     tr = graphics.col_tr;
        //     tg = graphics.col_tg;
        //     tb = graphics.col_tb;

        //     int temp = 50;
        //     graphics.drawsprite((160 - 96) + 0 * 32, temp, 23, tr, tg, tb);
        //     graphics.drawsprite((160 - 96) + 1 * 32, temp, 23, tr, tg, tb);
        //     graphics.drawsprite((160 - 96) + 2 * 32, temp, 23, tr, tg, tb);
        //     graphics.drawsprite((160 - 96) + 3 * 32, temp, 23, tr, tg, tb);
        //     graphics.drawsprite((160 - 96) + 4 * 32, temp, 23, tr, tg, tb);
        //     graphics.drawsprite((160 - 96) + 5 * 32, temp, 23, tr, tg, tb);
    // #if defined(MAKEANDPLAY)
        //     graphics.Print(-1,temp+35,"     MAKE AND PLAY EDITION",tr, tg, tb, true);
    // #endif

        //     graphics.Print(5, 175, "[ Press ACTION to Start ]", tr, tg, tb, true);
        //     graphics.Print(5, 195, "ACTION = Space, Z, or V", int(tr*0.5f), int(tg*0.5f), int(tb*0.5f), true);
        // }
        // else
        // {
        //     if(!game.colourblindmode) graphics.drawtowerbackground(graphics.titlebg);

            self.menu_render();

            self.tr = ((self.tr as f32) * 0.8) as i32;
            self.tg = ((self.tg as f32) * 0.8) as i32;
            self.tb = ((self.tb as f32) * 0.8) as i32;
            if self.tr <   0 { self.tr = 0   };
            if self.tr > 255 { self.tr = 255 };
            if self.tg <   0 { self.tg = 0   };
            if self.tg > 255 { self.tg = 255 };
            if self.tb <   0 { self.tb = 0   };
            if self.tb > 255 { self.tb = 255 };

            // graphics.drawmenu(tr, tg, tb, game.currentmenuname == Menu::levellist);
        // }

        // graphics.drawfade();

        // graphics.renderwithscreeneffects();
    }

    pub fn preloaderrender (&mut self) {}
    pub fn gamerender (&mut self) {}
    pub fn maprender (&mut self) {}
    pub fn teleporterrender (&mut self) {}
    pub fn gamecompleterender (&mut self) {}
    pub fn gamecompleterender2 (&mut self) {}

    pub fn menu_render (&mut self) {
        let temp = 50;

        // switch (game.currentmenuname)
        // case Menu::mainmenu:
        self.graphics.draw_sprite((160 - 96) + 0 * 32, temp, 23, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 1 * 32, temp, 23, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 2 * 32, temp, 23, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 3 * 32, temp, 23, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 4 * 32, temp, 23, self.tr, self.tg, self.tb);
        self.graphics.draw_sprite((160 - 96) + 5 * 32, temp, 23, self.tr, self.tg, self.tb);
        self.graphics.print( 310 - (4*8), 230, "v2.3".to_string(), self.tr/2, self.tg/2, self.tb/2, false);

        // if(music.mmmmmm){
        //     graphics.Print( 10, 230, "[MMMMMM Mod Installed]", tr/2, tg/2, tb/2);
        // }
    }

    pub fn get_render_rect (&mut self) -> sdl2::rect::Rect {
        self.graphics.get_back_buffer_rect()
    }

}
