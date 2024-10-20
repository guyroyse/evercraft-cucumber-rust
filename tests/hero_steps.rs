use cucumber::{ given, when, then, World };
use evercraft::*;

#[derive(World, Debug, Default)]
pub struct HeroWorld {
  hero: Option<Hero>,
}

#[given("a brand new hero")]
pub fn new_hero(world: &mut HeroWorld) {
  world.hero = Some(Hero::new());
}

#[when(regex = r"^the hero's name is set to (.+)$")]
pub fn set_name(world: &mut HeroWorld, name: String) {
  world.hero.as_mut().unwrap().set_name(name);
}

#[when(regex = r"^the hero's alignment is set to (Good|Neutral|Evil)$")]
pub fn set_alignment(world: &mut HeroWorld, alignment: Alignment) {
  world.hero.as_mut().unwrap().set_alignment(alignment);
}

#[when(regex = r"^the hero takes (\d+) points of damage$")]
pub fn take_damage(world: &mut HeroWorld, points: u8) {
  world.hero.as_mut().unwrap().damage(points);
}

#[then(regex = r"^the hero has no name$")]
pub fn name_is_empty(world: &mut HeroWorld) {
  assert_eq!(world.hero.as_ref().unwrap().name(), "");
}

#[then(regex = r"^the hero's name is (.+)$")]
pub fn name_is(world: &mut HeroWorld, name: String) {
  assert_eq!(world.hero.as_ref().unwrap().name(), name);
}

#[then(regex = r"^the hero's alignment is (Good|Neutral|Evil)$")]
pub fn alignment_is(world: &mut HeroWorld, alignment: Alignment) {
  assert_eq!(world.hero.as_ref().unwrap().alignment(), alignment);
}

#[then(regex = r"^the hero's armor class is (\d+)$")]
pub fn armor_class_is(world: &mut HeroWorld, armor_class: u8) {
  assert_eq!(world.hero.as_ref().unwrap().armor_class(), armor_class);
}

#[then(regex = r"^the hero's hit points are (\d+)$")]
pub fn hit_points_are(world: &mut HeroWorld, hit_points: u16) {
  assert_eq!(world.hero.as_ref().unwrap().hit_points(), hit_points);
}

#[then(regex = r"^the hero's attack modifier is ([+-]?\d+)$")]
pub fn attack_modifier_is(world: &mut HeroWorld, modifier: i8) {
  assert_eq!(world.hero.as_ref().unwrap().attack_modifier(), modifier);
}

#[then(regex = r"^the hero's attack damage is ([+]?\d+)$")]
pub fn attack_damage_is(world: &mut HeroWorld, damage: u8) {
  assert_eq!(world.hero.as_ref().unwrap().attack_damage(), damage);
}

#[then(regex = r"^the hero's crit damage is ([+]?\d+)$")]
pub fn crit_damage_is(world: &mut HeroWorld, damage: u8) {
  assert_eq!(world.hero.as_ref().unwrap().crit_damage(), damage);
}

#[then(regex = r"^the hero's current hit points are (\d+)$")]
pub fn current_hit_points_are(world: &mut HeroWorld, hit_points: u16) {
  assert_eq!(world.hero.as_mut().unwrap().current_hit_points(), hit_points);
}

#[then(regex = r"^the hero is (alive|dead)$")]
pub fn hero_is_alive_or_dead(world: &mut HeroWorld, status: String) {
  let expected_alive = status == "alive";
  assert_eq!(world.hero.as_mut().unwrap().alive(), expected_alive);
}
