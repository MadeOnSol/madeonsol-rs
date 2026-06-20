use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Signal Scorecard endpoints — out-of-sample, machine-readable reliability for
/// each MadeOnSol enrichment signal so bots can weight them programmatically.
#[derive(Debug, Clone)]
pub struct Signals {
    pub(crate) core: Arc<HttpCore>,
}

impl Signals {
    /// v0.16 — Live, out-of-sample backtest for a named signal: per-bucket hit
    /// rate vs base rate, lift, sample size, and test window. Open to any
    /// authenticated tier — it's aggregate, non-sensitive efficacy data.
    ///
    /// Valid `name` values: `dump_cluster_count`, `runner_rate`,
    /// `recycled_early_buyer_count`, `coordination_count`.
    ///
    /// Set [`SignalPerformanceParams::history`] to `Some(true)` to append the
    /// per-day snapshot series for drift inspection.
    pub async fn performance(
        &self,
        name: &str,
        params: &SignalPerformanceParams,
    ) -> Result<SignalPerformance> {
        self.core
            .get(&format!("/signals/{}/performance", name), params)
            .await
    }

    /// v0.16 — Discovery index for the Signal Scorecard: every available signal
    /// with its methodology and the `performance_endpoint` to fetch its live
    /// efficacy. Open to any authenticated tier.
    pub async fn catalog(&self) -> Result<SignalsCatalog> {
        self.core.get("/signals", &()).await
    }
}
