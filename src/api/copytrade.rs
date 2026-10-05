use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Copy-trade rules CRUD + fired signal history — PRO+.
///
/// Copy-trade rules can follow any valid Solana wallet, KOL or not (server
/// 2026-10-04, `source_admission` `any_wallet`; KOL membership is enrichment only
/// and copy-trade sources do not use Wallet Tracker quota). `operational_state` on
/// every rule says whether it can fire right now (`eligible`, or an infrastructure
/// state with `monitoring_reasons`); under the legacy `kol_only` engine only tracked
/// KOL wallets fire.
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
