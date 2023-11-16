/// Struct representing a file and its size
#[derive(Debug, Clone)]
pub struct SizedFile {
    pub path: String,
    pub size: usize,
}

/// Tree representing a file directory tree, storing minimal metadata about the files.
///
/// Directories are asserted to have zero overhead size of their own.
#[derive(Debug, PartialEq, Default)]
pub struct Tree {
    /// Size of this tree element, *excluding* children
    pub size: usize,

    pub children: std::collections::BTreeMap<String, Tree>,
}

impl Tree {
    fn add(&mut self, sf: SizedFile) {
        // If this directory includes children, zero out its size (`du` lists it
        // as a sum of all child node sizes).
        self.size = 0;
        let sp = sf.path.split_once('/');
        match sp {
            Some((car, cdr)) => {
                self.children
                    .entry(car.to_owned())
                    .or_insert(Default::default())
                    .add(SizedFile {
                        path: cdr.to_owned(),
                        size: sf.size,
                    });
            }
            None => {
                self.children.entry(sf.path.clone()).or_insert(
                    Tree {
                        size: sf.size,
                        children: Default::default(),
                    },
                );
            }
        }
    }

    pub fn from(files: Vec<SizedFile>) -> Tree {
        let mut result: Tree = Default::default();
        for sf in files {
            result.add(sf);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parent_dir_should_be_ignored() {
        let got = Tree::from(vec![
            SizedFile {
                path: "dir/file".to_owned(),
                size: 1,
            },
            SizedFile {
                path: "dir".to_owned(),
                size: 2,
            },
        ]);

        let want = Tree::from(vec![SizedFile {
            path: "dir/file".to_owned(),
            size: 1,
        }]);

        assert_eq!(want, got);
    }

    #[test]
    fn child_dir_should_replace() {
        let got = Tree::from(vec![
            SizedFile {
                path: "dir".to_owned(),
                size: 2,
            },
            SizedFile {
                path: "dir/file".to_owned(),
                size: 1,
            },
        ]);

        let want = Tree::from(vec![SizedFile {
            path: "dir/file".to_owned(),
            size: 1,
        }]);

        assert_eq!(want, got);
    }
}
