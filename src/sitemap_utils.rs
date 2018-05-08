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


/// Extract information in various formats from a sitemap.
#[derive(StructOpt, Debug)]
#[structopt(name = "sitemap_utils")]
struct Opt {
    /// Input sitemap (yaml) file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: Option<PathBuf>,

    /// The subtarget to consider.
    #[structopt(name = "subtarget")]
    subtarget: String,

    /// Output a make-like list of articles included for a subtarget.
    #[structopt(short = "a", long = "articles")]
    article_extension: Option<String>
}

fn filename_to_make(input: &str) -> String {
    input.replace(" ", "_")
        .replace(":", "@COLON@")
        .replace("(", "@LBR@")
        .replace(")", "@RBR@")
        .replace("'", "@SQUOTE@")
        .replace('"', "@DQUOTE@")
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

    let sitemap: mfnf_sitemap::Book = serde_yaml::from_str(&input)
        .expect("Error parsing sitemap:");

    let subtarget = opt.subtarget.trim().to_lowercase();
    if let Some(article_extension) = opt.article_extension {
        print!("{}: ", &subtarget);
        for part in &sitemap.parts {
            for chapter in &part.chapters {
                if chapter.markers.include.subtargets.iter().any(|t| t.name == subtarget)
                    || chapter.markers.exclude.subtargets.iter()
                        .any(|t| t.name == subtarget && !t.parameters.is_empty()) {

                    println!("{}/{}.{} \\", &filename_to_make(&chapter.path),
                        &chapter.revision, &article_extension)
                }
            }
        }
    }
}
