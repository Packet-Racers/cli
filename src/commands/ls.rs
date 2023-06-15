use std::path::Path;

use reedline_repl_rs::{clap::ArgMatches, Result as ReplResult};

use crate::context::PacketRacerContext;

pub async fn ls(args: ArgMatches, _context: &mut PacketRacerContext) -> ReplResult<Option<String>> {
  let fallback = String::from(".");
  let path = args.get_one::<String>("path").unwrap_or(&fallback);
  let path = Path::new(&path);

  let mut output = String::new();

  let mut dir = tokio::fs::read_dir(path)
    .await
    .map_err(|_| "error reading directory")
    .unwrap();

  while let Some(entry) = dir
    .next_entry()
    .await
    .map_err(|_| "error reading directory entry")
    .unwrap()
  {
    let path = entry.path();
    let path = path.to_str().unwrap_or("Invalid UTF-8");

    output.push_str(path);
    output.push('\n');
  }

  Ok(Some(output))
}
