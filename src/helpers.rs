macro_rules! as_mut_ptr {
  ($($arg:tt)*) => {{
      Box::into_raw(Box::new($($arg)*))
  }}
}
