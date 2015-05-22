#[cfg(feature="pkg-config")]
extern crate pkg_config;

fn main() {
  if !build_pkgconfig() {
    println!("cargo:rustc-flags=-l tag_c -l tag");
  }
}

#[cfg(not(feature="pkg-config"))]
fn build_pkgconfig() -> bool {
  false
}

#[cfg(feature="pkg-config")]
fn build_pkgconfig() -> bool {
  if pkg_config::find_library("taglib_c").is_err() {
    panic!("Could not find taglib_c via pkgconfig");
  }
  true
}

