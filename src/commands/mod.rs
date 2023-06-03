use reedline_repl_rs::{
  clap::{Arg, Command},
  Callback, Error, Result,
};

use crate::context::PacketRacerContext;

pub mod connect;
pub mod login;
pub mod ls;

pub struct Commands;

// PacketRacersCallback
type PacketRacersCallback = Callback<PacketRacerContext, Error>;

impl Commands {
  pub fn all() -> Result<Vec<(Command, PacketRacersCallback)>> {
    Ok(vec![Commands::connect()?, Commands::login()?, Commands::ls()?])
  }

  pub fn connect() -> Result<(Command, PacketRacersCallback)> {
    Ok((
      Command::new("connect")
        .arg(Arg::new("address").default_value("127.0.0.1"))
        .arg(Arg::new("port").default_value("80"))
        .about("Connect to a server"),
      connect::connect_to_server,
    ))
  }

  pub fn login() -> Result<(Command, PacketRacersCallback)> {
    Ok((
      Command::new("login").about("Login to the game"),
      login::login,
    ))
  }

  pub fn ls() -> Result<(Command, PacketRacersCallback)> {
    Ok((
      Command::new("ls")
        .arg(Arg::new("path").default_value("."))
        .about("List files in a directory"),
      ls::ls,
    ))
  }
}
