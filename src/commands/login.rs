use std::{
  net::{SocketAddr, TcpStream},
  str::FromStr,
  time::Duration,
};

use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{Result as ReplResult, clap::ArgMatches};

use crate::context::PacketRacerContext;

fn show_input_prompt() -> Result<(String, u16), AbortReason> {
  let name: String = Input::new("Enter your name", name_validation)
    .default_value("John Doe")
    .display()?;

  let port: u16 = Input::new("Enter the port", port_validation)
    .default_value("80")
    .display()?;

  Ok((name, port))
}

fn name_validation(name: &str) -> Result<String, String> {
  if name.is_empty() {
    Err("Name must not be empty.".into())
  } else if name.len() < 3 {
    Err("Name must be more than 2 characters.".into())
  } else if name.len() > 20 {
    Err("Name must not be more than 20 characters.".into())
  } else {
    Ok(name.to_string())
  }
}

fn port_validation(port: &str) -> Result<u16, String> {
  let port = match port.parse::<u16>() {
    Ok(port) => port,
    Err(_) => return Err("Port must be a number".into()),
  };

  let socket = match SocketAddr::from_str(&format!("127.0.0.1:{port}")) {
    Ok(socket) => socket,
    Err(_) => return Err("Invalid port".into()),
  };

  if TcpStream::connect_timeout(&socket, Duration::from_secs(1)).is_ok() {
    return Err("Port is already in use".into());
  }

  Ok(port)
}

pub fn login(
  _args: ArgMatches,
  context: &mut PacketRacerContext,
) -> ReplResult<Option<String>> {
  let (name, port) = match show_input_prompt() {
    Ok((name, port)) => (name, port),
    Err(reason) => return Ok(Some(format!("{reason:?}"))),
  };

  context.login(name, port);

  Ok(Some("Logged in".to_string()))
}
