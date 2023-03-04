// mob_drop.rs

use super::item::Item;

#[derive(Default, Clone)]
pub struct MobDrop {
    // ... includes Item
}

impl MobDrop {
    pub fn roll(&self) -> Option<Item> {
        todo!()
    }
}
