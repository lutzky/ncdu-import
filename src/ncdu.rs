//! # ncdu export
//!
//! This module provides a mechanism for exporting [ncdu] files.
//!
//! [ncdu]: https://dev.yorhel.nl/ncdu
//!
//! Format reference: <https://dev.yorhel.nl/ncdu/jsonfmt>
use crate::filetree::Tree;
use serde::Serialize;

#[derive(Serialize)]
/// A representation of `ncdu`'s export format
///
/// When serialized to JSON, this should be ingestible using `ncdu -f`.
pub struct Export(MajorVer, MinorVer, Metadata, Entry);

#[derive(Serialize)]
struct Metadata {
    progname: &'static str,
    progver: &'static str,
    timestamp: usize,
}

#[derive(Serialize, PartialEq)]
struct InfoBlock {
    name: String,
    dsize: usize,
}

#[derive(Serialize, PartialEq)]
#[serde(untagged)]
enum Entry {
    IB(InfoBlock),
    Vec(Vec<Entry>),
}

#[derive(Serialize)]
struct MajorVer(u16);
#[derive(Serialize)]
struct MinorVer(u16);

impl Entry {
    fn from_tree_and_name(name: String, tree: Tree) -> Entry {
        let dir_header = Entry::IB(InfoBlock {
            name,
            dsize: tree.size,
        });
        if tree.children.is_empty() {
            return dir_header;
        }
        let dir_header = std::iter::once(dir_header);
        let dir_entries = tree
            .children
            .into_iter()
            .map(|(name, subtree)| Entry::from_tree_and_name(name, subtree));
        let dir = dir_header.chain(dir_entries);
        Entry::Vec(dir.collect())
    }
}

const BIRTH_TIMESTAMP: usize = 1699656086;
const PROGNAME: &str = "ncdu-import";
const PROGVER: &str = "1.0";
const MAJOR_VER: u16 = 1;
const MINOR_VER: u16 = 2;

impl From<Tree> for Export {
    fn from(value: Tree) -> Self {
        Export(
            MajorVer(MAJOR_VER),
            MinorVer(MINOR_VER),
            Metadata {
                progname: PROGNAME,
                progver: PROGVER,
                timestamp: BIRTH_TIMESTAMP,
            },
            Entry::from_tree_and_name("ROOT".to_owned(), value),
        )
    }
}
