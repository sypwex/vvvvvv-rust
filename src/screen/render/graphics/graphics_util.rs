extern crate sdl2;
extern crate sdl2_sys;
use crate::{maths, sdl2u, utility_class};

pub struct ColourTransform {
    pub colour: u32,
    // Uint32 colour;
}

impl ColourTransform {
    pub fn new () -> ColourTransform {
        ColourTransform {
            colour: 0,
        }
    }

    pub fn set_colour (&mut self, c: u32) {
        self.colour = c;
    }
}

// void setRect( SDL_Rect& _r, int x, int y, int w, int h )
pub fn setRect(x: i32, y: i32, w: u32, h: u32) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(x, y, w, h)
}

// static SDL_Surface* RecreateSurfaceWithDimensions(SDL_Surface* surface, const int width, const int height)
pub fn RecreateSurfaceWithDimensions(surface: &sdl2::surface::SurfaceRef, width: u32, height: u32) -> sdl2::surface::Surface {
    let retsurface = sdl2u::sdl_create_rgb_surface(surface, width, height);
    let mut retval = sdl2u::surface_from_ll(retsurface);
    retval.set_blend_mode(surface.blend_mode());

    retval
}

// static SDL_Surface* RecreateSurface(SDL_Surface* surface)
pub fn RecreateSurface(surface: &sdl2::surface::SurfaceRef) -> sdl2::surface::Surface {
    RecreateSurfaceWithDimensions(
        surface,
        surface.width(),
        surface.height()
    )
}

// SDL_Surface* GetSubSurface( SDL_Surface* metaSurface, int x, int y, int width, int height )
pub fn GetSubSurface(metaSurface: &sdl2::surface::SurfaceRef, x: i32, y: i32, width: u32, height: u32) -> sdl2::surface::Surface {
    // Create an SDL_Rect with the area of the _surface
    let area = sdl2::rect::Rect::new(
        x, y, width, height,
    );

    //Convert to the correct display format after nabbing the new _surface or we will slow things down.
    let mut preSurface = RecreateSurfaceWithDimensions(metaSurface, width, height);

    // Lastly, apply the area from the meta _surface onto the whole of the sub _surface.
    // Return the new Bitmap _surface
    match metaSurface.blit(area, &mut preSurface, None) {
        Ok(_rect) => preSurface,
        Err(s) => panic!("error while recreating subsurface: {:?}", s),
    }
}

// static void DrawPixel( SDL_Surface *_surface, int x, int y, Uint32 pixel )
pub fn DrawPixel(surface: &mut sdl2::surface::SurfaceRef, x: i32, y: i32, pixel: u32) {
    unsafe {
        let raw_surface = *surface.raw();
        let format = *raw_surface.format;
        let pixels = raw_surface.pixels as *mut u8;

        let bpp = format.BytesPerPixel as i32;
        /* Here p is the address to the pixel we want to set */
        // let p = pixels + y*raw_surface.pitch + x*bpp;
        let p = pixels.offset((y*raw_surface.pitch + x*bpp) as isize);

        match bpp {
            1 => {
                *p = pixel as u8;
            },
            2 => {
                *(p as *mut u16) = pixel as u16;
            },
            3 => {
                *p = ((pixel >> 16) & 0xff) as u8;
                *(p.offset(1) as *mut u8) = ((pixel >> 8) & 0xff) as u8;
                *(p.offset(2) as *mut u8) = (pixel & 0xff) as u8;
            },
            4 => {
                *(p as *mut u32) = pixel;
            },
            _ => {},
        }
    }
}

// Uint32 ReadPixel( SDL_Surface *_surface, int x, int y )
pub fn ReadPixel(surface: &sdl2::surface::SurfaceRef, x: i32, y: i32) -> u32 {
    unsafe {
        let raw_surface = *surface.raw();
        let format = *raw_surface.format;
        let pixels = raw_surface.pixels as *mut u8;

        let bpp = format.BytesPerPixel as i32;
        /* Here p is the address to the pixel we want to retrieve */
        let p = pixels.offset((y*raw_surface.pitch + x*bpp) as isize);

        match bpp {
            1 => *p as u32,
            2 => (*p as u32) | ((*p.offset(1) as u32) << 8),
            3 => ((*p as u32) | (*p.offset(1) as u32) << 8 | (*p.offset(2) as u32) << 16) as u32,
            4 => ((*p as u32) | (*p.offset(1) as u32) << 8 | (*p.offset(2) as u32) << 16 | (*p.offset(3) as u32) << 24) as u32,
            _ => 0,
        }
    }
}
pub fn ReadPixel_Color(surface: &sdl2::surface::SurfaceRef, x: i32, y: i32) -> sdl2::pixels::Color {
    unsafe {
        let raw_surface = *surface.raw();
        let format = *raw_surface.format;
        let pixels = raw_surface.pixels as *mut u8;

        let bpp = format.BytesPerPixel as i32;
        /* Here p is the address to the pixel we want to retrieve */
        let p = pixels.offset((y*raw_surface.pitch + x*bpp) as isize);

        sdl2::pixels::Color::from_u32(&surface.pixel_format(), match bpp {
            1 => *p as u32,
            2 => (*p as u32) | ((*p.offset(1) as u32) << 8),
            3 => ((*p as u32) | (*p.offset(1) as u32) << 8 | (*p.offset(2) as u32) << 16) as u32,
            4 => ((*p as u32) | (*p.offset(1) as u32) << 8 | (*p.offset(2) as u32) << 16 | (*p.offset(3) as u32) << 24) as u32,
            _ => 0,
        })
    }
}

// SDL_Surface * ScaleSurface( SDL_Surface *_surface, int Width, int Height, SDL_Surface * Dest )
pub fn ScaleSurface(_surface: &sdl2::surface::SurfaceRef, Width: u32, Height: u32) -> sdl2::surface::Surface {
    let mut _ret = RecreateSurfaceWithDimensions(_surface, Width, Height);

    let _stretch_factor_x = (Width / _surface.width()) as i32;
    let _stretch_factor_y = (Height / _surface.height()) as i32;

    for y in 0.._surface.height() as i32 {
        for x in 0.._surface.width() as i32 {
            let gigantoPixel = setRect(x *_stretch_factor_x, y *_stretch_factor_y, _stretch_factor_x as u32, _stretch_factor_y as u32);
            // sdl2::SDL_FillRect(_ret, &gigantoPixel, ReadPixel(_surface, x, y));
            _ret.fill_rect(gigantoPixel, ReadPixel_Color(_surface, x, y));
        }
    }

    _ret
}

// SDL_Surface *  FlipSurfaceVerticle(SDL_Surface* _src)
pub fn FlipSurfaceVerticle(src: &sdl2::surface::SurfaceRef) -> sdl2::surface::Surface {
    let mut ret = RecreateSurface(src);

    for y in 0..(src.height() as i32) {
        for x in 0..(src.width() as i32) {
            DrawPixel(&mut ret, x, (src.height() as i32 - 1) - y, ReadPixel(src, x, y));
        }
    }

    ret
}

// void BlitSurfaceStandard( SDL_Surface* _src, SDL_Rect* _srcRect, SDL_Surface* _dest, SDL_Rect* _destRect )
pub fn BlitSurfaceStandard() {
    // SDL_BlitSurface( _src, _srcRect, _dest, _destRect );
    // use _src.blit(_srcRect, _dest, _destRect);
}

// void BlitSurfaceColoured(SDL_Surface* _src, SDL_Rect* _srcRect, SDL_Surface* _dest, SDL_Rect* _destRect, colourTransform& ct)
pub fn BlitSurfaceColoured<R1>(
    _src: &sdl2::surface::SurfaceRef,
    _srcRect: R1,
    _dest: &mut sdl2::surface::SurfaceRef,
    _destRect: sdl2::rect::Rect,
    colourTransformValue: u32
) where R1: Into<Option<sdl2::rect::Rect>> {
    // SDL_Surface* tempsurface = RecreateSurface(_src);
    let mut tempsurface = RecreateSurface(_src);

    // const SDL_PixelFormat& fmt = *(_src->format);
    let fmt;
    unsafe {
        fmt = *_src.pixel_format().raw();
    }

    let width = tempsurface.width() as i32;
    let height = tempsurface.height() as i32;

    let mut x = 0;
    while x < width {
        let mut y = 0;
        while y < height {
            // Uint32 pixel = ReadPixel(_src, x, y);
            let pixel = ReadPixel(_src, x, y);
            // Uint32 Alpha = pixel & fmt.Amask;
            let Alpha = pixel & fmt.Amask;
            // Uint32 result = ct.colour & 0x00FFFFFF;
            let result = colourTransformValue & 0x00FFFFFF;
            // Uint32 CTAlpha = ct.colour & fmt.Amask;
            let CTAlpha = colourTransformValue & fmt.Amask;
            // float div1 = ((Alpha >> 24) / 255.0f);
            let div1 = ((Alpha >> 24) as f32) / 255.0f32;
            // float div2 = ((CTAlpha >> 24) / 255.0f);
            let div2 = ((CTAlpha >> 24) as f32) / 255.0f32;
            // Uint32 UseAlpha = (div1 * div2) * 255.0f;
            let UseAlpha: u32 = (div1 * div2 * 255.0f32) as u32;
            // DrawPixel(tempsurface, x, y, result | (UseAlpha << 24));
            DrawPixel(tempsurface.as_mut(), x, y, result | (UseAlpha << 24));

            y += 1;
        }

        x += 1;
    }

    match tempsurface.blit(_srcRect, _dest, _destRect) {
        Ok(_rect) => (),
        Err(s) => println!("err: {:?}", s),
    }

    // sdl2_sys::SDL_FreeSurface(**tempsurface);
    drop(tempsurface.raw());
}

// void BlitSurfaceTinted(SDL_Surface* _src, SDL_Rect* _srcRect, SDL_Surface* _dest, SDL_Rect* _destRect, colourTransform& ct)
pub fn BlitSurfaceTinted<R1>(
    _src: &sdl2::surface::SurfaceRef,
    _srcRect: R1,
    _dest: &mut sdl2::surface::SurfaceRef,
    _destRect: sdl2::rect::Rect,
    ct: u32
) where R1: Into<Option<sdl2::rect::Rect>> {

}

/* Fill Rect */
// TODO: @sx refactor

// void FillRect( SDL_Surface* _surface, const int _x, const int _y, const int _w, const int _h, const int r, int g, int b )

// void FillRect( SDL_Surface* _surface, const int r, int g, int b )

// void FillRect( SDL_Surface* _surface, const int color )
pub fn FillRectWithColor(surface: &mut sdl2::surface::SurfaceRef, color: sdl2::pixels::Color) {
    let rect = sdl2::rect::Rect::new(0, 0, surface.width(), surface.height());
    surface.fill_rect(rect, color);
}

// void FillRect( SDL_Surface* _surface, const int x, const int y, const int w, const int h, int rgba )
pub fn FillRect(_surface: &mut sdl2::surface::SurfaceRef, x: u32, y: u32, w: u32, h: u32, rgba: sdl2::pixels::Color) {
    let rect = sdl2::rect::Rect::new(x as i32, y as i32, w, h);
    _surface.fill_rect(rect, rgba);
}

// void FillRect( SDL_Surface* _surface, SDL_Rect& _rect, const int r, int g, int b )
pub fn FillRect_rect_rgb(_surface: &mut sdl2::surface::SurfaceRef, rect: sdl2::rect::Rect, r: i32, g: i32, b: i32) {
    unsafe {
        let rgba = sdl2_sys::SDL_MapRGB(_surface.pixel_format().raw(), r as u8, g as u8, b as u8);
        let color = sdl2::pixels::Color::from_u32(&_surface.pixel_format(), rgba);
        FillRect(_surface, rect.x as u32, rect.y as u32, rect.w as u32, rect.h as u32, color);
    }
}

// void FillRect( SDL_Surface* _surface, SDL_Rect rect, int rgba )
pub fn FillRect_rect(_surface: &mut sdl2::surface::SurfaceRef, rect: sdl2::rect::Rect, rgba: sdl2::pixels::Color) {
    FillRect(_surface, rect.x as u32, rect.y as u32, rect.w as u32, rect.h as u32, rgba);
}
pub fn FillRect_rect_coloru32(_surface: &mut sdl2::surface::SurfaceRef, rect: sdl2::rect::Rect, rgba: u32) {
    let color = sdl2::pixels::Color::from_u32(&_surface.pixel_format(), rgba);
    FillRect(_surface, rect.x as u32, rect.y as u32, rect.w as u32, rect.h as u32, color);
}

/* */

// void ClearSurface(SDL_Surface* surface)
pub fn ClearSurface(surface: &mut sdl2::surface::SurfaceRef) {
    // SDL_FillRect(surface, NULL, 0x00000000);
    let rect_dst = sdl2::rect::Rect::new(0, 0, surface.width(), surface.height());
    match surface.fill_rect(rect_dst, sdl2::pixels::Color::RGBA(0, 0, 0, 0)) {
        Ok(_x) => (),
        Err(s) => panic!("{}", s),
    };
}

// void ScrollSurface( SDL_Surface* _src, int _pX, int _pY )
pub fn ScrollSurface (src: &mut sdl2::surface::SurfaceRef, pX: i32, pY: i32) {
    // TODO: @sx create Surface::clone() method
    let src_width = src.width();
    let src_height = src.height();
    let ret = sdl2u::sdl_create_rgb_surface(src, src_width, src_height);
    let mut ret = sdl2u::surface_from_ll(ret);
    ret.set_blend_mode(src.blend_mode());
    src.blit(None, &mut ret, None)
        .expect("unable to clone surface");

    let res = match (pX, pY) {
        (_, py) if py < 0 => {
            //scrolling up;
            let rect2 = setRect(0, 0, src_width, src_height - pY as u32);
            let mut part1 = GetSubSurface(&ret, rect2.x, rect2.y, rect2.w as u32, rect2.h as u32);
            part1.set_blend_mode(sdl2::render::BlendMode::None);
            let dst_rect = setRect(0,  pY as i32, pX as u32, src_height);
            Some((part1, dst_rect))
        },
        (_, py) if py > 0 => {
            let rect1 = setRect(0, 0, src_width, src_height - pY as u32);
            let mut part1 = GetSubSurface(&ret, rect1.x, rect1.y, rect1.w as u32, rect1.h as u32);
            part1.set_blend_mode(sdl2::render::BlendMode::None);
            let dst_rect = setRect(pX as i32, pY as i32, src_width, src_height - pY as u32);
            Some((part1, dst_rect))
        },
        (px, _) if px <= 0 => {
            //Right
            let rect2 = setRect(0, 0, src_width - pX as u32, src_height);
            let mut part1 = GetSubSurface(&ret, rect2.x, rect2.y, rect2.w as u32, rect2.h as u32);
            part1.set_blend_mode(sdl2::render::BlendMode::None);
            let dst_rect = setRect(pX as i32, 0, src_width - pX as u32, src_height);
            Some((part1, dst_rect))
        },
        (px, _) if px > 0 => {
            let rect1 = setRect(pX as i32, 0, src_width - pX as u32, src_height);
            let mut part1 = GetSubSurface(&ret, rect1.x, rect1.y, rect1.w as u32, rect1.h as u32);
            part1.set_blend_mode(sdl2::render::BlendMode::None);
            let dst_rect = setRect(0, 0, src_width - pX as u32, src_height);
            Some((part1, dst_rect))
        },
        (_, _) => (
            None
        ),
    };

    // SDL_BlitSurface (part1, NULL, _src, &destrect1);
    match res {
        Some((part1, dst_rect)) => {
            part1.blit(None, src, dst_rect).expect("unable to scroll surface");
        },
        None => panic!("nothing to scroll );"),
    }
}
