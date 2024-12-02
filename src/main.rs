use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, sleep};
use rand::Rng;

fn main() {
    println!("Start Dining Philosophers Simulations!");
    // initialize 5 forks
    let forks = Arc::new(vec![
        Mutex::new(false),
        Mutex::new(false),
        Mutex::new(false),
        Mutex::new(false),
        Mutex::new(false),
    ]);

    for i in 0..5 {
        let forks_clone = Arc::clone(&forks);
        let left_fork_index: usize = i;
        let right_fork_index: usize = (i + 1) % 5;
        let is_left_first = i == 0;
        thread::spawn(move || {
            philosopher(i, forks_clone, left_fork_index, right_fork_index, is_left_first);
        });
        
    }
    // simulation duration
    sleep(std::time::Duration::from_secs(600));
    
}

fn philosopher(id: usize, forks: Arc<Vec<Mutex<bool>>>, left_fork: usize, right_fork: usize, is_left_first: bool) {
    loop {
        println!("Philosopher {} is Thinking!", id);
        // think for 5 - 10 seconds before gets hungry
        let mut rng = rand::thread_rng();
        sleep(std::time::Duration::from_secs(rng.gen_range(5..10)));
        println!("Philosopher {} is Hungry!", id);

        let first_fork_lock: MutexGuard<bool>;
        let second_fork_lock: MutexGuard<bool>;
        if is_left_first {
            first_fork_lock = forks[left_fork].lock().unwrap();
            //wait for 1 second before picking up the other fork to make it easier to deadlock
            sleep(std::time::Duration::from_secs(1)); 
            second_fork_lock = forks[right_fork].lock().unwrap();
        } else {
            first_fork_lock = forks[right_fork].lock().unwrap();
            //wait for 1 second before picking up the other fork to make it easier to deadlock
            sleep(std::time::Duration::from_secs(1)); 
            second_fork_lock = forks[left_fork].lock().unwrap();
        }
        println!("Philosopher {} is Eating!", id);
        sleep(std::time::Duration::from_secs(rng.gen_range(1..3))); //eat for 1-3 seconds
        drop(first_fork_lock);
        drop(second_fork_lock);
        // drop forks and go back to thinking
    }
}

