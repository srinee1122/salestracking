// src-tauri/build.rs
fn main() {
  // This line tells rustc to expect the 'mobile' cfg flag (handles the warning)
  println!("cargo::rustc-check-cfg=cfg(mobile)");

  // This line runs the main Tauri build logic (fixes the error)
  tauri_build::build()
}