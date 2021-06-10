use std::env;
use std::path::Path;
use path_abs::PathAbs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    
    let abs_file_path = PathAbs::new(filename);
    match abs_file_path {
        Ok(v) => {
            let filepath:&Path = Path::new(&v);
            println!("{:?}", filepath);
        },
        Err(e) => println!("error {:?}", e)
    }
    println!("{:?}", filename);
}
