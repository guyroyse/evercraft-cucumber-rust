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
    match s {
      "Good" => Ok(Alignment::Good),
      "Neutral" => Ok(Alignment::Neutral),
      "Evil" => Ok(Alignment::Evil),
      _ => Err(()),
    }
  }
}
