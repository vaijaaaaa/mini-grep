use std :: env;
use std::fs;
use std ::process;


fn main() {
    let args:Vec<String> = env::args().collect();


    if args.len() != 3{
        eprintln!("Usage:minigrep <query> <file_path>");
        process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];


    println!("Searching for {}",query);
    println!("In file {}",file_path);

    let contents = fs::read_to_string(file_path).unwrap_or_else(|err|{
        eprintln!("Failed to read file  '{}'  {}",file_path,err);
        process::exit(1);
    });
     
    println!("With text:\n{}",contents);

}


