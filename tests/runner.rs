use cucumber::World;

mod ability_steps;
use ability_steps::*;

mod hero_steps;
use hero_steps::*;

mod attack_steps;
use attack_steps::*;

#[tokio::main]
async fn main() {
  HeroWorld::run("tests/features/hero.feature").await;
  AbilityWorld::run("tests/features/ability.feature").await;
  AttackWorld::run("tests/features/attack.feature").await;
}
