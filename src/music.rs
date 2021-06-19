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

	currentsong: i32,
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
    // void musicclass::destroy(void)
    // void musicclass::play(int t)
    pub fn play (&mut self, t: i32) {
        // println!("DEADBEEF: musicclass::play is not implemented yet");
    }

    // void musicclass::resume()
    // void musicclass::resumefade(const int fadein_ms)
    // void musicclass::fadein(void)
    // void musicclass::pause(void)
    // void musicclass::haltdasmusik(void)
    // void musicclass::silencedasmusik(void)
    // void musicclass::setfadeamount(const int fade_ms)
    // void musicclass::fadeMusicVolumeIn(int ms)
    // void musicclass::fadeMusicVolumeOut(const int fadeout_ms)
    // void musicclass::fadeout(const bool quick_fade_ /*= true*/)
    // void musicclass::processmusicfadein(void)
    // void musicclass::processmusicfadeout(void)
    // void musicclass::processmusic(void)
    pub fn processmusic (&mut self) {
        // println!("DEADBEEF: musicclass::processmusic is not implemented yet");
    }

    // void musicclass::niceplay(int t)
    // void musicclass::changemusicarea(int x, int y)
    // void musicclass::playef(int t)
    pub fn playef (&mut self, t: i32) {
        // println!("DEADBEEF: musicclass::playef is not implemented yet");
    }

    // void musicclass::pauseef(void)
    // void musicclass::resumeef(void)
}
