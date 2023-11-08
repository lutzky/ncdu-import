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
                    Some((car, cdr)) => dir
                        .entry(car.to_owned())
                        .or_insert(Tree::Dir(Default::default()))
                        .add(SizedFile {
                            path: cdr.to_owned(),
                            size: sf.size,
                        }),
                    None => {
                        dir.insert(sf.path.clone(), Tree::File(sf.size));
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
