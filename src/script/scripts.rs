pub fn load(script: &mut super::ScriptClass, name: &str) {
  //loads script name t into the array
  script.position = 0;
  script.commands.clear();
  script.running = true;

  match name {
    // if (SDL_strncmp(t, "custom_", 7) == 0)
    "intro" => {
      let lines = vec![
        "ifskip(quickstart)",
        //"createcrewman(232,113,cyan,0,faceright)",
        "createcrewman(96,177,green,0,faceright)",
        "createcrewman(122,177,purple,0,faceleft)",
        "fadein()",
        "untilfade()",
        "delay(90)",

        "flash(5)",
        "shake(20)",
        "playef(9)",
        "musicfadeout()",
        "changemood(player,1)",
        "delay(15)",
        "squeak(player)",
        "text(cyan,0,0,1)",
        "Uh oh...",
        "position(player,above)",
        //"backgroundtext",
        "speak_active",

        "squeak(purple)",
        "changeai(purple,followposition,175)",
        "text(purple,145,150,1)",
        "Is everything ok?",
        //"position(purple,above)",
        //"backgroundtext",
        "speak_active",

        "squeak(player)",
        "walk(left,2)",
        "text(cyan,0,0,2)",
        "No! We've hit some",
        "kind of interference...",
        "position(player,above)",
        //"backgroundtext",
        "speak_active",

        //"delay(30)",
        "endtext",

        "flash(5)",
        "shake(50)",
        "playef(9)",
        "changemood(green,1)",
        "changemood(purple,1)",
        "alarmon",

        "changedir(player,1)",
        "delay(30)",
        "endtext",

        "squeak(player)",
        "text(cyan,0,0,2)",
        "Something's wrong! We're",
        "going to crash!",
        "position(player,above)",
        //"backgroundtext",
        "speak_active",
        //"delay(100)",

        "endtext",

        "flash(5)",
        "shake(50)",
        "playef(9)",
        "changeai(green,followposition,-60)",
        "changeai(purple,followposition,-60)",
        "squeak(player)",
        "text(cyan,70,140,1)",
        "Evacuate!",
        "backgroundtext",
        "speak_active",
        "walk(left,35)",

        "endtextfast",

        //Ok, next room!

        "flash(5)",
        "shake(50)",
        "playef(9)",
        "gotoroom(3,10)",
        "gotoposition(310,177,0)",
        "createcrewman(208,177,green,1,followposition,120)",
        "createcrewman(240,177,purple,1,followposition,120)",
        "createcrewman(10,177,blue,1,followposition,180)",

        "squeak(blue)",
        "text(blue,80,150,1)",
        "Oh no!",
        "backgroundtext",
        "speak_active",
        "walk(left,20)",

        "endtextfast",

        //and the next!
        "flash(5)",
        "shake(50)",
        "playef(9)",
        "gotoroom(3,11)",
        "gotoposition(140,0,0)",

        "createcrewman(90,105,green,1,followblue)",
        "createcrewman(125,105,purple,1,followgreen)",
        "createcrewman(55,105,blue,1,followposition,-200)",

        "createcrewman(120,177,yellow,1,followposition,-200)",
        "createcrewman(240,177,red,1,faceleft)",

        "delay(5)",
        "changeai(red,followposition,-200)",

        "squeak(red)",
        "text(red,100,150,1)",
        "Everyone off the ship!",
        "backgroundtext",
        "speak_active",

        "walk(left,25)",

        "endtextfast",

        //final room:
        "flash(5)",
        "shake(80)",
        "playef(9)",
        "gotoroom(2,11)",
        "gotoposition(265,153,0)",

        "createcrewman(130,153,blue,1,faceleft)",
        "createcrewman(155,153,green,1,faceleft)",
        "createcrewman(180,153,purple,1,faceleft)",
        "createcrewman(205,153,yellow,1,faceleft)",
        "createcrewman(230,153,red,1,faceleft)",


        "squeak(yellow)",
        "text(yellow,0,0,1)",
        "This shouldn't be happening!",
        "position(yellow,below)",
        "backgroundtext",
        "speak_active",

        "activateteleporter()",

        "delay(10)",
        "changecolour(blue,teleporter)",
        "delay(10)",
        "changecolour(green,teleporter)",
        "delay(10)",
        "changecolour(purple,teleporter)",
        "delay(10)",
        "changecolour(yellow,teleporter)",
        "delay(10)",
        "changecolour(red,teleporter)",
        "delay(10)",

        //and teleport!
        "endtext",
        "alarmoff",
        "flash(5)",
        "shake(20)",
        "playef(10)",
        "blackout()",
        "changemood(player,0)",
        "changedir(player,1)",

        "delay(100)",
        "blackon()",
        "shake(20)",
        "playef(10)",

        //Finally, appear at the start of the game:
        "gotoroom(13,5)",
        "gotoposition(80,96,0)",
        "walk(right,20)",
        //"delay(45)",

        "squeak(player)",
        "text(cyan,0,0,1)",
        "Phew! That was scary!",
        "position(player,above)",
        "speak_active",

        "squeak(player)",
        "text(cyan,0,0,2)",
        "At least we all",
        "escaped, right guys?",
        "position(player,above)",
        "speak_active",

        "endtext",

        "delay(45)",
        "walk(left,3)",
        "delay(45)",
        "setcheckpoint()",

        "squeak(player)",
        "text(cyan,0,0,1)",
        "...guys?",
        "position(player,above)",
        "speak_active",

        "endtext",

        "delay(25)",
        "changemood(player,1)",
        "squeak(cry)",
        "delay(25)",

        "play(1)",
        "endcutscene()",
        "untilbars()",

        "hideship()",

        "gamestate(4)",
      ];

      script.filllines(lines);
    },
    "quickstart" => (),
    "firststeps" => (),
    "trenchwarfare" => (),
    "newtrenchwarfare" => (),
    "trinketcollector" => (),
    "newtrinketcollector" => (),
    "new2trinketcollector" => (),
    _ => (),
  }

  match name {
    "communicationstation" => (),
    "communicationstationskip" => (),
    "teleporterback" => (),
    "levelonecomplete" => (),
    "levelonecomplete_ending" => (),
    "levelonecompleteskip" => (),
    "bigopenworld" => (),
    "bigopenworldskip" => (),
    "rescueblue" => (),
    "skipblue" => (),
    "rescueyellow" => (),
    "skipyellow" => (),
    "rescuegreen" => (),
    "skipgreen" => (),
    "rescuered" => (),
    "skipred" => (),
    "startexpolevel_station1" => (),
    "startexpolevel_lab" => (),
    "startexpolevel_warp" => (),
    "startexpolevel_tower" => (),
    "skipint1" => (),
    "intermission_1" => (),
    "int1blue_1" => (),
    "int1blue_2" => (),
    "int1blue_3" => (),
    "int1blue_4" => (),
    "int1blue_5" => (),
    "int1blue_6" => (),
    "int1blue_7" => (),
    "int1green_1" => (),
    "int1green_2" => (),
    "int1green_3" => (),
    "int1green_4" => (),
    "int1green_5" => (),
    "int1green_6" => (),
    "int1green_7" => (),
    "int1red_1" => (),
    "int1red_2" => (),
    "int1red_3" => (),
    "int1red_4" => (),
    "int1red_5" => (),
    "int1red_6" => (),
    "int1red_7" => (),
    "int1yellow_1" => (),
    "int1yellow_2" => (),
    "int1yellow_3" => (),
    "int1yellow_4" => (),
    "int1yellow_5" => (),
    "int1yellow_6" => (),
    "int1yellow_7" => (),
    "skipint2" => (),
    "intermission_2" => (),
    "int2intro_yellow" => (),
    "int2intro_red" => (),
    "int2intro_green" => (),
    "int2intro_blue" => (),
    "int2_yellow" => (),
    "skipint2yellow" => (),
    "int2_red" => (),
    "skipint2red" => (),
    "int2_green" => (),
    "skipint2green" => (),
    "int2_blue" => (),
    "skipint2blue" => (),
    "startexpolevel_station2" => (),
    "finallevel_teleporter" => (),
    "skipfinal" => (),
    "startlevel_final" => (),
    "regularreturn" => (),
    "returntohub" => (),
    "resetgame" => (),
    "talkred" => (),
    "talkyellow" => (),
    "talkgreen" => (),
    "talkblue" => (),
    "talkpurple" => (),
    "talkred_1" => (),
    "talkred_2" => (),
    "talkred_3" => (),
    "talkred_4" => (),
    "talkred_5" => (),
    "talkred_6" => (),
    "talkred_7" => (),
    "talkred_8" => (),
    "talkred_9" => (),
    "talkred_10" => (),
    "talkred_11" => (),
    "talkred_12" => (),
    "talkred_13" => (),
    "talkred_14" => (),
    "talkyellow_1" => (),
    "talkyellow_2" => (),
    "talkyellow_3" => (),
    "talkyellow_4" => (),
    "talkyellow_5" => (),
    "talkyellow_6" => (),
    "talkyellow_7" => (),
    "talkyellow_8" => (),
    "talkyellow_9" => (),
    "talkyellow_10" => (),
    "talkyellow_11" => (),
    "talkyellow_12" => (),
    "talkgreen_1" => (),
    "talkgreen_2" => (),
    "talkgreen_3" => (),
    "talkgreen_4" => (),
    "talkgreen_5" => (),
    "talkgreen_6" => (),
    "talkgreen_7" => (),
    "talkgreen_8" => (),
    "talkgreen_9" => (),
    "talkgreen_10" => (),
    "talkgreen_11" => (),
    _ => (),
  }

  match name {
    "talkpurple_1" => (),
    "talkpurple_2" => (),
    "talkpurple_3" => (),
    "talkpurple_4" => (),
    "talkpurple_5" => (),
    "talkpurple_6" => (),
    "talkpurple_7" => (),
    "talkpurple_8" => (),
    "talkpurple_9" => (),
    "talkpurple_intermission1" => (),
    "talkpurple_intermission2" => (),
    "talkpurple_intermission3" => (),
    "talkpurple_intro" => (),
    "talkblue_1" => (),
    "talkblue_2" => (),
    "talkblue_3" => (),
    "talkblue_4" => (),
    "talkblue_5" => (),
    "talkblue_6" => (),
    "talkblue_7" => (),
    "talkblue_8" => (),
    "talkblue_9" => (),
    "talkblue_trinket1" => (),
    "talkblue_trinket2" => (),
    "talkblue_trinket3" => (),
    "talkblue_trinket4" => (),
    "talkblue_trinket5" => (),
    "talkblue_trinket6" => (),
    "talkyellow_trinket1" => (),
    "talkyellow_trinket2" => (),
    "talkyellow_trinket3" => (),
    "gamecomplete" => (),
    "gamecomplete_ending" => (),
    "startepilogue" => (),
    "returntolab" => (),
    _ => (),
  }
}
