use anyhow::Result;
use std::thread;

use crossbeam::channel;
use hash_zero_endings_finder::{args, finder, generator};
fn main() -> Result<()> {
    let args::Args { n, f } = args::Args::parse();

    let (hash_generator_tx, hash_generator_rx) = channel::bounded(1024);
    let (finder_tx, finder_rx) = channel::bounded(1024);

    let number_generator_handle =
        thread::spawn(move || generator::generate_number_loop(hash_generator_tx));

    let hash_generator_handle =
        thread::spawn(move || generator::generate_hash_loop(hash_generator_rx, finder_tx));

    let finder_generator_handle = thread::spawn(move || finder::finder_loop(finder_rx, n, f));

    let number_generator_result = number_generator_handle.join().unwrap();
    let hash_generator_result = hash_generator_handle.join().unwrap();
    finder_generator_handle.join().unwrap();

    number_generator_result?;
    hash_generator_result?;

    Ok(())
}
