extern crate mediawiki_parser;
extern crate mwparser_utils;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

mod sitemap;
mod builder;

use std::error::Error;

pub use sitemap::*;

pub fn parse_sitemap(input: &str) -> Result<Book, String> {
    let tree = mediawiki_parser::parse(input);
    // TODO: better errors
    let tree = match tree {
        Ok(t) => t,
        Err(error) => return Err(error.description().into())
    };
    return builder::parse_book(&tree);
}
