use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use rand::Rng;

fn main() {
    println!("Start Dining Philosophers Simulations!");
    let forks = Arc::new(vec![
        Mutex::new(false),
        Mutex::new(false),
    ]);

    let forks_2 = Arc::clone(&forks);
    
    thread::spawn(move || {
        loop {
            println!("Philosopher 1 is Thinking!");
            let mut rng = rand::thread_rng();
            sleep(std::time::Duration::from_secs(rng.gen_range(1..5)));
            println!("Philosopher 1 is Hungry!");

            // lock does block
            let left_fork = forks[0].lock().unwrap();
            sleep(std::time::Duration::from_secs(1));
            let right_fork = forks[1].lock().unwrap();
            println!("Philosopher 1 is Eating!");
            sleep(std::time::Duration::from_secs(rng.gen_range(1..5)));
            drop(left_fork);
            drop(right_fork);
        }
    });
    
    thread::spawn(move || {
        loop {
            println!("Philosopher 2 is Thinking!");
            let mut rng = rand::thread_rng();
            sleep(std::time::Duration::from_secs(rng.gen_range(1..5)));
            println!("Philosopher 2 is Hungry!");

            // lock does block
            let left_fork = forks_2[0].lock().unwrap();
            sleep(std::time::Duration::from_secs(1));
            let right_fork = forks_2[1].lock().unwrap();
            println!("Philosopher 2 is Eating!");
            sleep(std::time::Duration::from_secs(rng.gen_range(1..5)));
            drop(left_fork);
            drop(right_fork);
        }
    });

    sleep(std::time::Duration::from_secs(60));
    
}

