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
pub struct Export(MajorVer, MinorVer, Metadata, Vec<Entry>);

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
    fn from_tree_and_name(name: String, tree: Tree) -> Vec<Entry> {
        match tree {
            Tree::File(size) => vec![Entry::IB(InfoBlock { name, dsize: size })],
            Tree::Dir(map) => {
                let dir_header = std::iter::once(Entry::IB(InfoBlock { name, dsize: 0 }));
                let dir_entries = map
                    .into_iter()
                    .flat_map(|(name, subtree)| Entry::from_tree_and_name(name, subtree));
                let dir = dir_header.chain(dir_entries);
                vec![Entry::Vec(dir.collect())]
            }
        }
    }
}

const BIRTH_TIMESTAMP: usize = 1699656086;
const PROGNAME: &str = "ncdu-import";
const PROGVER: &str = "1.0";
const MAJOR_VER: u16 = 1;
const MINOR_VER: u16 = 2;

impl From<Tree> for Export {
    fn from(value: Tree) -> Self {
        // Because we construct a `ROOT` directory at the base of the recursion,
        // Entry::from_tree_and_name will always contain an external
        // [Vec([...])] that we need to unwrap.
        let Entry::Vec(entry) = Entry::from_tree_and_name("ROOT".to_owned(), value)
            .pop()
            .unwrap()
        else {
            unreachable!()
        };

        Export(
            MajorVer(MAJOR_VER),
            MinorVer(MINOR_VER),
            Metadata {
                progname: PROGNAME,
                progver: PROGVER,
                timestamp: BIRTH_TIMESTAMP,
            },
            entry,
        )
    }
}
