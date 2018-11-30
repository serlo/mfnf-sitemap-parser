extern crate mediawiki_parser;
extern crate mfnf_sitemap;
extern crate mwparser_utils;
extern crate serde_json;
extern crate structopt;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

/// Convert a sitemap MediaWiki article to a bookmap yaml.
///
/// Markers are propagated according to the marker inheritance rules.
#[derive(StructOpt, Debug)]
#[structopt(name = "mediawiki_bookmap")]
struct Opt {
    /// Input AST of a MediaWiki file as YAML.
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let mut input = String::new();
    match opt.input_file {
        Some(path) => {
            BufReader::new(File::open(&path).expect("Could not open input file!"))
                .read_to_string(&mut input)
                .expect("Could not read input file!");
        }
        None => {
            BufReader::new(io::stdin())
                .read_to_string(&mut input)
                .expect("Could not read input file!");
        }
    };

    let tree: mediawiki_parser::Element =
        serde_json::from_str(&input).expect("error reading input file:");

    let sitemap = &mfnf_sitemap::parse_sitemap(&tree).expect("Error parsing sitemap:");

    println!(
        "{}",
        serde_json::to_string(&sitemap).expect("Could not serialize sitemap:")
    );
}
