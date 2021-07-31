use std::time::SystemTime;

pub fn dump_surface(surface: &sdl2::surface::SurfaceRef, name: &str, suffix: &str) {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

    // let surface = sdl2::surface::Surface::from_ll(_surface);
    match surface.save_bmp(["tmp/", name, &now.to_string(), suffix, ".bmp"].concat()) {
        Ok(_x) => (),
        Err(s) => panic!("{}", s),
    };
}

pub fn dump_surface_pixels(surface: &sdl2::surface::SurfaceRef, text: &str) {
    unsafe {
        let raw_surface = *surface.raw();
        // let raw_surface = *surface;
        let mut v: Vec<u8> = Vec::new();

        let width = raw_surface.w as u32;
        let height = raw_surface.h as u32;
        let pxls = raw_surface.pixels as *const u8;
        let slice = std::slice::from_raw_parts(pxls, (height*width) as usize);
        std::io::Write::write(&mut v, slice).unwrap();
        println!("{:?}: {:?}", text, v);
    }
}

pub fn dump_surface_pixels_slice(s: &sdl2::surface::SurfaceRef, w: usize, h: usize, line_size: usize) {
    unsafe {
        let data = (*s.raw()).pixels as *const u8;
        let slice = std::slice::from_raw_parts(data, h * w);
        println!("surface raw: {:?}", slice[0..line_size].to_vec());
    }
}

pub fn print_surface_pixels(surface: &sdl2::surface::SurfaceRef, text: &str) {
    unsafe {
        let raw_surface = *surface.raw();
        // let raw_surface = *surface;
        let mut v: Vec<u8> = Vec::new();

        let width = raw_surface.w as u32;
        let height = raw_surface.h as u32;
        let pxls = raw_surface.pixels as *const u8;
        let slice = std::slice::from_raw_parts(pxls, (height*width) as usize);

        println!("{}", text);
        std::io::Write::write(&mut v, slice).unwrap();

        let width = width as usize;
        let height = height as usize;
        let mut i: usize = 0;
        while i < height {
        let mut j: usize = 0;
        while j < width {
            print_colored_char(v[(i*width)+j], v[(i*width)+j], v[(i*width)+j]);
            // print_colored_char(v[(i*width)+j], v[(i*width)+j], v[(i*width)+j]);

            j += 1;
        }

        i += 1;
        println!("");
        }
        println!("");
        println!("");
    }
}

// print!("\x1B[48;2;30;30;30m \x1B[0m");
pub fn print_colored_char(r: u8, g: u8, b: u8) {
    print!("\x1B[48;2;{};{};{}m \x1B[0m", r, g, b);
}
