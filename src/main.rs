use std::env;
use std::path::PathBuf;

fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(file) = args.get(1) {
    let file = PathBuf::from(file);
    let file_path = file.canonicalize();
    println!("File Path: {}", file_path.unwrap().display());
  } else {
    println!("Error: Missing input file.");
  }
}