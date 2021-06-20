use std::env;
use std::path::Path;
use path_abs::PathAbs;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut issues_list:Vec<String> = Vec::new();

    let re = Regex::new(r" *-*TODO-*\n([a-zA-Z1-9 ]*)").unwrap();

    let filename: &String = &args[1];
    
    let abs_file_path = PathAbs::new(filename).unwrap();
    let filepath:&Path = Path::new(&abs_file_path);

    if !filepath.exists() {
        panic!("File does not exist")
    }

    println!("{:?}", filepath);

    let mut file = File::open(filepath).expect("Unable To open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("can not read file contents");
    for cap in re.captures_iter(&contents) {
        println!("Issues are : {}", &cap[0]);
        issues_list.push(String::from(&cap[0]))
    }
    print!("contents of file are :::  {:?}", contents);
    println!("DONE STORED IN ISSUES_LIST");

}
