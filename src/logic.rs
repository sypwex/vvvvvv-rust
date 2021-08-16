use crate::{INBOUNDS_VEC, entity, filesystem, game, map, maths, music, scenes::RenderResult, screen::{self, render::graphics, renderfixed}, script, utility_class};


pub fn titlelogic(map: &mut map::Map, music: &mut music::Music, game: &mut game::Game, renderfixed: &mut renderfixed::RenderFixed, graphics: &mut graphics::Graphics, screen_params: screen::ScreenParams, screen_settings: screen::ScreenSettings, fs: &mut filesystem::FileSystem) -> Result<Option<RenderResult>, i32> {
    //Misc
    //map.updatetowerglow(&mut graphics.buffers.titlebg);
    renderfixed.update_glow();

    graphics.buffers.titlebg.bypos -= 2;
    graphics.buffers.titlebg.bscroll = -2;

    if game.menucountdown > 0 {
        game.menucountdown -= 1;

        if game.menucountdown == 0 {
            if game.menudest == game::MenuName::mainmenu {
                music.play(6, map, game);
            } else if game.menudest == game::MenuName::gameover2 {
                music.playef(11);
            } else if game.menudest == game::MenuName::timetrialcomplete3 {
                music.playef(3);
            }

            game.createmenu(game.menudest, Some(true), graphics, music, screen_params, map, screen_settings, fs);
        }
    }

    Ok(None)
}

pub fn maplogic(help: &mut utility_class::UtilityClass) -> Result<Option<RenderResult>, i32> {
    //Misc
    help.updateglow();

    Ok(None)
}

pub fn gamecompletelogic() -> Result<Option<RenderResult>, i32> {
    Ok(None)
}

pub fn gamecompletelogic2() -> Result<Option<RenderResult>, i32> {
    Ok(None)
}

pub fn gamelogic(game: &mut game::Game, graphics: &mut graphics::Graphics, map: &mut map::Map, music: &mut music::Music, obj: &mut entity::EntityClass , help: &mut utility_class::UtilityClass, script: &mut script::ScriptClass, screen_params: screen::ScreenParams, fs: &mut filesystem::FileSystem, screen_settings: screen::ScreenSettings) -> Result<Option<RenderResult>, i32> {
    /* Update old lerp positions of entities */
    for i in 0..obj.entities.len() {
        obj.entities[i].lerpoldxp = obj.entities[i].xp;
        obj.entities[i].lerpoldyp = obj.entities[i].yp;
    }

    if !game.blackout && !game.completestop {
        // for (i = 0; i < obj.entities.size(); ++i {
        for i in 0..obj.entities.len() {
            /* Is this entity on the ground? (needed for jumping) */
            if obj.entitycollidefloor(i, map, help) {
                obj.entities[i].onground = 2;
            } else {
                obj.entities[i].onground -= 1;
            }

            if obj.entitycollideroof(i, map, help) {
                obj.entities[i].onroof = 2;
            } else {
                obj.entities[i].onroof -= 1;
            }
        }
    }

    //Misc
    if map.towermode {
        map.updatetowerglow(&mut graphics.buffers.towerbg);
    }
    help.updateglow();

    if game.alarmon {
        game.alarmdelay -= 1;
        if game.alarmdelay <= 0 {
            music.playef(19);
            game.alarmdelay = 20;
        }
    }

    if obj.nearelephant {
        obj.upset += 1;
        if obj.upset == 300 {
            obj.upsetmode = true;
            //change player to sad
            let i = obj.getplayer() as usize;
            if INBOUNDS_VEC!(i, obj.entities) {
                obj.entities[i].tile = 144;
            }
            music.playef(2);
        }
        if obj.upset > 301 {
            obj.upset = 301;
        }
    } else if obj.upsetmode {
        obj.upset -= 1;
        if obj.upset <= 0 {
            obj.upset = 0;
            obj.upsetmode = false;
            //change player to happy
            let i = obj.getplayer() as usize;
            if INBOUNDS_VEC!(i, obj.entities) {
                obj.entities[i].tile = 0;
            }
        }
    } else {
        obj.upset = 0;
    }

    obj.oldtrophytext = obj.trophytext;

    if map.towermode {
        map.oldypos = map.ypos;
        map.oldspikeleveltop = map.spikeleveltop;
        map.oldspikelevelbottom = map.spikelevelbottom;
        if !game.completestop {
            if map.cameramode == 0 {
                //do nothing!
                //a trigger will set this off in the game
                map.cameramode = 1;
                graphics.buffers.towerbg.bscroll = 0;
            } else if map.cameramode == 1 {
                //move normally
                if graphics.buffers.towerbg.scrolldir == 0 {
                    map.ypos -= 2;
                    graphics.buffers.towerbg.bypos -= 1;
                    graphics.buffers.towerbg.bscroll = -1;
                } else {
                    map.ypos += 2;
                    graphics.buffers.towerbg.bypos += 1;
                    graphics.buffers.towerbg.bscroll = 1;
                }
            } else if map.cameramode == 2 {
                //do nothing, but cycle colours (for taking damage)
                graphics.buffers.towerbg.bscroll = 0;
            } else if map.cameramode == 4 {
                let i = obj.getplayer() as usize;
                if INBOUNDS_VEC!(i, obj.entities) {
                    map.cameraseek = map.ypos - (obj.entities[i].yp - 120);
                }

                map.cameraseek = map.cameraseek / 10;
                map.cameraseekframe = 10;

                map.cameramode = 5;

                graphics.buffers.towerbg.bscroll = map.cameraseek/2;
            } else if map.cameramode == 5 {
                //actually do it
                if map.spikeleveltop > 0 {
                    map.spikeleveltop -= 2;
                }
                if map.spikelevelbottom > 0 {
                    map.spikelevelbottom -= 2;
                }
                if map.cameraseekframe > 0 {
                    let i = obj.getplayer() as usize;
                    map.ypos -= map.cameraseek;
                    if map.cameraseek > 0 && INBOUNDS_VEC!(i, obj.entities) {
                        if map.ypos < obj.entities[i].yp - 120 {
                            map.ypos = obj.entities[i].yp - 120;
                        }
                    } else if INBOUNDS_VEC!(i, obj.entities) {
                        if map.ypos > obj.entities[i].yp - 120 {
                            map.ypos = obj.entities[i].yp - 120;
                        }
                    }
                    map.cameraseekframe -= 1;
                    graphics.buffers.towerbg.bypos = map.ypos / 2;
                } else {
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        map.ypos = obj.entities[i].yp - 120;
                    }
                    graphics.buffers.towerbg.bypos = map.ypos / 2;
                    map.cameramode = 0;
                    map.colsuperstate = 0;
                }
            }
        } else {
            graphics.buffers.towerbg.bscroll = 0;
        }

        if map.ypos <= 0 {
            map.ypos = 0;
            graphics.buffers.towerbg.bypos = 0;
            graphics.buffers.towerbg.bscroll = 0;
        }
        if map.towermode && map.minitowermode {
            if map.ypos >= 568 {
                map.ypos = 568;
                graphics.buffers.towerbg.bypos = map.ypos / 2;
                graphics.buffers.towerbg.bscroll = 0;
            } //100-29 * 8 = 568
        } else {
            if map.ypos >= 5368 {
                map.ypos = 5368;    //700-29 * 8 = 5368
                graphics.buffers.towerbg.bypos = map.ypos / 2;
            }
        }

        if game.lifeseq > 0 {
            if map.cameramode == 2 {
                map.cameraseekframe = 20;
                map.cameramode = 4;
                map.resumedelay = 4;
            }

            if map.cameraseekframe <= 0 {
                if map.resumedelay <= 0 {
                    game.lifesequence(obj);
                    if game.lifeseq == 0 {
                        map.cameramode = 1;
                    }
                } else {
                    map.resumedelay -= 1;
                }
            }
        }
    } else {
        game.lifesequence(obj);
    }

    graphics.kludgeswnlinewidth = false;

    if game.deathseq != -1 {
        if map.towermode {
            map.colsuperstate = 1;
            map.cameramode = 2;
        }

        // for (size_t i = 0; i < obj.entities.size(); i++ {
        for mut i in 0..obj.entities.len() {
            if game.roomx == 111 && game.roomy == 107 && !map.custommode {
                if obj.entities[i].r#type == 1 {
                    if obj.entities[i].xp < 152 {
                        //Move the platform to the right side of the disappearing platform,
                        //otherwise it will get stuck on the kludge 18,9 tile we placed
                        //(and if the tile wasn't there it would pass straight through again)
                        let prevx = obj.entities[i].xp;
                        let prevy = obj.entities[i].yp;
                        obj.disableblockat(prevx, prevy);

                        obj.entities[i].xp = 152;
                        obj.entities[i].newxp = 152.0;

                        obj.moveblockto(prevx, prevy, obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].w, obj.entities[i].h);
                    }
                }
            }
            if obj.entities[i].r#type == 2 && obj.entities[i].state == 3 {
                //Ok! super magical exception for the room with the intention death for the shiny trinket
                //fix this when the maps are finalised
                if game.roomx != 111 || game.roomy != 107 || map.custommode {
                    obj.entities[i].state = 4;
                } else {
                    obj.entities[i].state = 4;
                    map.settile(18, 9, 59);
                }
            } else if obj.entities[i].r#type == 2 && obj.entities[i].state == 2 {
                //ok, unfortunate where the disappearing platform hasn't fully disappeared. Accept a littl =>
                //graphical uglyness to avoid breaking the room!
                let mut entitygone = false;
                while obj.entities[i].state == 2 {
                    entitygone = obj.updateentities(i, game, map, music, script);
                    if entitygone {
                        i -= 1;
                        break;
                    }
                }
                if !entitygone {
                    obj.entities[i].state = 4;
                }
            } else if obj.entities[i].r#type == 23 && game.swnmode && game.deathseq < 15 {
                //if playing SWN, get the enemies offscreen.
                obj.entities[i].xp += obj.entities[i].vx as i32 * 5;
                obj.entities[i].yp += obj.entities[i].vy as i32 * 5;
            }
        }
        if game.swnmode {
            //if playing SWN game a, push the clock back to the nearest 10 second interval
            if game.swngame == 0 {
                game.swnpenalty();
            } else if game.swngame == 1 {
                game.swnstate = 0;
                game.swnstate2 = 0;
                game.swnstate3 = 0;
                game.swnstate4 = 0;
                game.swndelay = 0;
                if game.swntimer >= game.swnrecord {
                    game.swnrecord = game.swntimer;
                    if game.swnmessage == 0 {
                        music.playef(25);
                        game.savestatsandsettings(screen_settings, fs, music);
                    }
                    game.swnmessage = 1;
                }
            }
        }

        game.deathsequence(map, music, obj);
        game.deathseq -= 1;
        if game.deathseq <= 0 {
            if game.nodeathmode {
                game.deathseq = 1;
                game.gethardestroom(map);
                //start depressing sequence here...
                if game.gameoverdelay <= -10 && graphics.fademode == 0 {
                    graphics.fademode = 2;
                }
                if graphics.fademode == 1 {
                    game.copyndmresults();
                    script.resetgametomenu();
                }
            } else {
                if game.swnmode {
                    //if playing SWN game b, reset the clock
                    if game.swngame == 1 {
                        game.swntimer = 0;
                        game.swnmessage = 0;
                        game.swnrank = 0;
                    }
                }

                game.gethardestroom(map);
                game.hascontrol = true;

                game.gravitycontrol = game.savegc;
                graphics.textboxremove();
                map.resetplayer(Some(true));
            }
        }
    } else {
        //Update colour thingy
        if map.finalmode {
            if map.final_colormode {
                if map.final_colorframe > 0 {
                    map.final_colorframedelay -= 1;
                    if map.final_colorframedelay <= 0 {
                        if map.final_colorframe == 1 {
                            map.final_colorframedelay = 40;
                        } else if map.final_colorframe == 2 {
                            map.final_colorframedelay = 15;
                        }

                        let mut temp = 1 + maths::fRandom() as i32 * 6;
                        if temp == map.final_mapcol { temp = (temp + 1) % 6; }
                        if temp == 0 { temp = 6; }
                        map.changefinalcol(temp, obj);
                    }
                }
            }
        }
        //State machine for game logic
        game.updatestate(graphics, script, obj, music, map, screen_params, help, fs, screen_settings);
        if game.startscript {
            script::scripts::load(script, &game.newscript);
            game.startscript = false;
        }

        //Intermission 1 Logic
        //Player can't walk off a screen with SCM on it until they've left
        if game.supercrewmate {
            if game.roomx == 41 + game.scmprogress {
                //he's in the same room
                let i = obj.getplayer() as usize;
                if INBOUNDS_VEC!(i, obj.entities) && obj.entities[i].ax > 0.0 && obj.entities[i].xp > 280 {
                    obj.entities[i].ax = 0.0;
                    obj.entities[i].dir = 0;
                }
            }
        }

        //SWN Minigame Logic
        if game.swnmode {
            //which game?
            if game.swngame == 0 {
                //intermission, survive 60 seconds game
                game.swntimer -= 1;
                if game.swntimer <= 0 {
                    music.niceplay(8, game);
                    game.swngame = 5;
                } else {
                    obj.generateswnwave(0);
                }
            } else if game.swngame == 1 {
                //super gravitron game
                game.swntimer += 1;
                if game.swntimer > game.swnrecord {
                    game.swnrecord = game.swntimer;
                }

                if game.swntimer >= 150 && game.swnrank == 0 {
                    game.swnrank = 1;
                    if game.swnbestrank < 1 {
                        game.unlockAchievement("vvvvvvsupgrav5");
                        game.swnbestrank = 1;
                        game.swnmessage = 2+30;
                        music.playef(26);
                    }
                } else if game.swntimer >= 300 && game.swnrank == 1 {
                    game.swnrank = 2;
                    if game.swnbestrank < 2 {
                        game.unlockAchievement("vvvvvvsupgrav10");
                        game.swnbestrank = 2;
                        game.swnmessage = 2+30;
                        music.playef(26);
                    }
                } else if game.swntimer >= 450 && game.swnrank == 2 {
                    game.swnrank = 3;
                    if game.swnbestrank < 3 {
                        game.unlockAchievement("vvvvvvsupgrav15");
                        game.swnbestrank = 3;
                        game.swnmessage = 2+30;
                        music.playef(26);
                    }
                } else if game.swntimer >= 600 && game.swnrank == 3 {
                    game.swnrank = 4;
                    if game.swnbestrank < 4 {
                        game.unlockAchievement("vvvvvvsupgrav20");
                        game.swnbestrank = 4;
                        game.swnmessage = 2+30;
                        music.playef(26);
                    }
                } else if game.swntimer >= 900 && game.swnrank == 4 {
                    game.swnrank = 5;
                    if game.swnbestrank < 5 {
                        game.unlockAchievement("vvvvvvsupgrav30");
                        game.swnbestrank = 5;
                        game.swnmessage = 2+30;
                        music.playef(26);
                    }
                } else if game.swntimer >= 1800 && game.swnrank == 5 {
                    game.swnrank = 6;
                    if game.swnbestrank < 6 {
                        game.unlockAchievement("vvvvvvsupgrav60");
                        game.swnbestrank = 6;
                        game.swnmessage = 2+30;
                        music.playef(26);
                    }
                }

                obj.generateswnwave(1);

                game.swncoldelay -= 1;
                if game.swncoldelay <= 0 {
                    game.swncolstate = (game.swncolstate+1) % 6;
                    game.swncoldelay = 30;
                    graphics.rcol = game.swncolstate;
                    obj.swnenemiescol(game.swncolstate);
                }
            } else if game.swngame == 2 {
                //introduce game a
                game.swndelay -= 1;
                if game.swndelay <= 0 {
                    game.swngame = 0;
                    game.swndelay = 0;
                    game.swntimer = (60 * 30) - 1;
                    //game.swntimer = 15;
                }
            } else if game.swngame == 3 {
                //extend line
                let line = obj.getlineat(84 - 32);
                if INBOUNDS_VEC!(line, obj.entities) {
                    obj.entities[line as usize].w += 24;
                    if obj.entities[line as usize].w > 332 {
                        obj.entities[line as usize].w = 332;
                        game.swngame = 2;
                        graphics.kludgeswnlinewidth = true;
                    }
                }
            } else if game.swngame == 4 {
                //create top line
                game.swngame = 3;
                // (horizontal gravity line)
                obj.createentity(-8, 84 - 32, 11, Some(8), None, None, None, None, None, game);
                music.niceplay(2, game);
                game.swndeaths = game.deathcounts;
            } else if game.swngame == 5 {
                //remove line
                let line = obj.getlineat(148 + 32);
                if INBOUNDS_VEC!(line, obj.entities) {
                    obj.entities[line as usize].xp += 24;
                    if obj.entities[line as usize].xp > 320 {
                        obj.disableentity(line);
                        game.swngame = 8;
                    }
                }
            } else if game.swngame == 6 {
                //Init the super gravitron
                game.swngame = 7;
                music.niceplay(3, game);
            } else if game.swngame == 7 {
                //introduce game b
                game.swndelay -= 1;
                if game.swndelay <= 0 {
                    game.swngame = 1;
                    game.swndelay = 0;
                    game.swntimer = 0;
                    game.swncolstate = 3;
                    game.swncoldelay = 30;
                }
            } else if game.swngame == 8 {
                //extra kludge if player dies after game a ends
                let mut square_onscreen = false;
                // for (size_t i = 0; i < obj.entities.size(); i++ {
                for i in 0..obj.entities.len() {
                    if obj.entities[i].r#type == 23 {
                        square_onscreen = true;
                        break;
                    }
                }
                if !square_onscreen {
                    game.swnmode = false;
                }
            }
        }

        //Time trial stuff
        if game.intimetrial {

            if game.timetrialcountdown > 0 {
                game.hascontrol = true;
                game.timetrialcountdown -= 1;
                if game.timetrialcountdown > 30 {
                    game.hascontrol = false;
                }
                if game.timetrialcountdown == 120 { music.playef(21); }
                if game.timetrialcountdown == 90 { music.playef(21); }
                if game.timetrialcountdown == 60 { music.playef(21); }
                if game.timetrialcountdown == 30 {
                    match game.timetriallevel {
                        0 => music.play(1, map, game),
                        1 => music.play(3, map, game),
                        2 => music.play(2, map, game),
                        3 => music.play(1, map, game),
                        4 => music.play(12, map, game),
                        5 => music.play(15, map, game),
                        _ => music.playef(22),
                    };
                }
            }

            //Have we lost the par?
            if !game.timetrialparlost {
                if (game.minutes * 60) + game.seconds > game.timetrialpar {
                    game.timetrialparlost = true;
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].tile = 144;
                    }
                    music.playef(2);
                }
            }
        }

        //Update entities
        //Ok, moving platform fuckers
        if !game.completestop {
            if obj.vertplatforms {
                for i in (0..obj.entities.len()).rev() {
                    if !obj.entities[i].isplatform || (obj.entities[i].vx).abs() >= 0.000001f32 {
                        continue;
                    }

                    let prevx = obj.entities[i].xp;
                    let prevy = obj.entities[i].yp;
                    obj.disableblockat(prevx, prevy);

                    // Behavioral logic
                    let entitygone = obj.updateentities(i, game, map, music, script);
                    if entitygone {
                        continue;
                    }
                    // Basic Physics
                    obj.updateentitylogic(i, game);
                    // Collisions with walls
                    obj.entitymapcollision(i, map, help);

                    obj.moveblockto(prevx, prevy, obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].w, obj.entities[i].h);
                    let player = obj.getplayer() as usize;
                    obj.movingplatformfix(i, player, help, map);
                    if game.supercrewmate {
                        let scm = obj.getscm() as usize;
                        obj.movingplatformfix(i, scm, help, map);
                    }
                }
            }

            if obj.horplatforms {
                // for (int ie = obj.entities.size() - 1; ie >= 0;  ie-- {
                for ie in (0..obj.entities.len()).rev() {
                    if !obj.entities[ie].isplatform || (obj.entities[ie].vy).abs() >= 0.000001f32 {
                        continue;
                    }

                    let prevx = obj.entities[ie].xp;
                    let prevy = obj.entities[ie].yp;
                    obj.disableblockat(prevx, prevy);

                    // Behavioral logic
                    let entitygone = obj.updateentities(ie, game, map, music, script);
                    if entitygone { continue; }
                    // Basic Physics
                    obj.updateentitylogic(ie, game);
                    // Collisions with walls
                    obj.entitymapcollision(ie, map, help);

                    obj.moveblockto(prevx, prevy, obj.entities[ie].xp, obj.entities[ie].yp, obj.entities[ie].w, obj.entities[ie].h);
                }
                //is the player standing on a moving platform?
                let i = obj.getplayer() as usize;
                let mut j = obj.entitycollideplatformfloor(i, help);
                if INBOUNDS_VEC!(i, obj.entities) && j > -1000.0 {
                    obj.entities[i].newxp = obj.entities[i].xp as f32 + j;
                    obj.entitymapcollision(i, map, help);
                } else {
                    j = obj.entitycollideplatformroof(i, help);
                    if INBOUNDS_VEC!(i, obj.entities) && j > -1000.0 {
                        obj.entities[i].newxp = obj.entities[i].xp as f32 + j;
                        obj.entitymapcollision(i, map, help);
                    }
                }
            }

            // for (int ie = obj.entities.size() - 1; ie >= 0;  ie-- {
            for ie in (0..obj.entities.len()).rev() {
                if obj.entities[ie].isplatform {
                    continue;
                }

                // Behavioral logic
                let entitygone = obj.updateentities(ie, game, map, music, script);
                if entitygone { continue; }
                // Basic Physics
                obj.updateentitylogic(ie, game);
                // Collisions with walls
                obj.entitymapcollision(ie, map, help);
            }

            // Check ent v ent collisions, update states
            obj.entitycollisioncheck(game, graphics, map, music, help);

            if map.towermode {
                //special for tower: is the player touching any spike blocks?
                let player = obj.getplayer();
                if INBOUNDS_VEC!(player, obj.entities) && obj.checktowerspikes(player) && graphics.fademode == 0 {
                    game.deathseq = 30;
                }
            }

            if map.towermode && game.lifeseq == 0 {
                let player = obj.getplayer() as usize;
                if !map.invincibility && INBOUNDS_VEC!(player, obj.entities) {
                    if obj.entities[player].yp - map.ypos <= 0 {
                        game.deathseq = 30;
                    } else if obj.entities[player].yp - map.ypos >= 208 {
                        game.deathseq = 30;
                    }
                } else if INBOUNDS_VEC!(player, obj.entities) {
                    if obj.entities[player].yp - map.ypos <= 0 {
                        map.ypos-=10;
                        graphics.buffers.towerbg.bypos = map.ypos / 2;
                        graphics.buffers.towerbg.bscroll = 0;
                    } else if obj.entities[player].yp - map.ypos >= 208 {
                        map.ypos += 2;
                        graphics.buffers.towerbg.bypos = map.ypos / 2;
                        graphics.buffers.towerbg.bscroll = 0;
                    }
                }

                if INBOUNDS_VEC!(player, obj.entities) && obj.entities[player].yp - map.ypos <= 40 {
                    map.spikeleveltop += 1;
                    if map.spikeleveltop >= 8 {
                        map.spikeleveltop = 8;
                    }
                } else {
                    if map.spikeleveltop > 0 {
                        map.spikeleveltop -= 1;
                    }
                }

                if INBOUNDS_VEC!(player, obj.entities) && obj.entities[player].yp - map.ypos >= 164 {
                    map.spikelevelbottom += 1;
                    if map.spikelevelbottom >= 8 {
                        map.spikelevelbottom = 8;
                    }
                } else {
                    if map.spikelevelbottom > 0 {
                        map.spikelevelbottom -= 1;
                    }
                }

            }
        }

        //Using warplines?
        if obj.customwarpmode {
            if !game.glitchrunnermode {
                //Rewritten system for mobile update: basically, the new logic is to
                //check if the player is leaving the map, and if so do a special check against
                //warp lines for collision
                obj.customwarpmodehon = false;
                obj.customwarpmodevon = false;

                let i = obj.getplayer() as usize;
                if INBOUNDS_VEC!(i, obj.entities) && (
                    (game.door_down > -2 && obj.entities[i].yp >= 226-16) ||
                    (game.door_up > -2 && obj.entities[i].yp < -2+16) ||
                    (game.door_left > -2 && obj.entities[i].xp < -14+16) ||
                    (game.door_right > -2 && obj.entities[i].xp >= 308-16)
                ) {
                    //Player is leaving room
                    obj.customwarplinecheck(i as i32);
                }
            }

            if obj.customwarpmodehon {
                map.warpy = true;
            } else {
                map.warpy = false;
            }
            if obj.customwarpmodevon {
                map.warpx = true;
            } else {
                map.warpx = false;
            }
        }

        //Finally: Are we changing room?
        if map.warpx && !map.towermode {
            // for (size_t i = 0; i < obj.entities.size();  i++ {
            for i in 0..obj.entities.len() {
                if  //Don't warp warp lines
                    obj.entities[i].r#type < 50 &&
                    //Don't wrap SWN enemies
                    obj.entities[i].size < 12
                {
                    if game.roomx == 118 && game.roomy == 102 && obj.entities[i].rule == 1 && !map.custommode {
                        //ascii snakes
                        if obj.entities[i].xp <= -80 {
                            if obj.entities[i].isplatform {
                                obj.moveblockto(obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].xp + 400, obj.entities[i].yp, obj.entities[i].w, obj.entities[i].h);
                            }
                            obj.entities[i].xp += 400;
                            obj.entities[i].lerpoldxp += 400;
                        } else if obj.entities[i].xp > 320 {
                            if obj.entities[i].isplatform {
                                obj.moveblockto(obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].xp - 400, obj.entities[i].yp, obj.entities[i].w, obj.entities[i].h);
                            }
                            obj.entities[i].xp -= 400;
                            obj.entities[i].lerpoldxp -= 400;
                        }
                    } else {
                        if obj.entities[i].xp <= -10 {
                            if obj.entities[i].isplatform {
                                obj.moveblockto(obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].xp + 320, obj.entities[i].yp, obj.entities[i].w, obj.entities[i].h);
                            }
                            obj.entities[i].xp += 320;
                            obj.entities[i].lerpoldxp += 320;
                        } else if obj.entities[i].xp > 310 {
                            if obj.entities[i].isplatform {
                                obj.moveblockto(obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].xp - 320, obj.entities[i].yp, obj.entities[i].w, obj.entities[i].h);
                            }
                            obj.entities[i].xp -= 320;
                            obj.entities[i].lerpoldxp -= 320;
                        }
                    }
                }
            }
        }

        if map.warpy && !map.towermode {
            // for (size_t i = 0; i < obj.entities.size();  i++ {
            for i in 0..obj.entities.len() {
                if obj.entities[i].r#type < 50 {
                    //Don't warp warp lines
                    if obj.entities[i].yp <= -12 {
                        if obj.entities[i].isplatform {
                            obj.moveblockto(obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].xp, obj.entities[i].yp + 232, obj.entities[i].w, obj.entities[i].h);
                        }
                        obj.entities[i].yp += 232;
                        obj.entities[i].lerpoldyp += 232;
                    } else if obj.entities[i].yp > 226 {
                        if obj.entities[i].isplatform {
                            obj.moveblockto(obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].xp, obj.entities[i].yp - 232, obj.entities[i].w, obj.entities[i].h);
                        }
                        obj.entities[i].yp -= 232;
                        obj.entities[i].lerpoldyp -= 232;
                    }
                }
            }
        }

        if map.warpy && !map.warpx && !map.towermode {
            // for (size_t i = 0; i < obj.entities.size();  i++ {
            for i in 0..obj.entities.len() {
                if  //Don't warp warp lines
                    obj.entities[i].r#type < 50 &&
                    obj.entities[i].rule != 0
                {
                    if obj.entities[i].xp <= -30 {
                        if obj.entities[i].isplatform {
                            obj.moveblockto(obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].xp + 350, obj.entities[i].yp, obj.entities[i].w, obj.entities[i].h);
                        }
                        obj.entities[i].xp += 350;
                        obj.entities[i].lerpoldxp += 350;
                    } else if obj.entities[i].xp > 320 {
                        if obj.entities[i].isplatform {
                            obj.moveblockto(obj.entities[i].xp, obj.entities[i].yp, obj.entities[i].xp - 350, obj.entities[i].yp, obj.entities[i].w, obj.entities[i].h);
                        }
                        obj.entities[i].xp -= 350;
                        obj.entities[i].lerpoldxp -= 350;
                    }
                }
            }
        }

        let mut screen_transition = false;

        if !map.warpy && !map.towermode {
            //Normal! Just change room
            let player = obj.getplayer() as usize;
            if INBOUNDS_VEC!(player, obj.entities) && game.door_down > -2 && obj.entities[player].yp >= 238 {
                obj.entities[player].yp -= 240;
                map.gotoroom(game.roomx, game.roomy + 1, game, graphics, music, obj, help);
                screen_transition = true;
            }
            if INBOUNDS_VEC!(player, obj.entities) && game.door_up > -2 && obj.entities[player].yp < -2 {
                obj.entities[player].yp += 240;
                map.gotoroom(game.roomx, game.roomy - 1, game, graphics, music, obj, help);
                screen_transition = true;
            }
        }

        if !map.warpx && !map.towermode {
            //Normal! Just change room
            let player = obj.getplayer() as usize;
            if INBOUNDS_VEC!(player, obj.entities) && game.door_left > -2 && obj.entities[player].xp < -14 {
                obj.entities[player].xp += 320;
                map.gotoroom(game.roomx - 1, game.roomy, game, graphics, music, obj, help);
                screen_transition = true;
            }
            if INBOUNDS_VEC!(player, obj.entities) && game.door_right > -2 && obj.entities[player].xp >= 308 {
                obj.entities[player].xp -= 320;
                map.gotoroom(game.roomx + 1, game.roomy, game, graphics, music, obj, help);
                screen_transition = true;
            }
        }

        //Right so! Screenwraping for tower:
        if map.towermode && map.minitowermode {
            if graphics.buffers.towerbg.scrolldir == 1 {
                //This is minitower 1!
                let player = obj.getplayer() as usize;
                if INBOUNDS_VEC!(player, obj.entities) && game.door_left > -2 && obj.entities[player].xp < -14 {
                    obj.entities[player].xp += 320;
                    map.gotoroom(48, 52, game, graphics, music, obj, help);
                }
                if INBOUNDS_VEC!(player, obj.entities) && game.door_right > -2 && obj.entities[player].xp >= 308 {
                    obj.entities[player].xp -= 320;
                    obj.entities[player].yp -= 71*8;
                    map.gotoroom(game.roomx + 1, game.roomy+1, game, graphics, music, obj, help);
                }
            } else {
                //This is minitower 2!
                let player = obj.getplayer() as usize;
                if INBOUNDS_VEC!(player, obj.entities) && game.door_left > -2 && obj.entities[player].xp < -14 {
                    if obj.entities[player].yp > 300 {
                        obj.entities[player].xp += 320;
                        obj.entities[player].yp -= 71 * 8;
                        map.gotoroom(50, 54, game, graphics, music, obj, help);
                    } else {
                        obj.entities[player].xp += 320;
                        map.gotoroom(50, 53, game, graphics, music, obj, help);
                    }
                }
                if INBOUNDS_VEC!(player, obj.entities) && game.door_right > -2 && obj.entities[player].xp >= 308 {
                    obj.entities[player].xp -= 320;
                    map.gotoroom(52, 53, game, graphics, music, obj, help);
                }
            }
        } else if map.towermode {
            //Always wrap except for the very top and very bottom of the tower
            if map.ypos >= 500 && map.ypos <= 5000 {
                // for (size_t i = 0; i < obj.entities.size();  i++ {
                for i in 0..obj.entities.len() {
                    if obj.entities[i].xp <= -10 {
                        obj.entities[i].xp += 320;
                        obj.entities[i].lerpoldxp += 320;
                    } else if obj.entities[i].xp > 310 {
                        obj.entities[i].xp -= 320;
                        obj.entities[i].lerpoldxp -= 320;
                    }
                }
            } else {
                //Do not wrap! Instead, go to the correct room
                let player = obj.getplayer() as usize;
                if INBOUNDS_VEC!(player, obj.entities) && game.door_left > -2 && obj.entities[player].xp < -14 {
                    obj.entities[player].xp += 320;
                    obj.entities[player].yp -= 671 * 8;
                    map.gotoroom(108, 109, game, graphics, music, obj, help);
                }
                if INBOUNDS_VEC!(player, obj.entities) && game.door_right > -2 && obj.entities[player].xp >= 308 {
                    obj.entities[player].xp -= 320;
                    map.gotoroom(110, 104, game, graphics, music, obj, help);
                }
            }
        }

        //Warp tokens
        if map.custommode {
            if game.teleport && INBOUNDS_VEC!(game.edteleportent, obj.entities) {
                let edi = obj.entities[game.edteleportent as usize].behave;
                let edj = obj.entities[game.edteleportent as usize].para as i32;
                let edi2 = (edi-(edi%40)) / 40;
                let edj2 = (edj-(edj%30)) / 30;

                map.warpto(100+edi2, 100+edj2, obj.getplayer(), edi%40, (edj%30)+2);
                game.teleport = false;

                if game.teleport == false {
                    game.flashlight = 6;
                    game.screenshake = 25;
                }
            }
        } else {
            if game.teleport {
                if game.roomx == 117 && game.roomy == 102 {
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].yp = 225;
                    }
                    map.gotoroom(119, 100, game, graphics, music, obj, help);
                    game.teleport = false;
                } else if game.roomx == 119 && game.roomy == 100 {
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].yp = 225;
                    }
                    map.gotoroom(119, 103, game, graphics, music, obj, help);
                    game.teleport = false;
                } else if game.roomx == 119 && game.roomy == 103 {
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp = 0;
                    }
                    map.gotoroom(116, 103, game, graphics, music, obj, help);
                    game.teleport = false;
                } else if game.roomx == 116 && game.roomy == 103 {
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].yp = 225;
                    }
                    map.gotoroom(116, 100, game, graphics, music, obj, help);
                    game.teleport = false;
                } else if game.roomx == 116 && game.roomy == 100 {
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].xp = 0;
                    }
                    map.gotoroom(114, 102, game, graphics, music, obj, help);
                    game.teleport = false;
                } else if game.roomx == 114 && game.roomy == 102 {
                    let i = obj.getplayer() as usize;
                    if INBOUNDS_VEC!(i, obj.entities) {
                        obj.entities[i].yp = 225;
                    }
                    map.gotoroom(113, 100, game, graphics, music, obj, help);
                    game.teleport = false;
                } else if game.roomx == 116 && game.roomy == 104 {
                    //pre warp zone here
                    map.warpto(107, 101, obj.getplayer(), 14, 16);
                } else if game.roomx == 107 && game.roomy == 101 {
                    map.warpto(105, 119, obj.getplayer(), 5, 26);
                } else if game.roomx == 105 && game.roomy == 118 {
                    map.warpto(101, 111, obj.getplayer(), 34, 6);
                } else if game.roomx == 101 && game.roomy == 111 {
                    //There are lots of warp tokens in this room, so we have to distinguish!
                    match game.teleportxpos {
                        1 => map.warpto(108, 108, obj.getplayer(), 4, 27),
                        2 => map.warpto(101, 111, obj.getplayer(), 12, 27),
                        3 => map.warpto(119, 111, obj.getplayer(), 31, 7),
                        4 => map.warpto(114, 117, obj.getplayer(), 19, 16),
                        _ => (),
                    }
                } else if game.roomx == 108 && game.roomy == 106 {
                    map.warpto(119, 111, obj.getplayer(), 4, 27);
                } else if game.roomx == 100 && game.roomy == 111 {
                    map.warpto(101, 111, obj.getplayer(), 24, 6);
                } else if game.roomx == 119 && game.roomy == 107 {
                    //Secret lab, to super gravitron
                    map.warpto(119, 108, obj.getplayer(), 19, 10);
                }
                if game.teleport == false {
                    game.flashlight = 6;
                    game.screenshake = 25;
                }
            }
        }

        if screen_transition {
            map.twoframedelayfix(game, obj, script, help);
        }
    }

    //Update colour cycling for final level
    if map.finalmode && map.final_colormode {
        map.final_aniframedelay -= 1;
        if map.final_aniframedelay == 0 {
            graphics.foregrounddrawn = false;
        }
        if map.final_aniframedelay <= 0 {
            map.final_aniframedelay = 2;
            map.final_aniframe += 1;
            if map.final_aniframe >= 4 {
                map.final_aniframe = 0;
            }
        }
    }

    if game.roomchange {
        //We've changed room? Let's bring our companion along!
        game.roomchange = false;
        let i = obj.getplayer() as usize;
        if game.companion > 0 && INBOUNDS_VEC!(i, obj.entities) {
            //ok, we'll presume our companion has been destroyed in the room change. So:
            match game.companion {
                6 => {
                    //Y=121, the floor in that particular place!
                    obj.createentity(obj.entities[i].xp, 121, 15, Some(1), None, None, None, None, None, game);
                    let j = obj.getcompanion() as usize;
                    if INBOUNDS_VEC!(j, obj.entities) {
                        obj.entities[j].vx = obj.entities[i].vx;
                        obj.entities[j].dir = obj.entities[i].dir;
                    }
                },
                7 => {
                    //don't jump after him!
                    if game.roomy <= 105 {
                        if game.roomx == 110 {
                            //Y=86, the ROOF in that particular place!
                            obj.createentity(320, 86, 16, Some(1), None, None, None, None, None, game);
                        } else {
                            //Y=86, the ROOF in that particular place!
                            obj.createentity(obj.entities[i].xp, 86, 16, Some(1), None, None, None, None, None, game);
                        }
                        let j = obj.getcompanion() as usize;
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[j].vx = obj.entities[i].vx;
                            obj.entities[j].dir = obj.entities[i].dir;
                        }
                    }
                },
                8 => {
                    //don't jump after him!
                    if game.roomy >= 104 {
                        if game.roomx == 102 {
                            obj.createentity(310, 177, 17, Some(1), None, None, None, None, None, game);
                            let j = obj.getcompanion() as usize;
                            if INBOUNDS_VEC!(j, obj.entities) {
                                obj.entities[j].vx = obj.entities[i].vx;
                                obj.entities[j].dir = obj.entities[i].dir;
                            }
                        } else {
                            obj.createentity(obj.entities[i].xp, 177, 17, Some(1), None, None, None, None, None, game);
                            let j = obj.getcompanion() as usize;
                            if INBOUNDS_VEC!(j, obj.entities) {
                                obj.entities[j].vx = obj.entities[i].vx;
                                obj.entities[j].dir = obj.entities[i].dir;
                            }
                        }
                    }
                },
                9 => {
                    //don't go back into the tower!
                    if !map.towermode {
                        if game.roomx == 110 && obj.entities[i].xp < 20 {
                            obj.createentity(100, 185, 18, Some(15), Some(0), Some(1), None, None, None, game);
                        } else {
                            obj.createentity(obj.entities[i].xp, 185, 18, Some(15), Some(0), Some(1), None, None, None, game);
                        }
                        let j = obj.getcompanion() as usize;
                        if INBOUNDS_VEC!(j, obj.entities) {
                            obj.entities[j].vx = obj.entities[i].vx;
                            obj.entities[j].dir = obj.entities[i].dir;
                        }
                    }
                },
                10 => {
                    //intermission 2, choose colour based on lastsaved
                    if game.roomy == 51 {
                        if !obj.flags[59] {
                            obj.createentity(225, 169, 18, Some(graphics.crewcolour(game.lastsaved)), Some(0), Some(10), None, None, None, game);
                            let j = obj.getcompanion() as usize;
                            if INBOUNDS_VEC!(j, obj.entities) {
                                obj.entities[j].vx = obj.entities[i].vx;
                                obj.entities[j].dir = obj.entities[i].dir;
                            }
                        }
                    } else if game.roomy >= 52 {
                        if obj.flags[59] {
                            obj.createentity(160, 177, 18, Some(graphics.crewcolour(game.lastsaved)), Some(0), Some(18), Some(1), None, None, game);
                            let j = obj.getcompanion() as usize;
                            if INBOUNDS_VEC!(j, obj.entities) {
                                obj.entities[j].vx = obj.entities[i].vx;
                                obj.entities[j].dir = obj.entities[i].dir;
                            }
                        } else {
                            obj.flags[59] = true;
                            obj.createentity(obj.entities[i].xp, -20, 18, Some(graphics.crewcolour(game.lastsaved)), Some(0), Some(10), Some(0), None, None, game);
                            let j = obj.getcompanion() as usize;
                            if INBOUNDS_VEC!(j, obj.entities) {
                                obj.entities[j].vx = obj.entities[i].vx;
                                obj.entities[j].dir = obj.entities[i].dir;
                            }
                        }
                    }
                },
                11 => {
                    //Intermission 1: We're using the SuperCrewMate instead!
                    if game.roomx - 41 == game.scmprogress {
                        match game.scmprogress {
                            0 => {
                                obj.createentity(76, 161, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            1 => {
                                obj.createentity(10, 169, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            2 => {
                                obj.createentity(10, 177, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            3 => {
                                if game.scmmoveme {
                                    let player = obj.getplayer() as usize;
                                    obj.createentity(obj.entities[player].xp, 185, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                                    game.scmmoveme = false;
                                } else {
                                    obj.createentity(10, 177, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                                }
                            },
                            4 => {
                                obj.createentity(10, 185, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            5 => {
                                obj.createentity(10, 185, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            6 => {
                                obj.createentity(10, 185, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            7 => {
                                obj.createentity(10, 41, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            8 => {
                                obj.createentity(10, 169, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            9 => {
                                obj.createentity(10, 169, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            10 => {
                                obj.createentity(10, 129, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            11 => {
                                obj.createentity(10, 129, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            12 => {
                                obj.createentity(10, 65, 24, Some(graphics.crewcolour(game.lastsaved)), Some(2), None, None, None, None, game);
                            },
                            13 => {
                                obj.createentity(10, 177, 24, Some(graphics.crewcolour(game.lastsaved)), None, None, None, None, None, game);
                            },
                            _ => (),
                        };
                    }

                    if game.scmmoveme {
                        let scm = obj.getscm() as usize;
                        let player = obj.getplayer() as usize;
                        if INBOUNDS_VEC!(scm, obj.entities) && INBOUNDS_VEC!(player, obj.entities) {
                            obj.entities[scm].xp = obj.entities[player].xp;
                        }
                        game.scmmoveme = false;
                    }
                }
                _ => (),
            };
        }
    }

    game.activeactivity = obj.checkactivity(help);

    if game.hascontrol && !script.running && INBOUNDS_VEC!(game.activeactivity, obj.entities) {
        game.activity_lastprompt = String::from(obj.blocks[game.activeactivity as usize].prompt.to_owned());
        game.activity_r = obj.blocks[game.activeactivity as usize].r;
        game.activity_g = obj.blocks[game.activeactivity as usize].g;
        game.activity_b = obj.blocks[game.activeactivity as usize].b;
    }

    game.oldreadytotele = game.readytotele;
    if game.activetele && game.hascontrol && !script.running && !game.intimetrial {
        let i = obj.getplayer() as usize;
        let temprect = sdl2::rect::Rect::new(
            obj.entities[i].xp + obj.entities[i].cx,
            obj.entities[i].yp + obj.entities[i].cy,
            obj.entities[i].w as u32,
            obj.entities[i].h as u32
        );
        if help.intersects(game.teleblock, temprect) {
            game.readytotele += 25;
            if game.readytotele >= 255 {
                game.readytotele = 255;
            }
        } else {
            game.readytotele -= 50;
            if game.readytotele < 0 {
                game.readytotele = 0;
            }
        }
    } else {
        if game.readytotele > 0 {
            game.readytotele -= 50;
            if game.readytotele < 0 {
                game.readytotele = 0;
            }
        }
    }

    if game.teleport_to_new_area {
        script.teleport(game, obj, map, graphics, music, help, fs);
    }

    Ok(None)
}
