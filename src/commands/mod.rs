use reedline_repl_rs::{
  clap::{Arg, Command},
  AsyncCallback, Error, Result,
};

use crate::context::PacketRacerContext;

pub mod connect;
pub mod contact;
pub mod login;
pub mod ls;
pub mod send;

pub struct Commands;

// PacketRacersCallback
type PacketRacersCallback = AsyncCallback<PacketRacerContext, Error>;

impl Commands {
  pub fn all() -> Result<Vec<(Command, PacketRacersCallback)>> {
    Ok(vec![
      Commands::connect()?,
      Commands::login()?,
      Commands::send()?,
      Commands::contact()?,
      Commands::ls()?,
    ])
  }

  pub fn connect() -> Result<(Command, PacketRacersCallback)> {
    Ok((
      Command::new("connect")
        .arg(Arg::new("address"))
        .arg(Arg::new("port"))
        .about("Connect to a server"),
      |args, context| Box::pin(connect::connect_to_server(args, context)),
    ))
  }

  pub fn login() -> Result<(Command, PacketRacersCallback)> {
    Ok((
      Command::new("login").about("Login to the game"),
      |args, context| Box::pin(login::login(args, context)),
    ))
  }

  pub fn contact() -> Result<(Command, PacketRacersCallback)> {
    Ok((
      Command::new("contact")
        .arg(Arg::new("to"))
        .arg(Arg::new("protocol").value_parser(["tcp", "udp", "gudp"]))
        .about("Add a contact"),
      |args, context| Box::pin(contact::contact_to(args, context)),
    ))
  }

  pub fn send() -> Result<(Command, PacketRacersCallback)> {
    Ok((
      Command::new("send")
        .arg(Arg::new("to"))
        .arg(Arg::new("file"))
        .about("Send a to another user connected in one of yours connected networks"),
      |args, context| Box::pin(send::send_to(args, context)),
    ))
  }

  pub fn ls() -> Result<(Command, PacketRacersCallback)> {
    Ok((
      Command::new("ls")
        .arg(Arg::new("path").default_value("."))
        .about("List files in a directory"),
      |args, context| Box::pin(ls::ls(args, context)),
    ))
  }
}
