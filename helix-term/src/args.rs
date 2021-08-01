use anyhow::Result;
use clap::{App, Arg};

use std::path::PathBuf;

#[derive(Default)]
pub struct Args {
    pub verbosity: u64,
    pub files: Vec<PathBuf>,
    pub open_at_line_no: Option<usize>,
}

impl Args {
    pub fn parse_args() -> Result<Args> {
        let mut args = Args::default();
        let matches = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::with_name("v")
                    .short("v")
                    .multiple(true)
                    .help("Sets level of verbosity"),
            )
            .arg(
                Arg::with_name("line_no")
                    .short("+")
                    .long("line-no")
                    .takes_value(true)
                    .help("Sets line number to open first FILE at"),
            )
            .arg(Arg::with_name("FILE").index(1).multiple(true))
            .get_matches();

        args.verbosity = matches.occurrences_of("v");
        if let Some(files) = matches.values_of("FILE") {
            args.files = files.map(|filename| PathBuf::from(filename)).collect();
        }
        if let Some(line_no) = matches.value_of("line_no") {
            args.open_at_line_no = match str::parse::<usize>(&line_no) {
                Err(_) => None,
                Ok(line_no) => Some(line_no),
            };
        }
        Ok(args)
    }
}
