use crossbeam::channel::Receiver;

mod output;

pub fn finder_loop(
    finder_rx: Receiver<(usize, String)>,
    end_with_zero_numbers: usize,
    count_to_find: usize,
) {
    let mut result: Vec<(usize, String)> = vec![];

    let zero_numbers_str: String = "0".repeat(end_with_zero_numbers);

    while let Ok((number, digest)) = finder_rx.recv() {
        if digest.ends_with(&zero_numbers_str) {
            output::output_number_with_hash(&number, &digest);
            result.push((number, digest));
            if result.len() >= count_to_find {
                break;
            }
        }
    }
}
