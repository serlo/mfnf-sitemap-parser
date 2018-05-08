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
    input_file: Option<PathBuf>,

    /// Output a make-compatible book dependency file for a subtarget.
    #[structopt(short = "d", long = "deps")]
    subtarget: Option<String>,

    /// Target file extension for <subtarget>.
    #[structopt(short = "e", long = "ext")]
    target_extension: Option<String>
}

fn filename_to_make(input: &str) -> String {
    input.replace(" ", "_")
        .replace(":", "@COLON@")
        .replace("(", "@LBR@")
        .replace(")", "@RBR@")
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

    let sitemap = &mfnf_sitemap::parse_sitemap(&input)
        .expect("Error parsing sitemap:");
    if let Some(subtarget) = opt.subtarget {
        let subtarget = subtarget.trim().to_lowercase();
        for part in &sitemap.parts {
            for chapter in &part.chapters {
                if chapter.markers.include.subtargets.iter().any(|t| t.name == subtarget)
                    || chapter.markers.exclude.subtargets.iter()
                        .any(|t| t.name == subtarget && !t.parameters.is_empty()) {

                    println!("{}/latest.{}", &filename_to_make(&chapter.path),
                        &opt.target_extension.clone().unwrap_or("txt".into()))
                }
            }
        }
    } else {
        println!("{}",
            serde_yaml::to_string(&sitemap)
                .expect("Could not serialize sitemap:")
        );
    }
}
