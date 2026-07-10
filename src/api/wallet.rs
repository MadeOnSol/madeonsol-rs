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
}
