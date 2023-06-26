use ntex::http;
use serde::{Serialize, Deserialize};

/// ## VpnKit Client
///
/// This client is used to communicate with the VpnKit daemon
/// Principally, it is used to add and remove port forwards
///
pub struct VpnKitRc {
  /// Base URL for the API
  pub base_url: String,
  /// HTTP client
  pub client: http::client::Client,
}

/// ## VpnKit Error
///
/// Error message returned by the VpnKit daemon
///
#[derive(Deserialize)]
pub struct VpnKitError {
  /// Error message
  pub message: String,
}

/// ## VpnKit Protocol
///
/// Protocol to use for port forwarding
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VpnKitProtocol {
  /// TCP
  TCP,
  /// UDP
  UDP,
  /// UNIX socket
  UNIX,
}

/// ## Port
///
/// A port forward definition
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VpnKitPort {
  /// Port number to listen on
  #[serde(skip_serializing_if = "Option::is_none")]
  pub in_port: Option<usize>,
  /// Port number to forward to
  #[serde(skip_serializing_if = "Option::is_none")]
  pub out_port: Option<usize>,
  /// Protocol to use
  #[serde(skip_serializing_if = "Option::is_none")]
  pub proto: Option<VpnKitProtocol>,
  /// IP address to listen on
  #[serde(skip_serializing_if = "Option::is_none")]
  pub in_ip: Option<String>,
  /// IP address to forward to
  #[serde(skip_serializing_if = "Option::is_none")]
  pub out_ip: Option<String>,
  /// Path to UNIX socket
  #[serde(skip_serializing_if = "Option::is_none")]
  pub in_path: Option<String>,
  /// Path to UNIX socket to forward to
  #[serde(skip_serializing_if = "Option::is_none")]
  pub out_path: Option<String>,
}
