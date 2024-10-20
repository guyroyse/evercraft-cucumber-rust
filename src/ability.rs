use crate::error::AbilityOutOfRangeError;

#[derive(Debug)]
pub struct Ability {
  score: u8,
}

impl Ability {
  pub fn new() -> Self {
    Ability {
      score: 10,
    }
  }

  pub fn score(&self) -> u8 {
    self.score
  }

  pub fn set_score(&mut self, score: u8) -> Result<(), AbilityOutOfRangeError> {
    if score < 1 || score > 20 {
      Err(AbilityOutOfRangeError::new(score))
    } else {
      self.score = score;
      Ok(())
    }
  }

  pub fn modifier(&self) -> i8 {
    ((self.score as f32) / 2.0 - 5.0).floor() as i8
  }
}
