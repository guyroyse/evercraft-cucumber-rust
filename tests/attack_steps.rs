use cucumber::{ given, when, then, World };
use evercraft::*;

#[derive(Debug)]
struct MockCombatant {
  armor_class: u8,
}

impl Combatant for MockCombatant {
  fn armor_class(&self) -> u8 {
    self.armor_class
  }
}

#[derive(World, Debug, Default)]
pub struct AttackWorld {
  attacker: Option<Box<dyn Combatant>>,
  defender: Option<Box<dyn Combatant>>,
  attack_result: bool,
}

fn maybe_attacker(world: &AttackWorld) -> &Box<dyn Combatant> {
  match world.attacker {
    Some(ref attacker) => attacker,
    None => panic!("Attacker is not initialized"),
  }
}

fn maybe_defender(world: &AttackWorld) -> &Box<dyn Combatant> {
  match world.defender {
    Some(ref defender) => defender,
    None => panic!("Defender is not initialized"),
  }
}

#[given("an attacker")]
pub fn new_attacker(world: &mut AttackWorld) {
  world.attacker = Some(Box::new(Hero::new()));
}

#[given(regex = r"^a defender with an armor class of (\d+)$")]
pub fn new_defender(world: &mut AttackWorld, score: u8) {
  world.defender = Some(Box::new(MockCombatant { armor_class: score }));
}

#[when(regex = r"^the attacker attacks with a roll of (\d+)$")]
pub fn attack(world: &mut AttackWorld, roll: u8) {
  let attacker = maybe_attacker(world);
  let defender = maybe_defender(world);
  let attack = Attack::new();
  world.attack_result = attack.attack(attacker, defender, roll);
}

#[then(expr = "the attack is a {string}")]
pub fn attack_result(world: &mut AttackWorld, result: String) {
  let expected_result = result == "hit";
  assert_eq!(world.attack_result, expected_result);
}
