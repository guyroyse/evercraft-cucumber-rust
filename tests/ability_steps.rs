use cucumber::{ given, then, World };
use evercraft::*;

#[derive(World, Debug, Default)]
pub struct AbilityWorld {
  ability: Option<Ability>,
}

fn maybe_ability(world: &mut AbilityWorld) -> &mut Ability {
  match world.ability {
    Some(ref mut ability) => ability,
    None => panic!("Ability is not initialized"),
  }
}

#[given("a new ability")]
pub fn new_ability(world: &mut AbilityWorld) {
  world.ability = Some(Ability::new());
}

#[then(regex = r"^the ability's score is (\d+)$")]
pub fn score_is(world: &mut AbilityWorld, score: u8) {
  let ability = maybe_ability(world);
  assert_eq!(ability.score(), score);
}
