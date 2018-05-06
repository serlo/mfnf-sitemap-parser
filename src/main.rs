extern crate mfnf_sitemap;

fn main() {
    println!("{:#?}", mfnf_sitemap::parse_sitemap(r#"
= Beispielbuch =
* include:
** all
** print
* exclude:
** minimal

== Grundlegende Formatierungen von Wikibooks ==
* [[Mathe für Nicht-Freaks: Beispielkapitel: Grundlegende Formatierungen|Grundlegende Formatierungen]]

== Eigene Formatierungen ==
* [[Mathe für Nicht-Freaks: Beispielkapitel: Semantische Blöcke|Semantische Blöcke]]

== Noprint-Content ==
* [[Mathe für Nicht-Freaks: Beispielkapitel: Inhalte im Druck unterbinden|Inhalte im Druck unterbinden]]
** include:
*** minimal
** exclude:
*** print:
**** Parameter von Vorlagen löschen
"#));
}
