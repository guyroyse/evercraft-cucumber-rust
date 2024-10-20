use cucumber::{ given, then, when, World };
use evercraft::*;

#[derive(World, Debug, Default)]
pub struct AbilityWorld {
  ability: Option<Ability>,
  result: Option<Result<(), AbilityOutOfRangeError>>,
}

#[given("a new ability")]
pub fn new_ability(world: &mut AbilityWorld) {
  world.ability = Some(Ability::new());
}

#[when(regex = r"^the ability score is set to (\d+)$")]
pub fn set_score(world: &mut AbilityWorld, score: u8) {
  let result = world.ability.as_mut().unwrap().set_score(score);
  world.result = Some(result);
}

#[then(regex = r"^the ability score is (\d+)$")]
pub fn score_is(world: &mut AbilityWorld, score: u8) {
  assert_eq!(world.ability.as_ref().unwrap().score(), score);
}

#[then(regex = r"^the ability modifier is ([+-]?\d+)$")]
pub fn modifier_is(world: &mut AbilityWorld, modifier: i8) {
  assert_eq!(world.ability.as_ref().unwrap().modifier(), modifier);
}

#[then(regex = r"^an (Ok|Err) result is returned$")]
pub fn result_is(world: &mut AbilityWorld, result: String) {
  match world.result.as_ref().unwrap() {
    Ok(_) => assert_eq!(result, "Ok"),
    Err(_) => assert_eq!(result, "Err"),
  };
}

#[then(regex = r"^the error message is '(.*)'$")]
pub fn error_message(world: &mut AbilityWorld, message: String) {
  match world.result.as_ref().unwrap() {
    Ok(_) => panic!("Expected an error"),
    Err(error) => assert_eq!(error.to_string(), message),
  };
}
