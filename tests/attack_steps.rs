use cucumber::{ given, when, then, World };
use evercraft::*;

#[derive(Debug)]
struct MyMockCombatant {
  armor_class: u8,
  last_damage_received: u8,
}

impl MyMockCombatant {
  fn new() -> Self {
    MyMockCombatant {
      armor_class: 10,
      last_damage_received: 0,
    }
  }

  fn set_armor_class(&mut self, ac: u8) {
    self.armor_class = ac;
  }

  fn last_damage_received(&self) -> u8 {
    self.last_damage_received
  }
}

impl Combatant for MyMockCombatant {
  fn armor_class(&self) -> u8 {
    self.armor_class
  }

  fn damage(&mut self, points: u8) {
    self.last_damage_received = points;
  }
}

#[derive(World, Debug, Default)]
pub struct AttackWorld {
  attacker: Option<MyMockCombatant>,
  defender: Option<MyMockCombatant>,
  attack_result: Option<AttackResult>,
}

#[given("an attacker")]
pub fn new_attacker(world: &mut AttackWorld) {
  let attacker = MyMockCombatant::new();
  world.attacker = Some(attacker);
}

#[given(regex = r"^a defender with an armor class of (\d+)$")]
pub fn new_defender(world: &mut AttackWorld, ac: u8) {
  let mut defender = MyMockCombatant::new();
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
