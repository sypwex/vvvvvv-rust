pub fn dump_surface (name: &str, src: &sdl2::surface::SurfaceRef) {
  // let src = sdl2::surface::Surface::from_ll(_src);
  match src.save_bmp(["tmp/", name, ".bmp"].concat()) {
      Ok(_x) => (),
      Err(s) => panic!("{}", s),
  };
}
