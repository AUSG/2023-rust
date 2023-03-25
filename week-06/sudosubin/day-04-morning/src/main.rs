use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        let left_fork = self.left_fork.lock().unwrap();
        let right_fork = self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // Create forks
    let mut forks = Vec::new();
    for _ in 0..(PHILOSOPHERS.len()) {
        forks.push(Arc::new(Mutex::new(Fork {})))
    }

    // Create philosophers
    let (tx, rx) = mpsc::sync_channel(10);
    let mut philosophers = Vec::new();
    for (index, name) in PHILOSOPHERS.iter().enumerate() {
        let condition = index + 1 == PHILOSOPHERS.len();
        let index_cur = if condition { 0 } else { index };
        let index_next = if condition { index } else { index + 1 };

        philosophers.push(Philosopher {
            name: name.to_string(),
            left_fork: forks[index_cur].clone(),
            right_fork: forks[index_next].clone(),
            thoughts: tx.clone(),
        });
    }

    // Make them think and eat
    for philosopher in philosophers {
        thread::spawn(move || {
            for _ in 0..10 {
                philosopher.think();
                philosopher.eat();
            }
        });
    }

    drop(tx);

    // Output their thoughts
    for thought in rx {
        println!("{}", thought);
    }
}
