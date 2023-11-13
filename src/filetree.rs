/// Struct representing a file and its size
#[derive(Debug, Clone)]
pub struct SizedFile {
    pub path: String,
    pub size: usize,
}

/// Tree representing a file directory tree, storing minimal metadata about the files.
///
/// Directories are asserted to have zero overhead size of their own.
#[derive(Debug, PartialEq)]
pub enum Tree {
    /// Size of a leaf file
    File(usize),

    /// Map of entries within a directory
    Dir(std::collections::BTreeMap<String, Tree>),
}

impl Tree {
    fn add(&mut self, sf: SizedFile) {
        match self {
            Tree::File(_) => panic!(),
            Tree::Dir(ref mut dir) => {
                let sp = sf.path.split_once('/');
                match sp {
                    Some((car, cdr)) => {
                        if let Some(Tree::File(_)) = dir.get(car) {
                            dir.remove(car);
                        }
                        dir.entry(car.to_owned())
                            .or_insert(Tree::Dir(Default::default()))
                            .add(SizedFile {
                                path: cdr.to_owned(),
                                size: sf.size,
                            });
                    }
                    None => {
                        if let Some(Tree::Dir(_)) = dir.get(&sf.path) {
                        } else {
                            dir.insert(sf.path.clone(), Tree::File(sf.size));
                        };
                    }
                }
            }
        }
    }

    pub fn from(files: Vec<SizedFile>) -> Tree {
        let mut result: Tree = Tree::Dir(Default::default());
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
