use std::fmt::Debug;

pub trait Combatant: Debug {
  fn armor_class(&self) -> u8;
  fn damage(&mut self, points: u8);
}
