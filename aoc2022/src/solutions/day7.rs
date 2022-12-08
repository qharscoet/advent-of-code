use std::{rc::Rc, cell::{RefCell}};

use crate::solution::Solution;

pub struct Day7;

#[derive(Debug)]
pub enum Element {
    File {    name: String,    size : usize},
    Directory {    name : String,    contents : Vec<Rc<RefCell<Element>>>}
}

impl Element {
    fn add_content(&mut self, line : String) {
        if let Element::Directory { name:_, contents } = self {
            if let Some((l,r)) = line.split_once(' ') {
                match l {
                    "dir" => {contents.push(Rc::new(RefCell::new(Element::Directory{ name : r.to_string(), contents: vec![]})));}
                    _ => {contents.push(Rc::new(RefCell::new(Element::File { name: r.to_string(), size : l.parse::<usize>().unwrap_or_default()})));}
                };
            }
        }
        
    }
    
    fn get_size(&self) -> usize {
        match self {
            Element::Directory { name:_, contents } => contents.iter().map(|elem| elem.as_ref().borrow().get_size()).sum(),
            Element::File { name:_, size } => *size,
        }
    }    
}


impl Solution for Day7 {
    type Input = Rc<RefCell<Element>>;
    type ReturnType = usize;

    fn parse_input(&self, lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let mut root = Rc::new(RefCell::new(Element::Directory{name :"/".to_string(), contents : vec![]}));
        let mut current_dir = Rc::clone(&root);
        let mut prev_dir = vec![Rc::clone(&root)];
        
        
        let mut iter = lines.skip(1).peekable();
    
        loop {
            if let Some(l) = iter.next()  {
                match &l[2..=3]{
                    "ls" => {
                        while let Some(val) = iter.next_if(|w| !w.starts_with("$")) {
                            current_dir.borrow_mut().add_content(val);
                        }
                        //println!("{:?}", current_dir);
                        
                    },
                    "cd" => {
                        match &l[5..] {
                            "/" => { current_dir = Rc::clone(&mut root);}
                            ".." => { current_dir = Rc::clone(&prev_dir.pop().unwrap());},
                            dirname => { 
                                prev_dir.push(Rc::clone(&current_dir));
                                let mut found_dir = Rc::clone(&current_dir);
                                if let Element::Directory { name:_, contents } = &*current_dir.borrow_mut() {
                                    found_dir = Rc::clone(contents.iter().find(|&elem| match &*elem.borrow_mut() {
                                        Element::Directory { name : n , contents : _ } => { n == dirname},
                                        _ => false
                                        }).unwrap());
                                    //prev_dir_name = name;
                                }
                                
                                current_dir = found_dir;
                            }
                        }
                    },
                    _ => {}
                }
            }
            else {
                break;
            }
            
        }
        
        
        //println!("{:#?}", root);

       root
    }

    fn first_part(&self, input: &Self::Input) -> Self::ReturnType {
        let root = input.as_ref().borrow();
        
        fn inner(elem: &Element, n:usize) -> (usize,usize) {
            match elem {
                Element::Directory { name:_, contents } => {
                    let val = contents.iter().map(|elem| inner(&elem.as_ref().borrow(),n))
                                                                    .reduce(|acc, e| (acc.0 + e.0, acc.1 +e.1)).unwrap_or_default();
                    (if val.1 < n {val.0 + val.1} else {val.0}, val.1)
                },
                Element::File { name:_, size } => (0, *size),
            }
        }
        
        inner(&root, 100000).0
	}
    
    
    fn second_part(&self, input: &Self::Input) -> Self::ReturnType {
       let root = input.as_ref().borrow();
       let unused_space = 70000000 - root.get_size();
       let needed_space = 30000000 - unused_space;
        
        fn inner(elem: &Element, n:usize) -> (usize,usize) {
            match elem {
                Element::Directory { name:_, contents } => {
                    let val = contents.iter().map(|elem| inner(&elem.as_ref().borrow(),n))
                                                                    .reduce(|acc, e| (acc.0.min(e.0), acc.1 +e.1)).unwrap_or_default();
                    (if val.1 > n {val.0.min(val.1)} else {val.0}, val.1)
                },
                Element::File { name:_, size } => (usize::MAX, *size),
            }
        }
        
        inner(&root, needed_space).0
	}
}

#[cfg(test)]
mod tests {
    use super::Day7;
    use crate::solution::Solution;

		static INPUT_TEST : &str =
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_parse() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
        assert_eq!(input.borrow().get_size(), 48381165);
    }

    #[test]
    fn test_first_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
		assert_eq!(Day7.first_part(&input),
        95437)
    }

    #[test]
    fn test_second_part() {
        let lines = INPUT_TEST.split('\n').map(|s| s.to_string());
        let input = Day7.parse_input(lines);
		assert_eq!(Day7.second_part(&input),
            24933642)
    }
}