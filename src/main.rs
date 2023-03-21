extern crate time;
extern crate image;

// use image::io::Reader as ImageReader;
use std::env;
// use std::io::Cursor;
use std::path::PathBuf;


fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(file) = args.get(1) {
    let file = PathBuf::from(file);
    let file_path = file.canonicalize();
    println!("File Path: {}", file_path.unwrap().display());

    let now = time::OffsetDateTime::now_utc();
    println!("Time {now}");

    // let res = ImageReader::into_dimensions("/workspaces/rust-image-filters/assets/img/rainbow.jpg");
    // println!("{} {}", res.width);
  } else {
    println!("Error: Missing input file.");
  }
}