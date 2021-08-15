mod binary_blob;
use binary_blob::BinaryBlob;
mod sound_system;
use sound_system::{MusicTrack, SoundTrack, SoundSystem};

use crate::{INBOUNDS_VEC, filesystem, game, map};

/* The amount of "space" for the scale of the user-set volume. */
pub const USER_VOLUME_MAX: i32 = 256;

/* It is advised that USER_VOLUME_MAX be divisible by this. */
pub const USER_VOLUME_STEP: i32 = 32;

#[derive(Clone, Copy)]
enum BlobType {
    pppppp,
    mmmmmm,
}

pub struct Music {
    pub currentsong: i32,
    soundTracks: Vec<SoundTrack>,
    musicTracks: Vec<MusicTrack>,
    // musicTracks: Vec<sdl2::mixer::Music<'static>>,
    // musicTracks: Vec<*mut sdl2_sys::mixer::Mix_Music>,

    safeToProcessMusic: bool,
    m_doFadeInVol: bool,
    m_doFadeOutVol: bool,
    pub musicVolume: i32,
    FadeVolAmountPerFrame: i32,

    pub user_music_volume: Box<i32>,
    pub user_sound_volume: Box<i32>,

    nicechange: i32,
    nicefade: bool,
    quick_fade: bool,

    pppppp_blob: BinaryBlob,
    num_pppppp_tracks: i32,

    // @sx MMMMMM mod completely disabled
    // MMMMMM mod settings
    pub mmmmmm: bool,
    pub usingmmmmmm: bool,
    // mmmmmm_blob: BinaryBlob,
    num_mmmmmm_tracks: i32,
}

impl Music {
    // musicclass::musicclass(void)
    pub fn new() -> Self {
        SoundSystem::SoundSystem();

        Music {
            currentsong: 0,
            soundTracks: vec![],
            musicTracks: vec![],

            safeToProcessMusic: false,
            m_doFadeInVol: false,
            m_doFadeOutVol: false,
            musicVolume: 0,
            FadeVolAmountPerFrame: 0,

            user_music_volume: Box::new(USER_VOLUME_MAX),
            user_sound_volume: Box::new(USER_VOLUME_MAX),

            nicechange: -1,
            nicefade: false,
            quick_fade: true,

            pppppp_blob: BinaryBlob::new(),
            num_pppppp_tracks: 0,

            // MMMMMM mod settings
            mmmmmm: false,
            usingmmmmmm: false,
            // mmmmmm_blob: BinaryBlob::new(),
            num_mmmmmm_tracks: 0,
        }
    }

    // void musicclass::init(void)
    pub fn init(&mut self, fs: &mut filesystem::FileSystem) {
        self.soundTracks = vec![
            SoundTrack::new( "assets/sounds/jump.wav" ),
            SoundTrack::new( "assets/sounds/jump2.wav" ),
            SoundTrack::new( "assets/sounds/hurt.wav" ),
            SoundTrack::new( "assets/sounds/souleyeminijingle.wav" ),
            SoundTrack::new( "assets/sounds/coin.wav" ),
            SoundTrack::new( "assets/sounds/save.wav" ),
            SoundTrack::new( "assets/sounds/crumble.wav" ),
            SoundTrack::new( "assets/sounds/vanish.wav" ),
            SoundTrack::new( "assets/sounds/blip.wav" ),
            SoundTrack::new( "assets/sounds/preteleport.wav" ),
            SoundTrack::new( "assets/sounds/teleport.wav" ),
            SoundTrack::new( "assets/sounds/crew1.wav" ),
            SoundTrack::new( "assets/sounds/crew2.wav" ),
            SoundTrack::new( "assets/sounds/crew3.wav" ),
            SoundTrack::new( "assets/sounds/crew4.wav" ),
            SoundTrack::new( "assets/sounds/crew5.wav" ),
            SoundTrack::new( "assets/sounds/crew6.wav" ),
            SoundTrack::new( "assets/sounds/terminal.wav" ),
            SoundTrack::new( "assets/sounds/gamesaved.wav" ),
            SoundTrack::new( "assets/sounds/crashing.wav" ),
            SoundTrack::new( "assets/sounds/blip2.wav" ),
            SoundTrack::new( "assets/sounds/countdown.wav" ),
            SoundTrack::new( "assets/sounds/go.wav" ),
            SoundTrack::new( "assets/sounds/crash.wav" ),
            SoundTrack::new( "assets/sounds/combine.wav" ),
            SoundTrack::new( "assets/sounds/newrecord.wav" ),
            SoundTrack::new( "assets/sounds/trophy.wav" ),
            SoundTrack::new( "assets/sounds/rescue.wav" ),
        ];

        // // #ifdef VVV_COMPILEMUSIC
        //     binaryBlob musicWriteBlob;
        //     // #define FOREACH_TRACK(blob, track_name) blob.AddFileToBinaryBlob(track_name);
        //     TRACK_NAMES(musicWriteBlob)
        //     // #undef FOREACH_TRACK

        //     musicWriteBlob.writeBinaryBlob("data/BinaryMusic.vvv");
        //     musicWriteBlob.clear();
        // #endif

        self.num_mmmmmm_tracks = 0;
        self.num_pppppp_tracks = 0;

        // if !mmmmmm_blob.unPackBinary("mmmmmm.vvv") {
            self.mmmmmm = false;
            self.usingmmmmmm = false;

            let filepath = fs.getMountedPath("vvvvvvmusic.vvv");
            let ohCrap = self.pppppp_blob.unPackBinary(filepath);
            // TODO: @sx what to do with SDL_assert???
            // SDL_assert(ohCrap && "Music not found!");
        // } else {
            // self.mmmmmm = true;
            // int index;
            // SDL_RWops *rw;

            // TRACK_NAMES(mmmmmm_blob);
            // self.num_mmmmmm_tracks += musicTracks.size();
            // let index_ = 0;
            // while mmmmmm_blob.nextExtra(&index_) {
            //     rw = SDL_RWFromConstMem(mmmmmm_blob.getAddress(index_), mmmmmm_blob.getSize(index_));
            //     musicTracks.push_back(MusicTrack( rw ));
            //     self.num_mmmmmm_tracks += 1;
            //     index_ += 1;
            // }
            // let ohCrap = pppppp_blob.unPackBinary("vvvvvvmusic.vvv");
            // SDL_assert(ohCrap && "Music not found!");
            // }

            self.TRACK_NAMES(BlobType::pppppp);
        // #undef FOREACH_TRACK

        self.num_pppppp_tracks += self.musicTracks.len() as i32 - self.num_mmmmmm_tracks;

        let mut index_ = 0;
        while self.pppppp_blob.nextExtra(index_) {
            match self.pppppp_blob.getMusicTrack(index_) {
                Ok(track) => {
                    self.musicTracks.push(track);
                    info!("looks like some music track has been loaded: {:?}", self.pppppp_blob.m_headers[index_]);
                },
                Err(s) => eprintln!("error while loading track: {}", s),
            };

            self.num_pppppp_tracks += 1;
            index_ += 1;
        }

        info!("{}m/{}p music tracks and {} sounds loaded", self.num_mmmmmm_tracks, self.num_pppppp_tracks, self.soundTracks.len());
    }

    fn TRACK_NAMES(&mut self, blob: BlobType) {
        self.FOREACH_TRACK(blob, "data/music/0levelcomplete.ogg");
        self.FOREACH_TRACK(blob, "data/music/1pushingonwards.ogg");
        self.FOREACH_TRACK(blob, "data/music/2positiveforce.ogg");
        self.FOREACH_TRACK(blob, "data/music/3potentialforanything.ogg");
        self.FOREACH_TRACK(blob, "data/music/4passionforexploring.ogg");
        self.FOREACH_TRACK(blob, "data/music/5intermission.ogg");
        self.FOREACH_TRACK(blob, "data/music/6presentingvvvvvv.ogg");
        self.FOREACH_TRACK(blob, "data/music/7gamecomplete.ogg");
        self.FOREACH_TRACK(blob, "data/music/8predestinedfate.ogg");
        self.FOREACH_TRACK(blob, "data/music/9positiveforcereversed.ogg");
        self.FOREACH_TRACK(blob, "data/music/10popularpotpourri.ogg");
        self.FOREACH_TRACK(blob, "data/music/11pipedream.ogg");
        self.FOREACH_TRACK(blob, "data/music/12pressurecooker.ogg");
        self.FOREACH_TRACK(blob, "data/music/13pacedenergy.ogg");
        self.FOREACH_TRACK(blob, "data/music/14piercingthesky.ogg");
        self.FOREACH_TRACK(blob, "data/music/predestinedfatefinallevel.ogg");
    }

    // #define FOREACH_TRACK(blob, track_name)
    fn FOREACH_TRACK(&mut self, blob: BlobType, track_name: &str) {
        match blob {
            BlobType::pppppp => {
                if let Some(index) = self.pppppp_blob.getIndex(track_name) {
                    match self.pppppp_blob.getMusicTrack(index) {
                        Ok(track) => {
                            self.musicTracks.push(track);
                            info!("looks like some music track has been loaded: {:?}", self.pppppp_blob.m_headers[index]);
                        },
                        Err(s) => eprintln!("error while loading track: {}", s),
                    };
                }
            },
            BlobType::mmmmmm => todo!(),
        }
    }

    // void musicclass::destroy(void)
    pub fn destroy(&mut self) {
        println!("DEADBEEF: musicclass::destroy() method is not implemented yet");
    }

    // void musicclass::play(int t)
    pub fn play(&mut self, t: i32, map: &mut map::Map, game: &mut game::Game) {
        let mut t = t;

        if self.mmmmmm && self.usingmmmmmm {
            // Don't conjoin this if-statement with the above one...
            if self.num_mmmmmm_tracks > 0 {
                t %= self.num_mmmmmm_tracks;
            }
        } else if self.num_pppppp_tracks > 0 {
            t %= self.num_pppppp_tracks;
        }

        if self.mmmmmm && !self.usingmmmmmm {
            t += self.num_mmmmmm_tracks;
        }

        self.safeToProcessMusic = true;

        if self.currentsong == t && !self.m_doFadeOutVol {
            return;
        }

        self.currentsong = t;
        if t == -1 {
            return;
        }

        if !INBOUNDS_VEC!(t, self.musicTracks) {
            println!("play() out-of-bounds!");
            self.currentsong = -1;
            return;
        }

        if self.currentsong == 0 || self.currentsong == 7 || (
            !map.custommode && (
                self.currentsong == 0 + self.num_mmmmmm_tracks ||
                self.currentsong == 7 + self.num_mmmmmm_tracks
            )
        ) {
            // Level Complete theme, no fade in or repeat
            if let Some(track) = self.musicTracks.get(t as usize) {
                match track.play(-1) {
                    Ok(v) => {
                        self.musicVolume = sdl2_sys::mixer::MIX_MAX_VOLUME as i32;
                    },
                    Err(s) => eprintln!("Mix_PlayMusic: {}", s),
                }
            }
        } else {
            if self.m_doFadeOutVol {
                // We're already fading out
                self.nicechange = t;
                self.nicefade = true;
                self.currentsong = -1;

                if self.quick_fade {
                    // fade out quicker
                    self.fadeMusicVolumeOut(500, game);
                } else {
                    self.quick_fade = true;
                }
            } else if let Some(track) = self.musicTracks.get(t as usize) {
                match track.play(-1) {
                    Ok(_) => {
                        self.fadeMusicVolumeIn(3000, game);
                    },
                    Err(s) => eprintln!("Mix_PlayMusic: {}", s),
                }
            }
        }
    }

    // void musicclass::resume()
    pub fn resume(&mut self) {
        unsafe {
            sdl2_sys::mixer::Mix_ResumeMusic();
        }
    }

    // void musicclass::resumefade(const int fadein_ms)
    pub fn resumefade(&mut self, fadein_ms: i32, game: &mut game::Game) {
        self.resume();
        self.fadeMusicVolumeIn(fadein_ms, game);
    }

    // void musicclass::fadein(void)
    pub fn fadein(&mut self, game: &mut game::Game) {
        self.resumefade(3000, game); // 3000 ms fadei
    }

    // void musicclass::pause(void)
    pub fn pause(&mut self) {
        unsafe {
            sdl2_sys::mixer::Mix_PauseMusic();
        }
    }

    // void musicclass::haltdasmusik(void)
    pub fn haltdasmusik(&mut self) {
        /* Just pauses music. This is intended. */
        self.pause();
        self.currentsong = -1;
        self.m_doFadeInVol = false;
        self.m_doFadeOutVol = false;
    }

    // void musicclass::silencedasmusik(void)
    pub fn silencedasmusik(&mut self) {
        self.musicVolume = 0;
    }

    // void musicclass::setfadeamount(const int fade_ms)
    pub fn setfadeamount(&mut self, fade_ms: i32, game: &mut game::Game) {
        self.FadeVolAmountPerFrame = if fade_ms == 0 {
            sdl2_sys::mixer::MIX_MAX_VOLUME as i32
        } else {
            sdl2_sys::mixer::MIX_MAX_VOLUME as i32 / (fade_ms / game.get_timestep() as i32)
        }
    }

    // void musicclass::fadeMusicVolumeIn(int ms)
    pub fn fadeMusicVolumeIn(&mut self, ms: i32, game: &mut game::Game) {
        self.m_doFadeInVol = true;
        self.m_doFadeOutVol = false;

        /* Ensure it starts at 0 */
        self.musicVolume = 0;

        /* Fix 1-frame glitch */
        unsafe {
            sdl2_sys::mixer::Mix_VolumeMusic(0);
        }

        self.setfadeamount(ms, game);
    }

    // void musicclass::fadeMusicVolumeOut(const int fadeout_ms)
    pub fn fadeMusicVolumeOut(&mut self, fadeout_ms: i32, game: &mut game::Game) {
        self.m_doFadeInVol = false;
        self.m_doFadeOutVol = true;
        self.setfadeamount(fadeout_ms, game);
    }

    // void musicclass::fadeout(const bool quick_fade_ /*= true*/)
    pub fn fadeout(&mut self, quick_fade_: Option<bool>, game: &mut game::Game) {
        let quick_fade_ = quick_fade_.unwrap_or(true);
        self.fadeMusicVolumeOut(2000, game);
        self.quick_fade = quick_fade_;
    }

    // void musicclass::processmusicfadein(void)
    pub fn processmusicfadein(&mut self) {
        self.musicVolume += self.FadeVolAmountPerFrame;
        if self.musicVolume >= sdl2_sys::mixer::MIX_MAX_VOLUME as i32 {
            self.m_doFadeInVol = false;
        }
    }

    // void musicclass::processmusicfadeout(void)
    pub fn processmusicfadeout(&mut self) {
        self.musicVolume -= self.FadeVolAmountPerFrame;
        if self.musicVolume < 0 {
            self.musicVolume = 0;
            self.m_doFadeOutVol = false;
            self.haltdasmusik();
        }
    }

    // void musicclass::processmusic(void)
    pub fn processmusic(&mut self, map: &mut map::Map, game: &mut game::Game) {
        if !self.safeToProcessMusic {
            return;
        }

        if self.m_doFadeInVol {
            self.processmusicfadein();
        }

        if self.m_doFadeOutVol {
            self.processmusicfadeout();
        }

        /* This needs to come after processing fades */
        if self.nicefade && unsafe { sdl2_sys::mixer::Mix_PausedMusic() } == 1 {
            self.play(self.nicechange, map, game);
            self.nicechange = -1;
            self.nicefade = false;
        }
    }

    // void musicclass::niceplay(int t)
    pub fn niceplay(&mut self, t: i32, game: &mut game::Game) {
        // important: do nothing if the correct song is playing!
        if (!self.mmmmmm && self.currentsong != t) ||
           (self.mmmmmm && self.usingmmmmmm && self.currentsong != t ) ||
           (self.mmmmmm && !self.usingmmmmmm && self.currentsong != t + self.num_mmmmmm_tracks
        ) {
            if self.currentsong != -1 {
                self.fadeout(Some(false), game);
            }
            self.nicefade = true;
            self.nicechange = t;
        }
    }

    // void musicclass::changemusicarea(int x, int y)
    pub fn changemusicarea(&mut self, x: i32, y: i32) {
        // println!("DEADBEEF: musicclass::changemusicarea() method is not implemented yet");
    }

    // void musicclass::playef(int t)
    pub fn playef(&mut self, t: usize) {
        // let channel = unsafe {
        //     if let Some(chunk) = self.soundTracks[t].sound {
        //         // sdl2_sys::mixer::Mix_PlayChannelTimed(-1, chunk, 0, -1)
        //     }
        // };
        // if channel == -1 {
        //     eprintln!("Unable to play WAV file");
        // } else if channel == -2 {
        //     eprintln!("sound is not ready");
        // }

        if let Some(chunk) = &self.soundTracks[t].sound {
            if let Err(s) = sdl2::mixer::Channel::all().play_timed(chunk, 0, -1) {
                eprintln!("{}", s);
            }
        }
    }

    // void musicclass::pauseef(void)
    pub fn pauseef(&mut self) {
        // println!("DEADBEEF: musicclass::pauseef() method is not implemented yet");
    }

    // void musicclass::resumeef(void)
    pub fn resumeef(&mut self) {
        // println!("DEADBEEF: musicclass::resumeef() method is not implemented yet");
    }

}
