use std::collections::HashMap;

// Generally directories are a specific type of file, but for simplicity
// of implementation, just keep them seperate.

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct Directory {
    directories: HashMap<String, Directory>,
    files: HashMap<String, File>,
}

impl File {
    fn new(size: usize) -> File {
        File { size }
    }
}

impl Directory {
    fn new() -> Directory {
        Directory {
            directories: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn size(&self) -> usize {
        self.directories
            .values()
            .map(Directory::size)
            .sum::<usize>()
            + self.files.values().map(|f| f.size).sum::<usize>()
    }

    fn zipper(self) -> DirectoryZipper {
        DirectoryZipper {
            name: String::from("/"),
            directory: self,
            parent: None,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Directory> {
        let mut pending = vec![self];
        std::iter::from_fn(move || {
            pending.pop().map(|directory| {
                pending.extend(directory.directories.values());
                directory
            })
        })
    }
}

#[derive(Debug)]
struct DirectoryZipper {
    name: String,
    directory: Directory,
    parent: Option<Box<DirectoryZipper>>,
}

impl DirectoryZipper {
    fn child(mut self, name: String) -> DirectoryZipper {
        let directory = self
            .directory
            .directories
            .remove(&name)
            .unwrap_or_else(|| Directory::new());
        DirectoryZipper {
            name,
            directory,
            parent: Some(Box::new(self)),
        }
    }

    fn parent(self) -> DirectoryZipper {
        let DirectoryZipper {
            name,
            mut directory,
            parent,
        } = *self.parent.unwrap();
        directory.directories.insert(self.name, self.directory);
        DirectoryZipper {
            name,
            directory,
            parent,
        }
    }

    fn top(mut self) -> DirectoryZipper {
        while let Some(_) = self.parent {
            self = self.parent();
        }
        self
    }

    fn finish(self) -> Directory {
        self.top().directory
    }
}

fn main() {
    let mut zipper = Directory::new().zipper();
    for line in std::io::stdin().lines().flatten().skip(1) {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            zipper = match dir {
                "/" => zipper.top(),
                ".." => zipper.parent(),
                _ => zipper.child(dir.into()),
            }
        } else if line != "$ ls" && !line.starts_with("dir ") {
            let mut split = line.split(" ");
            let size = str::parse::<usize>(split.next().unwrap()).unwrap();
            let name = split.next().unwrap();
            zipper.directory.files.insert(name.into(), File::new(size));
        }
    }

    let root = zipper.finish();

    let part1: usize = root
        .iter()
        .map(Directory::size)
        .filter(|&x| x <= 100_000)
        .sum();
    println!("Part 1: {}", part1);

    let required = root.size() - (70_000_000 - 30_000_000);
    let part2: usize = root
        .iter()
        .map(Directory::size)
        .filter(|&x| x >= required)
        .min()
        .unwrap();
    println!("Part 2: {}", part2);
}
