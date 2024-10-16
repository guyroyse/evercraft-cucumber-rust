use std::fmt::Debug;

pub trait Combatant: Debug {
  fn armor_class(&self) -> u8;
}
