# mfnf-sitemap-parser
A parser for "Mathe für Nicht-Freaks"-like sitemaps. Allows dependency export in a make-compatible format.

## Sitemap Format

A site-/ bookmap is a mediawiki article referring to other articles in a structured way. This description focuses mainly on bookmaps.
Books currently allow only one level of structure: *parts*:

```markdown
= Beispielbuch =
== Grundlegende Formatierungen von Wikibooks ==
* [[Mathe für Nicht-Freaks: Beispielkapitel: Grundlegende Formatierungen|Grundlegende Formatierungen]]

== Eigene Formatierungen ==
* [[Mathe für Nicht-Freaks: Beispielkapitel: Semantische Blöcke|Semantische Blöcke]]
```

The top-level heading declares the book title: `Beispielbuch`. The second-level headings declare two *parts* with one *chapter* (article) each. Chapters are declared as a mediawiki list.

## Markers (Include / Exclude / After / ...)

For each layer of the bookmap hierarchy, some meta information can be declared. This can be useful for tools like [MFNF pdf export](https://github.com/lodifice/mfnf-pdf-export). 

Currently, all markers are:

| Marker  | Functionality |
| ------- | --------------|
| include | For non-chapters: Sets default wether to include chapters for this subtarget. For chapters: Includes a chapter for this subtarget. May have a sublist listing all headings to include, all others will be excluded. |
| exclude | For non-chapters: Sets default wether to exclude chapters for this subtarget. For chapters: Excludes a chapter for this subtarget. May have a sublist listing all headings to exclude, all others will be included. |
| after | Sets a marker to include something after this *part* or *chapter* |


This example is specific to the pdf export tool:

```markdown
= Beispielbuch =
* include:
** all
** print
* exclude:
** minimal

== Noprint-Content ==
* [[Mathe für Nicht-Freaks: Beispielkapitel: Inhalte im Druck unterbinden|Inhalte im Druck unterbinden]]
** include:
*** minimal
** exclude:
*** print:
**** Parameter von Vorlagen löschen
```

Markers on deeper levels inherit markers of the top level. For example: for the `minimal` subtarget, all *parts* are excluded by default. But this default was overridden by a *chapter*, making it still appear in `minimal`.
For the `print` subtarget, the heading `Parameter von Vorlagen löschen` wil be omitted from the *chapter*.

