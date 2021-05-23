use crate::{game, key_poll, screen::render::graphics};

mod preloader;

pub struct Scenes {
  pub preloader: preloader::Preloader,
}

impl Scenes {
  pub fn new() -> Scenes {
      Scenes {
          preloader: preloader::Preloader::new(),
      }
  }
}

pub enum RenderResult {
  None,
  MenuOffRender,
  Screenshake,
  Plain,
  WithScreenEffects,
}

pub trait InputTrait {
  fn input(&mut self, game: &mut game::Game, key_poll: &mut key_poll::KeyPoll);
}
pub trait RenderFixedTrait {
  fn render_fixed(&mut self, game: &mut game::Game);
}
pub trait RenderTrait {
  fn render(&mut self, graphics: &mut graphics::Graphics) -> RenderResult;
}
