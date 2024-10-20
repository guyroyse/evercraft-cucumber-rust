use crate::Alignment;
use crate::Combatant;

#[derive(Debug)]
pub struct Hero {
  name: String,
  alignment: Alignment,
  damage_taken: u16,
}

impl Combatant for Hero {
  fn armor_class(&self) -> u8 {
    self.armor_class()
  }
  fn damage(&mut self, points: u8) {
    self.damage(points)
  }
}

impl Hero {
  pub fn new() -> Self {
    Hero {
      name: String::from(""),
      alignment: Alignment::Neutral,
      damage_taken: 0,
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn alignment(&self) -> Alignment {
    self.alignment
  }

  pub fn set_alignment(&mut self, alignment: Alignment) {
    self.alignment = alignment;
  }

  pub fn armor_class(&self) -> u8 {
    10
  }

  pub fn hit_points(&self) -> u16 {
    5
  }

  pub fn current_hit_points(&self) -> u16 {
    if self.damage_taken > self.hit_points() { 0 } else { self.hit_points() - self.damage_taken }
  }

  pub fn damage(&mut self, points: u8) {
    self.damage_taken += points as u16;
  }

  pub fn alive(&self) -> bool {
    self.current_hit_points() > 0
  }
}
