#[derive(Default)]
pub struct PacketRacerContext {
  pub logged_in: bool,
  pub name: String,
  pub port: u16,
}

impl PacketRacerContext {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn login(&mut self, name: String, port: u16) {
    self.logged_in = true;
    self.name = name;
    self.port = port;
  }

  pub fn logout(&mut self) {
    self.logged_in = false;
  }

  pub fn is_logged_in(&self) -> bool {
    self.logged_in
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }
}
