use std::str::FromStr;
use crate::Combatant;

pub struct Attack;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttackResult {
  Hit,
  Miss,
  Crit,
}

impl FromStr for AttackResult {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "hit" => Ok(AttackResult::Hit),
      "miss" => Ok(AttackResult::Miss),
      "crit" => Ok(AttackResult::Crit),
      _ => Err(()),
    }
  }
}

impl Attack {
  pub fn new() -> Self {
    Attack
  }

  pub fn attack(
    &self,
    attacker: &mut dyn Combatant,
    defender: &mut dyn Combatant,
    roll: u8
  ) -> AttackResult {
    if roll == 20 {
      defender.damage(attacker.crit_damage());
      AttackResult::Crit
    } else if roll >= defender.armor_class() {
      defender.damage(attacker.attack_damage());
      AttackResult::Hit
    } else {
      AttackResult::Miss
    }
  }
}
