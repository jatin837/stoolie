use std::env;
use std::str;
use std::path::Path;
use path_abs::PathAbs;
use std::fs::{
    File, 
    read_to_string,
};
use std::io::Read;
use regex::Regex;
//use std::collections::HashMap;
//use hex_literal::hex;
use sha2::{
    Sha256,
    Digest,
};
use yaml_rust::{YamlLoader, Yaml};

// (file path) => |my function name| => (string fromm that file)

fn string_from_file(fpath: &String) -> String {
    let rel_path: &Path = Path::new(fpath);
    let abs_file_path = PathAbs::new(rel_path).unwrap();
    let filepath:&Path = Path::new(&abs_file_path);
    if !filepath.exists() {
        panic!("File does not exist")
    }

    let mut file = File::open(filepath).expect("Unable To open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("can not read file contents");
    contents
}

fn parse_yaml(fpath: &String) -> Vec<Yaml> {
    let contents: &String = &read_to_string(fpath).expect("can not read the file");
    let docs = YamlLoader::load_from_str(contents).unwrap();
    docs
}

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
    fn new() -> Vec<Issue> {
        let iss: Vec<Issue> = Vec::new();
        iss
    }
    fn load_issues() {
       //load unposted issues from .ISSUES file 
       todo!()
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

fn load_config() -> Vec<String> {
    //read file from ~/.config/stoolie/stoolie.yml
    //grap access_token, name, email and other relevent info about user
    //store them into User struct
    todo!()
}


fn read_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn main() {
    let args = read_args();
    let mut issues:Vec<Issue> = Issues::new();

    let re = Regex::new(r" *-*TODO-*([a-zA-Z1-9 ]*)").expect("you fuckin meet head");

    let contents = string_from_file(&args[1]);

    for cap in re.captures_iter(&contents) {
        issues.push(Issue{
            heading: String::from(&cap[0]),
            status: Status::Idle,
            digest: hash(String::from(&cap[0]))
        })
    }
    let test_config: &String = &String::from("test.yaml");
    let configs: Vec<Yaml> = parse_yaml(test_config);

    
    for issue in issues{
        println!("--{}--{}--", issue.heading, issue.digest)
    }
    println!("DONE STORED IN ISSUES_LIST");

}
