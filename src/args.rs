use clap::Arg;

pub struct Args {
    pub n: usize,
    pub f: usize,
    pub concurrent: Option<usize>,
}

impl Args {
    pub fn parse() -> Self {
        let matches = clap::Command::new("hash_zero_endings_finder")
            .bin_name("hash_zero_endings_finder")
            .args([
                Arg::new("N")
                    .short('N')
                    .required(true)
                    .value_parser(clap::value_parser!(usize))
                    .help("Number of zero-symbols sha256 digest should ends"),
                Arg::new("F")
                    .short('F')
                    .required(true)
                    .value_parser(clap::value_parser!(usize))
                    .help("Number of hashes application should find"),
                Arg::new("concurrent")
                    .long("concurrent")
                    .required(false)
                    .value_parser(clap::value_parser!(usize))
                    .help("Number of wished concurrent thread for hash cacl"),
            ])
            .get_matches();

        let n = matches
            .get_one::<usize>("N")
            .map(|s| s.to_owned())
            .unwrap_or_default();

        let f = matches
            .get_one::<usize>("F")
            .map(|s| s.to_owned())
            .unwrap_or_default();

        let concurrent = matches.get_one::<usize>("concurrent").map(|s| s.to_owned());

        Self { n, f, concurrent }
    }
}
