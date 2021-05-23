use crate::{game, scenes::RenderResult};
use crate::screen::render::graphics;
use crate::screen::renderfixed;

pub fn title_logic(game: &mut game::Game, renderfixed: &mut renderfixed::RenderFixed, graphics: &mut graphics::Graphics) -> Option<RenderResult> {
    // Misc
    // map.updatetowerglow(graphics.titlebg);
    renderfixed.update_glow();

    graphics.titlebg.bypos -= 2;
    graphics.titlebg.bscroll = -2;

    if game.menucountdown > 0 {
        game.menucountdown -= 1;

        if game.menucountdown == 0 {
            if game.menudest == game::MenuName::mainmenu {
              // music.play(6);
            } else if game.menudest == game::MenuName::gameover2 {
              // music.playef(11);
            } else if game.menudest == game::MenuName::timetrialcomplete3 {
              // music.playef(3);
            }

            // game.createmenu(game.menudest, true);
        }
    }

    None
}

pub fn maplogic () {

}

pub fn gamecompletelogic () {

}

pub fn gamecompletelogic2 () {

}

pub fn gamelogic () {

}
