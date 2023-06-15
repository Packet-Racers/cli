use reedline_repl_rs::{Repl, Result};

use packet_racers_cli::{
  commands::Commands, context::PacketRacerContext, prompt::PacketRacersPrompt,
};

#[tokio::main]
async fn main() -> Result<()> {
  let mut repl = Repl::new(PacketRacerContext::new())
    .with_name("Packet Racers CLI")
    .with_version("0.1.0")
    .with_banner("Welcome to Packet Racers CLI")
    .with_prompt("[📦🚘]")
    .with_description("Packet Racers CLI")
    .with_on_after_command_async(|context| Box::pin(PacketRacersPrompt::update_prompt(context)));

  for (command, callback) in Commands::all()? {
    repl = repl.with_command_async(command, callback);
  }

  // TODO: Add clean up function

  repl.run_async().await
}
