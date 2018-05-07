extern crate mfnf_sitemap;
extern crate serde_yaml;
#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use structopt::StructOpt;

/// Convert a sitemap MediaWiki article to a bookmap yaml.
///
/// Markers are propagated according to the marker inheritance rules.
#[derive(StructOpt, Debug)]
#[structopt(name = "mediawiki_bookmap")]
struct Opt {
    /// Input MediaWiki file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: Option<PathBuf>
}

fn main() {
    let opt = Opt::from_args();

    let mut input = String::new();
    match opt.input_file {
        Some(path) => {
            BufReader::new(
                File::open(&path).expect("Could not open input file!")
            ).read_to_string(&mut input)
                .expect("Could not read input file!");
        }
        None => {
            BufReader::new(io::stdin())
            .read_to_string(&mut input)
                .expect("Could not read input file!");
        }
    };

    println!("{}",
        serde_yaml::to_string(
            &mfnf_sitemap::parse_sitemap(&input)
                .expect("Error parsing sitemap:")
        ).expect("Could not serialize sitemap:")
    );
}
