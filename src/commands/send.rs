use std::str::FromStr;

use cli_prompts::{
  prompts::{AbortReason, Input, Selection},
  DisplayPrompt,
};
use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};

use crate::context::{ConnectionKey, PacketRacerContext};

pub async fn send_to(
  args: ArgMatches,
  context: &mut PacketRacerContext,
) -> ReplResult<Option<String>> {
  if !context.is_connected_to_any().await {
    return Ok(Some(
      "You must be connected to a network to establish a connection".to_string(),
    ));
  }

  if context.contacts().await.is_empty() {
    return Ok(Some(
      "You must have at least one contact to send a file".to_string(),
    ));
  }

  let contacts = context.contacts().await;

  let (to, file) = show_input_prompt(
    args.get_one::<String>("to"),
    args.get_one::<String>("file"),
    contacts,
  )
  .await
  .map_err(|_| "Failed to get input".to_string())
  .unwrap();

  context
    .send_file(to, file)
    .await
    .expect("Failed to send file");

  Ok(Some("File sent".to_string()))
}

pub async fn show_input_prompt(
  to: Option<&String>,
  file: Option<&String>,
  contacts: Vec<String>,
) -> Result<(ConnectionKey, String), AbortReason> {
  let to: ConnectionKey = match to.is_some() {
    true => ConnectionKey::from_str(to.unwrap()).expect("Invalid connection key {to::protocol}"),
    false => connection_key_validation(
      &Selection::new("Select the connection key", contacts.into_iter()).display()?,
    )
    .expect("Invalid connection key"),
  };
  let file: String = match file.is_some() {
    true => file_validation(file.unwrap()).expect("Invalid file path"),
    false => Input::new("Enter the file path", file_validation).display()?,
  };

  Ok((to, file))
}

pub fn connection_key_validation(connection_key: &str) -> Result<ConnectionKey, String> {
  ConnectionKey::from_str(connection_key).map_err(|_| "Invalid connection key".into())
}

pub fn file_validation(file: &str) -> Result<String, String> {
  if std::fs::metadata(file).is_ok() {
    Ok(file.to_string())
  } else {
    Err("File does not exist".into())
  }
}
