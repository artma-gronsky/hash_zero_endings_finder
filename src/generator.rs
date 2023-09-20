use anyhow::Result;
use crossbeam::channel::{Receiver, Sender};
use sha256;
use std::thread;

pub fn generate_number_loop(hash_generator_tx: Sender<usize>) -> Result<()> {
    for i in 1..=usize::MAX {
        if hash_generator_tx.send(i).is_err() {
            break;
        }
    }

    Ok(())
}

pub fn generate_hash_loop(
    hash_generator_rx: Receiver<usize>,
    finder_tx: Sender<(usize, String)>,
    num_threads: usize,
) -> Result<()> {
    let mut handlers = vec![];

    for _ in 0..num_threads {
        let rx = hash_generator_rx.clone();
        let tx = finder_tx.clone();
        let handler = thread::spawn(move || {
            while let Ok(number) = rx.recv() {
                let hash_digest = sha256::digest(number.to_string());
                if tx.send((number, hash_digest)).is_err() {
                    break;
                }
            }
        });

        handlers.push(handler);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());

    Ok(())
}
