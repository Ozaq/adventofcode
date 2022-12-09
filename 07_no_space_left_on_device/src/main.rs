use core::borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    let filename = env::args().nth(1).expect("Missing filename argument");
    let file = File::open(filename).expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>();
    let tree = parse_input(&lines);
    pt1(&tree);
    pt2(&tree);
}

#[derive(Debug, Clone)]
enum Input {
    EnterDirectoryCmd(String),
    LeaveDirectoryCmd,
    ListDirectoryCmd,
    FileEntry(String, u32),
    DirectoryEntry(String),
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Input, ()> {
        if s == "$ ls" {
            return Ok(Input::ListDirectoryCmd);
        } else if s == "$ cd .." {
            return Ok(Input::LeaveDirectoryCmd);
        } else if s.starts_with("$ cd") {
            let (_, dir_name) = s.split_at(5);
            return Ok(Input::EnterDirectoryCmd(dir_name.to_string()));
        } else if s.starts_with("dir") {
            let (_, dir_name) = s.split_at(4);
            return Ok(Input::DirectoryEntry(dir_name.to_string()));
        } else {
            let (size, name) = s.split_once(' ').unwrap();
            let size = size.parse::<u32>().unwrap();
            return Ok(Input::FileEntry(name.to_string(), size));
        }
    }
}

#[derive(Debug)]
enum TreeNode {
    File(u32, String),
    Directory(String, Vec<Node>, Option<u32>),
}

type Node = Rc<RefCell<TreeNode>>;

fn compute_sizes(node: Node) -> u32 {
    match *node.borrow_mut() {
        TreeNode::File(size, _) => size,
        TreeNode::Directory(_, ref children, ref mut size) => {
            let dir_size = children
                .iter()
                .fold(0, |sum, child| sum + compute_sizes(child.clone()));
            *size.insert(dir_size)
        }
    }
}

fn parse_input(input: &Vec<String>) -> Node {
    let tree = Rc::new(RefCell::new(TreeNode::Directory(
        "/".to_string(),
        vec![],
        None,
    )));
    let mut path = VecDeque::<Node>::new();
    path.push_front(tree.clone());
    for line in input.iter().skip(1) {
        match line.parse().unwrap() {
            Input::EnterDirectoryCmd(ref dir_name) => {
                let cur = path.front().unwrap().clone();
                if let TreeNode::Directory(_, ref children, _) = *cur.borrow() {
                    let new_dir = children
                        .iter()
                        .find(|x| {
                            if let TreeNode::Directory(ref name, _, _) = &*x.borrow() {
                                name == dir_name
                            } else {
                                false
                            }
                        })
                        .unwrap();
                    path.push_front(new_dir.clone());
                } else {
                    panic!();
                };
            }
            Input::LeaveDirectoryCmd => {
                path.pop_front();
            }
            Input::ListDirectoryCmd => {}
            Input::FileEntry(name, size) => {
                let new_file = Rc::new(RefCell::new(TreeNode::File(size, name)));
                if let TreeNode::Directory(_, ref mut children, _) =
                    &mut *path.front().unwrap().borrow_mut()
                {
                    children.push(new_file);
                } else {
                    panic!();
                }
            }
            Input::DirectoryEntry(dir_name) => {
                let new_dir = Rc::new(RefCell::new(TreeNode::Directory(dir_name, vec![], None)));
                if let TreeNode::Directory(_, ref mut children, _) =
                    &mut *path.front().unwrap().borrow_mut()
                {
                    children.push(new_dir.clone());
                } else {
                    panic!();
                }
            }
        }
    }
    compute_sizes(tree.clone());
    tree
}

fn pt1(tree: &Node) {
    let mut to_visit = VecDeque::<Node>::new();
    to_visit.push_back(tree.clone());
    let mut acc = 0;
    while let Some(node) = to_visit.pop_front() {
        if let TreeNode::Directory(_, ref children, size) = *node.borrow() {
            if let Some(size) = size {
                if size <= 100000 {
                    acc += size;
                }
            }
            for child in children {
                to_visit.push_back(child.clone());
            }
        }
    }
    println!("First solution {acc}");
}

fn pt2(tree: &Node) {
    let mut to_visit = VecDeque::<Node>::new();
    to_visit.push_back(tree.clone());

    let missing_space = match *tree.borrow() {
        TreeNode::Directory(_, _, Some(size)) => size,
        _ => panic!(),
    } - 40000000;
    let mut acc = u32::MAX;
    while let Some(node) = to_visit.pop_front() {
        if let TreeNode::Directory(_, ref children, size) = *node.borrow() {
            if let Some(size) = size {
                if size > missing_space && size < acc {
                    acc = size;
                }
            }
            for child in children {
                to_visit.push_back(child.clone());
            }
        }
    }
    println!("Second solution {acc}");
}
