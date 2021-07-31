use std::collections::HashMap;

use sdl2::{EventPump, controller::Button, event::{Event, WindowEvent}, keyboard::Keycode, mouse::MouseButton};
use sdl2_sys::SDL_bool::SDL_TRUE;

use crate::{game, helpers, music, screen};

pub struct KeyPoll {
    keymap: HashMap<Keycode, bool>,
    keybuffer: String,
    // controllers: HashMap<u32, GameController>,
    controllers: HashMap<u32, *mut sdl2_sys::SDL_GameController>,
    buttonmap: HashMap<Button, bool>,
    // wasFullscreen: u32,

    xVel: i32,
    yVel: i32,
    pub sensitivity: i32,
    leftbutton: i32,
    rightbutton: i32,
    middlebutton: i32,
    mx: i32,
    my: i32,
    pub resetWindow: bool,
    pressedbackspace: bool,
    pub linealreadyemptykludge: bool,
    pub isActive: bool,

    // static int mousetoggletimeout = 0;
    mousetoggletimeout: i32,
    wasFullscreen: bool,
}

impl KeyPoll {
    // KeyPoll::KeyPoll(void)
    pub fn new() -> KeyPoll {
        KeyPoll {
            keymap: HashMap::new(),
            keybuffer: String::new(),
            controllers: HashMap::new(),
            buttonmap: HashMap::new(),
            // wasFullscreen: u32,

            xVel: 0,
            yVel: 0,
            sensitivity: 2,
            leftbutton: 0,
            rightbutton: 0,
            middlebutton: 0,
            mx: 0,
            my: 0,
            resetWindow: false,
            pressedbackspace: false,
            linealreadyemptykludge: false,
            isActive: true,

            // static int mousetoggletimeout = 0;
            mousetoggletimeout: 0,
            wasFullscreen: false,
        }
    }

    // int inline KeyPoll::getThreshold(void)
    fn getThreshold(&mut self) -> i16 {
        match self.sensitivity {
            0 => 28000,
            1 => 16000,
            2 => 8000,
            3 => 4000,
            4 => 2000,
            _ => 8000,
        }
    }

    // void KeyPoll::enabletextentry(void)

    // void KeyPoll::disabletextentry(void)

    // bool KeyPoll::textentry(void)
    fn textentry(&mut self) -> bool {
        unsafe {
            sdl2_sys::SDL_IsTextInputActive() == SDL_TRUE
        }
    }

    // void KeyPoll::toggleFullscreen(void)
    fn toggleFullscreen(&mut self, game: &mut game::Game, gameScreen: &mut screen::Screen) {
        gameScreen.toggleFullScreen();

        self.keymap.clear(); /* we lost the input due to a new window. */
        if game.glitchrunnermode {
            game.press_left = false;
            game.press_right = false;
            game.press_action = true;
            game.press_map = false;
        }
    }

    // void KeyPoll::Poll(void)
    pub fn Poll(&mut self, event_pump: &mut EventPump, game: &mut game::Game, music: &mut music::Music, gameScreen: &mut screen::Screen) -> Result<(), i32> {
        let mut showmouse = false;
        let mut hidemouse = false;
        let altpressed = false;
        let mut fullscreenkeybind = false;

        for event in event_pump.poll_iter() {
            match event {

                /* Keyboard Input */
                Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                    self.keymap.insert(keycode, true);

                    match keycode {
                        Keycode::Backspace => self.pressedbackspace = true,
                        _ => (),
                    }

                    let returnpressed = keycode == Keycode::Return;
                    let fpressed = keycode == Keycode::F;
                    let f11pressed = keycode == Keycode::F11;
                    // #ifdef __APPLE__ /* OSX prefers the command keys over the alt keys. -flibit */
                    // altpressed = keymap[SDLK_LGUI] || keymap[SDLK_RGUI];
                    // #else
                    let altpressed = self.keymap.contains_key(&Keycode::LAlt) || self.keymap.contains_key(&Keycode::RAlt);
                    // #endif

                    if (altpressed && (returnpressed || fpressed)) || f11pressed {
                        fullscreenkeybind = true;
                    }

                    if self.textentry() {
                        if keycode == Keycode::Backspace && !self.keybuffer.is_empty() {
                            println!("DEADBEEF: KeyDown handler(Backspace) not fully implemented yet");
                            // std::string::iterator iter = keybuffer.end();
                            // utf8::unchecked::prior(iter);
                            // keybuffer = keybuffer.substr(0, iter - keybuffer.begin());
                            // if keybuffer.empty() {
                            //     linealreadyemptykludge = true;
                            // }
                        } else if keycode == Keycode::V && self.keymap.contains_key(&Keycode::LCtrl) {
                            println!("DEADBEEF: KeyDown handler(V) not fully implemented yet");
                            // let text = sdl2_sys::SDL_GetClipboardText();
                            // if text != NULL {
                            //     self.keybuffer += text;
                            //     sdl2_sys::SDL_free(text);
                            // }
                        }
                    }

                    if repeat {
                        hidemouse = true;
                    }
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    self.keymap.insert(keycode, false);

                    match keycode {
                        Keycode::Backspace => self.pressedbackspace = false,
                        _ => (),
                    }
                },
                Event::TextInput { text, .. } => {
                    if !altpressed {
                        self.keybuffer += &text;
                    }
                    hidemouse = true;
                },

                /* Mouse Input */
                Event::MouseMotion { x, y, .. } => {
                    showmouse = true;
                    self.mx = x;
                    self.my = y;
                },
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    showmouse = true;
                    match mouse_btn {
                        MouseButton::Left => {
                            self.mx = x;
                            self.my = y;
                            self.leftbutton = 1;
                        },
                        MouseButton::Right => {
                            self.mx = x;
                            self.my = y;
                            self.rightbutton = 1;
                        },
                        MouseButton::Middle => {
                            self.mx = x;
                            self.my = y;
                            self.middlebutton = 1;
                        },
                        _ => (),
                    }
                },
                Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            self.mx = x;
                            self.my = y;
                            self.leftbutton = 0;
                        },
                        MouseButton::Right => {
                            self.mx = x;
                            self.my = y;
                            self.rightbutton = 0;
                        },
                        MouseButton::Middle => {
                            self.mx = x;
                            self.my = y;
                            self.middlebutton = 0;
                        },
                        _ => (),
                    }
                },

                /* Controller Input */
                Event::ControllerButtonDown { button, .. } => {
                    hidemouse = true;
                    self.buttonmap.insert(button, true);
                },
                Event::ControllerButtonUp { button, .. } => {
                    self.buttonmap.insert(button, false);
                },
                Event::ControllerAxisMotion { axis, value, .. } => {
                    hidemouse = true;
                    let threshold = self.getThreshold();
                    match axis {
                        sdl2::controller::Axis::LeftX => {
                            self.xVel = match value > -threshold && value < threshold {
                                true => 0,
                                false => if value > 0 { 1 } else { -1 },
                            };
                        },
                        sdl2::controller::Axis::LeftY => {
                            self.yVel = match value > -threshold && value < threshold {
                                true => 0,
                                false => if value > 0 { 1 } else { -1 },
                            };
                        },
                        _ => (),
                    }
                },
                Event::ControllerDeviceAdded { which, .. } => {
                    unsafe {
                        // subsystem.open(which);
                        let toOpen = sdl2_sys::SDL_GameControllerOpen(which as i32);
                        info!("Opened SDL_GameController ID #{:?}, {:?}", which, sdl2_sys::SDL_GameControllerName(toOpen));
                        let jid = sdl2_sys::SDL_JoystickInstanceID(sdl2_sys::SDL_GameControllerGetJoystick(toOpen)) as u32;
                        if self.controllers.contains_key(&jid) != true {
                            self.controllers.insert(jid, toOpen);
                        }
                    }
                },
                Event::ControllerDeviceRemoved { which, .. } => {
                    unsafe {
                        if let Some((key, toClose)) = self.controllers.remove_entry(&which) {
                            info!("Closing {:?}", helpers::c_str_to_string(sdl2_sys::SDL_GameControllerName(toClose)));
                            // info!("Closing {:?}", toClose.name());
                            std::mem::drop(toClose);
                        }
                    }
                },

                /* Window Events */
                Event::Window { win_event, window_id, ..} => {
                    match win_event {
                        WindowEvent::Close => return Err(0),
                        /* Window Resize */
                        WindowEvent::Resized(_, _) => {
                            unsafe {
                                let id = sdl2_sys::SDL_GetWindowFromID(window_id);
                                let curflags = sdl2_sys::SDL_GetWindowFlags(id);
                                let s_d_l_window_flags = sdl2_sys::SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS as u32;

                                if curflags & s_d_l_window_flags == s_d_l_window_flags {
                                    self.resetWindow = true;
                                }
                            }
                        },

                        /* Window Focus */
                        WindowEvent::FocusGained => {
                            unsafe {
                                if !game.disablepause {
                                    self.isActive = true;
                                    music.resume();
                                    music.resumeef();
                                }
                                if helpers::c_str_to_string(sdl2_sys::SDL_GetCurrentVideoDriver()) == "x11" {
                                    if self.wasFullscreen {
                                        gameScreen.isWindowed = false;
                                        sdl2_sys::SDL_SetWindowFullscreen(
                                            sdl2_sys::SDL_GetWindowFromID(window_id),
                                            sdl2_sys::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as u32
                                        );
                                    }
                                }
                                sdl2_sys::SDL_DisableScreenSaver();
                            }
                        },
                        WindowEvent::FocusLost => {
                            unsafe {
                                if !game.disablepause {
                                    self.isActive = false;
                                    music.pause();
                                    music.pauseef();
                                }
                                if helpers::c_str_to_string(sdl2_sys::SDL_GetCurrentVideoDriver()) == "x11" {
                                    self.wasFullscreen = !gameScreen.isWindowed;
                                    gameScreen.isWindowed = true;
                                    sdl2_sys::SDL_SetWindowFullscreen(
                                        sdl2_sys::SDL_GetWindowFromID(window_id),
                                        0
                                    );
                                }
                                sdl2_sys::SDL_EnableScreenSaver();
                            }
                        },

                        /* Mouse Focus */
                        WindowEvent::Enter => {
                            unsafe {
                                sdl2_sys::SDL_DisableScreenSaver();
                            }
                        },
                        WindowEvent::Leave => {
                            unsafe {
                                sdl2_sys::SDL_EnableScreenSaver();
                            }
                        },
                        _ => (),
                    }
                }

                /* Quit Event */
                Event::Quit { .. } => return Err(0),

                _ => {},
            }
        }

        self.mousetoggletimeout = changemousestate(
            self.mousetoggletimeout,
            showmouse,
            hidemouse
        );

        if fullscreenkeybind {
            self.toggleFullscreen(game, gameScreen);
        }

        Ok(())
    }

    // bool KeyPoll::isDown(SDL_Keycode key)
    pub fn isDown(&mut self, key: i32) -> bool {
        if let Some(key) = sdl2::keyboard::Keycode::from_i32(key) {
            return match self.keymap.get(&key) {
                Some(v) => *v,
                None => false,
            }
        }
        false
    }

    // bool KeyPoll::isDown(SDL_Keycode key)
    pub fn isDownKeycode(&mut self, key: Keycode) -> bool {
        match self.keymap.get(&key) {
            Some(v) => *v,
            None => false,
        }
    }

    // bool KeyPoll::isDown(std::vector<SDL_GameControllerButton> buttons)
    pub fn isDownVec(&mut self, buttons: &Vec<Button>) -> bool {
        for button in buttons {
            if let Some(v) = self.buttonmap.get(&button) {
                if *v == true {
                    return true
                }
            };
        }

        false
    }

    // bool KeyPoll::isDown(SDL_GameControllerButton button)
    pub fn isDownSDLButton(&mut self, button: Button) -> bool {
        if let Some(v) = self.buttonmap.get(&button) {
            return *v
        }

        false
    }

    // bool KeyPoll::controllerButtonDown(void)
    pub fn controllerButtonDown(&mut self) -> bool {
        for (_, v) in &self.buttonmap {
            if *v == true {
                return true
            }
        };

        false
    }

    // bool KeyPoll::controllerWantsLeft(bool includeVert)
    pub fn controllerWantsLeft(&mut self, includeVert: bool) -> bool {
        let pad_left = *self.buttonmap.entry(Button::DPadLeft).or_default();
        let pad_up = *self.buttonmap.entry(Button::DPadUp).or_default();

        return pad_left || self.xVel < 0 || (includeVert && (pad_up || self.yVel < 0))
    }

    // bool KeyPoll::controllerWantsRight(bool includeVert)
    pub fn controllerWantsRight(&mut self, includeVert: bool) -> bool {
        let pad_right = *self.buttonmap.entry(Button::DPadRight).or_default();
        let pad_down = *self.buttonmap.entry(Button::DPadDown).or_default();

        return pad_right || self.xVel > 0 || (includeVert && (pad_down || self.yVel > 0))
    }

}

// static int changemousestate(int timeout, const bool show, const bool hide)
fn changemousestate(timeout: i32, show: bool, hide: bool) -> i32 {
    let prev: i32;
    let new_: SdlEnabled;

    if timeout > 0 {
        return --timeout;
    }

    /* If we want to both show and hide at the same time, prioritize showing */
    if show {
        new_ = SdlEnabled::SdlEnable;
    } else if hide {
        new_ = SdlEnabled::SdlDisable;
    } else {
        return timeout
    }

    unsafe {
        prev = sdl2_sys::SDL_ShowCursor(sdl2_sys::SDL_QUERY);

        if prev == new_ as i32 {
            return timeout
        }

        sdl2_sys::SDL_ShowCursor(new_ as i32);
    }

    match new_ {
        SdlEnabled::SdlDisable => 0,
        SdlEnabled::SdlEnable => 30,
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
enum SdlEnabled {
    SdlDisable = sdl2_sys::SDL_DISABLE,
    SdlEnable = sdl2_sys::SDL_ENABLE,
}
