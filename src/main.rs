use std::env;
use std::path::Path;
use path_abs::PathAbs;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let re = Regex::new(r" *-*TODO-*\n\/\/([a-zA-Z1-9 ]*)").unwrap();
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];
    
    let abs_file_path = PathAbs::new(filename);
    match abs_file_path {
        Ok(v) => {
            let filepath:&Path = Path::new(&v);
            let is_exist: bool = filepath.exists();
            if is_exist {
                println!("{:?}", filepath);
                let mut file = File::open(filepath).expect("Unable To open the file");
                let mut contents = String::new();

                file.read_to_string(&mut contents).expect("can not read file contents");
                for cap in re.captures_iter(&contents) {
                    println!("Issues are : {}", &cap[0]);
                }
                println!("contents of file are :::  {:?}", contents);

            } else {
                println!("No such file exists right now")
            }
        },
        Err(e) => println!("error {:?}", e)
    }
}
