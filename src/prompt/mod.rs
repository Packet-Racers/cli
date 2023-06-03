use std::fmt::Display;

pub struct PacketRacersPrompt;

impl Display for PacketRacersPrompt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(📦🚘)> ")
  }
}

