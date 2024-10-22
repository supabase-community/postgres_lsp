pub mod client_flags;

use anyhow::Result;
use serde::Serialize;
use tower_lsp::lsp_types::{notification::ShowMessage, MessageType, ShowMessageParams};
use tower_lsp::Client;

use crate::server::options::ClientConfigurationOptions;

#[derive(Debug, Clone)]
pub struct LspClient {
    client: Client,
}

impl LspClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn send_notification<N>(&self, params: N::Params) -> Result<()>
    where
        N: tower_lsp::lsp_types::notification::Notification,
        N::Params: Serialize,
    {
        self.client.send_notification::<N>(params);
        Ok(())
    }

    /// This will ignore any errors that occur while sending the notification.
    pub fn send_info_notification(&self, message: &str) {
        let _ = self.send_notification::<ShowMessage>(ShowMessageParams {
            message: message.into(),
            typ: MessageType::INFO,
        });
    }

    pub async fn send_request<R>(&self, params: R::Params) -> Result<R::Result>
    where
        R: tower_lsp::lsp_types::request::Request,
    {
        let response = self.client.send_request::<R>(params).await?;

        Ok(response)
    }

    pub fn parse_options(
        &self,
        mut value: serde_json::Value,
    ) -> Result<ClientConfigurationOptions> {
        // if there are multiple servers, we need to extract the options for pglsp first
        let options = match value.get_mut("pglsp") {
            Some(section) => section.take(),
            None => value,
        };

        let options = match serde_json::from_value(options) {
            Ok(new_options) => new_options,
            Err(why) => {
                let message = format!(
                    "The texlab configuration is invalid; using the default settings instead.\nDetails: {why}"
                );
                let typ = MessageType::WARNING;
                self.send_notification::<ShowMessage>(ShowMessageParams { message, typ })?;
                None
            }
        };

        Ok(options.unwrap_or_default())
    }
}
