/* The amount of "space" for the scale of the user-set volume. */
pub const USER_VOLUME_MAX: i32 = 256;

/* It is advised that USER_VOLUME_MAX be divisible by this. */
pub const USER_VOLUME_STEP: i32 = 32;


pub struct Music {
	safeToProcessMusic: bool,
	m_doFadeInVol: bool,
	m_doFadeOutVol: bool,
	musicVolume: i32,
	FadeVolAmountPerFrame: i32,

	user_music_volume: i32,
	user_sound_volume: i32,

	pub currentsong: i32,
	nicechange: i32,
	nicefade: bool,
	quick_fade: bool,

	// MMMMMM mod settings
	pub mmmmmm: bool,
	pub usingmmmmmm: bool,

	// binaryBlob pppppp_blob;
	// binaryBlob mmmmmm_blob;
	// int num_pppppp_tracks;
}

impl Music {
    // musicclass::musicclass(void)
    pub fn new() -> Self {
        Music {
            safeToProcessMusic: false,
            m_doFadeInVol: false,
            m_doFadeOutVol: false,
            musicVolume: 0,
            FadeVolAmountPerFrame: 0,

            user_music_volume: USER_VOLUME_MAX,
            user_sound_volume: USER_VOLUME_MAX,

            currentsong: 0,
            nicechange: -1,
            nicefade: false,
            quick_fade: true,

        	// MMMMMM mod settings
	        mmmmmm: false,
            usingmmmmmm: false,
        }
    }

    // void musicclass::init(void)
    pub fn init(&mut self) {
        println!("DEADBEEF: musicclass::init() method is not implemented yet");
    }

    // void musicclass::destroy(void)
    pub fn destroy(&mut self) {
        println!("DEADBEEF: musicclass::destroy() method is not implemented yet");
    }

    // void musicclass::play(int t)
    pub fn play(&mut self, t: i32) {
        println!("DEADBEEF: musicclass::play() method is not implemented yet");
    }

    // void musicclass::resume()
    pub fn resume(&mut self) {
        println!("DEADBEEF: musicclass::resume() method not implemented yet");
    }

    // void musicclass::resumefade(const int fadein_ms)
    pub fn resumefade(&mut self, fadein_ms: u32) {
        println!("DEADBEEF: musicclass::resumefade() method not implemented yet");
    }

    // void musicclass::fadein(void)
    pub fn fadein(&mut self) {
        println!("DEADBEEF: musicclass::fadein() method not implemented yet");
    }

    // void musicclass::pause(void)
    pub fn pause(&mut self) {
        println!("DEADBEEF: musicclass::pause() method not implemented yet");
    }

    // void musicclass::haltdasmusik(void)
    pub fn haltdasmusik(&mut self) {
        println!("DEADBEEF: musicclass::haltdasmusik() method not implemented yet");
    }

    // void musicclass::silencedasmusik(void)
    pub fn silencedasmusik(&mut self) {
        println!("DEADBEEF: musicclass::silencedasmusik() method not implemented yet");
    }

    // void musicclass::setfadeamount(const int fade_ms)
    pub fn setfadeamount(&mut self, fade_ms: u32) {
        println!("DEADBEEF: musicclass::setfadeamount() method not implemented yet");
    }

    // void musicclass::fadeMusicVolumeIn(int ms)
    pub fn fadeMusicVolumeIn(&mut self, ms: u32) {
        println!("DEADBEEF: musicclass::fadeMusicVolumeIn() method not implemented yet");
    }

    // void musicclass::fadeMusicVolumeOut(const int fadeout_ms)
    pub fn fadeMusicVolumeOut(&mut self, fadeout_ms: u32) {
        println!("DEADBEEF: musicclass::fadeMusicVolumeOut() method not implemented yet");
    }

    // void musicclass::fadeout(const bool quick_fade_ /*= true*/)
    pub fn fadeout(&mut self, quick_fade_: Option<bool>) {
        let quick_fade_ = quick_fade_.unwrap_or(true);
        println!("DEADBEEF: musicclass::fadeout() method not implemented yet");
    }

    // void musicclass::processmusicfadein(void)
    pub fn processmusicfadein(&mut self) {
        println!("DEADBEEF: musicclass::processmusicfadein() method not implemented yet");
    }

    // void musicclass::processmusicfadeout(void)
    pub fn processmusicfadeout(&mut self) {
        println!("DEADBEEF: musicclass::processmusicfadeout() method not implemented yet");
    }

    // void musicclass::processmusic(void)
    pub fn processmusic(&mut self) {
        // println!("DEADBEEF: musicclass::processmusic() method is not implemented yet");
    }

    // void musicclass::niceplay(int t)
    pub fn niceplay(&mut self, t: i32) {
        // println!("DEADBEEF: musicclass::niceplay() method is not implemented yet");
    }

    // void musicclass::changemusicarea(int x, int y)
    pub fn changemusicarea(&mut self, x: i32, y: i32) {
        // println!("DEADBEEF: musicclass::changemusicarea() method is not implemented yet");
    }

    // void musicclass::playef(int t)
    pub fn playef(&mut self, t: i32) {
        // println!("DEADBEEF: musicclass::playef() method is not implemented yet");
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
