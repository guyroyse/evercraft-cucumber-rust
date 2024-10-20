use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alignment {
  Good,
  Neutral,
  Evil,
}

impl Default for Alignment {
  fn default() -> Self {
    Alignment::Neutral
  }
}

impl FromStr for Alignment {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "good" => Ok(Alignment::Good),
      "neutral" => Ok(Alignment::Neutral),
      "evil" => Ok(Alignment::Evil),
      _ => Err(()),
    }
  }
}
