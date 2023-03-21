extern crate image;

use image::imageops::dither;
use image::imageops::BiLevel;
use std::env;
use std::path::PathBuf;


fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(file) = args.get(1) {
    let in_file = PathBuf::from(file);
    let in_file_name = in_file.file_name().unwrap().to_string_lossy();
    let in_file_path = in_file.canonicalize().unwrap().display().to_string();
    let out_file_name = format!("gray.{in_file_name}");
    let out_file_path = env::current_dir().unwrap().join(out_file_name.clone()).display().to_string();

    // covert to grayscale
    let img = image::open(in_file_path).unwrap();
    let img = img.grayscale();
    img.save(out_file_path).unwrap();
  } else {
    println!("Error: Missing input file.");
  }
}