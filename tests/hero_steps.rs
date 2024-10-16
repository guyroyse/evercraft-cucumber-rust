use cucumber::{ given, when, then, World };
use evercraft::*;

#[derive(World, Debug, Default)]
pub struct HeroWorld {
  hero: Option<Hero>,
}

fn maybe_hero(world: &mut HeroWorld) -> &mut Hero {
  match world.hero {
    Some(ref mut hero) => hero,
    None => panic!("Hero is not initialized"),
  }
}

#[given("a brand new hero")]
pub fn new_hero(world: &mut HeroWorld) {
  world.hero = Some(Hero::new());
}

#[when(regex = r"^the hero's name is set to (.+)$")]
pub fn set_name(world: &mut HeroWorld, name: String) {
  maybe_hero(world).set_name(name);
}

#[when(regex = r"^the hero's alignment is set to (Good|Neutral|Evil)$")]
pub fn set_alignment(world: &mut HeroWorld, alignment: Alignment) {
  maybe_hero(world).set_alignment(alignment);
}

#[when(regex = r"^the hero takes (\d+) points of damage$")]
pub fn take_damage(world: &mut HeroWorld, points: u8) {
  maybe_hero(world).damage(points);
}

#[then(regex = r"^the hero has no name$")]
pub fn name_is_empty(world: &mut HeroWorld) {
  assert_eq!(maybe_hero(world).name(), "");
}

#[then(regex = r"^the hero's name is (.+)$")]
pub fn name_is(world: &mut HeroWorld, name: String) {
  assert_eq!(maybe_hero(world).name(), name);
}

#[then(regex = r"^the hero's alignment is (Good|Neutral|Evil)$")]
pub fn alignment_is(world: &mut HeroWorld, alignment: Alignment) {
  assert_eq!(maybe_hero(world).alignment(), alignment);
}

#[then(regex = r"^the hero's armor class is (\d+)$")]
pub fn armor_class_is(world: &mut HeroWorld, armor_class: u8) {
  assert_eq!(maybe_hero(world).armor_class(), armor_class);
}

#[then(regex = r"^the hero's hit points are (\d+)$")]
pub fn hit_points_are(world: &mut HeroWorld, hit_points: i16) {
  assert_eq!(maybe_hero(world).hit_points(), hit_points);
}

#[then(regex = r"^the hero's current hit points are (-?\d+)$")]
pub fn current_hit_points_are(world: &mut HeroWorld, hit_points: i16) {
  assert_eq!(maybe_hero(world).current_hit_points(), hit_points);
}

#[then(regex = r"^the hero is (alive|dead)$")]
pub fn hero_is(world: &mut HeroWorld, status: String) {
  let expected_alive = status == "alive";
  assert_eq!(maybe_hero(world).alive(), expected_alive);
}
