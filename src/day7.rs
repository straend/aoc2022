use crate::helpers;
use std::{collections::HashMap, io};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day7_test.txt");

        assert_eq!(run_part1(&lines), 95437_usize);
        let lines = helpers::read_file_to_vec::<String>("inputs/day7.txt");
        assert_eq!(run_part1(&lines), 1086293_usize); // less than
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day7_test.txt");

        assert_eq!(run_part2(&lines), 24933642_usize);
    }
}
#[derive(Debug, PartialEq, Eq)]
enum FSType {
    FSRoot,
    File,
    Directory,
}
#[derive(Debug)]
struct FSEntry<'a> {
    name: String,
    size: usize,
    fstype: FSType,
    childs: HashMap<&'a str, FSEntry<'a>>,
}

fn parse_input(input: &Vec<String>) -> FSEntry {
    let vv: HashMap<&str, FSEntry> = HashMap::new();
    let mut root = FSEntry {
        name: String::from("/"),
        childs: vv,
        fstype: FSType::FSRoot,
        size: 0,
    };

    let mut path: Vec<String> = Vec::new();

    for line in input {
        let splits: Vec<&str> = line.split_whitespace().collect();
        match splits[0] {
            "$" => {
                match splits[1] {
                    "cd" => {
                        // Change current dir to splits[2]
                        match splits[2] {
                            "/" => {
                                // is Root dir, clear path
                                path.clear();
                                path.push(String::from("/"));
                            }

                            // Remove last dir
                            ".." => {
                                path.pop().unwrap();
                                ()
                            }

                            // Add dir
                            x => {
                                path.push(String::from(x));
                                ()
                            }
                        }
                    }
                    "ls" => {
                        // Should set flag for adding files to tree
                    }
                    _ => (),
                }
            }
            "dir" => {
                let mut x = &mut root;
                for p in path.iter().skip(1) {
                    let yy = x.childs.get_mut(p.as_str()).unwrap();

                    x = yy;
                }

                let vv: HashMap<&str, FSEntry> = HashMap::new();
                let v = FSEntry {
                    name: String::from(splits[1]),
                    childs: vv,
                    fstype: FSType::Directory,
                    size: 0,
                };

                x.childs.insert(splits[1], v);
                ()
            }
            _ => {
                match usize::from_str_radix(splits[0], 10) {
                    Ok(size) => {
                        let mut x = &mut root;
                        for p in path.iter().skip(1) {
                            let yy = x.childs.get_mut(p.as_str()).unwrap();
                            // Also add size of current folder to parent folders
                            if yy.fstype == FSType::Directory {
                                x.size += size;
                            }
                            x = yy;
                        }
                        let vv: HashMap<&str, FSEntry> = HashMap::new();
                        let v = FSEntry {
                            name: String::from(splits[1]),
                            childs: vv,
                            fstype: FSType::File,
                            size,
                        };
                        x.size += size;
                        x.childs.insert(splits[1], v);
                    }
                    _ => {}
                };
            }
        }
    }

    root
}

#[allow(dead_code)]
fn print_root(root: &FSEntry) {
    println!("{} ({})", root.size, root.name);
    print_child(1, &root.childs);
}

#[allow(dead_code)]
fn print_child(depth: usize, entries: &HashMap<&str, FSEntry>) {
    for (k, v) in entries {
        for _ in 0..depth {
            print!("    ");
        }
        if v.fstype == FSType::Directory {
            println!("{:10}\t({})", v.size, k);
        } else {
            println!("{:10}\t {} ", v.size, k);
        }
        if v.childs.len() > 0 {
            print_child(depth + 1, &v.childs)
        }
    }
}

fn get_size_under(size: usize, entries: &HashMap<&str, FSEntry>) -> usize {
    let mut ret = 0;
    for (_, v) in entries {
        if v.fstype == FSType::Directory && v.size <= size {
            ret += v.size;
        }
        for (_, vv) in &v.childs {
            ret += get_size_under(size, &vv.childs);
            if vv.fstype == FSType::Directory && vv.size < size {
                ret += vv.size;
            }
        }
    }
    ret
}

// Excluding root
fn get_folders_over(size: usize, entries: &HashMap<&str, FSEntry>) -> Vec<usize> {
    let mut ret = Vec::new();

    for (_, v) in entries {
        if v.fstype == FSType::Directory && v.size >= size {
            ret.push(v.size);
        }
        for (_, vv) in &v.childs {
            ret.append(&mut get_folders_over(size, &vv.childs));
            if vv.fstype == FSType::Directory && vv.size >= size {
                ret.push(vv.size);
            }
        }
    }
    ret
}

pub fn run_part1(input: &Vec<String>) -> usize {
    let root = parse_input(input);
    //print_root(&root);

    let tot = get_size_under(100000, &root.childs);

    tot
}

pub fn run_part2(input: &Vec<String>) -> usize {
    let fs_size: usize = 70000000;
    let needed_for_update: usize = 30000000;

    let root = parse_input(input);

    let need_to_free = needed_for_update - (fs_size - root.size);

    let all_over = get_folders_over(need_to_free, &root.childs);
    let smallest = all_over.iter().min().unwrap();

    *smallest
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 7");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day7.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1 {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
