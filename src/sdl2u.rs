use std::{ptr, slice::Iter};

extern crate sdl2;
use sdl2::{controller::Button, rect::Rect, surface::Surface};
use sdl2_sys::{SDL_PixelFormat, SDL_Surface};

/*
this mod holds abstractions over unsafe sdl2 methods in order to make it safe and not to fork sdl2
*/

// wrappers over sdl2_sys

pub fn sdl_create_rgb_surface(surface: &sdl2::surface::SurfaceRef, width: u32, height: u32) -> *mut SDL_Surface {
    unsafe {
        let flags = (*surface.raw()).flags;
        let format = *(*surface.raw()).format;

        sdl_create_rgb_surface_(
        flags,
        width as i32,
        height as i32,
        format.BitsPerPixel.into(),
        format.Rmask,
        format.Gmask,
        format.Bmask,
        format.Amask)
    }
}

pub fn sys_upper_blit<R1, R2>(
    src: &sdl2::surface::SurfaceRef,
    src_rect: R1,
    dst: &mut sdl2::surface::SurfaceRef,
    dst_rect: R2
) where R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>> {
    let src_rect = src_rect.into();
    let dst_rect = dst_rect.into();

    unsafe {
        let src_rect_ptr = src_rect.as_ref().map(|r| r.raw()).unwrap_or(ptr::null());
        let mut dst_rect = dst_rect;
        let dst_rect_ptr = dst_rect.as_mut().map(|r| r.raw_mut())
            .unwrap_or(ptr::null_mut());

        sdl2_sys::SDL_UpperBlit(src.raw(), src_rect_ptr, dst.raw(), dst_rect_ptr);
    }
}

// sdl2_sys crate

pub fn sys_fmodf (x: f32, y: f32) -> f32 {
    unsafe {
        sdl2_sys::SDL_fmodf(x, y)
    }
}

pub fn sys_map_rgb (format: *const SDL_PixelFormat, r: u8, g: u8, b: u8) -> u32 {
    unsafe {
        sdl2_sys::SDL_MapRGB(format, r as u8, g as u8, b as u8)
    }
}

pub fn sdl_create_rgb_surface_(flags: u32, width: libc::c_int, height: libc::c_int, depth: libc::c_int, Rmask: u32, Gmask: u32, Bmask: u32, Amask: u32) -> *mut SDL_Surface {
    unsafe {
        sdl2_sys::SDL_CreateRGBSurface(flags, width, height, depth, Rmask, Gmask, Bmask, Amask)
    }
}

pub fn malloc(size: usize) -> Result<Vec<u8>, ()> {
    debug!("sdl2u/malloc({})", size);

    if size == 0 {
        Ok(vec![0;0])
    } else {
        unsafe {
            #[cfg(not(target_family = "windows"))]
            let ptr = sdl2_sys::SDL_malloc(size as u64) as *mut u8;
            #[cfg(target_family = "windows")]
            let ptr = sdl2_sys::SDL_malloc(size as u32) as *mut u8;

            if ptr.is_null() {
                error!("SDL_malloc returned empty pointer!");
                Err(())
            } else {
                Ok(Vec::from_raw_parts(ptr, size, size))
            }
        }
    }
}

// sdl2 crate

pub fn surface_from_ll<'a> (raw: *mut SDL_Surface) -> Surface<'a> {
    unsafe {
        Surface::from_ll(raw)
    }
}

pub fn get_error() -> String {
    unsafe {
        let err = sdl2_sys::SDL_GetError();
        std::ffi::CStr::from_ptr(err as *const _).to_str().unwrap().to_owned()
    }
}

// sdl2 controller crate

pub trait ButtonIter {
    fn iterator() -> Iter<'static, Button>;
}

impl ButtonIter for Button {
    fn iterator() -> std::slice::Iter<'static, Button> {
        static BUTTONS: [Button; 11] = [
            Button::A,
            Button::B,
            Button::X,
            Button::Y,
            Button::Back,
            Button::Guide,
            Button::Start,
            Button::LeftStick,
            Button::RightStick,
            Button::LeftShoulder,
            Button::RightShoulder,
        ];
        BUTTONS.iter()
    }
}
