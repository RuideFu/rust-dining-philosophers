# Rust Dining Philosopher

## Paradigm
I decided to go with the functional programming paradigm and encapsulate philosopher's behavior as a function.
I didn't use any of the objected oriented feature of Rust.

## Prevent Race Condition
To prevent race condition and dead lock when several philosophers try to pick the same folk simultaneously,
I represnet forks as an array of mutex. 
Since each fork only have two states: used or idle, 
what is contained in the mutex doesn't really matter as all we need is to hold the lock.
I decided to make the mutex contains a boolean type to indicate idle as false and in-use as true.

Since all the forks are mutexes, they are shared data among all the philosophers which are running on individual threads.
The data sharing is achieved by cloning the mutex pointers to get around ownership related bugs.

## Channels
Since the philosophers have shared the state of the forks and the mutexes ensure atomic operations, 
there's no need for the philosophers to communicate to each other.
However, channels are important to signal all the philosophers to shut down after a signal is given from the main thread.

To achieve that, two-way communication is required:
* the main thread needs to send message to the philosophers so they can start shut down once finish dining or thinking
* the philosophers need to send message back to the main thread once they finish the shut down sequence

Hence I created two pair of channels for each philospher. 
and book keep the senders to and the receivers from the philosophers in the main thread to implement the shut down feature.

## Running the Simulation

I use cargo to manage package (`rand` for random thinking time), and compiling.

Input `cargo run` in the command line should start the simluation,
each philosopher will print to the console whenever their state changes.

To stop the simluation, input `exit`.
Philosophers will announce they receive the exit signal and report counts as they exit.