use crate::Combatant;

pub struct Attack;

impl Attack {
  pub fn new() -> Self {
    Attack
  }

  pub fn attack(
    &self,
    _attacker: &Box<dyn Combatant>,
    defender: &Box<dyn Combatant>,
    roll: u8
  ) -> bool {
    let ac = defender.armor_class();
    roll >= ac || roll == 20
  }
}
