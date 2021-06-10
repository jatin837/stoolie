use std::env;
use std::path::Path;
use path_abs::PathAbs;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    let filepath: &Path = Path::new(filename);
    let absFilePath = PathAbs::new(filepath);
    
    // get absolute path of filename
    println!("{:?}", filepath);
    println!("{:?}", filename);
    println!("{:?}", absFilePath);

   
}
