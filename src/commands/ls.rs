use std::path::Path;

use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};

use crate::context::PacketRacerContext;

pub fn ls(args: ArgMatches, context: &mut PacketRacerContext) -> ReplResult<Option<String>> {
  let fallback = String::from(".");
  let path = args.get_one::<String>("path").unwrap_or(&fallback);
  let path = Path::new(&path);

  let mut output = String::new();

  for entry in path.read_dir().unwrap() {
    let path = entry.unwrap().path();
    let path = path.to_str().unwrap_or("Invalid UTF-8");

    output.push_str(path);
    output.push('\n');
  }

  Ok(Some(output))
}
