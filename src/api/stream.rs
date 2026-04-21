use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// WebSocket streaming token issuance.
#[derive(Debug, Clone)]
pub struct Stream {
    pub(crate) core: Arc<HttpCore>,
}

impl Stream {
    /// Generate a 24-hour WebSocket streaming token.
    ///
    /// PRO/ULTRA: returns `ws_url` for KOL/deployer event streaming.
    /// ULTRA: also returns `dex_ws_url` for the all-DEX trade firehose.
    ///
    /// Connect by appending `?token=<token>` to the returned URL.
    pub async fn get_token(&self) -> Result<StreamToken> {
        self.core.post_empty("/stream/token").await
    }
}
