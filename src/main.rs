extern crate rand;
extern crate md5;
use std::{env, process, thread};
use rand::{Rng, distributions::Alphanumeric};
use std::sync::{mpsc, Arc};

static NUM_THREAD: i32 = 8;
static STR_SIZE: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid argument size!");
        process::exit(1);
    }

    let to_search = Arc::new(
        args.get(1).unwrap().clone());

    println!("Computing MD5 hashes that start with: {}\n", to_search);

    let mut workers = vec![];
    let mut comms = vec![];
    let (main_tx, main_rx) = mpsc::channel();

    for _i in 0..NUM_THREAD {
        let temp_str = Arc::clone(&to_search);
        let temp_main_tx = main_tx.clone();
        let (temp_tx, temp_rx) = mpsc::channel();
        comms.push(temp_tx);
        workers.push(thread::spawn(move || {
            bruteforce_md5(&temp_str, temp_rx);
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

fn bruteforce_md5(to_search: &str, rx: mpsc::Receiver<bool>) {
    while rx.try_recv().is_err() {
        let rand_string = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(STR_SIZE)
            .collect::<String>();

        let digest = md5::compute(&rand_string);
        let as_hex = format!("{:x}", digest);
        
        if as_hex.starts_with(to_search) {
            println!("String: {}\tMD5: {}", rand_string, as_hex);
            break;
        }
    }
}