use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Universal per-wallet endpoints — works on any Solana wallet, not just
/// curated KOLs. FIFO cost-basis PnL over the last 90 days. PRO+ on every
/// method.
///
/// Cached in `wallet_analyses` with dynamic TTL (5min / 1h / 24h based on
/// last activity). Cache hits don't count against your daily quota.
#[derive(Debug, Clone)]
pub struct Wallet {
    pub(crate) core: Arc<HttpCore>,
}

impl Wallet {
    /// Aggregate stats for any wallet over the last 90 days plus cross-product
    /// flags (KOL / alpha / deployer). Sub-100ms even on heavy wallets.
    /// Returns HTTP 404 if the wallet has no trades AND no flag-table presence.
    pub async fn stats(&self, address: &str) -> Result<WalletStatsResponse> {
        self.core
            .get(&format!("/wallet/{}", address), &())
            .await
    }

    /// Full FIFO cost-basis PnL: realized + unrealized SOL, profit factor,
    /// max drawdown, avg + median hold minutes, daily UTC PnL curve, closed
    /// positions sorted by pnl desc, open positions hydrated with live prices
    /// from the market-cap tracker.
    ///
    /// **Cost-basis honesty**: observable only inside the 90-day data window.
    /// Overflow sells (no matching buy in window) are silently discarded
    /// rather than fabricated. `notes.cost_basis_observable_from` makes the
    /// cutoff visible per call.
    pub async fn pnl(&self, address: &str) -> Result<WalletPnlResponse> {
        self.core
            .get(&format!("/wallet/{}/pnl", address), &())
            .await
    }

    /// Open positions only — lighter slice of `pnl()` for UIs that don't need
    /// the full summary or curve. Shares the `wallet_analyses` cache:
    /// calling this right after `pnl()` is an immediate cache hit.
    pub async fn positions(&self, address: &str) -> Result<WalletPositionsResponse> {
        self.core
            .get(&format!("/wallet/{}/positions", address), &())
            .await
    }

    /// Verified CURRENT on-chain holdings — reads the wallet's actual SPL +
    /// Token-2022 token accounts and SOL balance from chain, enriches each with
    /// our price / MC / name / symbol data, and computes `transfer_delta`
    /// (on-chain amount minus trade-derived net position) to expose non-swap
    /// flows: airdrops, insider funding, wallet-hopping.
    ///
    /// Distinct from [`positions`](Self::positions), which is trade-derived
    /// FIFO — holdings is "what they actually hold right now". `limit` 1–500
    /// (default 200), `min_value_usd` ≥ 0 (default 0). ULTRA only.
    pub async fn holdings(
        &self,
        address: &str,
        params: &WalletHoldingsParams,
    ) -> Result<WalletHoldingsResponse> {
        self.core
            .get(&format!("/wallet/{}/holdings", address), params)
            .await
    }

    /// Cursor-paginated raw trades. Default window is the last 90 days;
    /// override via `since` / `until` (Unix epoch seconds). Default limit
    /// 100, max 500.
    ///
    /// Cursor is base64 of `block_time:id` from the previous response's
    /// `next_cursor`. Stable across DESC pagination.
    pub async fn trades(
        &self,
        address: &str,
        params: &WalletTradesParams,
    ) -> Result<WalletTradesResponse> {
        self.core
            .get(&format!("/wallet/{}/trades", address), params)
            .await
    }

    /// v0.22 — Bulk wallet reputation flags for 1–100 wallets in one call
    /// (counts as 1 request against quota). PRO/ULTRA.
    ///
    /// Each [`WalletClassification`] carries `is_sniper` / `is_bundler` /
    /// `is_dumper` / `is_kol` (+ `kol_name`), `bot_confidence` (text enum
    /// `"none"`/`"low"`/`"medium"`/`"high"`, `None` when not alpha-tracked)
    /// and a `dump_cluster` cohort block — identical semantics to the `flags`
    /// block of [`stats`](Self::stats).
    ///
    /// Scope caveat: the reputation flags derive from the pump.fun trade
    /// pipeline — `false` means "not observed", NOT "verified clean".
    /// `is_bundler` is a lifetime flag; `is_dumper` uses a rolling 42-day
    /// window (recomputed daily, up to ~48h stale).
    pub async fn batch_classify(
        &self,
        wallets: Vec<String>,
    ) -> Result<WalletBatchClassifyResponse> {
        self.core
            .post_json("/wallet/batch/classify", &WalletBatchRequest { wallets })
            .await
    }
    /// Point-in-time reputation flags (`GET /wallet/{address}/flags`): every
    /// flag source's state at `as_of` (default now), optionally with the raw
    /// snapshot history. PRO+.
    pub async fn flags(
        &self,
        address: &str,
        params: &WalletFlagsParams,
    ) -> Result<WalletFlagsResponse> {
        self.core
            .get(&format!("/wallet/{}/flags", address), params)
            .await
    }

    /// Shared-funder evidence for a tracked wallet
    /// (`GET /wallet/{address}/funding`), forward-looking from monitoring start.
    /// PRO+; cross-wallet relationship counts inside `direct_funding` are
    /// ULTRA+. Evidence of a funding connection, not proof of common ownership.
    pub async fn funding(
        &self,
        address: &str,
        params: &WalletFundingParams,
    ) -> Result<WalletFundingResponse> {
        self.core
            .get(&format!("/wallet/{}/funding", address), params)
            .await
    }

    /// Recent trades for 1–50 wallets in one call (`POST /wallet/batch/trades`),
    /// newest first per wallet; page forward with `next_since`. PRO+.
    pub async fn batch_trades(
        &self,
        params: &WalletBatchTradesParams,
    ) -> Result<WalletBatchTradesResponse> {
        self.core.post_json("/wallet/batch/trades", params).await
    }

    /// Score a list of up to 200 wallets (`POST /wallet-list/score`): 0–100
    /// score from win rate + profit factor, PnL summary and reputation flags.
    /// At most 25 uncached wallets are computed live per call. ENTERPRISE.
    pub async fn list_score(
        &self,
        params: &WalletListScoreParams,
    ) -> Result<WalletListScoreResponse> {
        self.core.post_json("/wallet-list/score", params).await
    }
}
