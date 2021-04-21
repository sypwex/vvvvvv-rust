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
