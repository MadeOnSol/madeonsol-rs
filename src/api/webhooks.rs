use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Webhook management (PRO: 3, ULTRA: 10).
#[derive(Debug, Clone)]
pub struct Webhooks {
    pub(crate) core: Arc<HttpCore>,
}

impl Webhooks {
    /// List all webhooks.
    pub async fn list(&self) -> Result<WebhookListResponse> {
        self.core.get("/webhooks", &()).await
    }

    /// Create a new webhook.
    pub async fn create(&self, params: &WebhookCreateParams) -> Result<Webhook> {
        self.core.post_json("/webhooks", params).await
    }

    /// Update a webhook.
    pub async fn update(
        &self,
        id: i64,
        params: &WebhookUpdateParams,
    ) -> Result<Webhook> {
        self.core
            .patch_json(&format!("/webhooks/{}", id), params)
            .await
    }

    /// Delete a webhook.
    pub async fn delete(&self, id: i64) -> Result<WebhookDeleteResponse> {
        self.core.delete(&format!("/webhooks/{}", id)).await
    }

    /// Send a test payload to a webhook.
    pub async fn test(&self, webhook_id: i64) -> Result<serde_json::Value> {
        let body = serde_json::json!({ "webhook_id": webhook_id });
        self.core.post_json("/webhooks/test", &body).await
    }
}
