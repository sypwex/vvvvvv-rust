use crate::{INBOUNDS_VEC, entity, game, map, maths, scenes::RenderResult, screen::render::{BackGround, graphics}, script, utility_class};

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
    pub fn gamerenderfixed(&mut self, obj: &mut entity::EntityClass, game: &mut game::Game, map: &mut map::Map, graphics: &mut graphics::Graphics, script: &mut script::ScriptClass, help: &mut utility_class::UtilityClass) -> Option<RenderResult> {
        if !game.blackout && !game.completestop {
            // for size_t i = 0; i < obj.entities.size(); i++ {
            // for (i, entity) in obj.entities.iter_mut().enumerate() {
            for i in 0..obj.entities.len() {
                if obj.entitycollidefloor(i, map, help) {
                    obj.entities[i].visualonground = 2;
                } else {
                    obj.entities[i].visualonground -= 1;
                }

                if obj.entitycollideroof(i, map, help) {
                    obj.entities[i].visualonroof = 2;
                } else {
                    obj.entities[i].visualonroof -= 1;
                }

                //Animate the entities
                obj.animateentities(i as i32, game);
            }
        }

        game.prev_act_fade = game.act_fade;
        if INBOUNDS_VEC!(game.activeactivity, obj.blocks) && game.hascontrol && !script.running {
            if game.act_fade < 5 {
                game.act_fade = 5;
            }
            if game.act_fade < 10 {
                game.act_fade += 1;
            }
        } else if game.act_fade > 5 {
            game.act_fade -= 1;
        }

        if obj.trophytext > 0 {
            obj.trophytext -= 1;
        }

        graphics.cutscenebarstimer();

        graphics.updatetextboxes();

        if !game.colourblindmode {
            if map.towermode {
                graphics.updatetowerbackground(BackGround::Tower, map);
            } else {
                graphics.updatebackground(map.background);
            }
        }

        if !game.blackout {
            //Update line colours!
            if graphics.linedelay <= 0 {
                graphics.linestate += 1;
                if graphics.linestate >= 10 {
                    graphics.linestate = 0;
                }
                graphics.linedelay = 2;
            } else {
                graphics.linedelay -= 1;
            }
        }

        graphics.trinketcolset = false;
        // for int i = obj.entities.size() - 1; i >= 0; i-- {
        // for (_i, entity) in obj.entities.iter_mut().enumerate() {
        for i in (0..obj.entities.len()).rev() {
            if obj.entities[i].invis {
                continue;
            }

            obj.entities[i].updatecolour(game, graphics, help);
        }

        if map.finalmode {
            map.glitchname = map.getglitchname(game.roomx, game.roomy);
        }

        // // #ifndef NO_CUSTOM_LEVELS
        // ed.oldreturneditoralpha = ed.returneditoralpha;
        // if map.custommode && !map.custommodeforreal && ed.returneditoralpha > 0 {
        //     ed.returneditoralpha -= 15;
        // }

        // // Editor ghosts!
        // if game.ghostsenabled {
        //     if map.custommode && !map.custommodeforreal {
        //         if game.frames % 3 == 0 {
        //             int i = obj.getplayer();
        //             GhostInfo ghost;
        //             ghost.rx = game.roomx-100;
        //             ghost.ry = game.roomy-100;
        //             if INBOUNDS_VEC!(i, obj.entities) {
        //                 ghost.x = obj.entities[i].xp;
        //                 ghost.y = obj.entities[i].yp;
        //                 ghost.col = obj.entities[i].colour;
        //                 ghost.realcol = obj.entities[i].realcol;
        //                 ghost.frame = obj.entities[i].drawframe;
        //             }
        //             ed.ghosts.push_back(ghost);
        //         }
        //         if ed.ghosts.size() > 100 {
        //             ed.ghosts.erase(ed.ghosts.begin());
        //         }
        //     }
        // }
        // // #endif

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
