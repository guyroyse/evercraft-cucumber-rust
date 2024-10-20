use std::fmt::*;

#[derive(Debug)]
pub struct AbilityOutOfRangeError {
  score: u8,
}

impl Display for AbilityOutOfRangeError {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "Score was {}. Ability score must be between 1 and 20.", self.score)
  }
}

impl AbilityOutOfRangeError {
  pub fn new(score: u8) -> Self {
    AbilityOutOfRangeError { score }
  }
}
