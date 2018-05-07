//! Types representing the sitemap structure.

use std::collections::HashSet;

/// A book subtarget (e.g. `all`, `print`).
/// Parameters are only allowed for chapters.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Hash)]
pub struct Subtarget {
    pub name: String,
    pub parameters: Vec<String>,
}

/// A piece of meta data in the book hierarchy.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Markers {
    pub include: IncludeMarker,
    pub exclude: ExcludeMarker,
    pub todo: Option<TodoMarker>,
    pub after: Option<AfterMarker>,
}

/// Include a range of subtargets / headings.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct IncludeMarker {
    pub subtargets: HashSet<Subtarget>,
}

/// Exclude a range of subtargets / headings.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ExcludeMarker {
    pub subtargets: HashSet<Subtarget>,
}

/// Leave a todo message.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TodoMarker {
    pub message: String,
}

/// Include something after this node.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AfterMarker {
    pub path: String,
}

/// A complete book specification.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Book {
    pub title: String,
    pub markers: Markers,
    pub parts: Vec<Part>,
}

/// A part specification.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Part {
    pub title: String,
    pub markers: Markers,
    pub chapters: Vec<Chapter>,
}

/// A chapter / article specification.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Chapter {
    pub title: String,
    pub path: String,
    pub markers: Markers,
}

impl Normalize for Markers {
    fn normalize(&mut self) -> Result<(), String> {
        Ok(())
    }
}

trait Normalize {
    /// Propagate includes / excludes the chapters. Chapter markers take precedence.
    fn normalize(&mut self) -> Result<(), String>;
}

impl Normalize for Part {
    fn normalize(&mut self) -> Result<(), String> {
        Ok(())
    }
}
