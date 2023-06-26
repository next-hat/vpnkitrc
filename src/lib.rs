use ntex::rt;
use ntex::http;

pub mod stubs;

use stubs::*;
pub use stubs::VpnKitRc;

impl VpnKitRc {
  /// ## Connect uds
  ///
  /// Create a new client based on unix socket
  ///
  pub fn connect_uds(path: &str) -> Self {
    let path = path.to_owned();
    let client = http::client::Client::build()
      .connector(
        http::client::Connector::default()
          .connector(ntex::service::fn_service(move |_| {
            let path = path.trim_start_matches("unix://").to_string();
            async move { Ok(rt::unix_connect(path).await?) }
          }))
          .timeout(ntex::time::Millis::from_secs(100))
          .finish(),
      )
      .timeout(ntex::time::Millis::from_secs(100))
      .finish();
    VpnKitRc {
      client,
      base_url: "http://localhost".into(),
    }
  }

  /// ## List
  ///
  /// List existing port forwards
  ///
  pub async fn list(&self) -> std::io::Result<Vec<VpnKitPort>> {
    let mut res = self
      .client
      .get(format!("{}/forwards/list", self.base_url))
      .send()
      .await
      .map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
      })?;
    Self::is_api_error(&mut res).await?;
    let data = res.json::<Vec<VpnKitPort>>().await.map_err(|err| {
      std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    })?;
    Ok(data)
  }

  /// ## Dump state
  ///
  /// Dump the state of the port forwards
  ///
  pub async fn dump_state(&self) -> std::io::Result<String> {
    let mut res = self
      .client
      .get(format!("{}/forwards/dump", self.base_url))
      .send()
      .await
      .map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
      })?;
    Self::is_api_error(&mut res).await?;
    let data = res.body().await.map_err(|err| {
      std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    })?;
    let data = String::from_utf8(data.to_vec()).map_err(|err| {
      std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    })?;
    Ok(data)
  }

  /// ## Expose port
  ///
  /// Add a new port forward rule
  ///
  pub async fn expose_port(&self, port: &VpnKitPort) -> std::io::Result<()> {
    let mut res = self
      .client
      .put(format!("{}/forwards/expose/port", self.base_url))
      .send_json(port)
      .await
      .map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
      })?;
    Self::is_api_error(&mut res).await?;
    Ok(())
  }

  /// ## Unexpose port
  ///
  /// Remove a port forward rule
  ///
  pub async fn unexpose_port(&self, port: &VpnKitPort) -> std::io::Result<()> {
    let mut res = self
      .client
      .delete(format!("{}/forwards/unexpose/port", self.base_url))
      .send_json(port)
      .await
      .map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
      })?;
    Self::is_api_error(&mut res).await?;
    Ok(())
  }

  /// ## Expose pipe path
  ///
  /// Add a new pipe path forward rule eg: /tmp/unix.sock
  ///
  pub async fn expose_pipe_path(
    &self,
    port: &VpnKitPort,
  ) -> std::io::Result<()> {
    let mut res = self
      .client
      .put(format!("{}/forwards/expose/pipe", self.base_url))
      .send_json(port)
      .await
      .map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
      })?;
    Self::is_api_error(&mut res).await?;
    Ok(())
  }

  /// ## Unexpose pipe path
  ///
  /// Remove a pipe path forward rule eg: /tmp/unix.sock
  ///
  pub async fn unexpose_pipe_path(
    &self,
    port: &VpnKitPort,
  ) -> std::io::Result<()> {
    let mut res = self
      .client
      .delete(format!("{}/forwards/unexpose/pipe", self.base_url))
      .send_json(port)
      .await
      .map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
      })?;
    Self::is_api_error(&mut res).await?;
    Ok(())
  }

  /// ## Is API error
  ///
  /// **INTERNAL FUNC**
  /// This function is used internally to check if the response is an API error
  ///
  async fn is_api_error(
    res: &mut http::client::ClientResponse,
  ) -> std::io::Result<()> {
    let status = res.status();
    if status.is_client_error() || status.is_server_error() {
      let err = res.json::<VpnKitError>().await.map_err(|err| {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
      })?;
      return Err(std::io::Error::new(std::io::ErrorKind::Other, err.message));
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {

  /// ## Get socket path
  ///
  /// Get the socket path from the environment
  ///
  fn get_socket_path() -> String {
    std::env::var("VPNKIT_SOCKET")
      .expect("VPNKIT_SOCKET environment variable not set")
  }

  /// ## Generate client
  ///
  /// Generate a client for testing
  ///
  fn gen_client() -> super::VpnKitRc {
    let socket_path = get_socket_path();
    super::VpnKitRc::connect_uds(&socket_path)
  }

  /// ## Test list
  ///
  /// Test listing port forwards
  ///
  #[ntex::test]
  async fn test_list() {
    let client = gen_client();
    let res = client.list().await;
    assert!(res.is_ok());
  }

  /// ## Test dump state
  ///
  /// Test dumping the state of the port forwards
  ///
  #[ntex::test]
  async fn test_dump_state() {
    let client = gen_client();
    let res = client.dump_state().await;
    assert!(res.is_ok());
  }

  /// ## Test basic
  ///
  /// Test to add and remove a port forward
  ///
  #[ntex::test]
  async fn test_basic() {
    let client = gen_client();
    // Test TCP
    let port = super::VpnKitPort {
      in_port: Some(8081),
      out_port: Some(8081),
      proto: Some(super::VpnKitProtocol::TCP),
      in_ip: Some("127.0.0.1".into()),
      out_ip: Some("0.0.0.0".into()),
      in_path: None,
      out_path: None,
    };
    let res = client.expose_port(&port).await;
    assert!(res.is_ok());
    let res = client.unexpose_port(&port).await;
    assert!(res.is_ok());
    // Test UDP
    let port = super::VpnKitPort {
      in_port: Some(8081),
      out_port: Some(8081),
      proto: Some(super::VpnKitProtocol::UDP),
      in_ip: Some("127.0.0.1".into()),
      out_ip: Some("0.0.0.0".into()),
      in_path: None,
      out_path: None,
    };
    let res = client.expose_port(&port).await;
    assert!(res.is_ok());
    let res = client.unexpose_port(&port).await;
    assert!(res.is_ok());
    // Test unix socket
    let port = super::VpnKitPort {
      in_port: None,
      out_port: None,
      proto: Some(super::VpnKitProtocol::UNIX),
      in_ip: None,
      out_ip: None,
      in_path: Some("/run/guest-services/nanocl/nanocl.sock".into()),
      out_path: Some("/home/leone/test.sock".into()),
    };
    let res = client.expose_pipe_path(&port).await;
    println!("{:?}", res);
    assert!(res.is_ok());
    // let res = client.unexpose_pipe_path(&port).await;
    // assert!(res.is_ok());
  }
}
