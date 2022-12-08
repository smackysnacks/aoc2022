use std::collections::HashMap;

type DirId = usize;

#[derive(Debug)]
struct Dir {
    id: DirId,
    parent: Option<DirId>,

    name: String,
    files: Vec<File>,
    dirs: HashMap<String, DirId>,
    size: Option<usize>,
}

impl Dir {
    fn new(id: DirId, parent: DirId, name: &str) -> Self {
        Self {
            id,
            parent: Some(parent),
            name: name.into(),
            files: Vec::new(),
            dirs: HashMap::new(),
            size: None,
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Tree {
    next_id: DirId,
    cwd: DirId,
    arena: Vec<Dir>,
}

impl Tree {
    fn new() -> Self {
        let all_dirs = vec![Dir {
            id: 0,
            parent: None,
            name: "/".into(),
            files: Vec::new(),
            dirs: HashMap::new(),
            size: None,
        }];

        Tree {
            next_id: 1,
            cwd: 0,
            arena: all_dirs,
        }
    }

    fn cd(&mut self, arg: &str) {
        if arg == ".." {
            self.cwd = self.arena[self.cwd].parent.unwrap();
        } else {
            if !self.arena[self.cwd].dirs.contains_key(arg) {
                let new_dir = Dir::new(self.next_id, self.arena[self.cwd].id, arg);
                self.arena.push(new_dir);
                self.arena[self.cwd].dirs.insert(arg.into(), self.next_id);
                self.next_id += 1;
            }
            self.cwd = *self.arena[self.cwd].dirs.get(arg).unwrap();
        }
    }

    fn ls(&mut self, entries: &[&str]) {
        let files: Vec<File> = entries
            .iter()
            .filter_map(|line| {
                // skip directories
                if !line.starts_with("dir ") {
                    let file: Vec<&str> = line.split(' ').collect();
                    return Some(File {
                        name: file[1].into(),
                        size: file[0].parse().unwrap(),
                    });
                }
                None
            })
            .collect();

        self.arena[self.cwd].files = files;
    }

    fn compute_sizes(&mut self, starting: DirId) -> usize {
        let mut total = 0;
        let dirs = self.arena[starting]
            .dirs
            .values()
            .copied()
            .collect::<Vec<DirId>>();
        for subdir in dirs {
            match self.arena[subdir].size {
                Some(size) => total += size,
                None => total += self.compute_sizes(subdir),
            }
        }
        for file in &self.arena[starting].files {
            total += file.size;
        }
        self.arena[starting].size = Some(total);
        total
    }
}

fn build_tree(input: &str) -> Tree {
    let mut tree = Tree::new();

    let commands: Vec<Vec<&str>> = input
        .split("$ ")
        .map(str::trim_end)
        .map(|command| command.split('\n').collect())
        .skip(1)
        .collect();
    for command in commands {
        if command[0].starts_with("cd") {
            tree.cd(&command[0][3..]);
        } else if command[0] == "ls" {
            tree.ls(&command[1..]);
        } else {
            println!("usupported command: {}", command[0]);
        }
    }

    tree
}

fn part1(tree: &mut Tree) -> usize {
    tree.compute_sizes(0);
    tree.arena
        .iter()
        .map(|d| d.size.unwrap())
        .filter(|&size| size < 100000)
        .sum()
}

fn part2(tree: &mut Tree) -> usize {
    const TOTAL_DISK_SPACE: usize = 70000000;
    const UNUSED_NEEDED: usize = 30000000;

    let used = tree.arena[0].size.unwrap();
    let left = TOTAL_DISK_SPACE - used;
    let needed = UNUSED_NEEDED - left;

    let mut candidates: Vec<usize> = tree
        .arena
        .iter()
        .map(|d| d.size.unwrap())
        .filter(|&size| size >= needed)
        .collect();
    candidates.sort();
    candidates[0]
}

fn main() {
    let input = include_str!("../input.txt");
    let mut tree = build_tree(input);
    dbg!(part1(&mut tree));
    dbg!(part2(&mut tree));
}
