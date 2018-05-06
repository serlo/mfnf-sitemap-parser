//! implementation of the sitemap builder.

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
            None => return Err("No heading (book) of depth 1 found!".into()),
        };

        let title = extract_plain_text(&heading.caption)
            .trim().to_string();

        let mlist = heading.content.iter()
            .filter_map(|e|
                if let Element::List(ref l) = *e {Some(l)} else {None})
            .next();

        Ok(Book {
            title,
            markers: if let Some(l) = mlist {
                marker_list(l)?
            } else {
                vec![]
            },
            parts: vec![],
        })
    } else {
        Err("Root element must be a \"Document\"!".into())
    }
}

pub fn subtarget_list(list: &List) -> Result<Vec<Subtarget>, String> {
    let mut result = vec![];
    for item in &list.content {
        let item = if let Element::ListItem(ref item) = *item {
            item
        } else {
            return Err("Non-listitem in subtarget list!".into())
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

        result.push(Subtarget {
            name,
            parameters: params.unwrap_or(vec![]),
        });
    }
    Ok(result)
}

pub fn marker_list(list: &List) -> Result<Vec<Marker>, String> {
    let mut result = vec![];
    for item in &list.content {
        let item = if let Element::ListItem(ref item) = *item {
            item
        } else {
            return Err("Non-listitem in marker list!".into())
        };

        let content_str = extract_plain_text(&item.content);
        let marker_id = content_str
            .split(':')
            .next()
            .map(|id| id.trim().to_lowercase());
        let marker_id = match marker_id {
            Some(id) => id,
            None => return Err("Markers must not be empty!".into())
        };
        let sublist = item.content.iter()
            .filter_map(|e| if let Element::List(ref l) = *e {Some(l)} else {None})
            .next();

        result.push(match marker_id.as_str() {
            "include" => Marker::IncludeMarker(IncludeMarker {
                subtargets: if let Some(l) = sublist {
                    subtarget_list(l)?
                } else {
                    vec![]
                },
            }),
            "exclude" => Marker::ExcludeMarker(ExcludeMarker {
                subtargets: if let Some(l) = sublist {
                    subtarget_list(l)?
                } else {
                    vec![]
                },
            }),
            "todo" => Marker::TodoMarker(TodoMarker {
                message: content_str.split(':')
                    .skip(1).collect::<Vec<&str>>().join(":")
                    .trim().to_string()
            }),
            "after" => Marker::AfterMarker(AfterMarker {
                path: content_str.split(':')
                    .skip(1).collect::<Vec<&str>>().join(":")
                    .trim().to_string()
            }),
            _ => return Err(format!("unknown marker: {}", marker_id))
        })
    }
    Ok(result)
}
