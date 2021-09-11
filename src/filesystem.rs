use std::{fs::create_dir_all, path::{Path, PathBuf}};

use quick_xml::Error;

use crate::sdl2u;
extern crate physfs;
extern crate sdl2_sys;

pub struct FileSystem {
    saveDir: Option<PathBuf>,
    levelDir: Option<PathBuf>,
    assetDir: Option<&'static Path>,
    virtualMountPath: Option<&'static Path>,
    allocator: physfs::Allocator,
}

// static void* bridged_malloc(PHYSFS_uint64 size)
fn bridged_malloc(size: physfs::uint64) -> *mut ::std::os::raw::c_void {
    #[cfg(not(target_family = "windows"))]
    unsafe {
        sdl2_sys::SDL_malloc(size)
    }

    #[cfg(target_family = "windows")]
    unsafe {
        sdl2_sys::SDL_malloc(size as u32)
    }
}

// static void* bridged_realloc(void* ptr, PHYSFS_uint64 size)
fn bridged_realloc(ptr: *mut ::std::os::raw::c_void, size: physfs::uint64) -> *mut ::std::os::raw::c_void {
    #[cfg(not(target_family = "windows"))]
    unsafe {
        sdl2_sys::SDL_realloc(ptr, size)
    }

    #[cfg(target_family = "windows")]
    unsafe {
        sdl2_sys::SDL_realloc(ptr, size as u32)
    }
}

impl FileSystem {
    // int FILESYSTEM_init(char *argvZero, char* baseDir, char *assetsPath)
    pub fn new(argvZero: String, baseDir: Option<&Path>, assetsPath: Option<&Path>) -> Result<Self, String> {
        #[cfg(not(target_family = "windows"))]
        let allocator = physfs::Allocator {
            Init: None,
            Deinit: None,
            // Malloc: Some(bridged_malloc),
            Malloc: Some(sdl2_sys::SDL_malloc),
            Realloc: Some(sdl2_sys::SDL_realloc),
            Free: Some(sdl2_sys::SDL_free),
        };
        #[cfg(target_family = "windows")]
        let allocator = physfs::Allocator {
            Init: None,
            Deinit: None,
            // Malloc: Some(bridged_malloc),
            Malloc: Some(sdl2_sys::SDL_malloc),
            Realloc: Some(sdl2_sys::SDL_realloc),
            Free: Some(sdl2_sys::SDL_free),
        };

        let dirSeparator = physfs::getDirSeparator();
        let pathSep = Path::new(&dirSeparator);

        physfs::setAllocator(&allocator);

        if let Err(s) = physfs::init(&argvZero) {
            return Err(format!("Unable to initialize PhysFS({:?}): {}", argvZero, s));
        }

        physfs::permitSymbolicLinks(1);

        // @sx interesting pattern matching point
        /* Determine the OS user directory */
        let output = match baseDir {
            Some(baseDir) if baseDir.exists() => {
                /* We later append to this path and assume it ends in a slash */
                if baseDir.ends_with(pathSep) {
                    baseDir.to_path_buf()
                } else {
                    baseDir.join(pathSep)
                }
            },
            _ => {
                match PLATFORM_getOSDirectory() {
                    Ok(s) => PathBuf::from(s),
                    Err(s) => return Err(s),
                }
            },
        };

        /* Mount our base user directory */
        if let Err(s) = physfs::mount(output.to_str().unwrap(), None, 0) {
            return Err(format!("Could not mount {:?}: {}", output, s));
        }
        if let Err(s) = physfs::setWriteDir(output.to_str().unwrap()) {
            return Err(format!("Could not set write dir to {:?}: {}", output, physfs::getLastErrorMessage()));
        }
        info!("Base directory: {}", output.to_string_lossy());

        /* Store full save directory */
        let saveDir = output.join("saves/");
        match create_dir_all(&saveDir) {
            Ok(_) => info!("Save directory available @{}", saveDir.to_string_lossy()),
            Err(s) => error!("Unable to create save dir with message: {}", s),
        };

        /* Store full level directory */
        let levelDir = output.join("levels/");
        match create_dir_all(&levelDir) {
            Ok(_) => info!("Level directory available @{}", levelDir.to_string_lossy()),
            Err(s) => error!("Unable to create level dir with message: {}", s),
        };

        // TODO: @sx save migration TBI
        /* We didn't exist until now, migrate files! */
        // if VNEEDS_MIGRATION {
        //     PLATFORM_migrateSaveData(output);
        // }

        let basePath = match sdl2::filesystem::base_path() {
            Ok(path) => PathBuf::from(path),
            Err(s) => panic!("Unable to get base path! {}", s),
        };

        /* Mount the stock content last */
        let output = match assetsPath {
            Some(assetsPath) if assetsPath.exists() => output.join(assetsPath),
            _ => basePath.join("data.zip"),
        };
        info!("expecting assets @{}", output.to_string_lossy());

        if let Err(s) = physfs::mount(output.to_str().unwrap(), None, 1) {
            eprintln!("filesystem.rs: {}", s);
            eprintln!("Error: data.zip missing!
                        You do not have data.zip!
                        Grab it from your purchased copy of the game,
                        or get it from the free Make and Play Edition.");

            if let Err(m) = sdl2::messagebox::show_simple_message_box(
                sdl2::messagebox::MessageBoxFlag::ERROR,
                "data.zip missing!",
                "You do not have data.zip!
                    Grab it from your purchased copy of the game
                    or get it from the free Make and Play Edition.",
                None
            ) {
                eprintln!("unable to show SDL messagebox: {}", m);
            }
            return Err("data.zip missing!".to_string())
        }

        unsafe {
            let output = basePath.join("gamecontrollerdb.txt");
            if let Ok(rw) = sdl2::rwops::RWops::from_file(output, "rb") {
                if sdl2_sys::SDL_GameControllerAddMappingsFromRW(rw.raw(), 1) == -1 {
                    eprintln!("gamecontrollerdb.txt not found!... {}", sdl2u::get_error());
                }
            };
        }

        return Ok(Self {
            saveDir: Some(saveDir),
            levelDir: Some(levelDir),
            assetDir: None,
            virtualMountPath: None,
            allocator,
        })
    }

    // static int PLATFORM_getOSDirectory(char* output, const size_t output_size);
    // static void PLATFORM_migrateSaveData(char* output);
    // static void PLATFORM_copyFile(const char *oldLocation, const char *newLocation);
    // static void* bridged_malloc(PHYSFS_uint64 size)
    // static void* bridged_realloc(void* ptr, PHYSFS_uint64 size)

    // char *FILESYSTEM_getUserSaveDirectory(void)
    pub fn FILESYSTEM_getUserSaveDirectory(&self) -> Option<&PathBuf> {
        self.saveDir.as_ref()
    }

    // char *FILESYSTEM_getUserLevelDirectory(void)
    // bool FILESYSTEM_isFile(const char* filename)
    // bool FILESYSTEM_isMounted(const char* filename)
    // static bool FILESYSTEM_exists(const char *fname)
    // static void generateVirtualMountPath(char* path, const size_t path_size)
    // static bool FILESYSTEM_mountAssetsFrom(const char *fname)
    // void FILESYSTEM_loadZip(const char* filename)
    // void FILESYSTEM_mountAssets(const char* path)
    // void FILESYSTEM_unmountAssets(void)
    // static void getMountedPath(char* buffer, const size_t buffer_size, const char* filename)
    pub fn getMountedPath(&mut self, filename: &str) -> PathBuf {
        let assets_mounted = self.assetDir != None;

        let mounted_path = if assets_mounted && self.virtualMountPath.is_some() {
            self.virtualMountPath.unwrap().join(filename)
        } else {
            PathBuf::new()
        };

        if assets_mounted && physfs::exists(mounted_path.to_str().unwrap()) {
            mounted_path
        } else {
            PathBuf::from(filename)
        }
    }

    // bool FILESYSTEM_isAssetMounted(const char* filename)
    // void FILESYSTEM_freeMemory(unsigned char **mem);
    // void FILESYSTEM_loadFileToMemory(const char *name, unsigned char **mem, size_t *len, bool addnull)
    pub fn FILESYSTEM_loadFileToMemory() {
       warn!("DEADBEEF: FILESYSTEM_loadFileToMemory method not implemented yet");
    }

    // void FILESYSTEM_loadAssetToMemory(const char* name, unsigned char** mem, size_t* len, const bool addnull)
    // void FILESYSTEM_freeMemory(unsigned char **mem)

    // bool FILESYSTEM_saveTiXml2Document(const char *name, tinyxml2::XMLDocument& doc)
    pub fn FILESYSTEM_saveTiXml2Document(&mut self, name: &str, doc: Vec<u8>) -> bool {
        let saveDir = self.saveDir.clone().unwrap().clone().to_str().unwrap().to_owned();
        let path = saveDir + name;
        info!("FILESYSTEM_saveTiXml2Document: writing {} bytes into {}", doc.len(), path);
        std::fs::write(path, doc).expect("error while writing save");

        true
    }

    // bool FILESYSTEM_loadTiXml2Document(const char *name, tinyxml2::XMLDocument& doc)
    pub fn FILESYSTEM_loadTiXml2Document(&mut self, name: &str) -> Result<quick_xml::Reader<std::io::BufReader<std::fs::File>>, Error> {
        // TODO: @sx WTF, needs refactor...
        let saveDir = self.saveDir.clone().unwrap().clone().to_str().unwrap().to_owned();
        let path = saveDir + name;
        info!("loading save file from {:?}", path);
        quick_xml::Reader::from_file(path)
    }
    pub fn savefile_exists(&mut self, name: &str) -> bool {
        let saveDir = self.saveDir.clone().unwrap().clone().to_str().unwrap().to_owned();
        let path = saveDir + name;
        Path::new(&path).exists()
    }

    // pub fn get_savefile_writer(&mut self, name: &str) -> quick_xml::Writer<std::io::BufWriter<std::fs::File>> {
    //     let saveDir = self.saveDir.clone().unwrap().clone().to_str().unwrap().to_owned();
    //     let path = saveDir + name;
    //     quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
    // }

    // static PHYSFS_EnumerateCallbackResult enumerateCallback(void* data, const char* origdir, const char* filename)
    // void FILESYSTEM_enumerateLevelDirFileNames(void (*callback)(const char* filename))
    // bool FILESYSTEM_openDirectoryEnabled(void)
    // bool FILESYSTEM_openDirectory(const char *dname)
    // bool FILESYSTEM_delete(const char *name)
}

impl Drop for FileSystem {
    fn drop(&mut self) {
        if physfs::isInit() {
            physfs::deinit();
        }
    }
}

// static int PLATFORM_getOSDirectory(char* output, const size_t output_size)
fn PLATFORM_getOSDirectory() -> Result<String, String> {
    // // #ifdef _WIN32
    //     /* This block is here for compatibility, do not touch it! */
    //     WCHAR utf16_path[MAX_PATH];
    //     HRESULT retcode = SHGetFolderPathW(
    //         None,
    //         CSIDL_PERSONAL,
    //         None,
    //         SHGFP_TYPE_CURRENT,
    //         utf16_path
    //     );
    //
    //     if FAILED(retcode) {
    //         let ret = format!("Could not get OS directory: SHGetFolderPathW returned {}", retcode);
    //         return Err(ret);
    //     }
    //
    //     let num_bytes = WideCharToMultiByte(
    //         CP_UTF8,
    //         0,
    //         utf16_path,
    //         -1,
    //         output,
    //         output_size,
    //         None,
    //         None
    //     );
    //     if num_bytes == 0 {
    //         let ret = format!("Could not get OS directory: UTF-8 conversion failed with {}", GetLastError());
    //         return Err(ret);
    //     }
    //
    //     output = format!("{}{}", output, "\\VVVVVV\\");
    //     mkdir(output, 0777);
    //
    //     return Ok();
    // // #else
        match physfs::getPrefDir(&"distractionware", &"VVVVVV") {
            Ok(prefDir) => Ok(prefDir),
            Err(s) => Err(format!("Could not get OS directory: {:?}", physfs::getLastErrorMessage())),
        }
    // #endif
}

// static void PLATFORM_migrateSaveData(char* output)
fn PLATFORM_migrateSaveData(output: &str) {
    println!("DEADBEEF: PLATFORM_migrateSaveData is not implemented yet");
}

// static void PLATFORM_copyFile(const char *oldLocation, const char *newLocation)
fn PLATFORM_copyFile(oldLocation: &str, newLocation: &str) {
    println!("DEADBEEF: PLATFORM_copyFile is not implemented yet");
}
