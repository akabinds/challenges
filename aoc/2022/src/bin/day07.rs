use aoc_helper::AOCSolution;
use std::{collections::HashSet, error::Error};

struct Day7;

#[derive(Debug, Clone)]
struct Filesystem<'a> {
    directories: Vec<Dir<'a>>,
}

impl<'a> Filesystem<'a> {
    fn new() -> Self {
        Self {
            directories: vec![Dir::new(0, "/", None)],
        }
    }

    fn dirs(&self) -> impl Iterator<Item = &Dir<'a>> {
        self.directories.iter()
    }

    fn new_dir(&mut self, name: &'a str, parent: Option<usize>) -> &mut Dir<'a> {
        let dir = Dir::new(self.directories.len(), name, parent);
        self.directories.push(dir);
        self.directories.last_mut().unwrap()
    }

    fn get_dir(&self, id: DirID) -> &Dir<'a> {
        &self.directories[id.0]
    }

    fn get_dir_mut(&mut self, id: DirID) -> &mut Dir<'a> {
        &mut self.directories[id.0]
    }

    fn root(&self) -> DirID {
        self.directories.first().unwrap().id
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum DirEntry {
    File { size: usize },
    Dir(DirID),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct DirID(usize);

impl From<usize> for DirID {
    fn from(v: usize) -> Self {
        DirID(v)
    }
}

impl From<DirID> for usize {
    fn from(v: DirID) -> Self {
        v.0
    }
}

#[derive(Debug, Clone)]
struct Dir<'a> {
    id: DirID,
    name: &'a str,
    size: usize,
    parent: Option<DirID>,
    entries: HashSet<DirEntry>,
}

impl<'a> Dir<'a> {
    fn new<ID: Into<DirID>>(id: ID, name: &'a str, parent: Option<ID>) -> Self {
        Self {
            id: id.into(),
            name,
            parent: parent.map(Into::into),
            size: 0,
            entries: HashSet::new(),
        }
    }

    fn add_entry(&mut self, entry: DirEntry) {
        self.entries.insert(entry);
    }

    fn size(&self, fs: &Filesystem) -> usize {
        use DirEntry::*;

        self.entries.iter().fold(0, |acc, e| {
            acc + match e {
                File { size } => *size,
                Dir(id) => fs.get_dir(*id).size,
            }
        })
    }

    fn subdirs(&self) -> impl Iterator<Item = &DirEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(e, DirEntry::Dir(_)))
    }
}

impl<'a> AOCSolution<'a> for Day7 {
    type ParsedInput = Filesystem<'a>;
    type Part1Out = usize;
    type Part2Out = usize;

    fn parse(input: &'a str) -> Self::ParsedInput {
        let input = input.lines().skip(1).map(|s| {
            if let Some(cmd) = s.strip_prefix("$ ") {
                cmd
            } else {
                s
            }
        });
        let mut fs = Filesystem::new();
        let mut cur_dir_id = fs.root();

        for line in input {
            if line.starts_with("cd") {
                let Some((_, dir_name)) = line.split_once(' ') else {
                    panic!("Ill formatted input");
                };

                let cur_dir = fs.get_dir(cur_dir_id);

                match dir_name {
                    ".." => {
                        cur_dir_id = if let Some(ref parent) = cur_dir.parent {
                            *parent
                        } else {
                            panic!("Current directory has no parent directory.");
                        }
                    },
                    _ => {
                        let already_exists = cur_dir.subdirs().find(|subdir| {
                            let DirEntry::Dir(subdir_id) = subdir else {
                                panic!("Expected entry to be a directory");
                            };

                            fs.get_dir(*subdir_id).name == dir_name
                        });

                        cur_dir_id = if let Some(dir) = already_exists {
                            let DirEntry::Dir(dir) = dir else {
                                panic!("Expected entry to be a directory");
                            };

                            *dir
                        } else {
                            let new = fs.new_dir(dir_name, Some(cur_dir_id.into()));
                            new.id
                        };
                    }
                }
            } else if line == "ls" {
                continue
            } else if let Some((dir_or_size, name)) = line.split_once(' ') {
                match dir_or_size {
                    size if dir_or_size.chars().all(|c| c.is_numeric()) => {
                        let cur_dir = fs.get_dir_mut(cur_dir_id);
                        let size = size.parse().unwrap();
                        cur_dir.add_entry(DirEntry::File { size });
                        cur_dir.size += size;
                        fs.get_dir_mut(fs.root()).size += size;
                    },
                    _ => {},
                }
            }
        }

        fs
    }

    fn part1(parsed: Self::ParsedInput) -> Self::Part1Out {
        dbg!(parsed.dirs().collect::<Vec<_>>());

        parsed
            .dirs()
            .map(|d| d.size(&parsed))
            .filter(|size| *size <= 100000)
            .sum()
    }

    fn part2(parsed: Self::ParsedInput) -> Self::Part2Out {
        todo!()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let inp = include_str!("../../inputs/day07.txt");
    let parsed = Day7::parse(inp);

    dbg!(Day7::part1(parsed.clone()));

    // let (p1, p2) = (Day7::part1(parsed.clone()), Day7::part2(parsed));

    // println!("Part 1: {p1}");
    // println!("Part 2: {p2}");

    Ok(())
}
