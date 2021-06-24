use std::env;
use std::str;
use std::path::Path;
use path_abs::PathAbs;
use std::fs::File;
use std::io::Read;
use regex::Regex;
//use hex_literal::hex;
use sha2::{
    Sha256,
    Digest,
};

struct Issue {
    heading: String,
    status: Status,
    digest: String,
}

impl Issue {
    fn post_issue(idle: Vec<Issue>){
        //post idle issues to github
    }
}

struct Issues {
    list: Vec<Issue>,
}

impl Issues {
    fn new(mut self) -> Self {
       // init empty list 
    }
    fn load_issues() {
       //load unposted issues from .ISSUES file 
    }
}

enum Status {
    Posted,
    Idle,
}

fn hash(issue: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(issue.as_bytes());
    let result = hasher.finalize();
    let mut ret_res:Vec<char> = Vec::new();
    for c in result{
        ret_res.push(c as char)
    }
    let res: String = ret_res.into_iter().collect();
    res
}

struct User {
    name: String,
    access_token: String,
    email: String,
}

fn load_config() -> Vec<&String> {
    //read file from ~/.config/stoolie/stoolie.yml
    //grap access_token, name, email and other relevent info about user
    //store them into User struct
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
            digest: hash(String::from(&cap[0]))
        })
    }

    for issue in issues{
        println!("--{}--{}--", issue.heading, issue.digest)
    }
    println!("DONE STORED IN ISSUES_LIST");

}
