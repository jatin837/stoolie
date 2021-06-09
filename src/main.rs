use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    let possible_query = String::new("list")

    match query {
        possible_query => println!("------------------listing issues in {}--------", filename), 
        _ => println!("TOBE IMPLEMENTED IN FUTURE PERHAPS NOT IF IT DOES NOT MAKE SENSE"),
    };
}
