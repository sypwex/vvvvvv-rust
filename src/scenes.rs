use crate::{game::{self, GameState}, key_poll, screen::render::graphics};

pub mod preloader;

pub struct Scenes {
    pub gamestate_func_index: u8,
    pub num_gamestate_funcs: u8,
    pub gamestate_funcs: Vec<SceneFunc>,

    pub scene_functions: Vec<SceneFunc>,
}

impl Scenes {
    pub fn new() -> Scenes {
        Scenes {
            gamestate_func_index: 0,
            num_gamestate_funcs: 0,
            gamestate_funcs: vec![],

            scene_functions: vec![
            // GameState::PRELOADER
            SceneFunc { state: GameState::PRELOADER, fntype: FuncType::FuncInput, fnname: Fns::preloaderinput },
            SceneFunc { state: GameState::PRELOADER, fntype: FuncType::FuncFixed, fnname: Fns::preloaderrenderfixed },
            SceneFunc { state: GameState::PRELOADER, fntype: FuncType::FuncDelta, fnname: Fns::preloaderrender },

            // GameState::TITLEMODE
            SceneFunc { state: GameState::TITLEMODE, fntype: FuncType::FuncInput, fnname: Fns::titleinput },
            SceneFunc { state: GameState::TITLEMODE, fntype: FuncType::FuncFixed, fnname: Fns::titlerenderfixed },
            SceneFunc { state: GameState::TITLEMODE, fntype: FuncType::FuncDelta, fnname: Fns::titlerender },
            SceneFunc { state: GameState::TITLEMODE, fntype: FuncType::FuncFixed, fnname: Fns::titlelogic },

            // GameState::GAMEMODE => {
            //     // {Func_fixed, runscript},
            //     // {Func_fixed, gamerenderfixed},
            //     // {Func_delta, gamerender},
            //     // {Func_input, gameinput},
            //     // {Func_fixed, gamelogic},
            // },
            // GameState::MAPMODE => {
            //     // {Func_fixed, maprenderfixed},
            //     // {Func_delta, maprender},
            //     // {Func_input, mapinput},
            //     // {Func_fixed, maplogic},
            // },
            // GameState::TELEPORTERMODE => {
            //     // {Func_fixed, maprenderfixed},
            //     // {Func_delta, teleporterrender},
            //     // {Func_input, teleportermodeinput},
            //     // {Func_fixed, maplogic},
            // },
            // GameState::GAMECOMPLETE => {
            //     // {Func_fixed, gamecompleterenderfixed},
            //     // {Func_delta, gamecompleterender},
            //     // {Func_input, gamecompleteinput},
            //     // {Func_fixed, gamecompletelogic},
            // },
            // GameState::GAMECOMPLETE2 => {
            //     // {Func_delta, gamecompleterender2},
            //     // {Func_input, gamecompleteinput2},
            //     // {Func_fixed, gamecompletelogic2},
            // },
            // // #if !defined(NO_CUSTOM_LEVELS) && !defined(NO_EDITOR)
            // GameState::EDITORMODE => {
            //     // {Func_fixed, flipmodeoff},
            //     // {Func_input, editorinput},
            //     // {Func_fixed, editorrenderfixed},
            //     // {Func_delta, editorrender},
            //     // {Func_fixed, editorlogic},
            // },
            // // #endif
            // // TODO @sx
            // GameState::CLICKTOSTART => {
            //     // help.updateglow();
            // },
            // // TODO @sx
            // GameState::FOCUSMODE => {
            // },
            ],
        }
    }

    // previously was in main.cpp
    // static enum IndexCode increment_gamestate_func_index(void)
    pub fn increment_gamestate_func_index(&mut self, game: &game::Game) -> IndexCode {
        // println!("increment_gamestate_func_index");

        self.gamestate_func_index += 1;

        if self.gamestate_func_index == self.num_gamestate_funcs {
            /* Reached the end of current gamestate order.
            * Re-fetch for new order if gamestate changed.
            */
            self.update_gamestate_funcs(game.gamestate);

            /* Also run callbacks that were deferred to end of func sequence. */
            // DEFER_execute_callbacks();

            self.gamestate_func_index = 0;

            IndexCode::IndexEnd
        } else {
            IndexCode::IndexNone
        }
    }

    // previously was in main.cpp
    // static const inline struct ImplFunc* get_gamestate_funcs(const int gamestate, int* num_implfuncs)
    pub fn update_gamestate_funcs(&mut self, game_state: GameState) {
        self.gamestate_funcs = self.scene_functions
            .clone()
            .iter()
            .filter(|el| el.state == game_state)
            .cloned()
            .collect();
        self.num_gamestate_funcs = self.gamestate_funcs.len() as u8;
    }

    pub fn get_current_gamestate_func(&mut self) -> SceneFunc {
        self.gamestate_funcs[self.gamestate_func_index as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IndexCode {
    IndexNone,
    IndexEnd
}

pub enum RenderResult {
    None,
    MenuOffRender,
    Screenshake,
    Plain,
    WithScreenEffects,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FuncType {
    FuncNull,
    FuncFixed,
    FuncInput,
    FuncDelta
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SceneFunc {
    state: GameState,
    pub fntype: FuncType,
    pub fnname: Fns,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Fns {
    // GameState::PRELOADER
    preloaderinput,
    preloaderrenderfixed,
    preloaderrender,

    // GameState::TITLEMODE
    titleinput,
    titlerenderfixed,
    titlerender,
    titlelogic,
}

pub trait InputTrait {
    fn input(&mut self, game: &mut game::Game, key_poll: &mut key_poll::KeyPoll) -> Option<RenderResult>;
}
pub trait RenderFixedTrait {
    fn render_fixed(&mut self, game: &mut game::Game) -> Option<RenderResult>;
}
pub trait RenderTrait {
    fn render(&mut self, graphics: &mut graphics::Graphics) -> Option<RenderResult>;
}
