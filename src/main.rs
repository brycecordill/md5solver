extern crate rand;
extern crate md5;
use std::{env, process};
use rand::{Rng, distributions::Alphanumeric};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid argument size!");
        process::exit(1);
    }

    let to_search = match args.get(1){
        Some(arg) => arg,
        None => process::exit(1),
    };

    println!("Searching for MD5 hashes that start with: {}", to_search);

    loop {
        let rand_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .collect::<String>();

        let digest = md5::compute(&rand_string);
        let as_hex = format!("{:x}", digest);
        
        if as_hex.starts_with(to_search) {
            println!("String: {} \t MD5: {}", rand_string, as_hex);
        }
    }
}
