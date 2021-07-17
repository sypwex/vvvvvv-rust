#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use std::ffi::{CStr, CString};
use ::std::os::raw;

#[macro_use]
extern crate log;

include!("../physfs_bindings.rs");

pub const VER_MAJOR: u32 = PHYSFS_VER_MAJOR;
pub const VER_MINOR: u32 = PHYSFS_VER_MINOR;
pub const VER_PATCH: u32 = PHYSFS_VER_PATCH;
pub const EnumerateCallbackResult_PHYSFS_ENUM_ERROR: PHYSFS_EnumerateCallbackResult = PHYSFS_EnumerateCallbackResult_PHYSFS_ENUM_ERROR;
pub const EnumerateCallbackResult_PHYSFS_ENUM_STOP: PHYSFS_EnumerateCallbackResult = PHYSFS_EnumerateCallbackResult_PHYSFS_ENUM_STOP;
pub const EnumerateCallbackResult_PHYSFS_ENUM_OK: PHYSFS_EnumerateCallbackResult = PHYSFS_EnumerateCallbackResult_PHYSFS_ENUM_OK;
pub const FileType_PHYSFS_FILETYPE_REGULAR: PHYSFS_FileType = PHYSFS_FileType_PHYSFS_FILETYPE_REGULAR;
pub const FileType_PHYSFS_FILETYPE_DIRECTORY: PHYSFS_FileType = PHYSFS_FileType_PHYSFS_FILETYPE_DIRECTORY;
pub const FileType_PHYSFS_FILETYPE_SYMLINK: PHYSFS_FileType = PHYSFS_FileType_PHYSFS_FILETYPE_SYMLINK;
pub const FileType_PHYSFS_FILETYPE_OTHER: PHYSFS_FileType = PHYSFS_FileType_PHYSFS_FILETYPE_OTHER;
pub const ErrorCode_PHYSFS_ERR_OK: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_OK;
pub const ErrorCode_PHYSFS_ERR_OTHER_ERROR: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_OTHER_ERROR;
pub const ErrorCode_PHYSFS_ERR_OUT_OF_MEMORY: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_OUT_OF_MEMORY;
pub const ErrorCode_PHYSFS_ERR_NOT_INITIALIZED: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_NOT_INITIALIZED;
pub const ErrorCode_PHYSFS_ERR_IS_INITIALIZED: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_IS_INITIALIZED;
pub const ErrorCode_PHYSFS_ERR_ARGV0_IS_NULL: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_ARGV0_IS_NULL;
pub const ErrorCode_PHYSFS_ERR_UNSUPPORTED: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_UNSUPPORTED;
pub const ErrorCode_PHYSFS_ERR_PAST_EOF: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_PAST_EOF;
pub const ErrorCode_PHYSFS_ERR_FILES_STILL_OPEN: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_FILES_STILL_OPEN;
pub const ErrorCode_PHYSFS_ERR_INVALID_ARGUMENT: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_INVALID_ARGUMENT;
pub const ErrorCode_PHYSFS_ERR_NOT_MOUNTED: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_NOT_MOUNTED;
pub const ErrorCode_PHYSFS_ERR_NOT_FOUND: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_NOT_FOUND;
pub const ErrorCode_PHYSFS_ERR_SYMLINK_FORBIDDEN: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_SYMLINK_FORBIDDEN;
pub const ErrorCode_PHYSFS_ERR_NO_WRITE_DIR: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_NO_WRITE_DIR;
pub const ErrorCode_PHYSFS_ERR_OPEN_FOR_READING: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_OPEN_FOR_READING;
pub const ErrorCode_PHYSFS_ERR_OPEN_FOR_WRITING: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_OPEN_FOR_WRITING;
pub const ErrorCode_PHYSFS_ERR_NOT_A_FILE: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_NOT_A_FILE;
pub const ErrorCode_PHYSFS_ERR_READ_ONLY: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_READ_ONLY;
pub const ErrorCode_PHYSFS_ERR_CORRUPT: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_CORRUPT;
pub const ErrorCode_PHYSFS_ERR_SYMLINK_LOOP: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_SYMLINK_LOOP;
pub const ErrorCode_PHYSFS_ERR_IO: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_IO;
pub const ErrorCode_PHYSFS_ERR_PERMISSION: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_PERMISSION;
pub const ErrorCode_PHYSFS_ERR_NO_SPACE: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_NO_SPACE;
pub const ErrorCode_PHYSFS_ERR_BAD_FILENAME: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_BAD_FILENAME;
pub const ErrorCode_PHYSFS_ERR_BUSY: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_BUSY;
pub const ErrorCode_PHYSFS_ERR_DIR_NOT_EMPTY: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_DIR_NOT_EMPTY;
pub const ErrorCode_PHYSFS_ERR_OS_ERROR: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_OS_ERROR;
pub const ErrorCode_PHYSFS_ERR_DUPLICATE: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_DUPLICATE;
pub const ErrorCode_PHYSFS_ERR_BAD_PASSWORD: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_BAD_PASSWORD;
pub const ErrorCode_PHYSFS_ERR_APP_CALLBACK: PHYSFS_ErrorCode = PHYSFS_ErrorCode_PHYSFS_ERR_APP_CALLBACK;

pub type uint8 = PHYSFS_uint8;
pub type sint8 = PHYSFS_sint8;
pub type uint16 = PHYSFS_uint16;
pub type sint16 = PHYSFS_sint16;
pub type uint32 = PHYSFS_uint32;
pub type sint32 = PHYSFS_sint32;
pub type uint64 = PHYSFS_uint64;
pub type sint64 = PHYSFS_sint64;
pub type uint8IsOneByte = PHYSFS_compile_time_assert_uint8IsOneByte;
pub type sint8IsOneByte = PHYSFS_compile_time_assert_sint8IsOneByte;
pub type uint16IsTwoBytes = PHYSFS_compile_time_assert_uint16IsTwoBytes;
pub type sint16IsTwoBytes = PHYSFS_compile_time_assert_sint16IsTwoBytes;
pub type uint32IsFourBytes = PHYSFS_compile_time_assert_uint32IsFourBytes;
pub type sint32IsFourBytes = PHYSFS_compile_time_assert_sint32IsFourBytes;
pub type uint64IsEightBytes = PHYSFS_compile_time_assert_uint64IsEightBytes;
pub type sint64IsEightBytes = PHYSFS_compile_time_assert_sint64IsEightBytes;
pub type StringCallback = PHYSFS_StringCallback;
pub type EnumFilesCallback = PHYSFS_EnumFilesCallback;
pub type EnumerateCallbackResult = PHYSFS_EnumerateCallbackResult;
pub type EnumerateCallback = PHYSFS_EnumerateCallback;
pub type FileType = PHYSFS_FileType;
pub type ErrorCode = PHYSFS_ErrorCode;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct File {
    pub opaque: *mut raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArchiveInfo {
    pub extension: *const raw::c_char,
    pub description: *const raw::c_char,
    pub author: *const raw::c_char,
    pub url: *const raw::c_char,
    pub supportsSymlinks: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub major: uint8,
    pub minor: uint8,
    pub patch: uint8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Allocator {
    pub Init: Option<unsafe extern "C" fn() -> i32>,
    pub Deinit: Option<unsafe extern "C" fn()>,
    pub Malloc: Option<unsafe extern "C" fn(arg1: uint64) -> *mut raw::c_void>,
    pub Realloc: Option<
        unsafe extern "C" fn(
            arg1: *mut raw::c_void,
            arg2: uint64,
        ) -> *mut raw::c_void,
    >,
    pub Free: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Stat {
    pub filesize: sint64,
    pub modtime: sint64,
    pub createtime: sint64,
    pub accesstime: sint64,
    pub filetype: FileType,
    pub readonly: i32,
}

pub struct Io {}

pub struct Archiver {}

/* static methods */

pub fn getLinkedVersion(ver: *mut Version) {
    unsafe {
        PHYSFS_getLinkedVersion(ver as *mut PHYSFS_Version);
    }
}

pub fn init(argv0: &str) -> Result<(), String> {
    trace!("physfs/init({}): entering", argv0);

    unsafe {
        let argv0 = CString::new(argv0).unwrap();
        let r = PHYSFS_init(argv0.as_ptr());

        if r == 0 {
            return Err(getLastErrorMessage())
        }

        Ok(())
    }
}

pub fn deinit() -> Result<(), String> {
    unsafe {
        let r = PHYSFS_deinit();

        if r == 0 {
            return Err(getLastErrorMessage())
        }

        Ok(())
    }
}

pub fn supportedArchiveTypes() -> ArchiveInfo {
    unsafe {
        let ai = PHYSFS_supportedArchiveTypes();
        ArchiveInfo {
            extension: (*(*ai)).extension,
            description: (*(*ai)).description,
            author: (*(*ai)).author,
            url: (*(*ai)).url,
            supportsSymlinks: (*(*ai)).supportsSymlinks,
        }
    }
}

pub fn freeList(listVar: *mut raw::c_void) {
    unsafe {
        PHYSFS_freeList(listVar);
    }
}

#[deprecated]
pub fn getLastError() -> String {
    unsafe {
        let s = PHYSFS_getLastError();
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }
}

pub fn getDirSeparator() -> String {
    unsafe {
        let s = PHYSFS_getDirSeparator();
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }
}

pub fn permitSymbolicLinks(allow: i32) {
    unsafe {
        PHYSFS_permitSymbolicLinks(allow)
    }
}

pub fn getCdRomDirs() -> *mut *mut raw::c_char {
    unsafe {
        PHYSFS_getCdRomDirs()
    }
}

pub fn getBaseDir() -> String {
    unsafe {
        let s = PHYSFS_getBaseDir();
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }
}

#[deprecated]
pub fn getUserDir() -> String {
    unsafe {
        let s = PHYSFS_getUserDir();
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }
}

pub fn getWriteDir() -> String {
    unsafe {
        let s = PHYSFS_getWriteDir();
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }
}

pub fn setWriteDir(newDir: &str) -> Result<(), String> {
    trace!("physfs/setWriteDir({}): entering", newDir);

    unsafe {
        let newDir = CString::new(newDir).unwrap();
        let r = PHYSFS_setWriteDir(newDir.as_ptr());

        if r == 0 {
            return Err(getLastErrorMessage())
        }

        Ok(())
    }
}

#[deprecated]
pub fn addToSearchPath(
    newDir: &str,
    appendToPath: i32,
) -> i32 {
    unsafe {
        let newDir = CString::new(newDir).unwrap();
        PHYSFS_addToSearchPath(newDir.as_ptr(), appendToPath)
    }
}

#[deprecated]
pub fn removeFromSearchPath(
    oldDir: &str,
) -> i32 {
    unsafe {
        let oldDir = CString::new(oldDir).unwrap();
        PHYSFS_removeFromSearchPath(oldDir.as_ptr())
    }
}

pub fn getSearchPath() -> *mut *mut raw::c_char {
    unsafe {
        PHYSFS_getSearchPath()
    }
}

pub fn setSaneConfig(
    organization: *const raw::c_char,
    appName: *const raw::c_char,
    archiveExt: *const raw::c_char,
    includeCdRoms: i32,
    archivesFirst: i32,
) -> i32 {
    unsafe {
        PHYSFS_setSaneConfig(organization, appName, archiveExt, includeCdRoms, archivesFirst)
    }
}

pub fn mkdir(dirName: &str) -> i32 {
    unsafe {
        let dirName = CString::new(dirName).unwrap();
        PHYSFS_mkdir(dirName.as_ptr())
    }
}

pub fn delete(filename: &str) -> i32 {
    unsafe {
        let filename = CString::new(filename).unwrap();
        PHYSFS_delete(filename.as_ptr())
    }
}

pub fn getRealDir(
    filename: &str,
) -> String {
    unsafe {
        let filename = CString::new(filename).unwrap();
        let s = PHYSFS_getRealDir(filename.as_ptr());
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }
}

pub fn enumerateFiles(
    dir: &str,
) -> *mut *mut raw::c_char {
    unsafe {
        let dir = CString::new(dir).unwrap();
        PHYSFS_enumerateFiles(dir.as_ptr())
    }
}

pub fn exists(filename: &str) -> bool {
    trace!("physfs/exists({}): entering", filename);

    unsafe {
        let filenamec = CString::new(filename).unwrap();
        let exists = PHYSFS_exists(filenamec.as_ptr()) != 0;

        trace!("physfs/exists({}) -> {}", filename, exists);
        exists
    }
}

#[deprecated]
pub fn isDirectory(filename: &str) -> i32 {
    unsafe {
        let filename = CString::new(filename).unwrap();
        PHYSFS_isDirectory(filename.as_ptr())
    }
}

#[deprecated]
pub fn isSymbolicLink(filename: &str) -> i32 {
    unsafe {
        let filename = CString::new(filename).unwrap();
        PHYSFS_isSymbolicLink(filename.as_ptr())
    }
}

#[deprecated]
pub fn getLastModTime(filename: &str) -> PHYSFS_sint64 {
    unsafe {
        let filename = CString::new(filename).unwrap();
        PHYSFS_getLastModTime(filename.as_ptr())
    }
}

pub fn openWrite(filename: &str) -> *mut File {
    unsafe {
        let filename = CString::new(filename).unwrap();
        PHYSFS_openWrite(filename.as_ptr()) as *mut File
    }
}

pub fn openAppend(filename: &str) -> *mut File {
    unsafe {
        let filename = CString::new(filename).unwrap();
        PHYSFS_openAppend(filename.as_ptr()) as *mut File
    }
}

pub fn openRead(filename: &str) -> Result<*mut File, String> {
    trace!("physfs/openRead({}): entering", filename);

    unsafe {
        let filenamec = CString::new(filename).unwrap();
        let handle = PHYSFS_openRead(filenamec.as_ptr()) as *mut File;

        if handle.is_null() {
            return Err(getLastErrorMessage())
        }

        trace!("physfs/openRead({}): -> {:?}", filename, handle);
        Ok(handle)
    }
}

pub fn close(handle: *mut File) -> Result<(), String> {
    trace!("physfs/close({:?}): entering", handle);

    unsafe {
        let r = PHYSFS_close(handle as *mut PHYSFS_File);

        if r == 0 {
            return Err(getLastErrorMessage())
        }

        Ok(())
    }
}

#[deprecated]
pub fn read(
    handle: *mut File,
    buffer: *mut raw::c_void,
    objSize: PHYSFS_uint32,
    objCount: PHYSFS_uint32,
) -> PHYSFS_sint64 {
    unsafe {
        PHYSFS_read(handle as *mut PHYSFS_File, buffer, objSize, objCount)
    }
}

#[deprecated]
pub fn write(
    handle: *mut File,
    buffer: *const raw::c_void,
    objSize: PHYSFS_uint32,
    objCount: PHYSFS_uint32,
) -> PHYSFS_sint64 {
    unsafe {
        PHYSFS_write(handle as *mut PHYSFS_File, buffer, objSize, objCount)
    }
}

pub fn eof(handle: *mut File) -> i32 {
    unsafe {
        PHYSFS_eof(handle as *mut PHYSFS_File)
    }
}

pub fn tell(handle: *mut File) -> PHYSFS_sint64 {
    unsafe {
        PHYSFS_tell(handle as *mut PHYSFS_File)
    }
}

pub fn seek(handle: *mut File, pos: usize) -> Result<i32, String> {
    trace!("physfs/seek({:?},{}): entering", handle, pos);

    unsafe {
        let r = PHYSFS_seek(handle as *mut PHYSFS_File, pos as u64);

        if r == 0 {
            return Err(getLastErrorMessage())
        }

        trace!("physfs/seek({:?},{}) -> {}", handle, pos, r);
        Ok(r)
    }
}

pub fn fileLength(handle: *mut File) -> Option<usize> {
    trace!("physfs/fileLength({:?}): entering", handle);

    unsafe {
        let size = PHYSFS_fileLength(handle as *mut PHYSFS_File);

        if size == -1 {
            error!("physfs/fileLength({:?}): unable to get file length", handle);
            None
        } else {
            trace!("physfs/fileLength({:?}): file length is {}", handle, size);
            Some(size as usize)
        }
    }
}

pub fn setBuffer(
    handle: *mut File,
    bufsize: PHYSFS_uint64,
) -> i32 {
    unsafe {
        PHYSFS_setBuffer(handle as *mut PHYSFS_File, bufsize)
    }
}

pub fn flush(handle: *mut File) -> i32 {
    unsafe {
        PHYSFS_flush(handle as *mut PHYSFS_File)
    }
}

pub fn swapSLE16(val: PHYSFS_sint16) -> PHYSFS_sint16 {
    unsafe {
        PHYSFS_swapSLE16(val)
    }
}

pub fn swapULE16(val: PHYSFS_uint16) -> PHYSFS_uint16 {
    unsafe {
        PHYSFS_swapULE16(val)
    }
}

pub fn swapSLE32(val: PHYSFS_sint32) -> PHYSFS_sint32 {
    unsafe {
        PHYSFS_swapSLE32(val)
    }
}

pub fn swapULE32(val: PHYSFS_uint32) -> PHYSFS_uint32 {
    unsafe {
        PHYSFS_swapULE32(val)
    }
}

pub fn swapSLE64(val: PHYSFS_sint64) -> PHYSFS_sint64 {
    unsafe {
        PHYSFS_swapSLE64(val)
    }
}

pub fn swapULE64(val: PHYSFS_uint64) -> PHYSFS_uint64 {
    unsafe {
        PHYSFS_swapULE64(val)
    }
}

pub fn swapSBE16(val: PHYSFS_sint16) -> PHYSFS_sint16 {
    unsafe {
        PHYSFS_swapSBE16(val)
    }
}

pub fn swapUBE16(val: PHYSFS_uint16) -> PHYSFS_uint16 {
    unsafe {
        PHYSFS_swapUBE16(val)
    }
}

pub fn swapSBE32(val: PHYSFS_sint32) -> PHYSFS_sint32 {
    unsafe {
        PHYSFS_swapSBE32(val)
    }
}

pub fn swapUBE32(val: PHYSFS_uint32) -> PHYSFS_uint32 {
    unsafe {
        PHYSFS_swapUBE32(val)
    }
}

pub fn swapSBE64(val: PHYSFS_sint64) -> PHYSFS_sint64 {
    unsafe {
        PHYSFS_swapSBE64(val)
    }
}

pub fn swapUBE64(val: PHYSFS_uint64) -> PHYSFS_uint64 {
    unsafe {
        PHYSFS_swapUBE64(val)
    }
}

pub fn readSLE16(
    file: *mut File,
    val: *mut PHYSFS_sint16,
) -> i32 {
    unsafe {
        PHYSFS_readSLE16(file as *mut PHYSFS_File, val)
    }
}

pub fn readULE16(
    file: *mut File,
    val: *mut PHYSFS_uint16,
) -> i32 {
    unsafe {
        PHYSFS_readULE16(file as *mut PHYSFS_File, val)
    }
}

pub fn readSBE16(
    file: *mut File,
    val: *mut PHYSFS_sint16,
) -> i32 {
    unsafe {
        PHYSFS_readSBE16(file as *mut PHYSFS_File, val)
    }
}

pub fn readUBE16(
    file: *mut File,
    val: *mut PHYSFS_uint16,
) -> i32 {
    unsafe {
        PHYSFS_readUBE16(file as *mut PHYSFS_File, val)
    }
}

pub fn readSLE32(
    file: *mut File,
    val: *mut PHYSFS_sint32,
) -> i32 {
    unsafe {
        PHYSFS_readSLE32(file as *mut PHYSFS_File, val)
    }
}

pub fn readULE32(
    file: *mut File,
    val: *mut PHYSFS_uint32,
) -> i32 {
    unsafe {
        PHYSFS_readULE32(file as *mut PHYSFS_File, val)
    }
}

pub fn readSBE32(
    file: *mut File,
    val: *mut PHYSFS_sint32,
) -> i32 {
    unsafe {
        PHYSFS_readSBE32(file as *mut PHYSFS_File, val)
    }
}

pub fn readUBE32(
    file: *mut File,
    val: *mut PHYSFS_uint32,
) -> i32 {
    unsafe {
        PHYSFS_readUBE32(file as *mut PHYSFS_File, val)
    }
}

pub fn readSLE64(
    file: *mut File,
    val: *mut PHYSFS_sint64,
) -> i32 {
    unsafe {
        PHYSFS_readSLE64(file as *mut PHYSFS_File, val)
    }
}

pub fn readULE64(
    file: *mut File,
    val: *mut PHYSFS_uint64,
) -> i32 {
    unsafe {
        PHYSFS_readULE64(file as *mut PHYSFS_File, val)
    }
}

pub fn readSBE64(
    file: *mut File,
    val: *mut PHYSFS_sint64,
) -> i32 {
    unsafe {
        PHYSFS_readSBE64(file as *mut PHYSFS_File, val)
    }
}

pub fn readUBE64(
    file: *mut File,
    val: *mut PHYSFS_uint64,
) -> i32 {
    unsafe {
        PHYSFS_readUBE64(file as *mut PHYSFS_File, val)
    }
}

pub fn writeSLE16(file: *mut File, val: PHYSFS_sint16) -> i32 {
    unsafe {
        PHYSFS_writeSLE16(file as *mut PHYSFS_File, val)
    }
}

pub fn writeULE16(file: *mut File, val: PHYSFS_uint16) -> i32 {
    unsafe {
        PHYSFS_writeULE16(file as *mut PHYSFS_File, val)
    }
}

pub fn writeSBE16(file: *mut File, val: PHYSFS_sint16) -> i32 {
    unsafe {
        PHYSFS_writeSBE16(file as *mut PHYSFS_File, val)
    }
}

pub fn writeUBE16(file: *mut File, val: PHYSFS_uint16) -> i32 {
    unsafe {
        PHYSFS_writeUBE16(file as *mut PHYSFS_File, val)
    }
}

pub fn writeSLE32(file: *mut File, val: PHYSFS_sint32) -> i32 {
    unsafe {
        PHYSFS_writeSLE32(file as *mut PHYSFS_File, val)
    }
}

pub fn writeULE32(file: *mut File, val: PHYSFS_uint32) -> i32 {
    unsafe {
        PHYSFS_writeULE32(file as *mut PHYSFS_File, val)
    }
}

pub fn writeSBE32(file: *mut File, val: PHYSFS_sint32) -> i32 {
    unsafe {
        PHYSFS_writeSBE32(file as *mut PHYSFS_File, val)
    }
}

pub fn writeUBE32(file: *mut File, val: PHYSFS_uint32) -> i32 {
    unsafe {
        PHYSFS_writeUBE32(file as *mut PHYSFS_File, val)
    }
}

pub fn writeSLE64(file: *mut File, val: PHYSFS_sint64) -> i32 {
    unsafe {
        PHYSFS_writeSLE64(file as *mut PHYSFS_File, val)
    }
}

pub fn writeULE64(file: *mut File, val: PHYSFS_uint64) -> i32 {
    unsafe {
        PHYSFS_writeULE64(file as *mut PHYSFS_File, val)
    }
}

pub fn writeSBE64(file: *mut File, val: PHYSFS_sint64) -> i32 {
    unsafe {
        PHYSFS_writeSBE64(file as *mut PHYSFS_File, val)
    }
}

pub fn writeUBE64(file: *mut File, val: PHYSFS_uint64) -> i32 {
    unsafe {
        PHYSFS_writeUBE64(file as *mut PHYSFS_File, val)
    }
}

pub fn isInit() -> bool {
    unsafe {
        PHYSFS_isInit() == 0
    }
}

pub fn symbolicLinksPermitted() -> i32 {
    unsafe {
        PHYSFS_symbolicLinksPermitted()
    }
}

pub fn setAllocator(allocator: *const Allocator) -> i32 {
    unsafe {
        PHYSFS_setAllocator(allocator as *const PHYSFS_Allocator)
    }
}

pub fn mount(
    newDir: &str,
    mountPoint: Option<&str>,
    appendToPath: i32,
) -> Result<(), String> {
    trace!("physfs/mount({},{:?},{}): entering", newDir, mountPoint, appendToPath);

    unsafe {
        let newDirC = CString::new(newDir).unwrap();
        let mountPoint = match mountPoint {
            Some(s) => {
                let s = CString::new(s).unwrap();
                s.as_ptr()
            },
            None => std::ptr::null(),
        };

        let r = PHYSFS_mount(newDirC.as_ptr(), mountPoint, appendToPath);

        if r == 0 {
            let s = format!("{} while opening {}", getLastErrorMessage(), newDir);
            return Err(s)
        }

        Ok(())
    }
}

pub fn getMountPoint(
    dir: &str,
) -> String {
    unsafe {
        let dir = CString::new(dir).unwrap();
        let s = PHYSFS_getMountPoint(dir.as_ptr());
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }
}

pub fn getCdRomDirsCallback(c: PHYSFS_StringCallback, d: *mut raw::c_void) {
    unsafe {
        PHYSFS_getCdRomDirsCallback(c, d)
    }
}

pub fn getSearchPathCallback(c: PHYSFS_StringCallback, d: *mut raw::c_void) {
    unsafe {
        PHYSFS_getSearchPathCallback(c, d)
    }
}

#[deprecated]
pub fn enumerateFilesCallback(
    dir: &str,
    c: PHYSFS_EnumFilesCallback,
    d: *mut raw::c_void,
) {
    unsafe {
        let dir = CString::new(dir).unwrap();
        PHYSFS_enumerateFilesCallback(dir.as_ptr(), c, d)
    }
}

pub fn utf8FromUcs4(
    src: *const PHYSFS_uint32,
    dst: *mut raw::c_char,
    len: PHYSFS_uint64,
) {
    unsafe {
        PHYSFS_utf8FromUcs4(src, dst, len)
    }
}

pub fn utf8ToUcs4(
    src: *const raw::c_char,
    dst: *mut PHYSFS_uint32,
    len: PHYSFS_uint64,
) {
    unsafe {
        PHYSFS_utf8ToUcs4(src, dst, len)
    }
}

pub fn utf8FromUcs2(
    src: *const PHYSFS_uint16,
    dst: *mut raw::c_char,
    len: PHYSFS_uint64,
) {
    unsafe {
        PHYSFS_utf8FromUcs2(src, dst, len)
    }
}

pub fn utf8ToUcs2(
    src: *const raw::c_char,
    dst: *mut PHYSFS_uint16,
    len: PHYSFS_uint64,
) {
    unsafe {
        PHYSFS_utf8ToUcs2(src, dst, len)
    }
}

pub fn utf8FromLatin1(
    src: *const raw::c_char,
    dst: *mut raw::c_char,
    len: PHYSFS_uint64,
) {
    unsafe {
        PHYSFS_utf8FromLatin1(src, dst, len)
    }
}

pub fn caseFold(from: PHYSFS_uint32, to: *mut PHYSFS_uint32) -> i32 {
    unsafe {
        PHYSFS_caseFold(from, to)
    }
}

pub fn utf8stricmp(
    str1: *const raw::c_char,
    str2: *const raw::c_char,
) -> i32 {
    unsafe {
        PHYSFS_utf8stricmp(str1, str2)
    }
}

pub fn utf16stricmp(
    str1: *const PHYSFS_uint16,
    str2: *const PHYSFS_uint16,
) -> i32 {
    unsafe {
        PHYSFS_utf16stricmp(str1, str2)
    }
}

pub fn ucs4stricmp(
    str1: *const PHYSFS_uint32,
    str2: *const PHYSFS_uint32,
) -> i32 {
    unsafe {
        PHYSFS_ucs4stricmp(str1, str2)
    }
}

pub fn enumerate(
    dir: *const raw::c_char,
    c: PHYSFS_EnumerateCallback,
    d: *mut raw::c_void,
) -> i32 {
    unsafe {
        PHYSFS_enumerate(dir, c, d)
    }
}

pub fn unmount(oldDir: *const raw::c_char) -> i32 {
    unsafe {
        PHYSFS_unmount(oldDir)
    }
}

pub fn getAllocator() -> *const Allocator {
    unsafe {
        PHYSFS_getAllocator() as *const Allocator
    }
}

pub fn stat(
    filename: *const raw::c_char,
    stat: *mut Stat,
) -> i32 {
    unsafe {
        PHYSFS_stat(filename, stat as *mut PHYSFS_Stat)
    }
}

pub fn utf8FromUtf16(
    src: *const PHYSFS_uint16,
    dst: *mut raw::c_char,
    len: PHYSFS_uint64,
) {
    unsafe {
        PHYSFS_utf8FromUtf16(src, dst, len)
    }
}

pub fn utf8ToUtf16(
    src: *const raw::c_char,
    dst: *mut PHYSFS_uint16,
    len: PHYSFS_uint64,
) {
    unsafe {
        PHYSFS_utf8ToUtf16(src, dst, len)
    }
}

pub fn readBytes(
    handle: *mut File,
    buffer: *mut raw::c_void,
    len: usize,
) -> PHYSFS_sint64 {
    trace!("physfs/readBytes({:?},{:?},{}): entering", handle, buffer, len);

    unsafe {
        PHYSFS_readBytes(handle as *mut PHYSFS_File, buffer, len as u64)
    }
}

pub fn writeBytes(
    handle: *mut File,
    buffer: *const raw::c_void,
    len: usize,
) -> PHYSFS_sint64 {
    unsafe {
        PHYSFS_writeBytes(handle as *mut PHYSFS_File, buffer, len as u64)
    }
}

pub fn mountIo(
    io: *mut PHYSFS_Io,
    newDir: *const raw::c_char,
    mountPoint: *const raw::c_char,
    appendToPath: i32,
) -> i32 {
    unsafe {
        PHYSFS_mountIo(io, newDir, mountPoint, appendToPath)
    }
}

pub fn mountMemory(
    buf: *const raw::c_void,
    len: PHYSFS_uint64,
    del: ::std::option::Option<unsafe extern "C" fn(arg1: *mut raw::c_void)>,
    newDir: *const raw::c_char,
    mountPoint: *const raw::c_char,
    appendToPath: i32,
) -> i32 {
    unsafe {
        PHYSFS_mountMemory(buf, len, del, newDir, mountPoint, appendToPath)
    }
}

pub fn mountHandle(
    file: *mut File,
    newDir: *const raw::c_char,
    mountPoint: *const raw::c_char,
    appendToPath: i32,
) -> i32 {
    unsafe {
        PHYSFS_mountHandle(file as *mut PHYSFS_File, newDir, mountPoint, appendToPath)
    }
}

pub fn getLastErrorCode() -> PHYSFS_ErrorCode {
    unsafe {
        PHYSFS_getLastErrorCode()
    }
}

pub fn getErrorByCode(code: PHYSFS_ErrorCode) -> String {
    unsafe {
        let s = PHYSFS_getErrorByCode(code);
        CStr::from_ptr(s).to_string_lossy().into_owned()
    }
}

pub fn setErrorCode(code: PHYSFS_ErrorCode) {
    unsafe {
        PHYSFS_setErrorCode(code)
    }
}

pub fn getPrefDir(
    org: &str,
    app: &str,
) -> Result<String, String> {
    trace!("physfs/getPrefDir({},{})", org, app);

    unsafe {
        let _org = CString::new(org).unwrap();
        let _app = CString::new(app).unwrap();
        let s = PHYSFS_getPrefDir(_org.as_ptr(), _app.as_ptr());

        if s.is_null() {
            Err(getLastErrorMessage())
        } else {
            let s = CStr::from_ptr(s).to_string_lossy().into_owned();
            trace!("physfs/getPrefDir({},{}) -> {}", org, app, s);
            Ok(s)
        }
    }
}

pub fn registerArchiver(archiver: *const PHYSFS_Archiver) -> i32 {
    unsafe {
        PHYSFS_registerArchiver(archiver)
    }
}

pub fn deregisterArchiver(ext: *const raw::c_char) -> i32 {
    unsafe {
        PHYSFS_deregisterArchiver(ext)
    }
}

/* new functions */

pub fn getLastErrorMessage() -> String {
    let s = getErrorByCode(getLastErrorCode());
    error!("physfs/getLastErrorMessage() -> {}", s);
    s
}

#[cfg(test)]
mod tests {
}
