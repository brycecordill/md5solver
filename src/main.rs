extern crate rand;
extern crate md5;

pub mod args;

use args::Args;
use std::{env, process, thread};
use rand::{Rng, distributions::Alphanumeric};
use std::sync::{mpsc, Arc};

fn main() {
    let arg_vec: Vec<String> = env::args().collect();
    if arg_vec.len() < 2 || arg_vec.len() > 6 {
        eprintln!("\nError: Invalid argument size!\n");
        print_usage();
        process::exit(1);
    }

    let args = Args::new(arg_vec).unwrap_or_else(|e| {
        eprintln!("\nError: {}!\n", e);
        print_usage();
        process::exit(1);
    });

    println!("Computing MD5 hashes that start with \'{}\' using {} threads...\n",
        args.search_str, args.thread_num);

    let mut workers = vec![];
    let mut comms = vec![];
    let (main_tx, main_rx) = mpsc::channel();

    for _i in 0..args.thread_num {
        let temp_str = Arc::clone(&args.search_str);
        let temp_main_tx = main_tx.clone();
        let (temp_tx, temp_rx) = mpsc::channel();
        let temp_str_len = args.str_len;
        comms.push(temp_tx);
        workers.push(thread::spawn(move || {
            bruteforce_md5(&temp_str, temp_rx, temp_str_len);
            temp_main_tx.send(true).unwrap();
        }));
    }

    main_rx.recv().unwrap();

    for tx in comms {
        tx.send(true).ok();
    }

    for worker in workers {
        let _ = worker.join();
    }
}

fn print_usage() {
    unimplemented!("Usage info will go here");
}

fn bruteforce_md5(search_str: &str, rx: mpsc::Receiver<bool>, str_len: usize) {
    while rx.try_recv().is_err() {
        let rand_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(str_len)
            .collect::<String>();

        let digest = md5::compute(&rand_string);
        let as_hex = format!("{:x}", digest);
        
        if as_hex.starts_with(search_str) {
            println!("String: {}\tMD5: {}", rand_string, as_hex);
            break;
        }
    }
}