use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Coordination alert rules CRUD (v1.1) — PRO 5 rules, ULTRA 20 rules.
///
/// Alerts fire the moment a new KOL coordination cluster forms.
/// Delivered via WebSocket channel `kol:coordination` and/or HMAC-signed webhook.
#[derive(Debug, Clone)]
pub struct CoordinationAlerts {
    pub(crate) core: Arc<HttpCore>,
}

impl CoordinationAlerts {
    /// List your coordination alert rules.
    pub async fn list(&self) -> Result<CoordinationAlertListResponse> {
        self.core.get("/kol/coordination/alerts", &()).await
    }

    /// Create a coordination alert rule.
    /// Returns the rule plus a one-time `webhook_secret` — save it for HMAC verification.
    pub async fn create(
        &self,
        params: &CoordinationAlertCreateParams,
    ) -> Result<CoordinationAlertCreateResponse> {
        self.core
            .post_json("/kol/coordination/alerts", params)
            .await
    }

    /// Fetch a single rule by id.
    pub async fn get(&self, id: &str) -> Result<CoordinationAlertGetResponse> {
        self.core
            .get(&format!("/kol/coordination/alerts/{}", id), &())
            .await
    }

    /// Update a rule (toggle `is_active`, raise `min_score`, etc).
    pub async fn update(
        &self,
        id: &str,
        params: &CoordinationAlertUpdateParams,
    ) -> Result<CoordinationAlertUpdateResponse> {
        self.core
            .patch_json(&format!("/kol/coordination/alerts/{}", id), params)
            .await
    }

    /// Delete a rule.
    pub async fn delete(&self, id: &str) -> Result<CoordinationAlertDeleteResponse> {
        self.core
            .delete(&format!("/kol/coordination/alerts/{}", id))
            .await
    }
}
