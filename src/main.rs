extern crate pwhash;
extern crate regex;
extern crate rand;

mod random;

use pwhash::bcrypt;
use pwhash::bcrypt::BcryptSetup;
use pwhash::bcrypt::BcryptVariant;
use std::io;
use std::str;
use regex::Regex;
use std::collections::LinkedList;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

fn main() {

    let mut tasks = LinkedList::new();

    let thread_count = env::args().nth(1).unwrap().parse::<u32>().unwrap();
    let cost = env::args().nth(2).unwrap().parse::<u32>().unwrap();

    loop {
        let re = Regex::new(r"(.*)pw:(.*)").expect("bad regex");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(n) => {
                let capture = re.captures(&input).expect("bad capture");
                let row = capture.at(1).unwrap();
                let pw = capture.at(2).unwrap();
                tasks.push_back(Tasks {
                    row: row.to_string(),
                    pw: pw.to_string()
                });
            }
            _=> unreachable!(),
        }
    }

    assert_eq!(
        "$2y$15$1Q2yicJ4/mZnDIGrh2mWpegV9ZtJoEcan5t2naCU1i45Ss5LNNDsa",
        hash("abcdef", "1Q2yicJ4/mZnDIGrh2mWpo", 15)
    );

    let threadsafe_data = Arc::new(Mutex::new(tasks.clone()));

    let mut threads = vec![];

    for i in 0..thread_count {
        let thread_threadsafe_data = threadsafe_data.clone();
        threads.push(thread::spawn(move || {
            loop {
                let thread_task = {
                    let mut data = thread_threadsafe_data.lock().unwrap();
                    match data.pop_front() {
                        Some(thread_task) => thread_task,
                        _ => {
                            return;
                        }
                    }
                };

                let salt = random::gen_salt_str(22);
                println!("{}", thread_task.row.replace("{pw}", &hash(&thread_task.pw, &salt, cost)).replace("{salt}", &salt));
            }
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
}

fn hash(pw : &str, salt : &str, cost : u32) -> String {
    let foo = BcryptSetup {
        salt: Some(salt),
        cost: Some(cost),
        variant: Some(BcryptVariant::V2y)
    };

    bcrypt::hash_with(
        foo,
        pw
    ).unwrap()
}

#[derive(Debug, Clone)]
struct Tasks {
    pw: String,
    row: String
}

struct BcryptResult {
    res: String,
    salt: String
}
