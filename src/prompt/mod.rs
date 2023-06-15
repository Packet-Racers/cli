use crate::context::PacketRacerContext;
use reedline_repl_rs::Result as ReplResult;

pub struct PacketRacersPrompt;

impl PacketRacersPrompt {
  pub async fn update_prompt(context: &mut PacketRacerContext) -> ReplResult<Option<String>> {
    let is_logged_in = context.is_logged_in();
    let connections = context.connections().await.len();
    let contacts = context.contacts().await.len();

    let mut prompt = String::from("[📦🚘]");

    if is_logged_in {
      prompt += &format!("[{}]", context.get_name());
    }

    if connections > 0 {
      prompt += &format!("[🔗 {}]", connections);
    }

    if contacts > 0 {
      prompt += &format!("[👥 {}]", contacts);
    }

    Ok(Some(prompt))
  }
}
