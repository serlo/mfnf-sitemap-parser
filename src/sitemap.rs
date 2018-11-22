//! Types representing the sitemap structure.

use std::hash::{Hash, Hasher};
use std::collections::HashMap;

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
    pub alias: AliasMarker,
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

/// Define an alias for a subtarget.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct AliasMarker {
    pub mapping: HashMap<String, String>,
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
        let include_has = | name | self.include.subtargets.iter().any(|s| &s.name == name);
        let exclude_has = | name | self.exclude.subtargets.iter().any(|s| &s.name == name);

        for (alias, subtarget) in &self.alias.mapping {
            if include_has(alias) || exclude_has(alias) {
                return Err(format!("{} is an alias but also present in \
                                    include / exclude!", &subtarget))
            }
            if !(include_has(subtarget) || exclude_has(subtarget)) {
                return Err(format!("{0} is an alias for {1}, but {1} is not \
                                    present in includes or excludes!",
                                    &alias, &subtarget))
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
        self.markers.normalize()?;

        // expand aliases
        for (alias, subtarget) in &self.markers.alias.mapping {
            let include = self.markers.include.subtargets.iter()
                .find(|s| &s.name == subtarget).map(|e| e.to_owned());
            if let Some(include) = include {
                let mut new = include;
                new.name = alias.clone();
                self.markers.include.subtargets.push(new);
            }

            let exclude = self.markers.exclude.subtargets.iter()
                .find(|s| &s.name == subtarget).map(|e| e.to_owned());
            if let Some(exclude) = exclude {
                let mut new = exclude;
                new.name = alias.clone();
                self.markers.exclude.subtargets.push(new);
            }
        }
        Ok(())
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
            child.markers.alias.mapping.extend(self.markers.alias.mapping.clone());

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
            child.markers.alias.mapping.extend(self.markers.alias.mapping.clone());
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
