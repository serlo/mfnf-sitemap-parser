//! Types representing the sitemap structure.

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
    pub subtargets: Vec<Subtarget>,
}

/// Exclude a range of subtargets / headings.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct ExcludeMarker {
    pub subtargets: Vec<Subtarget>,
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
    pub revision: String,
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
        for include in &self.include.subtargets {
            if self.exclude.subtargets.contains(&include) {
                return Err(format!(
                    "{} is included and \
                     excluded at the same time!",
                    &include.name
                ));
            }
        }
        Ok(())
    }
}

fn deny_parameters(markers: &Markers) -> Result<(), String> {
    for subtarget in &markers.include.subtargets {
        if !subtarget.parameters.is_empty() {
            return Err("Include markers can only have parameters at \
                        the chapter level!"
                .into());
        }
    }
    for subtarget in &markers.exclude.subtargets {
        if !subtarget.parameters.is_empty() {
            return Err("Exclude markers can only have parameters at \
                        the chapter level!"
                .into());
        }
    }
    Ok(())
}

fn child_overrides(child_markers: &Markers, subtarget: &Subtarget) -> bool {
    child_markers.include.subtargets.contains(subtarget)
        || child_markers.exclude.subtargets.contains(subtarget)
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
        deny_parameters(&self.markers)?;

        for child in &mut self.chapters {
            for subtarget in &self.markers.include.subtargets {
                if !child_overrides(&child.markers, subtarget) {
                    child.markers.include.subtargets.push(subtarget.clone());
                }
            }
            for subtarget in &self.markers.exclude.subtargets {
                if !child_overrides(&child.markers, subtarget) {
                    child.markers.exclude.subtargets.push(subtarget.clone());
                }
            }
        }
        for chapter in &mut self.chapters {
            chapter.normalize()?;
        }
        Ok(())
    }
}

impl Normalize for Book {
    fn normalize(&mut self) -> Result<(), String> {
        self.markers.normalize()?;
        deny_parameters(&self.markers)?;

        for child in &mut self.parts {
            for subtarget in &self.markers.include.subtargets {
                if !child_overrides(&child.markers, subtarget) {
                    child.markers.include.subtargets.push(subtarget.clone());
                }
            }
            for subtarget in &self.markers.exclude.subtargets {
                if !child_overrides(&child.markers, subtarget) {
                    child.markers.exclude.subtargets.push(subtarget.clone());
                }
            }
        }

        for part in &mut self.parts {
            part.normalize()?;
        }
        let new_parts = self
            .parts
            .drain(..)
            .filter(|part| !part.chapters.is_empty())
            .collect();

        self.parts = new_parts;
        Ok(())
    }
}
