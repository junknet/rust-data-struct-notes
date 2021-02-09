mod binary_search_tree;
#[derive(Clone)]
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

fn main() {
    println!("Hello, world!");
}
