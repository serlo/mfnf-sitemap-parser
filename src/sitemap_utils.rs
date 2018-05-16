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
use std::collections::HashMap;


/// Extract information in various formats from a sitemap.
#[derive(StructOpt, Debug)]
#[structopt(name = "sitemap_utils")]
struct Opt {
    /// Input sitemap (yaml) file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: Option<PathBuf>,

    /// The target to build for.
    #[structopt(name = "target")]
    target: String,

    /// The subtarget to consider.
    #[structopt(name = "subtarget")]
    subtarget: String,

    /// Generate a list of dependencies for this sitemap
    #[structopt(short = "d", long = "deps")]
    generate_dependencies: bool,
}

fn filename_to_make(input: &str) -> String {
    input.replace(" ", "_")
        .replace(":", "@COLON@")
        .replace("(", "@LBR@")
        .replace(")", "@RBR@")
        .replace("/", "@SLASH@")
        .replace("'", "@SQUOTE@")
        .replace('"', "@DQUOTE@")
}

fn main() {
    let opt = Opt::from_args();

    let mut target_extension_map = HashMap::new();
    target_extension_map.insert("latex".to_string(), "tex");
    target_extension_map.insert("markdown".to_string(), "md");
    target_extension_map.insert("html".to_string(), "html");

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
    if opt.generate_dependencies {
        let article_extension = target_extension_map.get(&opt.target)
            .expect(&format!("no file extension defined for target {}!", &opt.target));

        print!("{}.{}: ", &opt.subtarget, &article_extension);
        let mut include_string = String::new();
        for part in &sitemap.parts {
            for chapter in &part.chapters {
                if chapter.markers.include.subtargets.iter().any(|t| t.name == subtarget)
                    || chapter.markers.exclude.subtargets.iter()
                        .any(|t| t.name == subtarget && !t.parameters.is_empty()) {

                    let chapter_path = filename_to_make(&chapter.path);
                    print!("{}/{}.dep {}/{}.{} ",
                        &chapter_path, &chapter.revision,
                        &chapter_path, &chapter.revision, &article_extension
                    );
                    include_string.push_str(&format!("include {}/{}.dep\n", &chapter_path, &chapter.revision));
                }
            }
        }
        println!();
        println!("{}", &include_string);
    }
}
