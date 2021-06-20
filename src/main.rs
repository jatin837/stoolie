use std::env;
use std::path::Path;
use path_abs::PathAbs;
use std::fs::File;
use std::io::Read;
use regex::Regex;
struct Issue {
    heading: String,
    status: Status,
}
enum Status {
    Posted,
    Idle,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut issues:Vec<Issue> = Vec::new();

    let re = Regex::new(r" *-*TODO-*([a-zA-Z1-9 ]*)").unwrap();

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
        issues.push(Issue{
            heading: String::from(&cap[0]),
            status: Status::Idle,
        })
    }

    for issue in issues{
        println!("--{}--", issue.heading)
    }
    println!("DONE STORED IN ISSUES_LIST");

}
