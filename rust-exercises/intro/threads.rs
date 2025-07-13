use std::sync::{Arc, Mutex}; 

fn main() { 
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 { 
        eprintln!("usage: threads <value>"); std::process::exit(1); 
    } 
    
    let loops: usize = args[1].parse().unwrap(); 
    let counter = Arc::new(Mutex::new(0)); 
    println!("Initial value: {}", *counter.lock().unwrap()); 
    
    let counter1 = Arc::clone(&counter); 
    let thread1 = std::thread::spawn(move || { 
        for _ in 0..loops { 
            let mut num = counter1.lock().unwrap(); 
            *num += 1; 
        } 
    }); 
    
    let counter2 = Arc::clone(&counter); 
    let thread2 = std::thread::spawn(move || { 
        for _ in 0..loops {
             let mut num = counter2.lock().unwrap(); 
             *num += 1; 
            } 
        }); 
        
    thread1.join().unwrap(); 
    thread2.join().unwrap(); 
    
    println!("Final value: {}", *counter.lock().unwrap());
}