//! Types representing the sitemap structure.

/// A book subtarget (e.g. `all`, `print`).
/// Parameters are only allowed for chapters.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Subtarget {
    pub name: String,
    pub parameters: Vec<String>,
}

/// A piece of meta data in the book hierarchy.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Marker {
    IncludeMarker(IncludeMarker),
    ExcludeMarker(ExcludeMarker),
    TodoMarker(TodoMarker),
    AfterMarker(AfterMarker),
}

/// Include a range of subtargets / headings.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct IncludeMarker {
    pub subtargets: Vec<Subtarget>,
}

/// Exclude a range of subtargets / headings.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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
    pub markers: Vec<Marker>,
    pub parts: Vec<Part>,
}

/// A part specification.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Part {
    pub title: String,
    pub markers: Vec<Marker>,
    pub chapters: Vec<Chapter>,
}

/// A chapter / article specification.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Chapter {
    pub title: String,
    pub markers: Vec<Marker>,
}
