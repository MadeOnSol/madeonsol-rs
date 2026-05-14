use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Account self-inspection — tier, quota state, and per-feature usage. v0.8 (server-side v1.7).
#[derive(Debug, Clone)]
pub struct Me {
    pub(crate) core: Arc<HttpCore>,
}

impl Me {
    /// Inspect your tier, daily/burst quota, subscription expiry, and per-feature usage.
    ///
    /// Reads from the same in-memory counters that drive rate-limit enforcement, so
    /// `quota.daily.remaining` is authoritative — no need to parse rate-limit headers.
    /// Available to every authenticated tier (BASIC/PRO/ULTRA).
    pub async fn get(&self) -> Result<MeResponse> {
        self.core.get("/me", &()).await
    }
}
