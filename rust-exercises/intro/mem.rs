use std::{env, process, thread};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    let mut p = Box::new(0);

    println!("({:?}) memory address of p: {:08x}", process::id(), *p as *const i32 as usize);

    loop {
        spin(1.0);
        *p += 1;
        println!("({:?}) p: {}", process::id(), *p);
    }
}

pub fn get_time() -> f64 {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    return duration.as_secs() as f64 + duration.subsec_micros() as f64/1e6;
}

pub fn spin(howlong: f64) {
    let t: f64 = get_time();
    while (get_time()-t) < howlong {

    }
}