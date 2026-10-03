use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Copy-trade rules CRUD + fired signal history — PRO+.
///
/// A rule fires on trades of tracked KOL wallets only: `source_wallets_untracked`
/// and `warnings` on every rule response say which source wallets can never fire.
/// Delivery is webhook (HMAC-signed; the secret is returned once), WebSocket or both.
#[derive(Debug, Clone)]
pub struct Copytrade {
    pub(crate) core: Arc<HttpCore>,
}

impl Copytrade {
    /// List your copy-trade rules.
    pub async fn list(&self) -> Result<CopytradeSubscriptionListResponse> {
        self.core.get("/copytrade/subscriptions", &()).await
    }

    /// Create a rule. Returns a one-time `webhook_secret` when a webhook URL was given.
    pub async fn create(
        &self,
        params: &CopytradeSubscriptionCreateParams,
    ) -> Result<CopytradeSubscriptionResponse> {
        self.core.post_json("/copytrade/subscriptions", params).await
    }

    /// Fetch one rule.
    pub async fn get(&self, id: i64) -> Result<CopytradeSubscriptionResponse> {
        self.core
            .get(&format!("/copytrade/subscriptions/{}", id), &())
            .await
    }

    /// Update a rule (only the fields you set are sent).
    pub async fn update(
        &self,
        id: i64,
        params: &CopytradeSubscriptionUpdateParams,
    ) -> Result<CopytradeSubscriptionResponse> {
        self.core
            .patch_json(&format!("/copytrade/subscriptions/{}", id), params)
            .await
    }

    /// Delete a rule.
    pub async fn delete(&self, id: i64) -> Result<CopytradeSubscriptionDeleteResponse> {
        self.core
            .delete(&format!("/copytrade/subscriptions/{}", id))
            .await
    }

    /// Fired signals for your rules (up to 7 days).
    pub async fn signals(&self, params: &CopytradeSignalsParams) -> Result<CopytradeSignalsResponse> {
        self.core.get("/copytrade/signals", params).await
    }
}
