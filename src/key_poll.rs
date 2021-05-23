use std::collections::HashMap;

use sdl2::{EventPump, controller::GameController, event::{Event, WindowEvent}, keyboard::Keycode, mouse::MouseButton};
use sdl2::controller::Button;
use sdl2_sys::SDL_bool::SDL_TRUE;

use crate::{game, sdl2u::ButtonIter};

pub struct KeyPoll {
    keymap: HashMap<Keycode, bool>,
    keybuffer: String,
    controllers: HashMap<u32, GameController>,
    buttonmap: HashMap<Button, bool>,
    // wasFullscreen: u32,

    xVel: i32,
    yVel: i32,
    sensitivity: i32,
    leftbutton: i32,
    rightbutton: i32,
    middlebutton: i32,
    mx: i32,
    my: i32,
    resetWindow: bool,
    pressedbackspace: bool,
    pub linealreadyemptykludge: bool,
    pub isActive: bool,

    // static int mousetoggletimeout = 0;
    mousetoggletimeout: i32,

}

impl KeyPoll {
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
        }
    }

    // void KeyPoll::Poll(void)
    pub fn Poll(&mut self, event_pump: &mut EventPump, game: &mut game::Game) {
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

                    // TODO @sx @impl
                    println!("DEADBEEF: KeyDown handler not fully implemented yet");
                    if self.textentry() {
                        if keycode == Keycode::Backspace && !self.keybuffer.is_empty() {
                            // std::string::iterator iter = keybuffer.end();
                            // utf8::unchecked::prior(iter);
                            // keybuffer = keybuffer.substr(0, iter - keybuffer.begin());
                            // if keybuffer.empty() {
                            //     linealreadyemptykludge = true;
                            // }
                        } else if keycode == Keycode::V && self.keymap.contains_key(&Keycode::LCtrl) {
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
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(kc) => {
                            self.keymap.insert(kc, false);

                            match kc {
                                Keycode::Backspace => self.pressedbackspace = false,
                                _ => (),
                            }
                        },
                        None => {}
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
                Event::ControllerDeviceAdded { .. } => {
                    // TODO @sx @impl
                    println!("DEADBEEF: ControllerDeviceAdded handler not implemented yet");

                    // unsafe {
                    //     let toOpen = sdl2_sys::SDL_GameControllerOpen(which);
                    //     println!("Opened SDL_GameController ID #{:?}, {:?}", which, sdl2_sys::SDL_GameControllerName(toOpen));
                    //     self.controllers[sdl2_sys::SDL_JoystickInstanceID(sdl2_sys::SDL_GameControllerGetJoystick(toOpen))] = toOpen;
                    // }
                },
                Event::ControllerDeviceRemoved { .. } => {
                    // TODO @sx @impl
                    println!("DEADBEEF: ControllerDeviceRemoved handler not implemented yet");

                    // unsafe {
                    //     SDL_GameController *toClose = controllers[evt.cdevice.which];
                    //     controllers.erase(evt.cdevice.which);
                    //     printf("Closing %s\n", SDL_GameControllerName(toClose));
                    //     SDL_GameControllerClose(toClose);
                    // }
                },

                /* Window Events */
                Event::Window { win_event, .. } => {
                    match win_event {
                        /* Window Resize */
                        WindowEvent::Resized(_, _) => {
                            // TODO @sx @impl
                            println!("DEADBEEF: Resized handler not implemented yet");

                            // unsafe {
                            //     if sdl2_sys::SDL_GetWindowFlags(sdl2_sys::SDL_GetWindowFromID(evt.window.windowID)) &
                            //        sdl2_sys::SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS as u32 {
                            //         self.resetWindow = true;
                            //     }
                            // }
                        },

                        /* Window Focus */
                        WindowEvent::FocusGained => {
                            // TODO @sx @impl
                            println!("DEADBEEF: FocusLost handler not implemented yet");

                            // unsafe {
                            //     if !game.disablepause {
                            //         isActive = true;
                            //         music.resume();
                            //         music.resumeef();
                            //     }
                            //     if SDL_strcmp(sdl2_sys::SDL_GetCurrentVideoDriver(), "x11") == 0 {
                            //         if wasFullscreen {
                            //             graphics.screenbuffer->isWindowed = false;
                            //             sdl2_sys::SDL_SetWindowFullscreen(
                            //                 sdl2_sys::SDL_GetWindowFromID(evt.window.windowID),
                            //                 SDL_WINDOW_FULLSCREEN_DESKTOP
                            //             );
                            //         }
                            //     }
                            //     sdl2_sys::SDL_DisableScreenSaver();
                            // }
                        },
                        WindowEvent::FocusLost => {
                            // TODO @sx @impl
                            println!("DEADBEEF: FocusLost handler not implemented yet");
                            // unsafe {
                            //     if !game.disablepause {
                            //         isActive = false;
                            //         music.pause();
                            //         music.pauseef();
                            //     }
                            //     if SDL_strcmp(sdl2_sys::SDL_GetCurrentVideoDriver(), "x11") == 0 {
                            //         let wasFullscreen = !graphics.screenbuffer.isWindowed;
                            //         graphics.screenbuffer.isWindowed = true;
                            //         sdl2_sys::SDL_SetWindowFullscreen(
                            //             sdl2_sys::SDL_GetWindowFromID(evt.window.windowID),
                            //             0
                            //         );
                            //     }
                            //     sdl2_sys::SDL_EnableScreenSaver();
                            // }
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
                Event::Quit { .. } => {
                    // TODO @sx @impl
                    println!("DEADBEEF: Event::Quit handler not implemented yet");
                    panic!("Event::Quit handler");
                    // VVV_exit(0);
                },

                _ => {},
            }
        }

        self.mousetoggletimeout = changemousestate(
            self.mousetoggletimeout,
            showmouse,
            hidemouse
        );

        if fullscreenkeybind {
            self.toggleFullscreen();
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
    fn textentry (&mut self) -> bool {
        unsafe {
            sdl2_sys::SDL_IsTextInputActive() == SDL_TRUE
        }
    }

    // void KeyPoll::toggleFullscreen(void)
    fn toggleFullscreen (&mut self) -> bool {
        // TODO @sx @impl
        println!("DEADBEEF: KeyPoll::toggleFullscreen method not implemented yet");

        false
    }

    // pub fn is_down<T: PressArtifactTrait>(self, press: T) -> bool {
    //     false
    // }

    pub fn isDownKeycode (&mut self, key: Keycode) -> bool {
        self.keymap.contains_key(&key)
    }

    pub fn isDownVec (&mut self, buttons: &Vec<Button>) -> bool {
        for button in buttons {
            if self.buttonmap.contains_key(&button) {
                return true
            }
        }

        false
    }

    pub fn isDownSDLButton (&mut self, button: Button) -> bool {
        self.buttonmap.contains_key(&button)
    }

    pub fn controllerButtonDown (&mut self) -> bool {
        for button in Button::iterator() {
            if self.isDownSDLButton(*button) {
                return true
            }
        }
        false
    }

    pub fn controllerWantsLeft (self, includeVert: bool) -> bool {
        return self.buttonmap[&Button::DPadLeft] || self.xVel < 0 || (
            includeVert && (
                self.buttonmap[&Button::DPadUp] || self.yVel < 0
            )
        )
    }

    pub fn controllerWantsRight (self, includeVert: bool) -> bool {
        return self.buttonmap[&Button::DPadRight] || self.xVel > 0 || (
            includeVert && (
                self.buttonmap[&Button::DPadDown] || self.yVel > 0
            )
        )
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
