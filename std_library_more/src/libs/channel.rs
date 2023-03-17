use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static NTHEARDS: i32 = 3;

pub fn play() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHEARDS {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            thread_tx.send(id).unwrap();

            println!("thread {} finished", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHEARDS as usize);
    for _ in 0..NTHEARDS {
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}
