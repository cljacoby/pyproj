// NOTE: Thinking it might make more sense to read the project file structue
// from a static config file, rather than making invidual flag options for every
// optional file/directory. This would also probably involve re-adding the builder
// for PyProj with all the CLI flags, and then creating a represenation of the file
// structue in the PyProj instance.

use std::path::Path;
use std::path::PathBuf;
use std::iter::{Iterator, IntoIterator};

#[derive(Debug)]
struct FSFile {
    name: String,
}

impl FSFile {
    pub fn new(name: &str) -> Self {
        FSFile { name: name.to_string() }
    }
}

#[derive(Debug)]
struct FSDir {
    name: String,
    children: Vec<FSLink>,
}

impl FSDir {

    pub fn new(name: &str) -> Self {
        FSDir {
            name: name.to_string(),
            children: Vec::new(),
        }
    }

    pub fn add_link(&mut self, child: FSLink) {
        self.children.push(child);
    }

    pub fn add_file(&mut self, name: &str) {
        let file = FSFile::new(name);
        self.add_link(FSLink::File(file));
    }

    pub fn add_dir(&mut self, name: &str) {
        let dir = FSDir::new(name);
        self.add_link(FSLink::Dir(dir));
    }

    pub fn with_link(mut self, child: FSLink) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_file(mut self, name: &str) -> Self {
        let file = FSFile::new(name);
        self.add_link(FSLink::File(file));
        self
    }

    pub fn with_dir(mut self, name: &str) -> Self {
        let dir = FSDir::new(name);
        self.add_link(FSLink::Dir(dir));
        self
    }

}

#[derive(Debug)]
struct FSDirIterator {
    children: Vec<FSLink>,
    index: i32,
}

impl Iterator for FSDirIterator {
    type Item = FSLink;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// impl IntoIterator for FSDir {
//     type Item = FSLink;
//     type IntoIter = std::vec

// }

#[derive(Debug)]
enum FSLink {
    File(FSFile),
    Dir(FSDir),
}


// impl Iterator for FSDir {
//     type Item = FSLink;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }


fn print_walk(root: &FSDir) {
    for child in root.children.iter() {
        match child {
            FSLink::File(file) => {
                println!("file: {:?}", file)
            },
            FSLink::Dir(dir) =>{
                println!("dir: {:?}", dir);
                print_walk(dir);
            },
        }
    }
}

fn test_print_walk() {
    let root = FSDir{
        name: "root".to_string(),
        children: vec![
            FSLink::File(FSFile { name: "file_1_1".to_string(), } ),
            FSLink::Dir(FSDir { name: "dir_1_1".to_string(), children: vec![
                FSLink::File(FSFile { name: "file_2_1".to_string(), } ),
            ] } ),
            FSLink::Dir(FSDir { name: "dir_1_2".to_string(), children: vec![
                FSLink::File(FSFile { name: "file_2_2".to_string(), } ),
            ] } ),
        ]
    };

    print_walk(&root);



}

#[derive(Debug)]
pub struct _PyProj {
    name: String,
    path: Option<PathBuf>,
    contents: FSDir,
}

impl _PyProj {

    pub fn new() -> Self {

        let src_tree = FSDir::new("src")
            .with_file("Dockerfile")
            .with_file("requirements.txt")
            .with_file("README.md");

        Self {
            name: "pyproj".to_string(),
            path: None,
            contents: src_tree,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fs() {        
        test_print_walk();
    }

}