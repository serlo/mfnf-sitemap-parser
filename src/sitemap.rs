//! Types representing the sitemap structure.

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// A book subtarget (e.g. `all`, `print`).
/// Parameters are only allowed for chapters.
#[derive(Serialize, Deserialize, Clone, Debug)]
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

impl PartialEq for Subtarget {
    fn eq(&self, other: &Subtarget) -> bool {
        self.name == other.name
    }
}

impl Eq for Subtarget {}

impl Hash for Subtarget {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Normalize for Markers {
    fn normalize(&mut self) -> Result<(), String> {
        if !self.include.subtargets.is_disjoint(&self.exclude.subtargets) {
            return Err("Subtargets cannot be included and \
                       excluded at the same time!".into())
        }
        Ok(())
    }
}

pub trait Normalize {
    /// Propagate includes / excludes the chapters. Chapter markers take precedence.
    fn normalize(&mut self) -> Result<(), String>;
}

impl Normalize for Chapter {
    fn normalize(&mut self) -> Result<(), String> {
        self.markers.normalize()
    }
}

impl Normalize for Part {
    fn normalize(&mut self) -> Result<(), String> {
        self.markers.normalize()?;
        for chapter in &mut self.chapters {
            chapter.normalize()?;
        }
        Ok(())
    }
}

impl Normalize for Book {
     fn normalize(&mut self) -> Result<(), String> {
        self.markers.normalize()?;
        for part in &mut self.parts {
            part.normalize()?;
        }
        Ok(())
    }
}


