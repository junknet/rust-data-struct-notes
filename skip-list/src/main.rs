use std::{cell::RefCell, rc::Rc};

use rand::random;

type ReadNode = Rc<RefCell<Node>>;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    next: Vec<Link>,
    data: String,
    offset: usize,
}

impl Node {
    fn new(next: Vec<Link>, data: String, offset: usize) -> ReadNode {
        Rc::new(RefCell::new(Self { next, data, offset }))
    }
}

struct SkipList {
    head: Link,
    tails: Vec<Link>,
    length: usize,
    max_level: usize,
}

impl SkipList {
    fn new(max_level: usize) -> Self {
        Self {
            head: None,
            tails: vec![None; max_level],
            length: 0,
            max_level,
        }
    }
    fn get_level(&self) -> usize {
        let mut n = 0;
        while random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    fn append(&mut self, offset: usize, data: String) {
        let level = if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };
        let node = Node::new(vec![None; level], data, offset);
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(node.clone());
            }
            self.tails[i] = Some(node.clone());
        }
        if self.head.is_none() {
            self.head = Some(node.clone())
        }
        self.length += 1;
    }

    fn find(&self, offset: usize) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level - 1;
                let mut node = head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                for level in (0..=start_level).rev() {
                    loop {
                        let next = node.clone();
                        match next.borrow().next[level] {
                            Some(ref tmp) => {
                                if tmp.borrow().offset <= offset {
                                    node = tmp.clone();
                                } else {
                                    break;
                                }
                            }
                            None => break,
                        };
                    }
                    if node.borrow().offset == offset {
                        result = Some(node.borrow().data.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }
}

fn main() {
    let mut skip_list = SkipList::new(6);
    skip_list.append(1, "hello".to_owned());
    skip_list.append(2, "world".to_owned());
    skip_list.append(3, "it".to_owned());
    skip_list.append(4, "rust".to_owned());
    println!("offset=3, data={:?}", skip_list.find(3));
}
