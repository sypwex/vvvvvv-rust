use sdl2::keyboard::Keycode;

use crate::{INBOUNDS_ARR, INBOUNDS_VEC, entity, game::{self, GameState}, key_poll, map, music, scenes::RenderResult, screen::render::graphics, utility_class};
pub mod scripts;

pub struct ScriptClass {
    //Script contents
    commands: Vec<String>,
    words: Vec<String>,
    txt: Vec<String>,
    scriptname: String,
    position: usize,
    looppoint: usize,
    loopcount: i32,

    scriptdelay: i32,
    pub running: bool,

    //Textbox stuff
    textx: i32,
    texty: i32,
    r: i32,
    g: i32,
    b: i32,
    textflipme: bool,

    //Misc
    i: i32,
    j: i32,
    k: i32,

    //Custom level stuff
    // std::vector<Script> customscripts;
}

impl ScriptClass {
    // scriptclass::scriptclass(void)
    pub fn new() -> Self {
        Self {
            //Script contents
            commands: Vec::new(),
            words: vec![String::new(); 40],
            txt: Vec::new(),
            scriptname: String::new(),
            position: 0,
            looppoint: 0,
            loopcount: 0,

            scriptdelay: 0,
            running: false,

            //Textbox stuff
            textx: 0,
            texty: 0,
            r: 0,
            g: 0,
            b: 0,
            textflipme: false,

            //Misc
            i: 0,
            j: 0,
            k: 0,

            //Custom level stuff
            // std::vector<Script> customscripts;
        }
    }

    // void scriptclass::clearcustom(void)
    // void scriptclass::tokenize( const std::string& t )
    fn tokenize(&mut self, t: &str) {
        // println!("tokenizing: {:?}", t);

        self.j = 0;
        let mut tempword = String::new();

        for (_i, currentletter) in t.chars().enumerate() {
            if (currentletter == '(') | (currentletter == ')') | (currentletter == ',') {
                self.words[self.j as usize] = tempword;
                self.j += 1;
                tempword = String::new();
            } else if currentletter == ' ' {
                //don't do anything - i.e. strip out spaces.
            } else {
                tempword.push(currentletter.to_ascii_lowercase());
            }

            if self.j >= self.words.len() as i32 {
                break;
            }
        }

        if tempword != "" && self.j < self.words.len() as i32 {
            self.words[self.j as usize] = tempword;
        }

        // println!("tokenize result: {:?}", self.words);
        println!("tokenize result: {:?}", self.words.iter().filter(|x| x.len() > 0).collect::<Vec<_>>());
    }

    // void scriptclass::run(void)
    pub fn run(&mut self, game: &mut game::Game, obj: &mut entity::EntityClass, map: &mut map::Map, graphics: &mut graphics::Graphics, help: &mut utility_class::UtilityClass, music: &mut music::Music, key: &mut key_poll::KeyPoll) -> Option<RenderResult> {
        if !self.running {
            return None;
        }

        // This counter here will stop the function when it gets too high
        let mut execution_counter: u8 = 0;
        while self.running && self.scriptdelay <= 0 && !game.pausescript {
            if INBOUNDS_VEC!(self.position, self.commands) {
                //Let's split or command in an array of words
                let command: &str = &self.commands[self.position].to_owned();
                self.tokenize(command);

                //For script assisted input
                game.press_left = false;
                game.press_right = false;
                game.press_action = false;
                game.press_map = false;

                //Ok, now we run a command based on that string
                if self.words[0] == "moveplayer" {
                    //USAGE: moveplayer(x offset, y offset)
                    let player = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(player, obj.entities) {
                        obj.entities[player].xp += utility_class::ss_toi(&self.words[1]);
                        obj.entities[player].yp += utility_class::ss_toi(&self.words[2]);
                        obj.entities[player].lerpoldxp = obj.entities[player].xp;
                        obj.entities[player].lerpoldyp = obj.entities[player].yp;
                    }
                    self.scriptdelay = 1;
                }
                // // #if !defined(NO_CUSTOM_LEVELS)
                // if self.words[0] == "warpdir" {
                //     let temprx = utility_class::ss_toi(&self.words[1])-1;
                //     let tempry = utility_class::ss_toi(&self.words[2])-1;
                //     ed.setroomwarpdir(temprx, tempry, utility_class::ss_toi(&self.words[3]));
                //     let room = ed.getroomprop(temprx, tempry);

                //     //Do we update our own room?
                //     if game.roomx-100 == temprx && game.roomy-100 == tempry {
                //         //If screen warping, then override all that:
                //         graphics.backgrounddrawn = false;
                //         map.warpx=false;
                //         map.warpy=false;

                //         if room.warpdir == 0 {
                //             map.background = 1;
                //             //Be careful, we could be in a Lab or Warp Zone room...
                //             if room.tileset == 2 {
                //                 //Lab
                //                 map.background = 2;
                //                 graphics.rcol = room.tilecol;
                //             } else if room.tileset == 3 {
                //                 //Warp Zone
                //                 map.background = 6;
                //             }
                //         } else if room.warpdir == 1 {
                //             map.warpx = true;
                //             map.background = 3;
                //             graphics.rcol = ed.getwarpbackground(temprx, tempry);
                //         } else if room.warpdir == 2 {
                //             map.warpy = true;
                //             map.background = 4;
                //             graphics.rcol = ed.getwarpbackground(temprx, tempry);
                //         } else if room.warpdir==3 {
                //             map.warpx = true;
                //             map.warpy=true;
                //             map.background = 5;
                //             graphics.rcol = ed.getwarpbackground(temprx, tempry);
                //         }
                //     }
                // }
                // if self.words[0] == "ifwarp" {
                //     let room = ed.getroomprop(utility_class::ss_toi(&self.words[1])-1, utility_class::ss_toi(&self.words[2])-1);
                //     if room.warpdir == utility_class::ss_toi(&self.words[3]) {
                //         scripts::load(self, "custom_" + self.words[4]);
                //         self.position -= 1;
                //     }
                // }
                // // #endif
                if self.words[0] == "destroy" {
                    if self.words[1] == "gravitylines" {
                        // for(size_t edi = 0; edi<obj.entities.size(); edi++ {
                        for edi in 0..obj.entities.len() {
                            if obj.entities[edi].r#type == 9 {
                                obj.disableentity(edi);
                            }
                            if obj.entities[edi].r#type == 10 {
                                obj.disableentity(edi);
                            }
                        }
                    } else if self.words[1] == "warptokens" {
                        for edi in 0..obj.entities.len() {
                            if obj.entities[edi].r#type == 11 {
                                obj.disableentity(edi);
                            }
                        }
                    } else if self.words[1] == "platforms" {
                        for edi in 0..obj.entities.len() {
                            if obj.entities[edi].rule == 2 && obj.entities[edi].animate == 100 {
                                obj.disableentity(edi);
                            }
                        }
                    }
                }
                if self.words[0] == "customiftrinkets" {
                    if game.trinkets(obj) >= utility_class::ss_toi(&self.words[1]) {
                        scripts::load(self, &("custom_".to_owned() + &self.words[2]));
                        self.position -= 1;
                    }
                }
                if self.words[0] == "customiftrinketsless" {
                    if game.trinkets(obj) < utility_class::ss_toi(&self.words[1]) {
                        scripts::load(self, &("custom_".to_owned() + &self.words[2]));
                        self.position -= 1;
                    }
                } else if self.words[0] == "customifflag" {
                    let flag = utility_class::ss_toi(&self.words[1]) as usize;
                    if INBOUNDS_ARR!(flag, obj.flags) && obj.flags[flag] {
                        scripts::load(self, &("custom_".to_owned() + &self.words[2]));
                        self.position -= 1;
                    }
                }
                if self.words[0] == "custommap" {
                    if self.words[1] == "on" {
                        map.customshowmm=true;
                    } else if self.words[1] == "off" {
                        map.customshowmm=false;
                    }
                }
                if self.words[0] == "delay" {
                    //USAGE: delay(frames)
                    self.scriptdelay = utility_class::ss_toi(&self.words[1]);
                }
                if self.words[0] == "flag" {
                    let flag = utility_class::ss_toi(&self.words[1]) as usize;
                    if INBOUNDS_ARR!(flag, obj.flags) {
                        if self.words[2] == "on" {
                            obj.flags[flag] = true;
                        } else if self.words[2] == "off" {
                            obj.flags[flag] = false;
                        }
                    }
                }
                if self.words[0] == "flash" {
                    //USAGE: flash(frames)
                    game.flashlight = utility_class::ss_toi(&self.words[1]);
                }
                if self.words[0] == "shake" {
                    //USAGE: shake(frames)
                    game.screenshake = utility_class::ss_toi(&self.words[1]);
                }
                if self.words[0] == "walk" {
                    //USAGE: walk(dir,frames)
                    if self.words[1] == "left" {
                        game.press_left = true;
                    } else if self.words[1] == "right" {
                        game.press_right = true;
                    }
                    self.scriptdelay = utility_class::ss_toi(&self.words[2]);
                }
                if self.words[0] == "flip" {
                    game.press_action = true;
                    self.scriptdelay = 1;
                }
                if self.words[0] == "tofloor" {
                    let player = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(player, obj.entities) && obj.entities[player].onroof > 0 {
                        game.press_action = true;
                        self.scriptdelay = 1;
                    }
                }
                if self.words[0] == "playef" {
                    music.playef(utility_class::ss_toi(&self.words[1]));
                }
                if self.words[0] == "play" {
                    music.play(utility_class::ss_toi(&self.words[1]));
                }
                if self.words[0] == "stopmusic" {
                    music.haltdasmusik();
                }
                if self.words[0] == "resumemusic" {
                    music.resumefade(0);
                }
                if self.words[0] == "musicfadeout" {
                    music.fadeout(Some(false));
                }
                if self.words[0] == "musicfadein" {
                    music.fadein();
                }
                if self.words[0] == "trinketscriptmusic" {
                    music.play(4);
                }
                if self.words[0] == "gotoposition" {
                    //USAGE: gotoposition(x position, y position, gravity position)
                    let player = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(player, obj.entities) {
                        obj.entities[player].xp = utility_class::ss_toi(&self.words[1]);
                        obj.entities[player].yp = utility_class::ss_toi(&self.words[2]);
                        obj.entities[player].lerpoldxp = obj.entities[player].xp;
                        obj.entities[player].lerpoldyp = obj.entities[player].yp;
                    }
                    game.gravitycontrol = utility_class::ss_toi(&self.words[3]);

                }
                if self.words[0] == "gotoroom" {
                    //USAGE: gotoroom(x,y) (manually add 100)
                    map.gotoroom(utility_class::ss_toi(&self.words[1])+100, utility_class::ss_toi(&self.words[2])+100, game, graphics, music, obj, help);
                }
                if self.words[0] == "cutscene" {
                    graphics.showcutscenebars = true;
                }
                if self.words[0] == "endcutscene" {
                    graphics.showcutscenebars = false;
                }
                if self.words[0] == "untilbars" {
                    if graphics.showcutscenebars {
                        if graphics.cutscenebarspos < 360 {
                            self.scriptdelay = 1;
                            self.position -= 1;
                        }
                    } else {
                        if graphics.cutscenebarspos > 0 {
                            self.scriptdelay = 1;
                            self.position -= 1;
                        }
                    }
                } else if self.words[0] == "text" {
                    //oh boy
                    //first word is the colour.
                    if self.words[1] == "cyan" {
                        self.r = 164;
                        self.g = 164;
                        self.b = 255;
                    } else if self.words[1] == "player" {
                        self.r = 164;
                        self.g = 164;
                        self.b = 255;
                    } else if self.words[1] == "red" {
                        self.r = 255;
                        self.g = 60;
                        self.b = 60;
                    } else if self.words[1] == "green" {
                        self.r = 144;
                        self.g = 255;
                        self.b = 144;
                    } else if self.words[1] == "yellow" {
                        self.r = 255;
                        self.g = 255;
                        self.b = 134;
                    } else if self.words[1] == "blue" {
                        self.r = 95;
                        self.g = 95;
                        self.b = 255;
                    } else if self.words[1] == "purple" {
                        self.r = 255;
                        self.g = 134;
                        self.b = 255;
                    } else if self.words[1] == "white" {
                        self.r = 244;
                        self.g = 244;
                        self.b = 244;
                    } else if self.words[1] == "gray" {
                        self.r = 174;
                        self.g = 174;
                        self.b = 174;
                    } else if self.words[1] == "orange" {
                        self.r = 255;
                        self.g = 130;
                        self.b = 20;
                    } else {
                        //use a gray
                        self.r = 174;
                        self.g = 174;
                        self.b = 174;
                    }

                    //next are the x,y coordinates
                    self.textx = utility_class::ss_toi(&self.words[2]);
                    self.texty = utility_class::ss_toi(&self.words[3]);

                    //Number of lines for the textbox!
                    self.txt.clear();
                    // for (int self.i = 0; self.i < utility_class::ss_toi(&self.words[4]); self.i++ {
                    for i in 0..utility_class::ss_toi(&self.words[4]) {
                        self.position += 1;
                        if INBOUNDS_VEC!(self.position, self.commands) {
                            let command = self.commands[self.position].to_owned();
                            self.txt.push(command);
                        }
                    }
                } else if self.words[0] == "position" {
                    //are we facing left or right? for some objects we don't care, default at 0.
                    self.j = 0;

                    //the first word is the object to position relative to
                    match self.words[1].as_str() {
                        "player" => {
                            self.i = obj.getplayer();
                            if INBOUNDS_VEC!(self.i, obj.entities) {
                                self.j = obj.entities[self.i as usize].dir;
                            }
                        },
                        "cyan" => {
                            self.i = obj.getcrewman(0);
                            self.j = obj.entities[self.i as usize].dir;
                        },
                        "purple" => {
                            self.i = obj.getcrewman(1);
                            self.j = obj.entities[self.i as usize].dir;
                        },
                        "yellow" => {
                            self.i = obj.getcrewman(2);
                            self.j = obj.entities[self.i as usize].dir;
                        },
                        "red" => {
                            self.i = obj.getcrewman(3);
                            self.j = obj.entities[self.i as usize].dir;
                        },
                        "green" => {
                            self.i = obj.getcrewman(4);
                            self.j = obj.entities[self.i as usize].dir;
                        },
                        "blue" => {
                            self.i = obj.getcrewman(5);
                            self.j = obj.entities[self.i as usize].dir;
                        },
                        "centerx" => {
                            self.words[2] = "donothing".to_string();
                            self.j = -1;
                            self.textx = -500;
                        },
                        "centery" => {
                            self.words[2] = "donothing".to_string();
                            self.j = -1;
                            self.texty = -500;
                        },
                        "center" => {
                            self.words[2] = "donothing".to_string();
                            self.j = -1;
                            self.textx = -500;
                            self.texty = -500;
                        },
                        _ => (),
                    }

                    //next is whether to position above or below
                    if INBOUNDS_VEC!(self.i, obj.entities) && self.words[2] == "above" {
                        if self.j == 1 {
                            //left
                            self.textx = obj.entities[self.i as usize].xp -10000; //tells the box to be oriented correctly later
                            self.texty = obj.entities[self.i as usize].yp - 16 - (self.txt.len()*8) as i32;
                        } else if self.j == 0 {
                            //Right
                            self.textx = obj.entities[self.i as usize].xp - 16;
                            self.texty = obj.entities[self.i as usize].yp - 18 - (self.txt.len() * 8) as i32;
                        }
                    } else if INBOUNDS_VEC!(self.i, obj.entities) {
                        if self.j == 1 {
                            //left
                            self.textx = obj.entities[self.i as usize].xp -10000; //tells the box to be oriented correctly later
                            self.texty = obj.entities[self.i as usize].yp + 26;
                        } else if self.j == 0 {
                            //Right
                            self.textx = obj.entities[self.i as usize].xp - 16;
                            self.texty = obj.entities[self.i as usize].yp + 26;
                        }
                    }
                } else if self.words[0] == "customposition" {
                    //are we facing left or right? for some objects we don't care, default at 0.
                    self.j = 0;

                    //the first word is the object to position relative to
                    if self.words[1] == "player" {
                        self.i = obj.getcustomcrewman(0);
                        self.j = obj.entities[self.i as usize].dir;
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcustomcrewman(0);
                        self.j = obj.entities[self.i as usize].dir;
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcustomcrewman(1);
                        self.j = obj.entities[self.i as usize].dir;
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcustomcrewman(2);
                        self.j = obj.entities[self.i as usize].dir;
                    } else if self.words[1] == "red" {
                        self.i = obj.getcustomcrewman(3);
                        self.j = obj.entities[self.i as usize].dir;
                    } else if self.words[1] == "green" {
                        self.i = obj.getcustomcrewman(4);
                        self.j = obj.entities[self.i as usize].dir;
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcustomcrewman(5);
                        self.j = obj.entities[self.i as usize].dir;
                    } else if self.words[1] == "centerx" {
                        self.words[2] = "donothing".to_string();
                        self.j = -1;
                        self.textx = -500;
                    } else if self.words[1] == "centery" {
                        self.words[2] = "donothing".to_string();
                        self.j = -1;
                        self.texty = -500;
                    } else if self.words[1] == "center" {
                        self.words[2] = "donothing".to_string();
                        self.j = -1;
                        self.textx = -500;
                        self.texty = -500;
                    }

                    if self.i==0 && self.words[1] != "player" && self.words[1] != "cyan" {
                        //Requested crewmate is not actually on screen
                        self.words[2] = "donothing".to_string();
                        self.j = -1;
                        self.textx = -500;
                        self.texty = -500;
                    }

                    //next is whether to position above or below
                    if INBOUNDS_VEC!(self.i, obj.entities) && self.words[2] == "above" {
                        if self.j == 1 {
                            //left
                            self.textx = obj.entities[self.i as usize].xp -10000; //tells the box to be oriented correctly later
                            self.texty = obj.entities[self.i as usize].yp - 16 - (self.txt.len()*8) as i32;
                        } else if self.j == 0 {
                            //Right
                            self.textx = obj.entities[self.i as usize].xp - 16;
                            self.texty = obj.entities[self.i as usize].yp - 18 - (self.txt.len() * 8) as i32;
                        }
                    } else if INBOUNDS_VEC!(self.i, obj.entities) {
                        if self.j == 1 {
                            //left
                            self.textx = obj.entities[self.i as usize].xp -10000; //tells the box to be oriented correctly later
                            self.texty = obj.entities[self.i as usize].yp + 26;
                        } else if self.j == 0 {
                            //Right
                            self.textx = obj.entities[self.i as usize].xp - 16;
                            self.texty = obj.entities[self.i as usize].yp + 26;
                        }
                    }
                } else if self.words[0] == "backgroundtext" {
                    game.backgroundtext = true;
                } else if self.words[0] == "flipme" {
                    self.textflipme = !self.textflipme;
                } else if (self.words[0] == "speak_active") | (self.words[0] == "speak") {
                    //Ok, actually display the textbox we've initilised now!
                    //If using "speak", don't make the textbox active (so we can use multiple textboxes)
                    if self.txt.is_empty() {
                        self.txt.resize(1, String::new());
                    }
                    graphics.createtextboxreal(&self.txt[0], self.textx, self.texty, self.r, self.g, self.b, self.textflipme);
                    self.textflipme = false;
                    if self.txt.len() > 1 {
                        // for (self.i = 1; self.i < (int) self.txt.len(); self.i++ {
                        for i in 1..self.txt.len() {
                            graphics.addline(&self.txt[self.i as usize]);
                        }
                    }

                    //the textbox cannot be outside the screen. Fix if it is.
                    if self.textx <= -1000 {
                        //position to the left of the player
                        self.textx += 10000;
                        self.textx -= graphics.textboxwidth();
                        self.textx += 16;
                        graphics.textboxmoveto(self.textx);
                    }

                    if (self.textx == -500) | (self.textx == -1) {
                        graphics.textboxcenterx();
                    }

                    if self.texty == -500 {
                        graphics.textboxcentery();
                    }

                    graphics.textboxadjust();
                    if self.words[0] == "speak_active" {
                        graphics.textboxactive();
                    }

                    if !game.backgroundtext {
                        game.advancetext = true;
                        game.hascontrol = false;
                        game.pausescript = true;
                        if key.isDown(90) | key.isDown(32) | key.isDown(86) | key.isDownKeycode(Keycode::Up) | key.isDownKeycode(Keycode::Down) {
                            game.jumpheld = true;
                        }
                    }
                    game.backgroundtext = false;
                } else if self.words[0] == "endtext" {
                    graphics.textboxremove();
                    game.hascontrol = true;
                    game.advancetext = false;
                } else if self.words[0] == "endtextfast" {
                    graphics.textboxremovefast();
                    game.hascontrol = true;
                    game.advancetext = false;
                } else if self.words[0] == "do" {
                    //right, loop from this point
                    self.looppoint = self.position;
                    self.loopcount = utility_class::ss_toi(&self.words[1]);
                } else if self.words[0] == "loop" {
                    //right, loop from this point
                    self.loopcount -= 1;
                    if self.loopcount > 0 {
                        self.position = self.looppoint;
                    }
                } else if self.words[0] == "vvvvvvman" {
                    //Create the super VVVVVV combo!
                    self.i = obj.getplayer();
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].xp = 30;
                        obj.entities[self.i as usize].yp = 46;
                        obj.entities[self.i as usize].size = 13;
                        obj.entities[self.i as usize].colour = 23;
                        obj.entities[self.i as usize].cx = 36;// 6;
                        obj.entities[self.i as usize].cy = 12+80;// 2;
                        obj.entities[self.i as usize].h = 126-80;// 21;
                    }
                } else if self.words[0] == "undovvvvvvman" {
                    //Create the super VVVVVV combo!
                    self.i = obj.getplayer();
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].xp = 100;
                        obj.entities[self.i as usize].size = 0;
                        obj.entities[self.i as usize].colour = 0;
                        obj.entities[self.i as usize].cx = 6;
                        obj.entities[self.i as usize].cy = 2;
                        obj.entities[self.i as usize].h = 21;
                    }
                } else if self.words[0] == "createentity" {
                    let word6 = self.words[6].to_owned();
                    let word7 = self.words[7].to_owned();
                    let word8 = self.words[8].to_owned();
                    let word9 = self.words[9].to_owned();
                    if self.words[6] == "" { self.words[6] = "0".to_string(); }
                    if self.words[7] == "" { self.words[7] = "0".to_string(); }
                    if self.words[8] == "" { self.words[8] = "320".to_string(); }
                    if self.words[9] == "" { self.words[9] = "240".to_string(); }
                    obj.createentity(
                        utility_class::ss_toi(&self.words[1]),
                        utility_class::ss_toi(&self.words[2]),
                        utility_class::ss_toi(&self.words[3]),
                        Some(utility_class::ss_toi(&self.words[4])),
                        Some(utility_class::ss_toi(&self.words[5])),
                        Some(utility_class::ss_toi(&self.words[6])),
                        Some(utility_class::ss_toi(&self.words[7])),
                        Some(utility_class::ss_toi(&self.words[8])),
                        Some(utility_class::ss_toi(&self.words[9])),
                        game
                    );
                    self.words[6] = word6;
                    self.words[7] = word7;
                    self.words[8] = word8;
                    self.words[9] = word9;
                } else if self.words[0] == "createcrewman" {
                    self.r = match self.words[3].as_str() {
                        "cyan" => 0,
                        "red" => 15,
                        "green" => 13,
                        "yellow" => 14,
                        "blue" => 16,
                        "purple" => 20,
                        "gray" => 19,
                        _ => 19,
                    };

                    //convert the command to the right index
                    if self.words[5] == "followplayer" { self.words[5] = "10".to_string(); }
                    if self.words[5] == "followpurple" { self.words[5] = "11".to_string(); }
                    if self.words[5] == "followyellow" { self.words[5] = "12".to_string(); }
                    if self.words[5] == "followred" { self.words[5] = "13".to_string(); }
                    if self.words[5] == "followgreen" { self.words[5] = "14".to_string(); }
                    if self.words[5] == "followblue" { self.words[5] = "15".to_string(); }

                    if self.words[5] == "followposition" { self.words[5] = "16".to_string(); }
                    if self.words[5] == "faceleft" {
                        self.words[5] = "17".to_string();
                        self.words[6] = "0".to_string();
                    }
                    if self.words[5] == "faceright" {
                        self.words[5] = "17".to_string();
                        self.words[6] = "1".to_string();
                    }
                    if self.words[5] == "faceplayer" {
                        self.words[5] = "18".to_string();
                        self.words[6] = "0".to_string();
                    }
                    if self.words[5] == "panic" {
                        self.words[5] = "20".to_string();
                        self.words[6] = "0".to_string();
                    }

                    if utility_class::ss_toi(&self.words[5]) >= 16 {
                        obj.createentity(
                            utility_class::ss_toi(&self.words[1]),
                            utility_class::ss_toi(&self.words[2]),
                            18,
                            Some(self.r),
                            Some(utility_class::ss_toi(&self.words[4])),
                            Some(utility_class::ss_toi(&self.words[5])),
                            Some(utility_class::ss_toi(&self.words[6])),
                            None, None, game
                        );
                    } else {
                        obj.createentity(
                            utility_class::ss_toi(&self.words[1]),
                            utility_class::ss_toi(&self.words[2]),
                            18,
                            Some(self.r),
                            Some(utility_class::ss_toi(&self.words[4])),
                            Some(utility_class::ss_toi(&self.words[5])),
                            None, None, None, game
                        );
                    }
                } else if self.words[0] == "changemood" {
                    if self.words[1] == "player" {
                        self.i = obj.getplayer();
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcrewman(0);
                    } else if self.words[1] == "customcyan" {
                        self.i = obj.getcustomcrewman(0);
                    } else if self.words[1] == "red" {
                        self.i = obj.getcrewman(3);
                    } else if self.words[1] == "green" {
                        self.i = obj.getcrewman(4);
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcrewman(2);
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcrewman(5);
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcrewman(1);
                    } else if self.words[1] == "pink" {
                        self.i = obj.getcrewman(1);
                    }

                    if INBOUNDS_VEC!(self.i, obj.entities) && utility_class::ss_toi(&self.words[2]) == 0 {
                        obj.entities[self.i as usize].tile = 0;
                    } else if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].tile = 144;
                    }
                } else if self.words[0] == "changecustommood" {
                    if self.words[1] == "player" {
                        self.i = obj.getcustomcrewman(0);
                        obj.customcrewmoods[0] = utility_class::ss_toi(&self.words[2]) != 0;
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcustomcrewman(0);
                        obj.customcrewmoods[0] = utility_class::ss_toi(&self.words[2]) != 0;
                    } else if self.words[1] == "customcyan" {
                        self.i = obj.getcustomcrewman(0);
                        obj.customcrewmoods[0] = utility_class::ss_toi(&self.words[2]) != 0;
                    } else if self.words[1] == "red" {
                        self.i = obj.getcustomcrewman(3);
                        obj.customcrewmoods[3] = utility_class::ss_toi(&self.words[2]) != 0;
                    } else if self.words[1] == "green" {
                        self.i = obj.getcustomcrewman(4);
                        obj.customcrewmoods[4] = utility_class::ss_toi(&self.words[2]) != 0;
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcustomcrewman(2);
                        obj.customcrewmoods[2] = utility_class::ss_toi(&self.words[2]) != 0;
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcustomcrewman(5);
                        obj.customcrewmoods[5] = utility_class::ss_toi(&self.words[2]) != 0;
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcustomcrewman(1);
                        obj.customcrewmoods[1] = utility_class::ss_toi(&self.words[2]) != 0;
                    } else if self.words[1] == "pink" {
                        self.i = obj.getcustomcrewman(1);
                        obj.customcrewmoods[1] = utility_class::ss_toi(&self.words[2]) != 0;
                    }

                    if INBOUNDS_VEC!(self.i, obj.entities) && utility_class::ss_toi(&self.words[2]) == 0 {
                        obj.entities[self.i as usize].tile = 0;
                    } else if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].tile = 144;
                    }
                } else if self.words[0] == "changetile" {
                    if self.words[1] == "player" {
                        self.i = obj.getplayer();
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcrewman(0);
                    } else if self.words[1] == "red" {
                        self.i = obj.getcrewman(3);
                    } else if self.words[1] == "green" {
                        self.i = obj.getcrewman(4);
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcrewman(2);
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcrewman(5);
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcrewman(1);
                    }

                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].tile = utility_class::ss_toi(&self.words[2]);
                    }
                } else if self.words[0] == "flipgravity" {
                    //not something I'll use a lot, I think. Doesn't need to be very robust!
                    if self.words[1] == "player" {
                        game.gravitycontrol = !game.gravitycontrol;
                    } else {
                        if self.words[1] == "cyan" {
                            self.i = obj.getcrewman(0);
                        } else if self.words[1] == "red" {
                            self.i = obj.getcrewman(3);
                        } else if self.words[1] == "green" {
                            self.i = obj.getcrewman(4);
                        } else if self.words[1] == "yellow" {
                            self.i = obj.getcrewman(2);
                        } else if self.words[1] == "blue" {
                            self.i = obj.getcrewman(5);
                        } else if self.words[1] == "purple" {
                            self.i = obj.getcrewman(1);
                        }

                        if INBOUNDS_VEC!(self.i, obj.entities) && obj.entities[self.i as usize].rule == 7 {
                            obj.entities[self.i as usize].rule = 6;
                            obj.entities[self.i as usize].tile = 0;
                        } else if INBOUNDS_VEC!(self.i, obj.entities) && obj.getplayer() != self.i {
                            // Don't destroy player entity
                            obj.entities[self.i as usize].rule = 7;
                            obj.entities[self.i as usize].tile = 6;
                        }
                    }
                } else if self.words[0] == "changegravity" {
                    //not something I'll use a lot, I think. Doesn't need to be very robust!
                    if self.words[1] == "player" {
                        self.i = obj.getplayer();
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcrewman(0);
                    } else if self.words[1] == "red" {
                        self.i = obj.getcrewman(3);
                    } else if self.words[1] == "green" {
                        self.i = obj.getcrewman(4);
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcrewman(2);
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcrewman(5);
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcrewman(1);
                    }

                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].tile +=12;
                    }
                } else if self.words[0] == "changedir" {
                    if self.words[1] == "player" {
                        self.i = obj.getplayer();
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcrewman(0);
                    } else if self.words[1] == "red" {
                        self.i = obj.getcrewman(3);
                    } else if self.words[1] == "green" {
                        self.i = obj.getcrewman(4);
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcrewman(2);
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcrewman(5);
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcrewman(1);
                    }

                    if INBOUNDS_VEC!(self.i, obj.entities) && utility_class::ss_toi(&self.words[2]) == 0 {
                        obj.entities[self.i as usize].dir = 0;
                    } else if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].dir = 1;
                    }
                } else if self.words[0] == "alarmon" {
                    game.alarmon = true;
                    game.alarmdelay = 0;
                } else if self.words[0] == "alarmoff" {
                    game.alarmon = false;
                } else if self.words[0] == "changeai" {
                    if self.words[1] == "player" {
                        self.i = obj.getplayer();
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcrewman(0);
                    } else if self.words[1] == "red" {
                        self.i = obj.getcrewman(3);
                    } else if self.words[1] == "green" {
                        self.i = obj.getcrewman(4);
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcrewman(2);
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcrewman(5);
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcrewman(1);
                    }

                    if self.words[2] == "followplayer" { self.words[2] = "10".to_string(); }
                    if self.words[2] == "followpurple" { self.words[2] = "11".to_string(); }
                    if self.words[2] == "followyellow" { self.words[2] = "12".to_string(); }
                    if self.words[2] == "followred" { self.words[2] = "13".to_string(); }
                    if self.words[2] == "followgreen" { self.words[2] = "14".to_string(); }
                    if self.words[2] == "followblue" { self.words[2] = "15".to_string(); }

                    if self.words[2] == "followposition" { self.words[2] = "16".to_string(); }
                    if self.words[2] == "faceleft" {
                        self.words[2] = "17".to_string();
                        self.words[3] = "0".to_string();
                    }
                    if self.words[2] == "faceright" {
                        self.words[2] = "17".to_string();
                        self.words[3] = "1".to_string();
                    }

                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].state = utility_class::ss_toi(&self.words[2]);
                        if obj.entities[self.i as usize].state == 16 {
                            obj.entities[self.i as usize].para = utility_class::ss_toi(&self.words[3]) as f32;
                        } else if obj.entities[self.i as usize].state == 17 {
                            obj.entities[self.i as usize].dir = utility_class::ss_toi(&self.words[3]);
                        }
                    }
                } else if self.words[0] == "activateteleporter" {
                    self.i = obj.getteleporter();
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].tile = 6;
                        obj.entities[self.i as usize].colour = 102;
                    }
                } else if self.words[0] == "changecolour" {
                    if self.words[1] == "player" {
                        self.i = obj.getplayer();
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcrewman(0);
                    } else if self.words[1] == "red" {
                        self.i = obj.getcrewman(3);
                    } else if self.words[1] == "green" {
                        self.i = obj.getcrewman(4);
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcrewman(2);
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcrewman(5);
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcrewman(1);
                    }

                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        if self.words[2] == "cyan" {
                            obj.entities[self.i as usize].colour = 0;
                        } else if self.words[2] == "red" {
                            obj.entities[self.i as usize].colour = 15;
                        } else if self.words[2] == "green" {
                            obj.entities[self.i as usize].colour = 13;
                        } else if self.words[2] == "yellow" {
                            obj.entities[self.i as usize].colour = 14;
                        } else if self.words[2] == "blue" {
                            obj.entities[self.i as usize].colour = 16;
                        } else if self.words[2] == "purple" {
                            obj.entities[self.i as usize].colour = 20;
                        } else if self.words[2] == "teleporter" {
                            obj.entities[self.i as usize].colour = 102;
                        }
                    }
                } else if self.words[0] == "squeak" {
                    if self.words[1] == "player" {
                        music.playef(11);
                    } else if self.words[1] == "cyan" {
                        music.playef(11);
                    } else if self.words[1] == "red" {
                        music.playef(16);
                    } else if self.words[1] == "green" {
                        music.playef(12);
                    } else if self.words[1] == "yellow" {
                        music.playef(14);
                    } else if self.words[1] == "blue" {
                        music.playef(13);
                    } else if self.words[1] == "purple" {
                        music.playef(15);
                    } else if self.words[1] == "cry" {
                        music.playef(2);
                    } else if self.words[1] == "terminal" {
                        music.playef(20);
                    }
                } else if self.words[0] == "blackout" {
                    game.blackout = true;
                } else if self.words[0] == "blackon" {
                    game.blackout = false;
                } else if self.words[0] == "setcheckpoint" {
                    self.i = obj.getplayer();
                    game.savepoint = 0;
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        game.savex = obj.entities[self.i as usize].xp ;
                        game.savey = obj.entities[self.i as usize].yp;
                    }
                    game.savegc = game.gravitycontrol;
                    game.saverx = game.roomx;
                    game.savery = game.roomy;
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        game.savedir = obj.entities[self.i as usize].dir;
                    }
                } else if self.words[0] == "gamestate" {
                    game.state = utility_class::ss_toi(&self.words[1]);
                    game.statedelay = 0;
                } else if self.words[0] == "textboxactive" {
                    graphics.textboxactive();
                } else if self.words[0] == "gamemode" {
                    if self.words[1] == "teleporter" {
                        game.mapmenuchange(GameState::TELEPORTERMODE, graphics, map);

                        game.useteleporter = false; //good heavens don't actually use it
                    } else if self.words[1] == "game" {
                        graphics.resumegamemode = true;
                        game.prevgamestate = GameState::GAMEMODE;
                    }
                } else if self.words[0] == "ifexplored" {
                    if map.isexplored(utility_class::ss_toi(&self.words[1]), utility_class::ss_toi(&self.words[2])) {
                        scripts::load(self, &self.words[3].to_owned());
                        self.position -= 1;
                    }
                } else if self.words[0] == "iflast" {
                    if game.lastsaved == utility_class::ss_toi(&self.words[1]) {
                        scripts::load(self, &self.words[2].to_owned());
                        self.position -= 1;
                    }
                } else if self.words[0] == "ifskip" {
                    if game.nocutscenes {
                        scripts::load(self, &self.words[1].to_owned());
                        self.position -= 1;
                    }
                } else if self.words[0] == "ifflag" {
                    let flag = utility_class::ss_toi(&self.words[1]) as usize;
                    if INBOUNDS_ARR!(flag, obj.flags) && obj.flags[flag] {
                        scripts::load(self, &self.words[2].to_owned());
                        self.position -= 1;
                    }
                } else if self.words[0] == "ifcrewlost" {
                    let crewmate = utility_class::ss_toi(&self.words[1]) as usize;
                    if INBOUNDS_ARR!(crewmate, game.crewstats) && !game.crewstats[crewmate] {
                        scripts::load(self, &self.words[2].to_owned());
                        self.position -= 1;
                    }
                } else if self.words[0] == "iftrinkets" {
                    if game.trinkets(obj) >= utility_class::ss_toi(&self.words[1]) {
                        scripts::load(self, &self.words[2].to_owned());
                        self.position -= 1;
                    }
                } else if self.words[0] == "iftrinketsless" {
                    if game.stat_trinkets < utility_class::ss_toi(&self.words[1]) {
                        scripts::load(self, &self.words[2].to_owned());
                        self.position -= 1;
                    }
                } else if self.words[0] == "hidecoordinates" {
                    map.setexplored(utility_class::ss_toi(&self.words[1]), utility_class::ss_toi(&self.words[2]), false);
                } else if self.words[0] == "showcoordinates" {
                    map.setexplored(utility_class::ss_toi(&self.words[1]), utility_class::ss_toi(&self.words[2]), true);
                } else if self.words[0] == "hideship" {
                    map.hideship();
                } else if self.words[0] == "showship" {
                    map.showship();
                } else if self.words[0] == "showsecretlab" {
                    map.setexplored(16, 5, true);
                    map.setexplored(17, 5, true);
                    map.setexplored(18, 5, true);
                    map.setexplored(17, 6, true);
                    map.setexplored(18, 6, true);
                    map.setexplored(19, 6, true);
                    map.setexplored(19, 7, true);
                    map.setexplored(19, 8, true);
                } else if self.words[0] == "hidesecretlab" {
                    map.setexplored(16, 5, false);
                    map.setexplored(17, 5, false);
                    map.setexplored(18, 5, false);
                    map.setexplored(17, 6, false);
                    map.setexplored(18, 6, false);
                    map.setexplored(19, 6, false);
                    map.setexplored(19, 7, false);
                    map.setexplored(19, 8, false);
                } else if self.words[0] == "showteleporters" {
                    map.showteleporters = true;
                } else if self.words[0] == "showtargets" {
                    map.showtargets = true;
                } else if self.words[0] == "showtrinkets" {
                    map.showtrinkets = true;
                } else if self.words[0] == "hideteleporters" {
                    map.showteleporters = false;
                } else if self.words[0] == "hidetargets" {
                    map.showtargets = false;
                } else if self.words[0] == "hidetrinkets" {
                    map.showtrinkets = false;
                } else if self.words[0] == "hideplayer" {
                    let player = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(player, obj.entities) {
                        obj.entities[player].invis = true;
                    }
                } else if self.words[0] == "showplayer" {
                    let player = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(player, obj.entities) {
                        obj.entities[player].invis = false;
                    }
                } else if self.words[0] == "teleportscript" {
                    game.teleportscript = self.words[1].to_owned();
                } else if self.words[0] == "clearteleportscript" {
                    game.teleportscript = "".into();
                } else if self.words[0] == "nocontrol" {
                    game.hascontrol = false;
                } else if self.words[0] == "hascontrol" {
                    game.hascontrol = true;
                } else if self.words[0] == "companion" {
                    game.companion = utility_class::ss_toi(&self.words[1]);
                } else if self.words[0] == "befadein" {
                    graphics.setfade(0);
                    graphics.fademode= 0;
                } else if self.words[0] == "fadein" {
                    graphics.fademode = 4;
                } else if self.words[0] == "fadeout" {
                    graphics.fademode = 2;
                } else if self.words[0] == "untilfade" {
                    if graphics.fademode>1 {
                        self.scriptdelay = 1;
                        self.position -= 1;
                    }
                } else if self.words[0] == "entersecretlab" {
                    game.unlocknum(8);
                    game.insecretlab = true;
                } else if self.words[0] == "leavesecretlab" {
                    game.insecretlab = false;
                } else if self.words[0] == "resetgame" {
                    map.resetnames();
                    map.resetmap();
                    map.resetplayer(None);
                    graphics.buffers.towerbg.tdrawback = true;

                    obj.resetallflags();
                    self.i = obj.getplayer();
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].tile = 0;
                    }

                    for i in 0..100 {
                        obj.collect[self.i as usize] = false;
                        obj.customcollect[self.i as usize] = false;
                    }
                    game.deathcounts = 0;
                    game.advancetext = false;
                    game.hascontrol = true;
                    game.resetgameclock();
                    game.gravitycontrol = 0;
                    game.teleport = false;
                    game.companion = 0;
                    game.roomchange = false;
                    game.teleport_to_new_area = false;
                    game.teleport_to_x = 0;
                    game.teleport_to_y = 0;

                    game.teleportscript = "".to_string();

                    //get out of final level mode!
                    map.finalmode = false;
                    map.final_colormode = false;
                    map.final_mapcol = 0;
                    map.final_colorframe = 0;
                    map.finalstretch = false;
                } else if self.words[0] == "loadscript" {
                    scripts::load(self, &self.words[1].to_owned());
                    self.position -= 1;
                } else if self.words[0] == "rollcredits" {
                    game.gamestate = GameState::GAMECOMPLETE;
                    graphics.fademode = 4;
                    game.creditposition = 0;
                } else if self.words[0] == "finalmode" {
                    map.finalmode = true;
                    map.gotoroom(utility_class::ss_toi(&self.words[1]), utility_class::ss_toi(&self.words[2]), game, graphics, music, obj, help);
                } else if self.words[0] == "rescued" {
                    if self.words[1] == "red" {
                        game.crewstats[3] = true;
                    } else if self.words[1] == "green" {
                        game.crewstats[4] = true;
                    } else if self.words[1] == "yellow" {
                        game.crewstats[2] = true;
                    } else if self.words[1] == "blue" {
                        game.crewstats[5] = true;
                    } else if self.words[1] == "purple" {
                        game.crewstats[1] = true;
                    } else if self.words[1] == "player" {
                        game.crewstats[0] = true;
                    } else if self.words[1] == "cyan" {
                        game.crewstats[0] = true;
                    }
                } else if self.words[0] == "missing" {
                    if self.words[1] == "red" {
                        game.crewstats[3] = false;
                    } else if self.words[1] == "green" {
                        game.crewstats[4] = false;
                    } else if self.words[1] == "yellow" {
                        game.crewstats[2] = false;
                    } else if self.words[1] == "blue" {
                        game.crewstats[5] = false;
                    } else if self.words[1] == "purple" {
                        game.crewstats[1] = false;
                    } else if self.words[1] == "player" {
                        game.crewstats[0] = false;
                    } else if self.words[1] == "cyan" {
                        game.crewstats[0] = false;
                    }
                } else if self.words[0] == "face" {
                    if self.words[1] == "player" {
                        self.i = obj.getplayer();
                    } else if self.words[1] == "cyan" {
                        self.i = obj.getcrewman(0);
                    } else if self.words[1] == "red" {
                        self.i = obj.getcrewman(3);
                    } else if self.words[1] == "green" {
                        self.i = obj.getcrewman(4);
                    } else if self.words[1] == "yellow" {
                        self.i = obj.getcrewman(2);
                    } else if self.words[1] == "blue" {
                        self.i = obj.getcrewman(5);
                    } else if self.words[1] == "purple" {
                        self.i = obj.getcrewman(1);
                    }

                    self.j = match self.words[2].as_str() {
                        "player" => obj.getplayer(),
                        "cyan" => obj.getcrewman(0),
                        "red" => obj.getcrewman(3),
                        "green" => obj.getcrewman(4),
                        "yellow" => obj.getcrewman(2),
                        "blue" => obj.getcrewman(5),
                        "purple" => obj.getcrewman(1),
                        _ => self.j,
                    };

                    if INBOUNDS_VEC!(self.i, obj.entities) && INBOUNDS_VEC!(self.j, obj.entities) && obj.entities[self.j as usize].xp > obj.entities[self.i as usize].xp + 5 {
                        obj.entities[self.i as usize].dir = 1;
                    } else if INBOUNDS_VEC!(self.i, obj.entities) && INBOUNDS_VEC!(self.j, obj.entities) && obj.entities[self.j as usize].xp < obj.entities[self.i as usize].xp - 5 {
                        obj.entities[self.i as usize].dir = 0;
                    }
                } else if self.words[0] == "jukebox" {
                    // for (j = 0; j < (int) obj.entities.size(); j++ {
                    for j in 0..obj.entities.len() {
                        if obj.entities[j].r#type == 13 {
                            obj.entities[j].colour = 4;
                        }
                    }
                    if utility_class::ss_toi(&self.words[1]) == 1 {
                        obj.createblock(5, 88 - 4, 80, 20, 16, Some(25), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 88 && obj.entities[j].yp == 80 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 2 {
                        obj.createblock(5, 128 - 4, 80, 20, 16, Some(26), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 128 && obj.entities[j].yp == 80 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 3 {
                        obj.createblock(5, 176 - 4, 80, 20, 16, Some(27), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 176 && obj.entities[j].yp == 80 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 4 {
                        obj.createblock(5, 216 - 4, 80, 20, 16, Some(28), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 216 && obj.entities[j].yp == 80 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 5 {
                        obj.createblock(5, 88 - 4, 128, 20, 16, Some(29), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 88 && obj.entities[j].yp == 128 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 6 {
                        obj.createblock(5, 176 - 4, 128, 20, 16, Some(30), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 176 && obj.entities[j].yp == 128 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 7 {
                        obj.createblock(5, 40 - 4, 40, 20, 16, Some(31), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 40 && obj.entities[j].yp == 40 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 8 {
                        obj.createblock(5, 216 - 4, 128, 20, 16, Some(32), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 216 && obj.entities[j].yp == 128 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 9 {
                        obj.createblock(5, 128 - 4, 128, 20, 16, Some(33), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 128 && obj.entities[j].yp == 128 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    } else if utility_class::ss_toi(&self.words[1]) == 10 {
                        obj.createblock(5, 264 - 4, 40, 20, 16, Some(34), None);
                        // for (j = 0; j < (int) obj.entities.size(); j++ {
                        for j in 0..obj.entities.len() {
                            if obj.entities[j].xp == 264 && obj.entities[j].yp == 40 {
                                obj.entities[j].colour = 5;
                            }
                        }
                    }
                } else if self.words[0] == "createactivityzone" {
                    if self.words[1] == "red" {
                        self.i = 3;
                    } else if self.words[1] == "green" {
                        self.i = 4;
                    } else if self.words[1] == "yellow" {
                        self.i = 2;
                    } else if self.words[1] == "blue" {
                        self.i = 5;
                    } else if self.words[1] == "purple" {
                        self.i = 1;
                    }

                    let crewman = obj.getcrewman(self.i) as usize;
                    if INBOUNDS_VEC!(crewman, obj.entities) && self.i == 4 {
                        obj.createblock(5, obj.entities[crewman].xp - 32, obj.entities[crewman].yp-20, 96, 60, Some(self.i), None);
                    } else if INBOUNDS_VEC!(crewman, obj.entities) {
                        obj.createblock(5, obj.entities[crewman].xp - 32, 0, 96, 240, Some(self.i), None);
                    }
                } else if self.words[0] == "createrescuedcrew" {
                    //special for final level cutscene
                    //starting at 180, create the rescued crewmembers (ingoring violet, who's at 155)
                    self.i = 215;
                    if game.crewstats[2] && game.lastsaved != 2 {
                        obj.createentity(self.i, 153, 18, Some(14), Some(0), Some(17), Some(0), None, None, game);
                        self.i += 25;
                    }
                    if game.crewstats[3] && game.lastsaved != 3 {
                        obj.createentity(self.i, 153, 18, Some(15), Some(0), Some(17), Some(0), None, None, game);
                        self.i += 25;
                    }
                    if game.crewstats[4] && game.lastsaved != 4 {
                        obj.createentity(self.i, 153, 18, Some(13), Some(0), Some(17), Some(0), None, None, game);
                        self.i += 25;
                    }
                    if game.crewstats[5] && game.lastsaved != 5 {
                        obj.createentity(self.i, 153, 18, Some(16), Some(0), Some(17), Some(0), None, None, game);
                        self.i += 25;
                    }
                } else if self.words[0] == "restoreplayercolour" {
                    self.i = obj.getplayer();
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].colour = 0;
                    }
                } else if self.words[0] == "changeplayercolour" {
                    self.i = obj.getplayer();

                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        if self.words[1] == "cyan" {
                            obj.entities[self.i as usize].colour = 0;
                        } else if self.words[1] == "red" {
                            obj.entities[self.i as usize].colour = 15;
                        } else if self.words[1] == "green" {
                            obj.entities[self.i as usize].colour = 13;
                        } else if self.words[1] == "yellow" {
                            obj.entities[self.i as usize].colour = 14;
                        } else if self.words[1] == "blue" {
                            obj.entities[self.i as usize].colour = 16;
                        } else if self.words[1] == "purple" {
                            obj.entities[self.i as usize].colour = 20;
                        } else if self.words[1] == "teleporter" {
                            obj.entities[self.i as usize].colour = 102;
                        }
                    }
                } else if self.words[0] == "altstates" {
                    obj.altstates = utility_class::ss_toi(&self.words[1]);
                } else if self.words[0] == "activeteleporter" {
                    self.i = obj.getteleporter();
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].colour = 101;
                    }
                } else if self.words[0] == "foundtrinket" {
                    music.silencedasmusik();
                    music.playef(3);

                    let trinket = utility_class::ss_toi(&self.words[1]) as usize;
                    if trinket < obj.collect.len() {
                        obj.collect[trinket] = true;
                    }

                    graphics.textboxremovefast();

                    graphics.createtextboxflipme("        Congratulations!       ", 50, 85, 174, 174, 174);
                    graphics.addline("");
                    graphics.addline("You have found a shiny trinket!");
                    graphics.textboxcenterx();

                    let usethisnum;
                    // #if !defined(NO_CUSTOM_LEVELS)
                    // if map.custommode {
                    //     usethisnum = help.number(ed.numtrinkets());
                    // } else {
                    // #endif
                        usethisnum = "Twenty";
                    // }
                    graphics.createtextboxflipme(&format!(" {} out of {} ", help.number(game.trinkets(obj)), usethisnum), 50, 135, 174, 174, 174);
                    graphics.textboxcenterx();

                    if !game.backgroundtext {
                        game.advancetext = true;
                        game.hascontrol = false;
                        game.pausescript = true;
                        if key.isDown(90) | key.isDown(32) | key.isDown(86) | key.isDownKeycode(Keycode::Up) | key.isDownKeycode(Keycode::Down) {
                            game.jumpheld = true;
                        }
                    }
                    game.backgroundtext = false;
                } else if self.words[0] == "foundlab" {
                    music.playef(3);

                    graphics.textboxremovefast();

                    graphics.createtextbox("        Congratulations!       ", 50, 85, 174, 174, 174);
                    graphics.addline("");
                    graphics.addline("You have found the secret lab!");
                    graphics.textboxcenterx();
                    graphics.textboxcentery();

                    if !game.backgroundtext {
                        game.advancetext = true;
                        game.hascontrol = false;
                        game.pausescript = true;
                        if key.isDown(90) | key.isDown(32) | key.isDown(86) | key.isDownKeycode(Keycode::Up) | key.isDownKeycode(Keycode::Down) {
                            game.jumpheld = true;
                        }
                    }
                    game.backgroundtext = false;
                } else if self.words[0] == "foundlab2" {
                    graphics.textboxremovefast();

                    graphics.createtextbox("The secret lab is separate from", 50, 85, 174, 174, 174);
                    graphics.addline("the rest of the game. You can");
                    graphics.addline("now come back here at any time");
                    graphics.addline("by selecting the new SECRET LAB");
                    graphics.addline("option in the play menu.");
                    graphics.textboxcenterx();
                    graphics.textboxcentery();

                    if !game.backgroundtext {
                        game.advancetext = true;
                        game.hascontrol = false;
                        game.pausescript = true;
                        if key.isDown(90) | key.isDown(32) | key.isDown(86) | key.isDownKeycode(Keycode::Up) | key.isDownKeycode(Keycode::Down) {
                            game.jumpheld = true;
                        }
                    }
                    game.backgroundtext = false;
                } else if self.words[0] == "everybodysad" {
                    // for (self.i = 0; self.i < (int) obj.entities.size(); self.i++ {
                    for i in 0..obj.entities.len() {
                        if (obj.entities[self.i as usize].rule == 6) | (obj.entities[self.i as usize].rule == 0) {
                            obj.entities[self.i as usize].tile = 144;
                        }
                    }
                } else if self.words[0] == "startintermission2" {
                    map.finalmode = true; //Enable final level mode

                    game.savex = 228;
                    game.savey = 129;
                    game.saverx = 53;
                    game.savery = 49;
                    game.savegc = 0;
                    game.savedir = 0; //Intermission level 2
                    game.savepoint = 0;
                    game.gravitycontrol = 0;

                    map.gotoroom(46, 54, game, graphics, music, obj, help);
                } else if self.words[0] == "telesave" {
                    if !game.intimetrial && !game.nodeathmode && !game.inintermission {
                        game.savetele();
                    }
                } else if self.words[0] == "createlastrescued" {
                    self.r = match game.lastsaved {
                        2 => 14,
                        3 => 15,
                        4 => 13,
                        5 => 16,
                        _ => 19,
                    };

                    obj.createentity(200, 153, 18, Some(self.r), Some(0), Some(19), Some(30), None, None, game);
                    self.i = obj.getcrewman(game.lastsaved);
                    if INBOUNDS_VEC!(self.i, obj.entities) {
                        obj.entities[self.i as usize].dir = 1;
                    }
                } else if self.words[0] == "specialline" {
                    match utility_class::ss_toi(&self.words[1]) {
                        1 => {
                            self.txt.resize(1, String::new());

                            self.txt[0] = format!("I'm worried about {}, Doctor!", game.unrescued());
                        },
                        2 => {
                            self.txt.resize(3, String::new());

                            if game.crewrescued() < 5 {
                                self.txt[1] = "to helping you find the".to_owned();
                                self.txt[2] = "rest of the crew!".to_owned();
                            } else {
                                self.txt.resize(2, String::new());
                                self.txt[1] = format!("to helping you find {}!", game.unrescued());
                            }
                        },
                        _ => (),
                    }
                } else if self.words[0] == "trinketbluecontrol" {
                    if game.trinkets(obj) == 20 && obj.flags[67] {
                        scripts::load(self, "talkblue_trinket6");
                        self.position -= 1;
                    } else if game.trinkets(obj) >= 19 && !obj.flags[67] {
                        scripts::load(self, "talkblue_trinket5");
                        self.position -= 1;
                    } else {
                        scripts::load(self, "talkblue_trinket4");
                        self.position -= 1;
                    }
                } else if self.words[0] == "trinketyellowcontrol" {
                    if game.trinkets(obj) >= 19 {
                        scripts::load(self, "talkyellow_trinket3");
                        self.position -= 1;
                    } else {
                        scripts::load(self, "talkyellow_trinket2");
                        self.position -= 1;
                    }
                } else if self.words[0] == "redcontrol" {
                    if game.insecretlab {
                        scripts::load(self, "talkred_14");
                        self.position -= 1;
                    } else if game.roomx != 104 {
                        if game.roomx == 100 {
                            scripts::load(self, "talkred_10");
                            self.position -= 1;
                        } else if game.roomx == 107 {
                            scripts::load(self, "talkred_11");
                            self.position -= 1;
                        } else if game.roomx == 114 {
                            scripts::load(self, "talkred_12");
                            self.position -= 1;
                        }
                    } else if obj.flags[67] {
                        //game complete
                        scripts::load(self, "talkred_13");
                        self.position -= 1;
                    } else if obj.flags[35] && !obj.flags[52] {
                        //Intermission level
                        obj.flags[52] = true;
                        scripts::load(self, "talkred_9");
                        self.position -= 1;
                    } else if !obj.flags[51] {
                        //We're back home!
                        obj.flags[51] = true;
                        scripts::load(self, "talkred_5");
                        self.position -= 1;
                    } else if !obj.flags[48] && game.crewstats[5] {
                        //Victoria's back
                        obj.flags[48] = true;
                        scripts::load(self, "talkred_6");
                        self.position -= 1;
                    } else if !obj.flags[49] && game.crewstats[4] {
                        //Verdigris' back
                        obj.flags[49] = true;
                        scripts::load(self, "talkred_7");
                        self.position -= 1;
                    } else if !obj.flags[50] && game.crewstats[2] {
                        //Vitellary's back
                        obj.flags[50] = true;
                        scripts::load(self, "talkred_8");
                        self.position -= 1;
                    } else if !obj.flags[45] && !game.crewstats[5] {
                        obj.flags[45] = true;
                        scripts::load(self, "talkred_2");
                        self.position -= 1;
                    } else if !obj.flags[46] && !game.crewstats[4] {
                        obj.flags[46] = true;
                        scripts::load(self, "talkred_3");
                        self.position -= 1;
                    } else if !obj.flags[47] && !game.crewstats[2] {
                        obj.flags[47] = true;
                        scripts::load(self, "talkred_4");
                        self.position -= 1;
                    } else {
                        obj.flags[45] = false;
                        obj.flags[46] = false;
                        obj.flags[47] = false;
                        scripts::load(self, "talkred_1");
                        self.position -= 1;
                    }
                }
                //TODO: Non Urgent fix compiler nesting errors without adding complexity
                if self.words[0] == "greencontrol" {
                    if game.insecretlab {
                        scripts::load(self, "talkgreen_11");
                        self.position -= 1;
                    } else if game.roomx == 103 && game.roomy == 109 {
                        scripts::load(self, "talkgreen_8");
                        self.position -= 1;
                    } else if game.roomx == 101 && game.roomy == 109 {
                        scripts::load(self, "talkgreen_9");
                        self.position -= 1;
                    } else if obj.flags[67] {
                        //game complete
                        scripts::load(self, "talkgreen_10");
                        self.position -= 1;
                    } else if obj.flags[34] && !obj.flags[57] {
                        //Intermission level
                        obj.flags[57] = true;
                        scripts::load(self, "talkgreen_7");
                        self.position -= 1;
                    } else if !obj.flags[53] {
                        //Home!
                        obj.flags[53] = true;
                        scripts::load(self, "talkgreen_6");
                        self.position -= 1;
                    } else if !obj.flags[54] && game.crewstats[2] {
                        obj.flags[54] = true;
                        scripts::load(self, "talkgreen_5");
                        self.position -= 1;
                    } else if !obj.flags[55] && game.crewstats[3] {
                        obj.flags[55] = true;
                        scripts::load(self, "talkgreen_4");
                        self.position -= 1;
                    } else if !obj.flags[56] && game.crewstats[5] {
                        obj.flags[56] = true;
                        scripts::load(self, "talkgreen_3");
                        self.position -= 1;
                    } else if !obj.flags[58] {
                        obj.flags[58] = true;
                        scripts::load(self, "talkgreen_2");
                        self.position -= 1;
                    } else {
                        scripts::load(self, "talkgreen_1");
                        self.position -= 1;
                    }
                } else if self.words[0] == "bluecontrol" {
                    if game.insecretlab {
                        scripts::load(self, "talkblue_9");
                        self.position -= 1;
                    } else if obj.flags[67] {
                        //game complete, everything changes for victoria
                        if obj.flags[41] && !obj.flags[42] {
                            //second trinket conversation
                            obj.flags[42] = true;
                            scripts::load(self, "talkblue_trinket2");
                            self.position -= 1;
                        } else if !obj.flags[41] && !obj.flags[42] {
                            //Third trinket conversation
                            obj.flags[42] = true;
                            scripts::load(self, "talkblue_trinket3");
                            self.position -= 1;
                        } else {
                            //Ok, we've already dealt with the trinket thing; so either you have them all, or you don't. If you do:
                            if game.trinkets(obj) >= 20 {
                                scripts::load(self, "startepilogue");
                                self.position -= 1;
                            } else {
                                scripts::load(self, "talkblue_8");
                                self.position -= 1;
                            }
                        }
                    } else if obj.flags[33] && !obj.flags[40] {
                        //Intermission level
                        obj.flags[40] = true;
                        scripts::load(self, "talkblue_7");
                        self.position -= 1;
                    } else if !obj.flags[36] && game.crewstats[5] {
                        //Back on the ship!
                        obj.flags[36] = true;
                        scripts::load(self, "talkblue_3");
                        self.position -= 1;
                    } else if !obj.flags[41] && game.crewrescued() <= 4 {
                        //First trinket conversation
                        obj.flags[41] = true;
                        scripts::load(self, "talkblue_trinket1");
                        self.position -= 1;
                    } else if obj.flags[41] && !obj.flags[42] && game.crewrescued() == 5 {
                        //second trinket conversation
                        obj.flags[42] = true;
                        scripts::load(self, "talkblue_trinket2");
                        self.position -= 1;
                    } else if !obj.flags[41] && !obj.flags[42] && game.crewrescued() == 5 {
                        //Third trinket conversation
                        obj.flags[42] = true;
                        scripts::load(self, "talkblue_trinket3");
                        self.position -= 1;
                    } else if !obj.flags[37] && game.crewstats[2] {
                        obj.flags[37] = true;
                        scripts::load(self, "talkblue_4");
                        self.position -= 1;
                    } else if !obj.flags[38] && game.crewstats[3] {
                        obj.flags[38] = true;
                        scripts::load(self, "talkblue_5");
                        self.position -= 1;
                    } else if !obj.flags[39] && game.crewstats[4] {
                        obj.flags[39] = true;
                        scripts::load(self, "talkblue_6");
                        self.position -= 1;
                    } else {
                        //if all else fails:
                        //if yellow is found
                        if game.crewstats[2] {
                            scripts::load(self, "talkblue_2");
                            self.position -= 1;
                        } else {
                            scripts::load(self, "talkblue_1");
                            self.position -= 1;
                        }
                    }
                } else if self.words[0] == "yellowcontrol" {
                    if game.insecretlab {
                        scripts::load(self, "talkyellow_12");
                        self.position -= 1;
                    } else if obj.flags[67] {
                        //game complete
                        scripts::load(self, "talkyellow_11");
                        self.position -= 1;
                    } else if obj.flags[32] && !obj.flags[31] {
                        //Intermission level
                        obj.flags[31] = true;
                        scripts::load(self, "talkyellow_6");
                        self.position -= 1;
                    } else if !obj.flags[27] && game.crewstats[2] {
                        //Back on the ship!
                        obj.flags[27] = true;
                        scripts::load(self, "talkyellow_10");
                        self.position -= 1;
                    } else if !obj.flags[43] && game.crewrescued() == 5 && !game.crewstats[5] {
                        //If by chance we've rescued everyone except Victoria by the end, Vitellary provides you with
                        //the trinket information instead.
                        obj.flags[43] = true;
                        obj.flags[42] = true;
                        obj.flags[41] = true;
                        scripts::load(self, "talkyellow_trinket1");
                        self.position -= 1;
                    } else if !obj.flags[24] && game.crewstats[5] {
                        obj.flags[24] = true;
                        scripts::load(self, "talkyellow_8");
                        self.position -= 1;
                    } else if !obj.flags[26] && game.crewstats[4] {
                        obj.flags[26] = true;
                        scripts::load(self, "talkyellow_7");
                        self.position -= 1;
                    } else if !obj.flags[25] && game.crewstats[3] {
                        obj.flags[25] = true;
                        scripts::load(self, "talkyellow_9");
                        self.position -= 1;
                    } else if !obj.flags[28] {
                        obj.flags[28] = true;
                        scripts::load(self, "talkyellow_3");
                        self.position -= 1;
                    } else if !obj.flags[29] {
                        obj.flags[29] = true;
                        scripts::load(self, "talkyellow_4");
                        self.position -= 1;
                    } else if !obj.flags[30] {
                        obj.flags[30] = true;
                        scripts::load(self, "talkyellow_5");
                        self.position -= 1;
                    } else if !obj.flags[23] {
                        obj.flags[23] = true;
                        scripts::load(self, "talkyellow_2");
                        self.position -= 1;
                    } else {
                        scripts::load(self, "talkyellow_1");
                        self.position -= 1;
                        obj.flags[23] = false;
                    }
                } else if self.words[0] == "purplecontrol" {
                    //Controls Purple's conversion
                    //Crew rescued:
                    if game.insecretlab {
                        scripts::load(self, "talkpurple_9");
                        self.position -= 1;
                    } else if obj.flags[67] {
                        //game complete
                        scripts::load(self, "talkpurple_8");
                        self.position -= 1;
                    } else if !obj.flags[17] && game.crewstats[4] {
                        obj.flags[17] = true;
                        scripts::load(self, "talkpurple_6");
                        self.position -= 1;
                    } else if !obj.flags[15] && game.crewstats[5] {
                        obj.flags[15] = true;
                        scripts::load(self, "talkpurple_4");
                        self.position -= 1;
                    } else if !obj.flags[16] && game.crewstats[3] {
                        obj.flags[16] = true;
                        scripts::load(self, "talkpurple_5");
                        self.position -= 1;
                    } else if !obj.flags[18] && game.crewstats[2] {
                        obj.flags[18] = true;
                        scripts::load(self, "talkpurple_7");
                        self.position -= 1;
                    } else if obj.flags[19] && !obj.flags[20] && !obj.flags[21] {
                        //intermission one: if played one / not had first conversation / not played two [conversation one]
                        obj.flags[21] = true;
                        scripts::load(self, "talkpurple_intermission1");
                        self.position -= 1;
                    } else if obj.flags[20] && obj.flags[21] && !obj.flags[22] {
                        //intermission two: if played two / had first conversation / not had second conversation [conversation two]
                        obj.flags[22] = true;
                        scripts::load(self, "talkpurple_intermission2");
                        self.position -= 1;
                    } else if obj.flags[20] && !obj.flags[21] && !obj.flags[22] {
                        //intermission two: if played two / not had first conversation / not had second conversation [conversation three]
                        obj.flags[22] = true;
                        scripts::load(self, "talkpurple_intermission3");
                        self.position -= 1;
                    } else if !obj.flags[12] {
                        //Intro conversation
                        obj.flags[12] = true;
                        scripts::load(self, "talkpurple_intro");
                        self.position -= 1;
                    } else if !obj.flags[14] {
                        //Shorter intro conversation
                        obj.flags[14] = true;
                        scripts::load(self, "talkpurple_3");
                        self.position -= 1;
                    } else {
                        //if all else fails:
                        //if green is found
                        if game.crewstats[4] {
                            scripts::load(self, "talkpurple_2");
                            self.position -= 1;
                        } else {
                            scripts::load(self, "talkpurple_1");
                            self.position -= 1;
                        }
                    }
                }

                self.position += 1;
            } else {
                self.running = false;
            }
            // Don't increment if we're at the max, signed int overflow is UB
            if execution_counter == u8::MAX {
                // We must be in an infinite loop
                println!("Warning: execution counter got to {}, stopping script", u8::MAX);
                self.running = false;
            } else {
                execution_counter += 1;
            }
        }

        if self.scriptdelay > 0 {
            self.scriptdelay -= 1;
        }

        None
    }

    // void scriptclass::resetgametomenu(void)
    pub fn resetgametomenu(&self) {
        println!("DEADBEEF: scriptclass::resetgametomenu() method not implemented yet");
    }

    // void scriptclass::startgamemode( int t )
    pub fn startgamemode(&mut self, t: i32, game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, obj: &mut entity::EntityClass, music: &mut music::Music, help: &mut utility_class::UtilityClass) {
        match t {
        0 => {
            game.gamestate = GameState::GAMEMODE;
            self.hardreset(game, map, graphics, obj);
            game.start(music);
            game.jumpheld = true;
            graphics.showcutscenebars = true;
            graphics.setbars(320);

            //set flipmode
            if graphics.setflipmode {
                graphics.flipmode = true;
            } else {
                obj.flags[73] = true;
            }

            if obj.entities.is_empty() {
                //In this game, constant, never destroyed
                obj.createentity(game.savex, game.savey, 0, Some(0), None, None, None, None, None, game);
            } else {
                map.resetplayer(None);
            }
            map.gotoroom(game.saverx, game.savery, game, graphics, music, obj, help);
            map.initmapdata();

            scripts::load(self, "intro");
        },
        1..=19 | 100 => println!("gamemode {} not implemented yet", t),
        _ => println!("incorrect game mode"),
        }
    }

    // void scriptclass::teleport(void)
    pub fn teleport(&self) {
        println!("DEADBEEF: scriptclass::teleport() method not implemented yet");
    }

    // void scriptclass::hardreset(void)
    fn hardreset(&mut self, game: &mut game::Game, map: &mut map::Map, graphics: &mut graphics::Graphics, obj: &mut entity::EntityClass) {
        //Game:
        game.hascontrol = true;
        game.gravitycontrol = 0;
        game.teleport = false;
        game.companion = 0;
        game.roomchange = false;
        if !game.glitchrunnermode {
            // Ironically, resetting more variables makes the janky fadeout system in glitchrunnermode even more glitchy
            game.roomx = 0;
            game.roomy = 0;
        }
        game.prevroomx = 0;
        game.prevroomy = 0;
        game.teleport_to_new_area = false;
        game.teleport_to_x = 0;
        game.teleport_to_y = 0;
        game.teleportscript = "".to_string();

        game.tapleft = 0;
        game.tapright = 0;
        game.startscript = false;
        game.newscript = "".to_string();
        game.alarmon = false;
        game.alarmdelay = 0;
        game.blackout = false;
        game.useteleporter = false;
        game.teleport_to_teleporter = 0;

        game.nodeathmode = false;
        game.nocutscenes = false;

        for mut crewstat in game.crewstats.iter() {
            crewstat = &false;
        }
        game.crewstats[0] = true;
        game.lastsaved = 0;

        game.deathcounts = 0;
        game.gameoverdelay = 0;
        game.resetgameclock();
        game.gamesaved = false;
        game.gamesavefailed = false;
        game.savetime = "00:00".to_string();
        game.savearea = "nowhere".to_string();
        game.savetrinkets = 0;
        if !game.glitchrunnermode {
            // Ironically, resetting more variables makes the janky fadeout system in glitchrunnermode even more glitchy
            game.saverx = 0;
            game.savery = 0;
        }

        game.intimetrial = false;
        game.timetrialcountdown = 0;
        game.timetrialshinytarget = 0;
        game.timetrialparlost = false;
        game.timetrialpar = 0;

        game.totalflips = 0;
        game.hardestroom = "Welcome Aboard".to_string();
        game.hardestroomdeaths = 0;
        game.currentroomdeaths = 0;

        game.swnmode = false;
        game.swntimer = 0;
        game.swngame = 0;//Not playing sine wave ninja!
        game.swnstate = 0;
        game.swnstate2 = 0;
        game.swnstate3 = 0;
        game.swnstate4 = 0;
        game.swndelay = 0;
        game.swndeaths = 0;
        game.supercrewmate = false;
        game.scmhurt = false;
        game.scmprogress = 0;
        game.scmmoveme = false;
        game.swncolstate = 0;
        game.swncoldelay = 0;
        game.swnrank = 0;
        game.swnmessage = 0;
        game.creditposx = 0;
        game.creditposy = 0;
        game.creditposdelay = 0;

        game.inintermission = false;
        game.insecretlab = false;

        game.state = 0;
        game.statedelay = 0;

        game.hascontrol = true;
        if !game.glitchrunnermode {
            // Keep the "- Press ACTION to advance text -" prompt around,
            // apparently the speedrunners call it the "text storage" glitch
            game.advancetext = false;
        }

        game.pausescript = false;
        game.completestop = false;

        game.flashlight = 0;
        game.screenshake = 0;

        game.activeactivity = -1;
        game.act_fade = 5;

        //dwgraphicsclass
        graphics.backgrounddrawn = false;
        graphics.textbox.clear();
        graphics.flipmode = false; //This will be reset if needs be elsewhere
        graphics.showcutscenebars = false;
        graphics.setbars(0);

        //mapclass
        map.warpx = false;
        map.warpy = false;
        map.showteleporters = false;
        map.showtargets = false;
        map.showtrinkets = false;
        map.finalmode = false;
        map.finalstretch = false;
        map.final_colormode = false;
        map.final_colorframe = 0;
        map.final_colorframedelay = 0;
        map.final_mapcol = 0;
        map.final_aniframe = 0;
        map.final_aniframedelay = 0;
        map.rcol = 0;
        map.resetnames();
        map.custommode = false;
        map.custommodeforreal = false;
        if !game.glitchrunnermode {
            // Ironically, resetting more variables makes the janky fadeout system even more glitchy
            map.towermode = false;
        }
        map.cameraseekframe = 0;
        map.resumedelay = 0;
        graphics.buffers.towerbg.scrolldir = 0;
        map.customshowmm = true;

        map.roomdeaths = [0i32; 20*20];
        map.roomdeathsfinal = [0i32; 20*20];
        map.resetmap();
        //entityclass
        obj.nearelephant = false;
        obj.upsetmode = false;
        obj.upset = 0;

        obj.trophytext = 0 ;
        obj.trophytype = 0;
        obj.altstates = 0;

        obj.resetallflags();

        for mut customcrewmood in obj.customcrewmoods.iter() {
            customcrewmood = &true;
        }

        obj.collect = [false; 100];
        obj.customcollect = [false; 100];
        self.i = 100; //previously a for-loop iterating over collect/customcollect set this to 100

        let theplayer = obj.getplayer() as usize;
        match obj.entities.get_mut(theplayer) {
            Some(v) => {
                v.tile = 0;
            },
            None => (),
        };

        /* Disable duplicate player entities */
        // for int self.i = 0; self.i < (int) obj.entities.size(); self.i++ {
        //     if obj.entities[self.i as usize].rule == 0 && self.i != theplayer {
        //         obj.disableentity(self.i);
        //     }
        // }
        // for (self.i, mut entity) in obj.entities.iter().enumerate() {
        //     let self.i = self.i as i32;
        //     if entity.rule == 0 && self.i != theplayer {
        //         obj.disableentity(self.i);
        //     }
        // }
        println!("DEADBEEF: Disable duplicate player entities not implemented yet");

        //Script Stuff
        self.position = 0;
        self.commands.clear();
        self.scriptdelay = 0;
        self.scriptname = "null".to_string();
        self.running = false;
    }

    // void scriptclass::loadcustom(const std::string& t)
    fn loadcustom(&self, t: &str) {
        println!("DEADBEEF: scriptclass::loadcustom() method not implemented yet");
    }

    fn filllines(&mut self, lines: Vec<&str>) {
        for line in lines {
            self.commands.push(line.to_string());
        }
    }

}
