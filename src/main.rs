extern crate mfnf_sitemap;
extern crate mediawiki_parser;
extern crate mwparser_utils;
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
    /// Input AST of a MediaWiki file as YAML.
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: Option<PathBuf>,

    /// Path to texvccheck binary to transform formulas in headings.
    #[structopt(short = "p", long = "texvccheck-path", parse(from_os_str))]
    texvccheck_path: Option<PathBuf>,
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

    let mut tree: mediawiki_parser::Element = serde_yaml::from_str(&input)
        .expect("error reading input file:");

    if let Some(ref path) = opt.texvccheck_path {
        let checker = mwparser_utils::util::CachedTexChecker::new(path, 1000);
        tree = mwparser_utils::transformations::normalize_math_formulas(tree, &checker)
            .expect("error in formula normalization:")
    }

    let sitemap = &mfnf_sitemap::parse_sitemap(&tree)
        .expect("Error parsing sitemap:");

    println!("{}",
        serde_yaml::to_string(&sitemap)
            .expect("Could not serialize sitemap:")
    );
}
