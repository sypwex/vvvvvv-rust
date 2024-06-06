use std::{mem, path::PathBuf};

use crate::{INBOUNDS_ARR, sdl2u};

use super::sound_system::MusicTrack;

pub const max_headers: usize = 128;

#[repr(C)]
#[derive(Debug,Clone)]
struct resourceheader_c {
	name: [i8;48],
	start_UNUSED: i32,
	size: i32,
	valid: bool,
}

impl Default for resourceheader_c {
    fn default() -> Self {
        Self {
            name: [0;48],
            start_UNUSED: 0,
            size: 0,
            valid: false,
        }
    }
}

#[derive(Debug,Clone)]
pub struct resourceheader {
	name: String,
	start_UNUSED: i32,
	size: usize,
	valid: bool,
}

impl Default for resourceheader {
    fn default() -> Self {
        Self {
            name: String::new(),
            start_UNUSED: 0,
            size: 0,
            valid: false,
        }
    }
}

impl resourceheader {
    fn from_c(rh: &resourceheader_c) -> Self {
        let name = unsafe {
            let bytes = std::ffi::CStr::from_ptr(rh.name.as_ptr() as *const i8).to_bytes();
            String::from_utf8_lossy(bytes).to_string()
        };

        Self {
            name,
            start_UNUSED: rh.start_UNUSED,
            size: rh.size as usize,
            valid: rh.valid,
        }
    }

}

pub struct BinaryBlob {
	numberofHeaders: i32,
	pub m_headers: Vec<resourceheader>,
	pub m_memblocks: Vec<Vec<u8>>,
}

impl BinaryBlob {
    // binaryBlob::binaryBlob(void) {
    pub fn new() -> Self {
        // broken implementation
        // m_headers: [resourceheader; max_headers],
        // pub m_memblocks: [Vec<u8>; max_headers],
        // let m_headers = unsafe {
        //     let mut data: [mem::MaybeUninit<resourceheader>; max_headers] = mem::MaybeUninit::uninit().assume_init();
        //     for el in &mut data[..] {
        //         std::ptr::write(el.as_mut_ptr(), resourceheader {
        //             name: String::new(),
        //             start_UNUSED: 0,
        //             size: 0,
        //             valid: false,
        //         });
        //     }
        //     mem::transmute::<_, [resourceheader; max_headers]>(data)
        // };
        //
        // let m_memblocks = unsafe {
        //     let mut data: [mem::MaybeUninit<Vec<u8>>; max_headers] = mem::MaybeUninit::uninit().assume_init();
        //     for el in &mut data[..] {
        //         std::ptr::write(el.as_mut_ptr(), vec![0]);
        //     }
        //     mem::transmute::<_, [Vec<u8>; max_headers]>(data)
        // };

        let m_headers = vec![resourceheader::default(); max_headers];
        let m_memblocks = vec![vec![]; max_headers];
        trace!("0 header: {:?}", m_headers[0]);

        Self {
            numberofHeaders: 0,
            m_headers,
            m_memblocks,
        }
    }

    //     #ifdef VVV_COMPILEMUSIC
    //     // void binaryBlob::AddFileToBinaryBlob(const char* _path)
    //     fn AddFileToBinaryBlob(const char* _path) {
    //         long size;
    //         char * memblock;
    //
    //         FILE *file = fopen(_path, "rb");
    //         if (file != NULL) {
    //             fseek(file, 0, SEEK_END);
    //             size = ftell(file);
    //             fseek(file, 0, SEEK_SET);
    //
    //             memblock = (char*) SDL_malloc(size);
    //             if (memblock == NULL) {
    //                 VVV_exit(1);
    //             }
    //             fread(memblock, 1, size, file);
    //
    //             fclose(file);
    //
    //             trace!("The complete file size: %li", size);
    //
    //             self.m_memblocks[numberofHeaders] = memblock;
    //             for (int i = 0; _path[i]; i += 1) {
    //                 self.m_headers[numberofHeaders].name[i] = _path[i];
    //             }
    //
    //             self.m_headers[numberofHeaders].valid = true;
    //             self.m_headers[numberofHeaders].size = size;
    //             numberofHeaders += 1;
    //         } else {
    //             error!("Unable to open file");
    //         }
    //     }
    //
    //     // void binaryBlob::writeBinaryBlob(const char* _name)
    //     fn writeBinaryBlob(name: &str) {
    //         FILE *file = fopen(_name, "wb");
    //         if (file != NULL) {
    //             fwrite((char*) &self.m_headers, 1, sizeof(self.m_headers), file);
    //
    //             for (int i = 0; i < numberofHeaders; i += 1) {
    //                 fwrite(self.m_memblocks[i], 1, self.m_headers[i].size, file);
    //             }
    //
    //             fclose(file);
    //         } else {
    //             error!("Unable to open new file for writing. Feels bad.");
    //         }
    //     }
    //     #endif

    // bool binaryBlob::unPackBinary(const char* name)
    pub fn unPackBinary(&mut self, filepath: PathBuf) -> bool {
        self.FILESYSTEM_loadBinaryBlob(filepath)
    }

    // void binaryBlob::clear(void)
    fn clear(&mut self, ) {
        // unsafe {
        //     for i in 0..self.m_headers.len() {
        //         sdl2_sys::SDL_free(self.m_memblocks[i]);
        //     }
        //     sdl2_sys::SDL_zeroa(self.m_memblocks);
        //     sdl2_sys::SDL_zeroa(self.m_headers);
        // }
    }

    // int binaryBlob::getIndex(const char* _name)
    pub fn getIndex(&mut self, _name: &str) -> Option<usize> {
        for i in 0..self.m_headers.len() {
            if (_name == self.m_headers[i].name) && self.m_headers[i].valid {
                return Some(i);
            }
        }

        return None;
    }

    // int binaryBlob::getSize(int _index)
    pub fn getSize(&mut self, _index: usize) -> usize {
        if !INBOUNDS_ARR!(_index, self.m_headers) {
            warn!("getSize() out-of-bounds!");
            return 0;
        }

        return self.m_headers[_index].size;
    }

    // char* binaryBlob::getAddress(int _index)
    pub fn validAddress(&mut self, _index: usize) -> bool {
        if !INBOUNDS_ARR!(_index, self.m_memblocks) {
            warn!("getAddress() out-of-bounds!");
            false
        } else {
            true
        }
    }

    // bool binaryBlob::nextExtra(size_t* start)
    pub fn nextExtra(&mut self, start: usize) -> bool {
        for idx in start..self.m_headers.len() {
            if self.m_headers[idx].valid {
                return true
            }
        }
        return false
    }

    /* methods from filesystem.rs */

    // bool FILESYSTEM_loadBinaryBlob(binaryBlob* blob, const char* filename)
    fn FILESYSTEM_loadBinaryBlob(&mut self, filepath: std::path::PathBuf) -> bool {
        trace!("0 header: {:?}", self.m_headers[0]);
        trace!("attempting to load {:?}", filepath);
        let mut offset;

        let handle = match physfs::openRead(filepath.to_str().unwrap()) {
            Ok(handle) => handle,
            Err(s) => {
                error!("Unable to open file {:?}", filepath);
                return false;
            },
        };

        let size = handle.fileLength().unwrap();

        let mut headers = vec![resourceheader_c::default(); max_headers];

        handle.readBytes(
            headers.as_mut_ptr() as *mut ::std::os::raw::c_void,
            mem::size_of::<[resourceheader_c; max_headers]>()
        );

        offset = mem::size_of::<[resourceheader_c; max_headers]>();

        for i in 0..headers.len() {
            self.m_headers[i] = resourceheader::from_c(&headers[i]);
            trace!("current header: {:?}", self.m_headers[i]);

            /* Name can be stupid, just needs to be terminated */
            // let last_char = mem::size_of(self.m_headers[i].name) - 1;
            // self.m_headers[i].name[last_char] = '\0';

            if /* Must be nonzero and positive */
                self.m_headers[i].size < 1 ||
                /* Bogus size value */
                offset + self.m_headers[i].size > size {
                self.m_headers[i].valid = false;
                break;
            }

            if let Err(s) = handle.seek(offset) {
                panic!("failure while seeking offset {} for physfs handle {:?}", offset, handle);
            }
            self.m_memblocks[i] = match sdl2u::malloc(self.m_headers[i].size) {
                Ok(data) => {
                    handle.readBytes(data.as_ptr() as *mut libc::c_void, self.m_headers[i].size);
                    data
                },
                Err(_) => {
                    panic!("Oh god we're out of memory, just bail");
                    // VVV_exit(1); /* Oh god we're out of memory, just bail */
                },
            };

            offset += self.m_headers[i].size;
            continue;
        }

        for i in 0..self.m_headers.len() {
            if !self.m_headers[i].valid {
                continue;
            }

            info!("{} unpacked", self.m_headers[i].name);
        }

        return true;
    }

    pub fn getMusicTrack(&mut self, index: usize) -> Result<MusicTrack, String> {
        let size = self.getSize(index);
        match MusicTrack::from_blob(&self.m_memblocks[index], size) {
            Ok(track) => Ok(track),
            Err(_) => Err(format!("unable to load track with index {}", index)),
        }

        // let rw = unsafe {
        //     let raw = sdl2_sys::SDL_RWFromConstMem(self.m_memblocks[index].as_ptr() as *const libc::c_void, size as i32);
        //     if raw.is_null() {
        //         return Err("unable to get rwops object".to_string())
        //     }

        //     sdl2::rwops::RWops::from_ll(raw)
        // };


        // MusicTrack::from_rwops(&rw, 1)
    }

}
