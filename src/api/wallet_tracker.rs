use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Wallet tracker — watch arbitrary Solana wallets for swap and transfer events.
/// Limits: BASIC=10, PRO=50, ULTRA=100. 120-day event retention.
#[derive(Debug, Clone)]
pub struct WalletTracker {
    pub(crate) core: Arc<HttpCore>,
}

impl WalletTracker {
    /// List your tracked wallets and remaining capacity.
    pub async fn watchlist(&self) -> Result<WatchlistResponse> {
        self.core.get("/wallet-tracker/watchlist", &()).await
    }

    /// Add a wallet to your watchlist. Returns HTTP 409 if already tracked or limit reached.
    pub async fn add_to_watchlist(
        &self,
        params: &WatchlistAddParams,
    ) -> Result<WatchlistAddResponse> {
        self.core
            .post_json("/wallet-tracker/watchlist", params)
            .await
    }

    /// Remove a wallet from your watchlist.
    pub async fn remove_from_watchlist(
        &self,
        wallet: &str,
    ) -> Result<WalletTrackerDeleteResponse> {
        self.core
            .delete(&format!("/wallet-tracker/watchlist/{}", wallet))
            .await
    }

    /// Update a wallet's label (or pass `label: None` to clear it).
    pub async fn update_label(
        &self,
        wallet: &str,
        params: &WatchlistUpdateParams,
    ) -> Result<WalletEntry> {
        self.core
            .patch_json(
                &format!("/wallet-tracker/watchlist/{}", wallet),
                params,
            )
            .await
    }

    /// Historical swap and transfer events for your watched wallets.
    /// BASIC: truncated wallets, no `tx_signature`, no `counterparty`.
    pub async fn trades(
        &self,
        params: &WalletTrackerTradesParams,
    ) -> Result<WalletTrackerTradesResponse> {
        self.core.get("/wallet-tracker/trades", params).await
    }

    /// Per-wallet stats across your watchlist.
    pub async fn summary(
        &self,
        params: &WalletTrackerSummaryParams,
    ) -> Result<WalletTrackerSummaryResponse> {
        self.core.get("/wallet-tracker/summary", params).await
    }
}
