use sdl2::mixer::LoaderRWops;

pub struct MusicTrack {
    // pub m_music: sdl2::mixer::Music,
    pub m_music: *mut sdl2_sys::mixer::_Mix_Music,
}

pub struct SoundTrack {
    pub sound: Option<sdl2::mixer::Chunk>,
}

pub struct SoundSystem {
}

impl MusicTrack {
    // MusicTrack::MusicTrack(const char* fileName)
    pub fn from_filename(fileName: &str) -> Result<sdl2::mixer::Music, String> {
        let m_music = match sdl2::mixer::Music::from_file(fileName) {
            Ok(m) => m,
            Err(s) => {
                eprintln!("Unable to load Ogg Music file: {}", s);
                return Err(format!("Unable to load Ogg Music file: {}", s))
            },
        };

        Ok(m_music)
    }

    // MusicTrack::MusicTrack(SDL_RWops *rw)
    pub fn from_rwops(rw: &'static sdl2::rwops::RWops<'static>, freesrc: i32) -> Result<sdl2::mixer::Music, String> {
        let m_music = match rw.load_music(freesrc) {
            Ok(m) => m,
            Err(s) => {
                eprintln!("Unable to load Ogg Music file: {}", s);
                return Err(format!("Unable to load Ogg Music file: {}", s))
            },
        };

        Ok(m_music)
    }

    pub fn from_blob(mem: &Vec<u8>, size: usize) -> Result<Self, String> {
        unsafe {
            info!("sound_system::from_blob(): loading music track from blob size {}, reading size {}", mem.len(), size);
            let rw = sdl2_sys::SDL_RWFromConstMem(mem.as_ptr() as *const libc::c_void, size as i32);
            let m_music = sdl2_sys::mixer::Mix_LoadMUS_RW(rw, 1);

            Ok(Self {
                m_music
            })
        }
    }

    pub fn play(&self, loops: i32) -> Result<(), String> {
        let ret = unsafe {
            sdl2_sys::mixer::Mix_PlayMusic(self.m_music, loops)
        };

        if ret == -1 {
            Err("unable to play track".to_string())
        } else {
            Ok(())
        }
    }

}

impl SoundTrack {
    // SoundTrack::SoundTrack(const char* fileName)
    pub fn new(fileName: &str) -> Self {
        let sound = match physfs::openRead(&fileName) {
            Ok(handle) => {
                let length = handle.fileLength().unwrap();
                let mut buffer = vec![0u8; length as usize];
                handle.readBytes(buffer.as_mut_ptr() as *mut std::ffi::c_void, length);

                let data = sdl2::rwops::RWops::from_bytes(&buffer).ok();
                data.and_then(|data| data.load_wav().ok())
            },
            Err(e) => panic!("Unable to load WAV file: {}: {}", e, fileName),
        };

        Self {
            sound
        }
    }
}

impl SoundSystem {
    // SoundSystem::SoundSystem(void)
    pub fn SoundSystem() {
        let audio_rate = 44100;
        let audio_format = sdl2::mixer::AUDIO_S16SYS;
        let audio_channels = 2;
        let audio_buffers = 1024;

        if let Err(s) = sdl2::mixer::open_audio(audio_rate, audio_format, audio_channels, audio_buffers) {
            eprintln!("Unable to initialize audio: {}", s);
            // TODO: @sx what to do with SDL_assert???
            // sdl2::SDL_assert(0 && "Unable to initialize audio!");
        };
    }
}
