extern crate image;

use std::env;
use std::path::PathBuf;


fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(file) = args.get(1) {
    let in_file = PathBuf::from(file);
    let in_file_name = in_file.file_name().unwrap().to_string_lossy();
    let in_file_path = in_file.canonicalize().unwrap().display().to_string();
    println!("Input {in_file_path}");

    // TODO: read OUTPUT_DIR envar
    // let out_file_name = format!("gray.{in_file_name}");
    // if let Some(output_dir) = env::var("OUTPUT_DIR") {
    //   let out_file_path = output_dir.unwrap().join(out_file_name.clone()).display().to_string();
    // }
    // else {
    //   let out_file_path = env::current_dir().unwrap().join(out_file_name.clone()).display().to_string();
    // }

    let out_file_name = format!("gray.{in_file_name}");
    let out_file_path = env::current_dir().unwrap().join(out_file_name.clone()).display().to_string();
    println!("Output {out_file_path}");

    // covert to grayscale
    let img = image::open(in_file_path).unwrap();
    let img = img.grayscale();
    img.save(out_file_path).unwrap();
  } else {
    println!("Error: Missing input file.");
  }
}