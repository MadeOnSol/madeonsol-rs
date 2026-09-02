use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Pump.fun deployer intelligence endpoints — leaderboard, alerts, trajectory, bonded tokens.
#[derive(Debug, Clone)]
pub struct Deployer {
    pub(crate) core: Arc<HttpCore>,
}

impl Deployer {
    /// Global statistics across all tracked deployer wallets.
    pub async fn stats(&self) -> Result<DeployerStats> {
        self.core.get("/deployer-hunter/stats", &()).await
    }

    /// Deployers ranked by bonding rate or recent performance.
    pub async fn leaderboard(
        &self,
        params: &DeployerLeaderboardParams,
    ) -> Result<DeployerLeaderboardResponse> {
        self.core
            .get("/deployer-hunter/leaderboard", params)
            .await
    }

    /// Full profile for a single deployer wallet.
    pub async fn profile(&self, wallet: &str) -> Result<DeployerProfile> {
        self.core
            .get(&format!("/deployer-hunter/{}", wallet), &())
            .await
    }

    /// All tokens deployed by a specific wallet.
    pub async fn tokens(
        &self,
        wallet: &str,
        params: &DeployerTokensParams,
    ) -> Result<DeployerTokensResponse> {
        self.core
            .get(
                &format!("/deployer-hunter/{}/tokens", wallet),
                params,
            )
            .await
    }

    /// Real-time deploy alerts — fired when a tracked deployer launches a new token.
    /// `tier` filter is PRO/ULTRA only — BASIC subscribers receive HTTP 403.
    pub async fn alerts(
        &self,
        params: &DeployerAlertsParams,
    ) -> Result<DeployerAlertsResponse> {
        self.core.get("/deployer-hunter/alerts", params).await
    }

    /// Aggregated alert statistics by tier.
    pub async fn alert_stats(
        &self,
        params: &DeployerAlertStatsParams,
    ) -> Result<DeployerAlertStats> {
        self.core
            .get("/deployer-hunter/alert-stats", params)
            .await
    }

    /// Top-performing tokens from tracked deployers by peak market cap.
    pub async fn best_tokens(
        &self,
        params: &BestTokensParams,
    ) -> Result<BestTokensResponse> {
        self.core
            .get("/deployer-hunter/best-tokens", params)
            .await
    }

    /// Most recently bonded tokens from tracked deployers.
    pub async fn recent_bonds(
        &self,
        params: &RecentBondsParams,
    ) -> Result<RecentBondsResponse> {
        self.core
            .get("/deployer-hunter/recent-bonds", params)
            .await
    }

    /// Deployer skill curve — streaks, rolling bond rate, improvement trend.
    /// PRO/ULTRA only.
    ///
    /// Pass `include: Some("daily_snapshots")` for up to 90 daily tier/bonding
    /// snapshots in the response.
    pub async fn trajectory(
        &self,
        wallet: &str,
        params: &DeployerTrajectoryParams,
    ) -> Result<DeployerTrajectoryResponse> {
        self.core
            .get(
                &format!("/deployer-hunter/{}/trajectory", wallet),
                params,
            )
            .await
    }

    /// v0.20 — Daily performance snapshots for a deployer wallet — tier,
    /// tracked status, cumulative deploys/bonds, bonding rates, and peak-MC
    /// history over time. `limit` (1..=365) caps the number of daily snapshots;
    /// an unset value is omitted from the query string. `is_deployer` is `false`
    /// when the wallet has never deployed a token.
    pub async fn history(
        &self,
        wallet: &str,
        limit: Option<u32>,
    ) -> Result<DeployerHistoryResponse> {
        self.core
            .get(
                &format!("/deployer-hunter/{}/history", wallet),
                &DeployerHistoryParams { limit },
            )
            .await
    }

    /// v0.28 — A deployer's reputation exactly as it stood on `date` — the
    /// latest write-on-change snapshot at or before it, so a backtest sees only
    /// what was knowable then. `date` (YYYY-MM-DD, UTC) defaults to today when
    /// `None`; nothing is ever synthesized — `response.as_of` is `false` and
    /// `response.snapshot` is `None` before the deployer's first snapshot.
    pub async fn as_of(
        &self,
        wallet: &str,
        date: Option<&str>,
    ) -> Result<DeployerAsOfResponse> {
        self.core
            .get(
                &format!("/deployer-hunter/{}/as-of", wallet),
                &DeployerAsOfParams {
                    date: date.map(|d| d.to_string()),
                },
            )
            .await
    }

    /// v0.28 — pump.fun creator-fee rewards for a wallet, answered two ways
    /// that are never merged: `collected` (what actually reached the wallet)
    /// and `attributed` (every payout on the tokens it deployed, split
    /// `to_self`/`to_others`). Works for non-deployers too (`is_deployer: false`).
    pub async fn rewards(&self, wallet: &str) -> Result<DeployerRewardsResponse> {
        self.core
            .get(&format!("/deployer-hunter/{}/rewards", wallet), &())
            .await
    }
}
