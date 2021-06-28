extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::env;
use std::fs;

fn main() {
    let filename = "test.yaml";
    
    let contents = fs::read_to_string(filename).expect("something went wrong");

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", doc);

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
    println!("With text:\n{}", contents);
}
