use crate::{map, music, screen};
use crate::{game, scenes::RenderResult};
use crate::screen::render::graphics;
use crate::screen::renderfixed;

pub fn titlelogic(map: &mut map::Map, music: &mut music::Music, game: &mut game::Game, renderfixed: &mut renderfixed::RenderFixed, graphics: &mut graphics::Graphics, screen_params: screen::ScreenParams) -> Option<RenderResult> {
    //Misc
    //map.updatetowerglow(&mut graphics.buffers.titlebg);
    renderfixed.update_glow();

    graphics.buffers.titlebg.bypos -= 2;
    graphics.buffers.titlebg.bscroll = -2;

    if game.menucountdown > 0 {
        game.menucountdown -= 1;

        if game.menucountdown == 0 {
            if game.menudest == game::MenuName::mainmenu {
                music.play(6);
            } else if game.menudest == game::MenuName::gameover2 {
                music.playef(11);
            } else if game.menudest == game::MenuName::timetrialcomplete3 {
                music.playef(3);
            }

            game.createmenu(game.menudest, Some(true), graphics, music, screen_params, map);
        }
    }

    None
}

pub fn maplogic() {

}

pub fn gamecompletelogic() {

}

pub fn gamecompletelogic2() {

}

pub fn gamelogic() -> Option<RenderResult> {
    println!("DEADBEEF: gamelogic method not implemented yet");
    None
}
