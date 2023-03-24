extern crate image;

use std::env;
use std::path::PathBuf;


fn main() {
  let args: Vec<String> = env::args().collect();

  // Check existence of input arguments
  let program = args[0].clone();
  if args.len() < 2 {
    println!("USAGE: {program} <IMAGE - required> <OUTPUT_DIR - optional>");
    return;
  }

  if let Some(file) = args.get(1) {
    let in_file = PathBuf::from(file);
    let in_file_name = in_file.file_name().unwrap().to_string_lossy();
    let in_file_path = in_file.canonicalize().unwrap().display().to_string();
    println!("Input {in_file_path}");

    // covert to grayscale
    let img = image::open(in_file_path).unwrap();
    let img = img.grayscale();

    // Save image to out_file_path
    let out_file_name = format!("gray.{in_file_name}");
    if env::var("OUTPUT_DIR").is_ok() {
      let output_dir = env::var("OUTPUT_DIR").unwrap();
      let out_file_path = format!("{}/{}", output_dir.to_string(), out_file_name.to_string());
      println!("Output {out_file_path}");

      img.save(out_file_path).unwrap();
    } else {
      let output_dir = env::current_dir().unwrap();
      let out_file_path = output_dir.join(out_file_name.clone()).display().to_string();
      println!("Output {out_file_path}");

      img.save(out_file_path).unwrap();
    }
  } else {
    println!("Error: Missing input file.");
  }
}