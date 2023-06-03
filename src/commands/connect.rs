use reedline_repl_rs::{clap::ArgMatches, Result};

use crate::context::PacketRacerContext;

pub fn connect_to_server(
  args: ArgMatches,
  context: &mut PacketRacerContext,
) -> Result<Option<String>> {
  if !context.is_logged_in() {
    return Ok(Some(
      "You must be logged in to connect to a server".to_string(),
    ));
  }

  let address = args.get_one::<String>("address").unwrap();
  let port = args.get_one::<u16>("port").unwrap();

  Ok(Some(format!("Connecting to {address}:{port}")))
}
