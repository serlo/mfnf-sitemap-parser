extern crate mediawiki_parser;
extern crate mwparser_utils;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

mod builder;
mod sitemap;

pub use sitemap::*;

pub fn parse_sitemap(tree: &mediawiki_parser::Element) -> Result<Book, String> {
    return builder::book(&tree);
}
