use std::ptr;
extern crate sdl2;
extern crate sdl2_sys;

pub fn RecreateSurface(surface: *mut sdl2_sys::SDL_Surface, blend_mode: sdl2::render::BlendMode) -> sdl2::surface::Surface<'static> {
    unsafe {
        let width = (*surface).w;
        let height = (*surface).h;

        RecreateSurfaceWithDimensions(
            *surface,
            blend_mode,
            width,
            height
        )
    }
}

pub fn RecreateSurfaceWithDimensions(
    // surface: Box<*mut sdl2_sys::SDL_Surface>,
    surface: sdl2_sys::SDL_Surface,
    blend_mode: sdl2::render::BlendMode,
    width: i32,
    height: i32
) -> sdl2::surface::Surface<'static> {
    // let retval: Box<*mut sdl2_sys::SDL_Surface>;
    let mut retval;
    // let blend_mode: *mut sdl2_sys::SDL_BlendMode;
    // let blend_mode: *mut sdl2::render::BlendMode;

    unsafe {
        // let rawsurface = *(*surface).raw();
        let flags = surface.flags;
        let format = *surface.format;
        let retsurface = sdl2_sys::SDL_CreateRGBSurface(
            flags,
            width,
            height,
            format.BitsPerPixel.into(),
            format.Rmask,
            format.Gmask,
            format.Bmask,
            format.Amask
        );
        retval = sdl2::surface::Surface::from_ll(retsurface);
    }

        // sdl2_sys::SDL_GetSurfaceBlendMode(*surface, blend_mode);
        // sdl2_sys::SDL_SetSurfaceBlendMode(*retval, *blend_mode);
        // let blend_mode = (*surface).blend_mode();
        retval.set_blend_mode(blend_mode);

    retval
}

pub fn GetSubSurface(metaSurface: sdl2::surface::Surface, x: i32, y: i32, w: i32,  h: i32) -> sdl2::surface::Surface {
    unsafe {
        // Create an SDL_Rect with the area of the _surface
        let area = sdl2_sys::SDL_Rect{
            x, y, w, h,
        };
        let src_surface = (*metaSurface).raw();

        //Convert to the correct display format after nabbing the new _surface or we will slow things down.
        let preSurface = RecreateSurfaceWithDimensions(
            *metaSurface.raw(),
            metaSurface.blend_mode(),
            w,
            h
        );

        // Lastly, apply the area from the meta _surface onto the whole of the sub _surface.
        sdl2_sys::SDL_UpperBlit(src_surface, &area, preSurface.raw(), ptr::null_mut());

        // Return the new Bitmap _surface
        preSurface
    }
}

fn DrawPixel(surf: sdl2_sys::SDL_Surface, x: i32, y: i32, pixel: u32) {
    unsafe {
        // let surf = *(**_surface).raw();
        let format = *surf.format;
        let pixels = surf.pixels as *mut u8;

        let bpp = format.BytesPerPixel as i32;
        /* Here p is the address to the pixel we want to set */
        // let p = pixels + y*surf.pitch + x*bpp;
        let p = pixels.offset((y*surf.pitch + x*bpp) as isize);

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

fn ReadPixel(surface: sdl2_sys::SDL_Surface, x: i32, y: i32) -> u32 {
    unsafe {
        let format = *surface.format;
        let pixels = surface.pixels as *mut u8;

        let bpp = format.BytesPerPixel as i32;
        /* Here p is the address to the pixel we want to retrieve */
        let p = pixels.offset((y*surface.pitch + x*bpp) as isize);

        match bpp {
            1 => *p as u32,
            2 => (*p as u32) | ((*p.offset(1) as u32) << 8),
            3 => ((*p as u32) | (*p.offset(1) as u32) << 8 | (*p.offset(2) as u32) << 16) as u32,
            4 => ((*p as u32) | (*p.offset(1) as u32) << 8 | (*p.offset(2) as u32) << 16 | (*p.offset(3) as u32) << 24) as u32,
            _ => 0,
        }
    }
}

pub fn BlitSurfaceColoured(
    // SDL_Surface* _src,
    _src: *mut sdl2_sys::SDL_Surface,
    src_blend_mode: sdl2::render::BlendMode,
    // SDL_Rect* _srcRect,
    _srcRect: sdl2::rect::Rect,
    // SDL_Surface* _dest,
    _dest: *mut sdl2::surface::SurfaceRef,
    // SDL_Rect* _destRect,
    _destRect: *mut sdl2_sys::SDL_Rect,
    // colourTransform& ct
    colourTransformValue: u32,
    c: char,
) {
    // SDL_Rect *tempRect = _destRect;
    let tempRect = _destRect;

    // SDL_Surface* tempsurface = RecreateSurface(_src);
    let tempsurface = RecreateSurface(_src, src_blend_mode);

    // const SDL_PixelFormat& fmt = *(_src->format);

    unsafe {
        let fmt = (*_src).format;
        let raw_surface = *tempsurface.raw();
        let width = tempsurface.width() as i32;
        let height = tempsurface.height() as i32;

        let mut x = 0;
        while x < width {
            let mut y = 0;
            while y < height {
                // Uint32 pixel = ReadPixel(_src, x, y);
                let pixel = ReadPixel(*_src, x, y);
                // Uint32 Alpha = pixel & fmt.Amask;
                let Alpha = pixel & (*fmt).Amask;
                // Uint32 result = ct.colour & 0x00FFFFFF;
                let result = colourTransformValue & 0x00FFFFFF;
                // Uint32 CTAlpha = ct.colour & fmt.Amask;
                let CTAlpha = colourTransformValue & (*fmt).Amask;
                // float div1 = ((Alpha >> 24) / 255.0f);
                let div1 = ((Alpha >> 24) as f32) / 255.0f32;
                // float div2 = ((CTAlpha >> 24) / 255.0f);
                let div2 = ((CTAlpha >> 24) as f32) / 255.0f32;
                // Uint32 UseAlpha = (div1 * div2) * 255.0f;
                let UseAlpha: u32 = (div1 * div2 * 255.0f32) as u32;
                // DrawPixel(tempsurface, x, y, result | (UseAlpha << 24));
                DrawPixel(*tempsurface.raw(), x, y, result);

                // println!("{:?} ===> {:?} |||| {:?} {:?} {:?} {:?} {:?} ||| {:?}", pixel, result, UseAlpha, div2, div1, CTAlpha, Alpha, colourTransformValue);

                y += 1;
            }

            x += 1;
        }

        // let mut v: Vec<u8> = Vec::new();
        // // let pxls = (*_src).pixels as *const u8;
        // let pxls = (*tempsurface.raw()).pixels as *const u8;
        // let slice = std::slice::from_raw_parts(pxls, 10);
        // std::io::Write::write(&mut v, slice).unwrap();
        // println!("raw pixel data: {:?}", v);

        // sdl2_sys::SDL_UpperBlit(tempsurface.raw(), _srcRect.as_ref(), (*_dest).raw(), tempRect);
        sdl2_sys::SDL_UpperBlit(_src, _srcRect.as_ref(), (*_dest).raw(), tempRect);

        // let src = sdl2::surface::Surface::from_ll(_src);
        // crate::rustutil::dump_surface(&["BlitSurfaceColoured-", &c.to_string(), "-0-srcsurface"].concat(), &src);
        // crate::rustutil::dump_surface(&["BlitSurfaceColoured-", &c.to_string(), "-1-dstsurface"].concat(), _dest.as_ref().unwrap());
        // crate::rustutil::dump_surface(&["BlitSurfaceColoured-", &c.to_string(), "-2-tempsurface"].concat(), &tempsurface);

        // sdl2_sys::SDL_FreeSurface(**tempsurface);
        drop(tempsurface.raw());
    }
}

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
}

// void FillRect( SDL_Surface* _surface, const int color ) {
//     SDL_Rect rect = {0,0,Uint16(_surface->w) ,Uint16(_surface->h) };
//     SDL_FillRect(_surface, &rect, color);
// }
pub fn FillRectWithColor (_surface: &mut sdl2::surface::SurfaceRef, color: sdl2::pixels::Color ) {
    let rect = sdl2::rect::Rect::new(0, 0, _surface.width(), _surface.height());
    _surface.fill_rect(rect, color);
}

// void FillRect( SDL_Surface* _surface, const int x, const int y, const int w, const int h, int rgba ) {
//     SDL_Rect rect = {Sint16(x)  ,Sint16(y) ,Sint16(w) ,Sint16(h) };
//     SDL_FillRect(_surface, &rect, rgba);
// }
pub fn FillRect (_surface: &mut sdl2::surface::SurfaceRef, x: i32, y: i32, w: u32, h: u32, rgba: sdl2::pixels::Color ) {
    let rect = sdl2::rect::Rect::new(x, y, w, h);
    _surface.fill_rect(rect, rgba);
}

// void setRect( SDL_Rect& _r, int x, int y, int w, int h ) {
//     _r.x = x;
//     _r.y = y;
//     _r.w = w;
//     _r.h = h;
// }
// pub fn setRect (x: i32, y: i32, w: u32, h: u32) -> sdl2::rect::Rect {
//     let rect = sdl2::rect::Rect::new(x, y, w, h);
// }

// void BlitSurfaceStandard( SDL_Surface* _src, SDL_Rect* _srcRect, SDL_Surface* _dest, SDL_Rect* _destRect )
// {
//     //SDL_Rect tempRect = *_destRect;
//     //tempRect.w  ;
//     //tempRect.h  ;
//     //tempRect.x *=globalScale;
//     //tempRect.y *=globalScale;


//     //if(globalScale != 1)
//     //{
//     //	SDL_Surface* tempScaled = ScaleSurface(_src, tempRect.w, tempRect.h);

//     //	SDL_BlitSurface( tempScaled, _srcRect, _dest, &tempRect );

//     //	SDL_FreeSurface(tempScaled);
//     //}
//     //else
//     //{
//     SDL_BlitSurface( _src, _srcRect, _dest, _destRect );
//     //}
// }
// pub fn blit_surface_standard () {
//     buffer.blit(sdl2::rect::Rect::new(), self.m_screen, rect);
// }
