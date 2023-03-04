// item.rs

use super::mob::Mob;
use super::vector::Vector;

#[derive(Default, Clone)]
pub struct Item {}

impl Item {
    pub fn make_pickup(&self, _pos: Vector<2, f64>) -> Mob {
        todo!()
    }
}
