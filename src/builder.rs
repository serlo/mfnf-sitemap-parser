//! implementation of the sitemap builder.

use std::collections::HashSet;

use sitemap::*;
use mediawiki_parser::*;
use mwparser_utils::util::extract_plain_text;

pub fn book(root: &Element) -> Result<Book, String> {
    if let Element::Document(ref doc) = *root {
        let heading = doc.content.iter()
            .filter_map(|e|
                if let Element::Heading(ref h) = *e {Some(h)} else {None})
            .find(|h| h.depth == 1);

        let heading = match heading {
            Some(h) => h,
            None => return Err(format!("line: {}: No heading (book) of depth 1 found!",
                root.get_position().start.line)),
        };

        let title = extract_plain_text(&heading.caption)
            .trim().to_string();

        let mlist = heading.content.iter()
            .filter_map(|e|
                if let Element::List(ref l) = *e {Some(l)} else {None})
            .next();
        let subheadings = heading.content.iter()
            .filter_map(|e| if let Element::Heading(ref h) = *e {Some(h)} else {None});

        let mut parts = vec![];
        for subheading in subheadings {
            parts.push(part(subheading)?)
        }

        let mut book = Book {
            title,
            markers: match mlist {
                Some(l) => marker_list(l)?,
                None => Markers::default()
            },
            parts,
        };
        book.normalize()?;
        Ok(book)
    } else {
        Err(format!("line: {}: Root element must be a \"Document\"!",
            root.get_position().start.line))
    }
}

pub fn part(heading: &Heading) -> Result<Part, String> {
    let title = extract_plain_text(&heading.caption)
        .trim().to_string();
    let lists = heading.content.iter()
        .filter_map(|e| if let Element::List(ref l) = *e {Some(l)} else {None})
        .collect::<Vec<&List>>();
    let first_is_marker = lists.first()
        .map(|l| l.content.iter()
            .filter_map(|e| if let Element::ListItem(ref i) = *e {Some(i)} else {None})
            .all(|i| i.content.iter().all(
                |e| if let Element::InternalReference(_) = *e {false} else {true})
            )
        ).unwrap_or(false);

    let markers = if first_is_marker {
        match lists.first() {
            Some(list) => marker_list(list)?,
            None => Markers::default()
        }
    } else {
        Markers::default()
    };

    let chapters = if first_is_marker {
        lists.get(1)
    } else {
        lists.get(0)
    };

    let chapters = match chapters {
        Some(list) => {
            let mut result = vec![];
            for item in &list.content {
                if let Element::ListItem(ref i) = *item {
                    result.push(chapter(i)?)
                }
            }
            result
        },
        None => vec![],
    };

    Ok(Part {
        title,
        markers,
        chapters,
    })
}

pub fn chapter(item: &ListItem) -> Result<Chapter, String> {
    let article_ref = item.content.iter().filter_map(
        |e| if let Element::InternalReference(ref i) = *e {Some(i)} else {None})
        .next();
    let article_ref = match article_ref {
        Some(r) => r,
        None => return Err(format!("line {}: Chapter list item must have an \
                           internal reference to an article!", item.position.start.line))
    };

    let mlist = item.content.iter().filter_map(
        |e| if let Element::List(ref l) = *e {Some(l)} else {None})
        .next();
    let markers = match mlist {
        Some(list) => marker_list(&list)?,
        None => Markers::default(),
    };

    Ok(Chapter {
        title: extract_plain_text(&article_ref.caption).trim().to_string(),
        path: extract_plain_text(&article_ref.target).trim().to_string(),
        revision: "latest".into(),
        markers,
    })
}

pub fn subtarget_list(list: &List) -> Result<HashSet<Subtarget>, String> {
    let mut result = HashSet::new();
    for item in &list.content {
        let item = if let Element::ListItem(ref item) = *item {
            item
        } else {
            return Err(format!("line: {}: Non-listitem in subtarget list!",
                list.position.start.line))
        };
        let name = extract_plain_text(&item.content)
            .trim()
            .trim_right_matches(":")
            .to_lowercase();

        let params = item.content.iter()
            .filter_map(|e| if let Element::List(ref l) = *e {Some(l)} else {None})
            .next()
            .map(|l| l.content.iter()
                 .filter_map(|e|
                     if let Element::ListItem(ref i) = *e {Some(i)} else {None})
                 .map(|i| extract_plain_text(&i.content).trim().to_string())
                 .collect::<Vec<String>>()
            );

        result.insert(Subtarget {
            name,
            parameters: params.unwrap_or(vec![]),
        });
    }
    Ok(result)
}

pub fn marker_list(list: &List) -> Result<Markers, String> {
    let mut result = Markers::default();
    for item in &list.content {
        let item = if let Element::ListItem(ref item) = *item {
            item
        } else {
            return Err(format!("line: {}: Non-listitem in marker list!",
                list.position.start.line))
        };

        let content_str = extract_plain_text(&item.content);
        let value_str = content_str.split(':')
            .skip(1).collect::<Vec<&str>>().join(":")
            .trim().to_string();

        let marker_id = content_str
            .split(':')
            .next()
            .map(|id| id.trim().to_lowercase());
        let marker_id = match marker_id {
            Some(id) => id,
            None => return Err(format!("line: {}: Markers must not be empty!",
                list.position.start.line))
        };
        let sublist = item.content.iter()
            .filter_map(|e| if let Element::List(ref l) = *e {Some(l)} else {None})
            .next();

        match marker_id.as_str() {
            "include" => if let Some(l) = sublist {
                    result.include.subtargets = subtarget_list(l)?
                },
            "exclude" => if let Some(l) = sublist {
                    result.exclude.subtargets = subtarget_list(l)?
                },
            "todo" => result.todo = Some(TodoMarker {
                message: value_str
            }),
            "after" => result.after = Some(AfterMarker {
                path: value_str
            }),
            _ => return Err(format!("line: {}: unknown marker: {}",
                item.position.start.line, marker_id))
        };
    }
    Ok(result)
}
