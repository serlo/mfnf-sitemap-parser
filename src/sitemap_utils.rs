extern crate mfnf_sitemap;
extern crate serde_yaml;
#[macro_use]
extern crate structopt;
extern crate mwparser_utils;

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io;
use structopt::StructOpt;
use std::collections::HashMap;
use std::process;

use mwparser_utils::util::filename_to_make;

/// Extract information in various formats from a sitemap.
#[derive(StructOpt, Debug)]
#[structopt(name = "sitemap_utils", about = "various tools for \
extracting information from a mfnf sitemap")]
struct Opt {
    /// Input sitemap (yaml) file
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_file: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Generate a list of dependencies for this sitemap
    #[structopt(name = "deps")]
    Deps {
        /// The target to build for.
        #[structopt(name = "target")]
        target: String,

        /// The subtarget (configuration) to consider.
        #[structopt(name = "subtarget")]
        subtarget: String,
    },
    /// Get markers for an article.
    /// Also prepend subtargets with target, like: print -> latex.print.
    #[structopt(name = "markers")]
    Markers {
        /// Name of the article to get markers for.
        #[structopt(name = "article")]
        article: String,

        /// The target to prepend
        #[structopt(name = "target")]
        target: String,
    }
}

fn main() {
    let opt = Opt::from_args();

    let mut target_extension_map = HashMap::new();
    target_extension_map.insert("latex".to_string(), "tex");
    target_extension_map.insert("markdown".to_string(), "md");
    target_extension_map.insert("html".to_string(), "html");
    target_extension_map.insert("pdf".to_string(), "pdf");

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

    match opt.cmd {
        Command::Deps { ref subtarget, ref target } => {
            let subtarget = subtarget.trim().to_lowercase();
            let article_extension = target_extension_map.get(target)
                .expect(&format!("no file extension defined for target {}!", &target));

            match target.as_str() {
                "pdf" => {
                    println!("{}.{}: $(BASE)/book_exports/$(BOOK)/latex/{}/{}.tex",
                        &subtarget, &article_extension, &subtarget, &subtarget);
                    return
                }
                _ => (),
            };

            print!("{}.{}: ", &subtarget, &article_extension);
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
                        include_string.push_str(&format!("include {}/{}.dep\n",
                            &chapter_path, &chapter.revision));
                    }
                }
            }
            println!();
            println!("{}", &include_string);
        },
        Command::Markers { ref article, ref target } => {
            let article = article.trim().to_lowercase();
            for part in &sitemap.parts {
                for chapter in &part.chapters {
                    if chapter.path.trim().to_lowercase() == article {
                        let mut markers = chapter.markers.clone();
                        let update_name = |name: &mut String| {
                            name.insert(0, '.');
                            name.insert_str(0, target);
                        };
                        let new_include = markers.include.subtargets.drain(..)
                            .map(|mut subtarget| {update_name(&mut subtarget.name); subtarget})
                            .collect();
                        let new_exclude = markers.exclude.subtargets.drain(..)
                            .map(|mut subtarget| {update_name(&mut subtarget.name); subtarget})
                            .collect();

                        markers.include.subtargets = new_include;
                        markers.exclude.subtargets = new_exclude;

                        println!("{}", &serde_yaml::to_string(&markers)
                            .expect("could not serialize markers:"));
                        return
                    }
                }
            }
            eprintln!("chapter not found: {}", article);
            process::exit(1);
        }
    }
}
