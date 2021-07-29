use path_abs::PathAbs;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::env;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

fn string_from_file(fpath: &String) -> String {
    let rel_path: &Path = Path::new(fpath);
    let abs_file_path = PathAbs::new(rel_path).unwrap();
    let filepath: &Path = Path::new(&abs_file_path);
    if !filepath.exists() {
        panic!("File does not exist")
    }

    let mut file = File::open(filepath).expect("Unable To open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("can not read file contents");
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
    fn post_issue(idle: Vec<Issue>) {
        //post idle issues to github
    }
}

struct Issues {
    list: Vec<Issue>,
}

impl Issues {
    fn new() -> Vec<Issue> {
        Vec::new()
    }
    fn load_issues() {
        // .ISSUE format
        // \tISSUE_HASH\tISSUE_DISCRIPTION\tISSUE_STATUS
        //load unposted issues from .ISSUES file
        //  \[([0-9a-f]*)\] \[((\w*| )*)\] \[(P|U)\]/g
        let contents = string_from_file(&"./.ISSUES".to_string());
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
    let mut ret_res: Vec<char> = Vec::new();
    for c in result {
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
    let test_config_path: &String = &String::from("test.yaml");
    let configs: Vec<Yaml> = parse_yaml(test_config_path);
    let name: String = configs[0]["user"][0]
        .as_str()
        .expect("something wrong")
        .clone()
        .to_string();
    let email: String = configs[0]["user"][1]
        .as_str()
        .expect("something wrong")
        .clone()
        .to_string();
    let acces_token: String = configs[0]["user"][2]
        .as_str()
        .expect("something wrong")
        .clone()
        .to_string();
    println!(
        "name -> {:?}, email -> {:?}, access_token -> {:?}",
        name, email, acces_token
    );
    vec![name, email, acces_token]
}

fn read_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn main() {
    let args = read_args();

    if args.len() == 1 {
        panic!("NEED AN ARGUMENT");
    }

    let mut issues: Vec<Issue> = Issues::new();
    let re = Regex::new(r" *-*TODO-*([a-zA-Z1-9 ]*)").expect("you fuckin meet head");
    let contents = string_from_file(&args[1]);
    for cap in re.captures_iter(&contents) {
        issues.push(Issue {
            heading: String::from(&cap[0]),
            status: Status::Idle,
            digest: hash(String::from(&cap[0])),
        })
    }
    let config_vals: Vec<String> = load_config();
    for config_val in config_vals.iter() {
        println!("{:?} ", config_val);
    }
    for issue in issues {
        println!("--{}--{}--", issue.heading, issue.digest)
    }
    println!("DONE STORED IN ISSUES_LIST");
}
