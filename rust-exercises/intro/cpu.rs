use std::{env, process};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc != 2 {
        eprintln!("usage: cpu <string>");
        process::exit(1);
    }

    let stri: &String = &argv[1];
    loop {
        Spin(1.0);
        println!("{}", stri);
    }
}

pub fn GetTime() -> f64 {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    duration.as_secs() as f64 + duration.subsec_micros() as f64/1e6
}

pub fn Spin(howlong: f64) {
    let t: f64 = GetTime();
    while (GetTime()-t) < howlong {
    }
}
