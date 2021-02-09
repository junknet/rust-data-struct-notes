use std::{mem, ops::Deref};

use mem::replace;

use crate::IoTDevice;

type Tree = Option<Box<Node>>;

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}

impl Node {
    fn new(dev: IoTDevice) -> Tree {
        Some(Box::new(Self {
            dev,
            left: None,
            right: None,
        }))
    }
}

pub struct DeviceRegistry {
    root: Tree,
    pub length: u64,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }
    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }
    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if device.number_id >= n.dev.number_id {
                    n.left = self.add_rec(n.left, device);
                } else {
                    n.right = self.add_rec(n.right, device)
                }
                Some(n)
            }
            None => Node::new(device),
        }
    }
    pub fn find(&self, number_id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, number_id)
    }
    fn find_r(&self, node: &Tree, number_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.number_id == number_id {
                    Some(n.dev.clone())
                } else if number_id >= n.dev.number_id {
                    self.find_r(&n.left, number_id)
                } else {
                    self.find_r(&n.right, number_id)
                }
            }
            None => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}
