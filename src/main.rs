use std::env;
use std::path::Path;
use path_abs::PathAbs;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }
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
                println!("contents of file are :::  {:?}", contents);

            } else {
                println!("No such file exists right now")
            }
        },
        Err(e) => println!("error {:?}", e)
    }
}
