use cli_prompts::{
  prompts::{AbortReason, Input},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};

use crate::context::PacketRacerContext;

fn show_input_prompt(
  address: Option<&String>,
  port: Option<&String>,
) -> Result<(String, u16), AbortReason> {
  let address: String = match address.is_some() {
    true => address.unwrap().to_string(),
    false => Input::new("Enter the server address", address_validation)
      .default_value("127.0.0.1")
      .display()?,
  };

  let port: u16 = match port.is_some() {
    true => port.unwrap().parse::<u16>().expect("Invalid port"),
    false => Input::new("Enter the server port", port_validation)
      .default_value("8080")
      .display()?,
  };

  Ok((address, port))
}

fn address_validation(address: &str) -> Result<String, String> {
  if address.is_empty() {
    Err("Address must not be empty.".into())
  } else if address.len() < 3 {
    Err("Address must be more than 2 characters.".into())
  } else if address.len() > 20 {
    Err("Address must not be more than 20 characters.".into())
  } else {
    Ok(address.to_string())
  }
}

fn port_validation(port: &str) -> Result<u16, String> {
  let port = match port.parse::<u16>() {
    Ok(port) => port,
    Err(_) => return Err("Port must be a number".into()),
  };

  Ok(port)
}

pub async fn connect_to_server(
  args: ArgMatches,
  context: &mut PacketRacerContext,
) -> ReplResult<Option<String>> {
  if !context.is_logged_in() {
    return Ok(Some(
      "You must be logged in to connect to a server".to_string(),
    ));
  }

  let (address, port) = show_input_prompt(
    args.get_one::<String>("address"),
    args.get_one::<String>("port"),
  )
  .map_err(|_| "Failed to get input".to_string())
  .unwrap();

  context
    .connect_to_server(address.clone(), port)
    .await
    .expect("Failed to connect to server");

  Ok(Some("Connected to server".to_string()))
}
