use cli_prompts::{
  prompts::{AbortReason, Input, Selection},
  DisplayPrompt,
};
use packet_racers::protocol::ProtocolType;
use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};
use uuid::Uuid;

use crate::context::PacketRacerContext;

pub async fn contact_to(
  args: ArgMatches,
  context: &mut PacketRacerContext,
) -> ReplResult<Option<String>> {
  if !context.is_connected_to_any().await {
    return Ok(Some(
      "You must be connected to a network to establish a connection".to_string(),
    ));
  }

  let (to, protocol) = show_input_prompt(
    args.get_one::<String>("to"),
    args.get_one::<String>("protocol"),
  )
  .map_err(|_| "Failed to get input".to_string())
  .unwrap();

  match context.establish_connection(to, protocol).await {
    Ok(_) => Ok(Some("Connection established".to_string())),
    Err(_) => Ok(Some("Failed to establish connection".to_string())),
  }
}

pub fn show_input_prompt(
  to: Option<&String>,
  protocol: Option<&String>,
) -> Result<(Uuid, ProtocolType), AbortReason> {
  let protocols = vec!["tcp", "udp", "gudp"];

  let to: Uuid = match to.is_some() {
    true => Uuid::parse_str(to.unwrap()).expect("Invalid UUID"),
    false => Input::new("Enter the user UUID", uuid_validation).display()?,
  };
  let protocol: ProtocolType = match protocol.is_some() {
    true => protocol_validation(protocol.unwrap()).expect("Invalid protocol"),
    false => {
      protocol_validation(Selection::new("Enter the protocol", protocols.into_iter()).display()?)
        .expect("Invalid protocol")
    }
  };

  Ok((to, protocol))
}

pub fn uuid_validation(uuid: &str) -> Result<Uuid, String> {
  let uuid = match Uuid::parse_str(uuid) {
    Ok(uuid) => uuid,
    Err(_) => return Err("Invalid UUID".into()),
  };

  Ok(uuid)
}

pub fn protocol_validation(protocol: &str) -> Result<ProtocolType, String> {
  let protocol = match protocol.to_lowercase().as_str() {
    "tcp" => ProtocolType::Tcp,
    "udp" => ProtocolType::Udp,
    "gudp" => ProtocolType::GuaranteedUdp,
    _ => return Err("Invalid protocol".into()),
  };

  Ok(protocol)
}
