use std::{cell::RefCell, vec};

use rand::seq::SliceRandom;
use rand::thread_rng;

mod binary_search_tree;
#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub number_id: u64,
    pub path: String,
    pub address: String,
}

impl IoTDevice {
    pub fn new(number_id: u64, path: impl Into<String>, address: impl Into<String>) -> Self {
        Self {
            number_id,
            path: path.into(),
            address: address.into(),
        }
    }
}
impl PartialEq for IoTDevice {
    fn eq(&self, other: &Self) -> bool {
        self.number_id == other.number_id && self.address == other.address
    }
}

fn new_device_with_id(id: u64) -> IoTDevice {
    new_device_with_id_path(id, "")
}

fn new_device_with_id_path(id: u64, path: impl Into<String>) -> IoTDevice {
    IoTDevice::new(id, format!("My address is {}", id), path)
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn binary_search_tree_in_order() {
    let len = 10;
    let mut tree = binary_search_tree::DeviceRegistry::new();
    let mut items: Vec<IoTDevice> = (0..len).map(new_device_with_id).collect();
    items.shuffle(&mut thread_rng());
    for item in items.iter() {
        tree.add(item.clone());
    }
    let v: RefCell<Vec<IoTDevice>> = RefCell::new(vec![]);
    tree.walk(|f| v.borrow_mut().push(f.clone()));
    // let mut items = items;
    items.sort_by(|a, b| b.number_id.cmp(&a.number_id));
    for item in items.iter() {
        println!("{:?}", item)
    }
    for item in v.borrow().iter() {
        println!("{:?}", item)
    }
}
