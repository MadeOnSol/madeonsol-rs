use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Alpha wallet intelligence — leaderboard, profiles, cap tables, buyer quality scores.
#[derive(Debug, Clone)]
pub struct Alpha {
    pub(crate) core: Arc<HttpCore>,
}

impl Alpha {
    /// Leaderboard of statistically profitable wallets ranked by win rate, PnL, or ROI.
    /// BASIC: 25 results. PRO: 100. ULTRA: 500 + behavioral signals.
    pub async fn leaderboard(
        &self,
        params: &AlphaLeaderboardParams,
    ) -> Result<AlphaLeaderboardResponse> {
        self.core.get("/alpha/leaderboard", params).await
    }

    /// Full alpha profile for a single wallet. **ULTRA only** — BASIC/PRO get HTTP 403.
    pub async fn wallet(&self, wallet: &str) -> Result<AlphaWalletResponse> {
        self.core
            .get(&format!("/alpha/{}", wallet), &())
            .await
    }

    /// Wallets behaviorally linked to this one (co-bought 3+ tokens within 2s).
    /// **ULTRA only**.
    pub async fn linked(&self, wallet: &str) -> Result<AlphaLinkedResponse> {
        self.core
            .get(&format!("/alpha/{}/linked", wallet), &())
            .await
    }

    /// Cap table: first early buyers for a token, enriched with historical metrics.
    /// PRO: top 10 (truncated). ULTRA: top 20 (full wallets).
    pub async fn cap_table(&self, mint: &str) -> Result<AlphaCapTableResponse> {
        self.core
            .get(&format!("/tokens/{}/cap-table", mint), &())
            .await
    }

    /// 0–100 buyer quality score for a token's early-buyer cohort. 5-min cache.
    pub async fn buyer_quality(&self, mint: &str) -> Result<AlphaBuyerQualityResponse> {
        self.core
            .get(&format!("/tokens/{}/buyer-quality", mint), &())
            .await
    }
}
