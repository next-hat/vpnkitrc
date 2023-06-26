use vpnkitrc::VpnKitRc;
use vpnkitrc::stubs::*;

#[ntex::main]
async fn main() -> std::io::Result<()> {
  println!("Hello, world!");
  let home = std::env::var("HOME").unwrap();
  let path = format!("{home}/.docker/desktop/vpnkit.port.sock");
  println!("Connecting to {}", path);
  let client = VpnKitRc::connect_uds(&path);
  let ports = client.list().await?;
  println!("Ports: {:?}", ports);
  let port = VpnKitPort {
    in_port: Some(8081),
    out_port: Some(8081),
    proto: Some(VpnKitProtocol::TCP),
    in_ip: Some("127.0.0.1".into()),
    out_ip: Some("0.0.0.0".into()),
    in_path: None,
    out_path: None,
  };
  client.expose_port(&port).await?;
  let ports = client.list().await?;
  println!("Ports: {:?}", ports);
  client.unexpose_port(&port).await?;
  let ports = client.list().await?;
  println!("Ports: {:?}", ports);
  Ok(())
}
