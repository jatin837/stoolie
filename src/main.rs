use std::env;
use std::path::Path;
use path_abs::PathAbs;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    let filepath: &Path = Path::new(filename);
    let abs_file_path = PathAbs::new(filepath);
    match abs_file_path {
        Ok(v) => println!("absolute path is {:?}", v),
        Err(e) => println!("error {:?}", e)
    }
    
    // get absolute path of filename
    println!("{:?}", filepath);
    println!("{:?}", filename);

   
}
