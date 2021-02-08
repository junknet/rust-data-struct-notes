use std::{cell::RefCell, rc::Rc, usize, vec};

use rand::random;
type RealNode = Rc<RefCell<Node>>;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    next: Vec<Link>,
    data: String,
    offset: usize,
}

impl Node {
    fn new(next: Vec<Link>, data: String, offset: usize) -> RealNode {
        Rc::new(RefCell::new(Self { next, data, offset }))
    }
}

#[derive(Clone, Debug)]
struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    length: u64,
}

impl SkipList {
    fn new(max_level: usize) -> Self {
        Self {
            head: None,
            tails: vec![None; max_level],
            max_level: max_level - 1,
            length: 0,
        }
    }
    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1
        }
        n
    }
    fn append(&mut self, offset: usize, data: String) {
        let level = 1 + if self.head.is_none() {
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

        self.length += 1
    }
    fn level_path(&self) {
        match self.head {
            Some(ref head) => {
                let node = head.clone();

                for level in (0..=self.max_level).rev() {
                    let mut n = node.clone();
                    print!("level={:?} ", level);
                    loop {
                        let next = n.clone();
                        print!(
                            "offset={:?}, data={:?}\t",
                            next.borrow().offset,
                            next.borrow().data
                        );
                        match next.borrow().next[level] {
                            Some(ref next) => {
                                n = next.clone();
                            }
                            _ => break,
                        };
                    }
                    println!("");
                }
            }
            None => {}
        }
    }
    fn find(&self, offset: usize) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level - 1;
                let node = head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref tmp) => {
                                if tmp.borrow().offset <= offset {
                                    n = tmp.clone();
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.data.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }
}

fn main() {}

#[test]
fn skip_list_find() {
    let mut list = SkipList::new(3);
    list.append(1, "INSERT INTO mytable VALUES (1)".to_owned());
    list.append(2, "INSERT INTO mytable VALUES (2)".to_owned());
    list.append(3, "INSERT INTO mytable VALUES (3)".to_owned());
    list.append(4, "INSERT INTO mytable VALUES (4)".to_owned());
    list.append(5, "INSERT INTO mytable VALUES (5)".to_owned());
    list.append(6, "INSERT INTO mytable VALUES (6)".to_owned());
    list.append(7, "INSERT INTO mytable VALUES (7)".to_owned());
    list.level_path();
    println!(":qwe")
    // assert_eq!(list.length, 7);
    // assert_eq!(
    //     list.find(7),
    //     Some("INSERT INTO mytable VALUES (7)".to_owned())
    // );
    // assert_eq!(
    //     list.find(6),
    //     Some("INSERT INTO mytable VALUES (6)".to_owned())
    // );
    // assert_eq!(
    //     list.find(5),
    //     Some("INSERT INTO mytable VALUES (5)".to_owned())
    // );
    // assert_eq!(
    //     list.find(4),
    //     Some("INSERT INTO mytable VALUES (4)".to_owned())
    // );
    // assert_eq!(
    //     list.find(3),
    //     Some("INSERT INTO mytable VALUES (3)".to_owned())
    // );
    // assert_eq!(
    //     list.find(2),
    //     Some("INSERT INTO mytable VALUES (2)".to_owned())
    // );
    // assert_eq!(
    //     list.find(1),
    //     Some("INSERT INTO mytable VALUES (1)".to_owned())
    // );
}
