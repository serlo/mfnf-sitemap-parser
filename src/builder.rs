//! implementation of the sitemap builder.

use sitemap::*;
use mediawiki_parser::*;
use mwparser_utils::util::extract_plain_text;

pub fn parse_book(root: &Element) -> Result<Book, String> {
    if let Element::Document(ref doc) = *root {
        let heading = doc.content.iter()
            .filter_map(|e|
                if let Element::Heading(ref h) = *e {Some(h)} else {None})
            .find(|h| h.depth == 1);

        let heading = match heading {
            Some(h) => h,
            None => return Err("No heading (book) of depth 1 found!".into()),
        };

        let title = extract_plain_text(&heading.caption)
            .trim().to_string();

        Ok(Book {
            title,
            markers: vec![],
            parts: vec![],
        })
    } else {
        Err("Root element must be a \"Document\"!".into())
    }
}

