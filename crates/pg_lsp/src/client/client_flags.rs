/// Contains information about the client's capabilities.
/// This is used to determine which features the server can use.
#[derive(Debug, Clone)]
pub struct ClientFlags {
    /// If `true`, the server can pull the configuration from the client.
    pub configuration_pull: bool,

    /// If `true`, the client notifies the server when the configuration changes.
    pub configuration_push: bool,
}
