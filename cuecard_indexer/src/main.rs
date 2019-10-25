extern crate structopt;
extern crate cuecard_indexer;
extern crate env_logger;

use structopt::StructOpt;
use std::path::PathBuf;

use std::env;

#[derive(StructOpt, Debug)]
#[structopt(name = "cuecard_indexer")]
/// Creates an index of cue cards in a Sqlite database
struct ProgramOptions {
    #[structopt(short, long)]
    /// Sets the path to the databbase to use
    database: Option<String>,

    #[structopt(parse(from_os_str))]
    /// Sets the base directory for the cue card collection
    input: PathBuf
}

fn main() {
    env_logger::init();
    let options = ProgramOptions::from_args();

    let database_url = match options.database {
        Some(opt) => opt.to_string(),
        _ => env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };

    let config = cuecard_indexer::Config {
        basepath: options.input.to_str().expect("Base directory of the cue card collection exptected").to_string(),
        database_url
    };

    cuecard_indexer::run(&config);
}
