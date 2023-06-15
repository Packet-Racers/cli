use std::{collections::HashMap, net::SocketAddr, str::FromStr, sync::Arc};

use packet_racers::{file_transfer::FileTransfer, protocol::ProtocolType, user::User};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct ConnectionKey {
  id: Uuid,
  protocol: ProtocolType,
}

impl ConnectionKey {
  pub fn new(id: Uuid, protocol: ProtocolType) -> Self {
    Self { id, protocol }
  }
}

impl FromStr for ConnectionKey {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split("::");

    let id = split
      .next()
      .ok_or_else(|| "Invalid connection key".to_string())?;

    let protocol = split
      .next()
      .ok_or_else(|| "Invalid connection key".to_string())?;

    let id = Uuid::parse_str(id).map_err(|e| e.to_string())?;

    let protocol = ProtocolType::from_str(protocol)?;

    Ok(Self { id, protocol })
  }
}

impl std::fmt::Display for ConnectionKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}::{:?}", self.id, self.protocol)
  }
}

impl Eq for ConnectionKey {}

impl PartialEq for ConnectionKey {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.protocol as i32 == other.protocol as i32
  }
}

impl std::hash::Hash for ConnectionKey {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.id.hash(state);
    (self.protocol as i32).hash(state);
  }
}

#[derive(Default)]
pub struct PacketRacerContext {
  pub logged_in: bool,
  pub name: String,
  pub port: u16,
  pub user: Option<Arc<Mutex<User>>>,
  pub connections: HashMap<ConnectionKey, Arc<Mutex<FileTransfer>>>,
}

impl PacketRacerContext {
  pub fn new() -> Self {
    Self::default()
  }

  pub async fn login(&mut self, name: String, port: u16) -> Result<(), String> {
    self.logged_in = true;
    self.name = name;
    self.port = port;

    let address = SocketAddr::from_str(&format!("127.0.0.1:{port}")).map_err(|e| e.to_string())?;
    self.user = Some(User::new(address).await);

    // Start users listener
    let user = self.user.as_ref().unwrap().lock().await;

    user.clone().start_listening().map_err(|e| e.to_string())?;

    Ok(())
  }

  pub fn logout(&mut self) {
    self.logged_in = false;
  }

  pub fn is_logged_in(&self) -> bool {
    self.logged_in
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub async fn connections(&self) -> Vec<String> {
    let user = self.user.clone().unwrap();

    let user = user.lock().await;

    user
      .networks()
      .await
      .iter()
      .map(|n| n.to_string())
      .collect()
  }

  pub async fn is_connected_to_any(&self) -> bool {
    let user = self.user.clone().unwrap();

    let user = user.lock().await;

    !user.networks().await.is_empty()
  }

  pub async fn connect_to_server(&mut self, address: String, port: u16) -> Result<(), String> {
    let user = self.user.clone().unwrap();
    let user = user.lock().await;

    let network_addr = SocketAddr::from_str(&format!("{address}:{port}")).expect("Invalid address");
    user
      .connect_to_network(network_addr)
      .await
      .map_err(|e| e.to_string())?;

    Ok(())
  }

  pub async fn establish_connection(
    &mut self,
    to: Uuid,
    protocol: ProtocolType,
  ) -> Result<(), String> {
    let user = self.user.clone().unwrap();
    let user = user.lock().await;

    let receiver_address = user.get_address_by_uuid(to).await.unwrap();
    let connection = user
      .create_file_transfer(receiver_address, protocol)
      .await
      .map_err(|e| e.to_string())?;

    self.connections.insert(
      ConnectionKey::new(to, protocol),
      Arc::new(Mutex::new(connection)),
    );

    Ok(())
  }

  pub async fn contacts(&self) -> Vec<String> {
    self.connections.keys().map(|key| key.to_string()).collect()
  }

  pub async fn send_file(&mut self, to: ConnectionKey, file_path: String) -> Result<(), String> {
    let connection = self.connections.get(&to).unwrap();
    let mut connection = connection.lock().await;

    connection.send(&file_path).await?;

    Ok(())
  }
}
