use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
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
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    files: Vec<DirFile>,
    subdirs: Vec<Rc<RefCell<Directory>>>,
}

impl From<Directory> for u32 {
    fn from(dir: Directory) -> Self {
        let mut total_size: u32 = dir.files.iter().fold(0, |a, b| a + b.size);

        if dir.subdirs.len() > 0 {
            total_size += dir
                .subdirs
                .iter()
                .fold(0, |a, b| a + u32::from(b.into_inner()));
        }
        total_size
    }
}

trait SizeCalculation {
    fn calculate_size(&self) -> u32;
}

trait FileManager {
    fn get_dir(&self, name: &str) -> &Rc<RefCell<Directory>>;
    fn mk_dir(&mut self, name: &str);
    fn add_file(&mut self, name: &str, size: u32);
}

impl FileManager for Directory {
    fn get_dir(&self, name: &str) -> &Rc<RefCell<Directory>> {
        if name == ".." {
            return &Rc::clone(&self.parent.unwrap());
        }
        self.subdirs
            .iter()
            .find(|dir| dir.as_ref().borrow().name == name)
            .unwrap()
    }
    fn mk_dir(&mut self, name: &str) {
        let dir = Directory {
            name: name.to_string(),
            parent: Some(Rc::new(RefCell::new(*self))),
            files: Vec::new(),
            subdirs: Vec::new(),
        };
        self.subdirs.push(Rc::new(RefCell::new(dir)));
    }
    fn add_file(&mut self, name: &str, size: u32) {
        self.files.push(DirFile {
            name: name.to_string(),
            size: size,
        })
    }
}

fn parse_file(path: &str) -> Rc<RefCell<Directory>> {
    let mut current_dir = Rc::new(RefCell::new(Directory {
        name: "/".to_string(),
        parent: None,
        files: Vec::new(),
        subdirs: Vec::new(),
    }));
    let parent_dir = Rc::clone(&current_dir);

    let file = File::open(path).expect("Can't open file");
    let buf = BufReader::new(file);

    let line_iter = buf.lines().map(|l| l.unwrap());

    for line in line_iter {
        if line.starts_with("$") {
            let command: Vec<&str> = line.split_whitespace().collect();
            match command[1] {
                "cd" => {
                    let cloned_current = Rc::clone(&current_dir);
                    current_dir = Rc::clone(cloned_current.as_ref().borrow().get_dir(command[1]));
                }
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
