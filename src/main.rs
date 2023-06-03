use reedline_repl_rs::{Repl, Result};

use packet_racers_cli::{commands::Commands, context::PacketRacerContext};

fn main() -> Result<()> {
  let mut repl = Repl::new(PacketRacerContext::new())
    .with_name("Packet Racers CLI")
    .with_version("0.1.0")
    .with_banner("Welcome to Packet Racers CLI")
    .with_prompt("[📦🚘]")
    .with_description("Packet Racers CLI")
    .with_on_after_command(|context| {
      let is_logged_in = context.is_logged_in();
      let logged_prompt_str = if is_logged_in {
        format!("[{}]", context.get_name())
      } else {
        String::from("")
      };
      Ok(Some(format!("[📦🚘]{}", logged_prompt_str)))
    });

  for (command, callback) in Commands::all()? {
    repl = repl.with_command(command, callback);
  }

  repl.run()
}
