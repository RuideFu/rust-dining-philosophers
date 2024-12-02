use rand::Rng;
use std::io;
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread::{self, sleep};

fn main() {
    println!("Start Dining Philosophers Simulations!");
    // initialize 5 forks
    let forks = Arc::new([
        Mutex::new(false),
        Mutex::new(false),
        Mutex::new(false),
        Mutex::new(false),
        Mutex::new(false),
    ]);

    // initialize vector for sending message from main thread to each philosopher
    let mut from_main_txes: Vec<mpsc::Sender<&str>> = Vec::new();
    // initialize vector for receving message from each philosopher for main thread
    let mut to_main_rxes: Vec<mpsc::Receiver<(usize, u32, u32)>> = Vec::new();

    for id in 0..5 {
        let forks_clone = Arc::clone(&forks);
        let left_fork_index: usize = id;
        let right_fork_index: usize = (id + 1) % 5;
        let is_left_first = id == 0;

        let (from_main_tx, from_main_rx): (mpsc::Sender<&str>, mpsc::Receiver<&str>) =
            mpsc::channel();
        let (to_main_tx, to_main_rx): (
            mpsc::Sender<(usize, u32, u32)>,
            mpsc::Receiver<(usize, u32, u32)>,
        ) = mpsc::channel();

        from_main_txes.push(from_main_tx);
        to_main_rxes.push(to_main_rx);

        thread::spawn(move || {
            let mut think_count: u32 = 0;
            let mut dine_count: u32 = 0;
            loop {
                // check if there is a message from main thread
                let msg = from_main_rx.try_recv().ok();
                // if there is a message, check if it is "exit"
                if msg.is_some() {
                    let msg = msg.unwrap();
                    if msg == "exit" {
                        // if it is "exit", break the loop
                        println!("Philosopher {} is Exiting!", id);
                        to_main_tx.send((id, think_count, dine_count)).unwrap();
                        break;
                    }
                }
                println!("Philosopher {} is Thinking!", id);
                // think for 5 - 10 seconds before gets hungry
                let mut rng = rand::thread_rng();
                sleep(std::time::Duration::from_secs(rng.gen_range(5..10)));
                think_count += 1;
                println!("Philosopher {} is Hungry!", id);

                let first_fork_lock: MutexGuard<bool>;
                let second_fork_lock: MutexGuard<bool>;
                if is_left_first {
                    first_fork_lock = forks_clone[left_fork_index].lock().unwrap();
                    //wait for 1 second before picking up the other fork to make it easier to deadlock
                    sleep(std::time::Duration::from_secs(1));
                    second_fork_lock = forks_clone[right_fork_index].lock().unwrap();
                } else {
                    first_fork_lock = forks_clone[right_fork_index].lock().unwrap();
                    //wait for 1 second before picking up the other fork to make it easier to deadlock
                    sleep(std::time::Duration::from_secs(1));
                    second_fork_lock = forks_clone[left_fork_index].lock().unwrap();
                }
                println!("Philosopher {} is Eating!", id);
                sleep(std::time::Duration::from_secs(rng.gen_range(1..3))); //eat for 1-3 seconds
                drop(first_fork_lock);
                drop(second_fork_lock);
                // drop forks and go back to thinking
                dine_count += 1;
            }
        });
    }
    // simulation duration
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_string();
        if input == "exit" {
            for tx in from_main_txes.iter() {
                tx.send("exit").unwrap();
            }
            for rx in to_main_rxes.iter() {
                let (id, think_count, dine_count) = rx.recv().unwrap();
                println!(
                    "Philosopher {} thought {} times and dined {} times",
                    id, think_count, dine_count
                );
            }
            println!("Dining Philosophers Simulations! Terminated!");
            break;
        }
    }
}
