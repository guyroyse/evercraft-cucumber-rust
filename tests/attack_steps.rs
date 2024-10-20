use cucumber::{ given, when, then, World };
use evercraft::*;

#[derive(Debug)]
struct MockCombatant {
  armor_class: u8,
  attack_damage: u8,
  crit_damage: u8,
  last_damage_received: u8,
}

impl MockCombatant {
  fn new() -> Self {
    MockCombatant {
      armor_class: 10,
      last_damage_received: 0,
      attack_damage: 1,
      crit_damage: 2,
    }
  }

  fn set_armor_class(&mut self, ac: u8) {
    self.armor_class = ac;
  }

  fn set_attack_damage(&mut self, damage: u8) {
    self.attack_damage = damage;
  }

  fn set_crit_damage(&mut self, damage: u8) {
    self.crit_damage = damage;
  }

  fn last_damage_received(&self) -> u8 {
    self.last_damage_received
  }
}

impl Combatant for MockCombatant {
  fn armor_class(&self) -> u8 {
    self.armor_class
  }

  fn damage(&mut self, points: u8) {
    self.last_damage_received = points;
  }

  fn attack_damage(&self) -> u8 {
    self.attack_damage
  }

  fn crit_damage(&self) -> u8 {
    self.crit_damage
  }
}

#[derive(World, Debug, Default)]
pub struct AttackWorld {
  attacker: Option<MockCombatant>,
  defender: Option<MockCombatant>,
  attack_result: Option<AttackResult>,
}

#[given(regex = r"^an attacker with an attack damage of (\d+) and a crit damage of (\d+)$")]
pub fn new_attacker(world: &mut AttackWorld, attack_damage: u8, crit_damage: u8) {
  let mut attacker = MockCombatant::new();
  attacker.set_attack_damage(attack_damage);
  attacker.set_crit_damage(crit_damage);
  world.attacker = Some(attacker);
}

#[given(regex = r"^a defender with an armor class of (\d+)$")]
pub fn new_defender(world: &mut AttackWorld, ac: u8) {
  let mut defender = MockCombatant::new();
  defender.set_armor_class(ac);
  world.defender = Some(defender);
}

#[when(regex = r"^the attacker attacks with a roll of (\d+)$")]
pub fn attack(world: &mut AttackWorld, roll: u8) {
  let attacker: &mut dyn Combatant = world.attacker.as_mut().unwrap();
  let defender: &mut dyn Combatant = world.defender.as_mut().unwrap();

  let attack = Attack::new();
  world.attack_result = Some(attack.attack(attacker, defender, roll));
}

#[then(regex = r"^the attack is a (hit|miss|crit)$")]
pub fn attack_result(world: &mut AttackWorld, result: AttackResult) {
  assert_eq!(world.attack_result.unwrap(), result);
}

#[then(regex = r"^the defender is damaged for (\d+) points?$")]
pub fn defender_hit_points(world: &mut AttackWorld, damage: u8) {
  let defender = world.defender.as_mut().unwrap();
  assert_eq!(defender.last_damage_received(), damage);
}
