use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct DirFile {
    name: String,
    size: u32,
}

impl From<DirFile> for u32 {
    fn from(file: DirFile) -> Self {
        file.size
    }
}

#[derive(Clone)]
struct Directory<'dir> {
    name: String,
    parent: Option<Box<&'dir Directory<'dir>>>,
    files: Vec<DirFile>,
    subdirs: Vec<&'dir Directory<'dir>>,
}

impl From<Directory<'_>> for u32 {
    fn from(dir: Directory) -> Self {
        let mut total_size: u32 = dir.files.iter().fold(0, |a, b| a + b.size);

        if dir.subdirs.len() > 0 {
            total_size += dir.subdirs.iter().fold(0, |a, b| a + u32::from(**b));
        }
        total_size
    }
}

trait SizeCalculation {
    fn calculate_size(&self) -> u32;
}

trait FileManager {
    fn get_dir(&mut self, name: &str) -> &Directory<'_>;
    fn mk_dir(&mut self, name: &str) -> &Directory<'_>;
    fn add_file(&mut self, name: &str, size: u32);
}

impl SizeCalculation for

impl FileManager for Directory<'_> {
    fn get_dir(&mut self, name: &str) -> &Directory<'_> {
        if name == self.name {
            return self;
        } else if name == ".." {
            return *self.parent.unwrap();
        }
        self.subdirs.iter().find(|dir| dir.name == name).unwrap()
    }
    fn mk_dir(&mut self, name: &str) -> &Directory<'_> {
        let dir = &Directory {
            name: name.to_string(),
            parent: Some(Box::new(self)),
            files: Vec::new(),
            subdirs: Vec::new(),
        };
        self.subdirs.push(dir);
        dir
    }
    fn add_file(&mut self, name: &str, size: u32) {
        self.files.push(DirFile {
            name: name.to_string(),
            size: size,
        })
    }
}

fn parse_file(path: &str) -> &Directory {
    let mut current_dir = &Directory {
        name: "/".to_string(),
        parent: None,
        files: Vec::new(),
        subdirs: Vec::new(),
    };
    let parent_dir = &current_dir;

    let file = File::open(path).expect("Can't open file");
    let buf = BufReader::new(file);

    let mut line_iter = buf.lines().map(|l| l.unwrap());

    for line in line_iter {
        if line.starts_with("$") {
            let command: Vec<&str> = line.split_whitespace().collect();
            match command[1] {
                "cd" => current_dir = current_dir.get_dir(command[2]),
                "ls" => continue,
                _ => panic!("Bad command"),
            }
        }
    }
    parent_dir
}

fn main() {
    println!("Hello, world!");
}
