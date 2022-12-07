#![feature(iter_advance_by)]

use anyhow::{anyhow, Context, Error, Result};
use std::{collections::{HashMap, HashSet}, str::FromStr, ops::ControlFlow};

enum Node {
    Dir { name: String },
    File { name: String, size: usize },
}

struct FileSystem {
    inner: std::collections::HashMap<String, Vec<Node>>,
}

impl FromStr for FileSystem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner: HashMap<String, Vec<Node>> = HashMap::new();
        let mut seen = HashSet::new();

        let mut current_key = Vec::new();
        for line in s.lines() {
            if line.starts_with("$ cd ..") {
                current_key.pop().context("tried to pop key but nothing left")?;
                continue
            } else if line.starts_with("$ ls") {
                continue
            }

            if let Some(key) = line.strip_prefix("$ cd ") {
                current_key.push(key);

                if !seen.insert(current_key.join("/")) {
                    return Err(anyhow!(format!("tried to insert duplicate key {key}")));
                }
            } else if let Some(dir_name) = line.strip_prefix("dir ") {
                inner
                    .entry(current_key.join("/"))
                    .or_default()
                    .push(Node::Dir {
                        name: current_key.join("/") + "/" + dir_name,
                    })
            } else {
                let (size, name) = line
                    .split_once(' ')
                    .context("can't get file size and name")?;

                inner
                    .entry(current_key.join("/"))
                    .or_default()
                    .push(Node::File {
                        name: name.into(),
                        size: size.parse().context(format!("can't parse size {size} {line}"))?,
                    })
            }
        }
        Ok(Self { inner })
    }
}

impl FileSystem {
    fn dir_size(&self) -> HashMap<String, usize> {
        let mut dir_size = HashMap::new();

        loop {
            let before = dir_size.len();
            for (dir, children) in &self.inner {
                if dir_size.contains_key(dir) { continue }

                let ControlFlow::Continue(size) = children.iter().try_fold(0,  |memo, item| {
                    match item {
                        Node::Dir { name } => match dir_size.get(name) {
                            Some(val) => ControlFlow::Continue(val + memo),
                            None => ControlFlow::Break(()),
                        },
                        Node::File { size, .. } => ControlFlow::Continue(size + memo),
                    }
                }) else { continue };
                dir_size.insert(dir.to_string(), size);
            }

            if before == dir_size.len() { break }
        }

        dir_size
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    let fs: FileSystem = input.parse()?;
    let dir_sizes = fs.dir_size();
    let part1: usize = dir_sizes.iter().filter_map(|(_, size)| (size <= &100_000).then_some(size)).sum();
    println!("part1: {part1}");

    let free: usize = 70_000_000 - *dir_sizes.get("/").context("can't get outer size")?;
    let need = 30_000_000usize.saturating_sub(free);
    let mut big_enough: Vec<(&String, &usize)> = dir_sizes.iter().filter(|(_, size)| (size >= &&need)).collect();
    big_enough.sort_by(|(_, val1), (_, val2)| val1.cmp(val2));
    let part2 = big_enough.first().context("couldn't get anything big enough")?.1;
    println!("part2: {part2}");

    Ok(())
}
