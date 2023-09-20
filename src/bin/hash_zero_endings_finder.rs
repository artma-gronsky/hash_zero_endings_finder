use anyhow::Result;
use std::thread;

use crossbeam::channel;
use hash_zero_endings_finder::{args, finder, generator};
fn main() -> Result<()> {
    let args::Args { n, f, concurrent } = args::Args::parse();

    let num_cpus = concurrent.unwrap_or(num_cpus::get());
    let (hash_generator_tx, hash_generator_rx) = channel::bounded(num_cpus);
    let (finder_tx, finder_rx) = channel::unbounded();

    let number_generator_handle =
        thread::spawn(move || generator::generate_number_loop(hash_generator_tx));

    let hash_generator_handle = thread::spawn(move || {
        generator::generate_hash_loop(hash_generator_rx, finder_tx, num_cpus)
    });

    let finder_generator_handle = thread::spawn(move || finder::finder_loop(finder_rx, n, f));

    let number_generator_result = number_generator_handle.join().unwrap();
    let hash_generator_result = hash_generator_handle.join().unwrap();
    finder_generator_handle.join().unwrap();

    number_generator_result?;
    hash_generator_result?;

    Ok(())
}
