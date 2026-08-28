use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Token intelligence endpoints — comprehensive per-mint snapshot and batch lookups.
#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) core: Arc<HttpCore>,
}

impl Token {
    /// Comprehensive per-mint snapshot: price (VWAP), market cap, 24h volume,
    /// deployer reputation, KOL smart-money activity, first_seen_at / age_seconds,
    /// and blacklist status — all in one call.
    ///
    /// **ULTRA** adds individual KOL wallet addresses in `kol_activity.top_buyers[].wallet`.
    pub async fn get(&self, mint: &str) -> Result<TokenResponse> {
        self.core.get(&format!("/token/{}", mint), &()).await
    }

    /// Batch lookup of up to 50 mints. Returns the same per-mint shape as `get()`
    /// in a single round-trip — DB queries batched with `IN(...)`, dex-stream and
    /// RPC fan-outs run in parallel. Roughly 10-20× cheaper than N sequential calls.
    pub async fn batch(&self, mints: Vec<String>) -> Result<TokenBatchResponse> {
        self.core
            .post_json("/token/batch", &MintBatchRequest { mints })
            .await
    }

    /// Batch buyer-quality scoring for up to 50 mints. Shares the same 5-minute
    /// LRU cache as `alpha::buyer_quality(mint)` — already-warm mints return at
    /// near-zero cost. Response includes a `cache_hits` counter.
    pub async fn batch_buyer_quality(
        &self,
        mints: Vec<String>,
    ) -> Result<AlphaBuyerQualityBatchResponse> {
        self.core
            .post_json("/tokens/batch/buyer-quality", &MintBatchRequest { mints })
            .await
    }

    /// v0.19 — Batch rug-risk scoring for up to 50 mints in one round-trip
    /// (PRO/ULTRA). Each tracked mint returns the same transparent
    /// per-factor breakdown as [`risk`](Self::risk), plus an `as_of` timestamp.
    /// Untracked mints come back as error entries (`error =
    /// Some("not_tracked")`) instead of failing the batch — check
    /// [`BatchRiskResult::is_error`]. `tokens` preserves de-duplicated input
    /// order; `count` is the number of unique mints.
    pub async fn batch_risk(&self, mints: Vec<String>) -> Result<BatchRiskResponse> {
        self.core
            .post_json("/tokens/batch/risk", &MintBatchRequest { mints })
            .await
    }

    /// v1.9 — KOL consensus on a token: how many KOLs bought/sold, exit rate,
    /// net flow, median entry MC. ULTRA gets individual wallet arrays.
    pub async fn kol_consensus(&self, mint: &str) -> Result<KolConsensusResponse> {
        self.core
            .get(&format!("/tokens/{}/kol-consensus", mint), &())
            .await
    }

    /// v1.9 — Peak MC history for a token: ATH, decline from peak, MC at bond
    /// and at 1h/6h/24h/7d after bond.
    pub async fn peak_history(&self, mint: &str) -> Result<PeakHistoryResponse> {
        self.core
            .get(&format!("/tokens/{}/peak-history", mint), &())
            .await
    }

    /// v0.14 — Transparent 0–100 token rug-risk / safety score (PRO/ULTRA).
    /// Higher means riskier. Returns the overall `risk_score` and `band`
    /// alongside a per-factor breakdown (mint/freeze authority, liquidity,
    /// transfer fee, launch cohort, deployer reputation, blacklist, …) and the
    /// raw `inputs` each factor was derived from — nothing is opaque.
    ///
    /// v0.23 adds the top-level `dev` block ([`RiskDev`], `None` when the mint
    /// has no tracked deploy row): dev buy at create, post-create buys/sells,
    /// live on-chain `holdings_tokens` / `wallet_empty`, and a coverage-gated
    /// `transferred_out` flag.
    pub async fn risk(&self, mint: &str) -> Result<TokenRisk> {
        self.core
            .get(&format!("/tokens/{}/risk", mint), &())
            .await
    }

    /// v0.23 — Per-pool price-impact / slippage depth (PRO+). Answers "how
    /// much SOL to move the price N%" and the impact of each buy size, per
    /// pool. Exact for constant-product AMMs (served from streamed reserves,
    /// zero-RPC) and correct for pump.fun / bonk curves via a live read of the
    /// curve's virtual reserves. Concentrated pools (CLMM/Orca/DLMM),
    /// Meteora-DBC curves, and unclassified pools come back in
    /// `unsupported_pools` with a `reason` rather than a wrong number.
    ///
    /// Use [`DepthParams`] to pick the SOL buy sizes (`?sizes=` CSV, max 8,
    /// each `> 0` and `<= 10000`; default `0.5,1,5,10`) —
    /// [`DepthParams::from_sizes`] builds the CSV from numbers. Each
    /// [`DepthPool`] carries `spot_price_sol`, per-size [`DepthQuote`]s
    /// (`tokens_out`, `avg_price_sol`, `price_impact_pct`), and
    /// [`DepthToMovePrice`] (SOL to move the price 1/5/10%).
    pub async fn depth(&self, mint: &str, params: &DepthParams) -> Result<TokenDepthResponse> {
        self.core
            .get(&format!("/tokens/{}/depth", mint), params)
            .await
    }

    /// v0.20 — Bundle intelligence for a token (PRO/ULTRA): detects wallets
    /// that bought in the same atomic transaction (`bundle_kind = atomic_tx`) or
    /// the same slot (`same_slot`), how much of supply the cohort still holds,
    /// and whether it has `fully_exited`. Returns a [`BundleSummary`]
    /// (`wallet_count`, `bundle_kind`, `held_ratio`, `buy_volume`, …) plus a
    /// per-wallet [`BundleWallet`] breakdown.
    ///
    /// **ULTRA** populates the per-wallet identity fields (`is_kol`, `kol_name`,
    /// `win_rate`, `bot_confidence`); on lower tiers `wallets` may be empty or
    /// those fields `None`.
    pub async fn bundle(&self, mint: &str) -> Result<TokenBundle> {
        self.core
            .get(&format!("/tokens/{}/bundle", mint), &())
            .await
    }

    /// v0.20 — All liquidity pools for a token across every tracked DEX, plus an
    /// aggregate [`PoolsSummary`] (`pool_count`, `active_pool_count`, `dex_count`,
    /// `dexes`, `total_liquidity_usd`, `primary_pool`, `primary_dex`,
    /// `top_pool_share_pct`). Each [`Pool`] carries its `pool_address`, `dex`,
    /// `quote_mint`, `liquidity_usd`, `last_price_sol`, `last_swap_at`, `amm_id`,
    /// and an `is_active` flag.
    pub async fn pools(&self, mint: &str) -> Result<TokenPoolsResponse> {
        self.core
            .get(&format!("/tokens/{}/pools", mint), &())
            .await
    }

    /// v0.25 — Live holders, holder count + concentration for a Solana mint
    /// (`GET /tokens/{mint}/holders`, PRO+): a full holder census read from
    /// the ledger at `confirmed` (every token account of the mint, owner +
    /// balance, merged per owner) — who holds NOW, as opposed to
    /// [`Alpha::cap_table`](crate::api::alpha::Alpha::cap_table) (who bought
    /// first).
    ///
    /// Hard truths the payload states rather than hides:
    /// - `amount_raw` on every [`TokenHolder`] and [`TokenHoldersExcluded`] is
    ///   a raw u64 **string** — never a float; parse it yourself. `amount` is
    ///   the UI-scaled convenience number.
    /// - [`TokenHoldersConcentration::holder_count`] is EXACT (distinct
    ///   non-zero owners minus excluded pools/curves/burns) and `None` ONLY
    ///   when the provider refused the census for a mega-cap mint — then
    ///   `source.method` is [`HoldersMethod::GetTokenLargestAccounts`] (top-20
    ///   view) and `source.census_fallback_reason` is set. It is never
    ///   estimated from trades.
    /// - Pools, bonding curves, burns and unattributed program accounts are
    ///   EXCLUDED from the circulating denominator and listed in `excluded`,
    ///   each NAMED where we can ([`HolderExcludedReason::Pool`] + `dex` +
    ///   `pool_address`, `BondingCurve` for pump.fun/LaunchLab, `Burn`, else
    ///   `ProgramAccount`). The #1 raw account of a fresh memecoin is its own
    ///   bonding curve.
    /// - Disclosure is tier-gated (**PRO** ranks 1–10, **ULTRA** 1–50,
    ///   **BUSINESS** 1–100); the concentration maths is tier-independent.
    /// - `labels` are from MadeOnSol data — empty means unknown to us, not clean.
    ///
    /// Latency: fresh pump.fun mints <1 s; 200k–550k-account tokens 6–11 s.
    /// While the upstream scan is still running the API answers **503** with
    /// `error_kind: "holder_scan_in_progress"` and `retry_after_seconds: 20` —
    /// the scan continues and is cached, so the retry is instant. 503
    /// `holder_rpc_unavailable` (`retry_after_seconds: 15`) is a fail-closed
    /// RPC outage. Both arrive as [`Error::Api`](crate::Error::Api)
    /// with `status == 503`; read `error_kind` / `retry_after_seconds` from
    /// the `body`. Unknown mint: 404 `error_kind: "not_a_mint"`.
    pub async fn holders(&self, mint: &str) -> Result<TokenHoldersResponse> {
        self.core
            .get(&format!("/tokens/{}/holders", mint), &())
            .await
    }

    /// v0.26 — Token locks & vesting on a mint (`GET /tokens/{mint}/locks`,
    /// PRO+): every on-chain Streamflow stream, Jupiter Lock `VestingEscrow`
    /// and Bonfida token-vesting contract, decoded from the locker programs'
    /// account state — *did the team lock, how much, until when, and can they
    /// pull it*.
    ///
    /// Each [`TokenLock`] carries the schedule (`start_at` / `cliff_at` /
    /// `end_at` / `period_seconds`, `cliff_amount_raw`, `amount_per_period_raw`),
    /// the terms (`cancelable_by_sender` — funds are locked against the
    /// recipient, not the locker, so a cancelable lock is a weaker promise —
    /// `cancelable_by_recipient`, `transferable`, `can_topup`) and a
    /// live-derived view computed at request time: `locked_raw` (still locked
    /// now), `unlocked_raw`, `withdrawn_raw`, `claimable_raw`, [`LockStatus`]
    /// and `next_unlock`. [`TokenLocksSummary`] rolls up the exact
    /// `lock_count`, `distinct_lockers`, locked / deposited totals (raw + ui +
    /// usd + % of supply), the `unlocking_7d_*` / `unlocking_30d_*` forward
    /// schedule, the nearest `next_unlock` and `active_cancelable_by_sender`.
    ///
    /// Every `*_raw` amount is base units as a **`String`** — never a float;
    /// the ui / `*_usd` / `*_pct_of_supply` companions are `None` when
    /// decimals or price are unknown (`token.facts_resolved`). Use
    /// [`TokenLocksParams`] to filter the list by [`LockStatus`] /
    /// [`LockProgram`] (the summary always covers all rows) and cap `limit`
    /// (1–500, default 200). **LP locks are NOT included** — token / vesting
    /// locks only. Keyed API only (not on the x402 rail); BASIC gets 403.
    pub async fn locks(
        &self,
        mint: &str,
        params: &TokenLocksParams,
    ) -> Result<TokenLocksResponse> {
        self.core
            .get(&format!("/tokens/{}/locks", mint), params)
            .await
    }

    /// v0.26 — Cross-token feed of NEW lock / vesting contracts
    /// (`GET /tokens/locks`, PRO+), newest first — who just locked tokens, of
    /// what mint, how much, until when. Same [`TokenLock`] rows as
    /// [`Token::locks`] plus a per-row [`TokenLock::token`] block (symbol,
    /// decimals, price, market cap).
    ///
    /// Poll forward by passing `pagination.next_since` back as
    /// [`TokenLocksFeedParams::since`], page back with `next_before` →
    /// `before`, or subscribe to the **`token:locks`** WebSocket channel
    /// (event `token:lock`, payload [`TokenLockEvent`]) — the response carries
    /// a [`StreamPointer`]. `status` / `min_usd` / `min_pct_of_supply`
    /// post-filter with a ×4 over-fetch, so a page may be shorter than
    /// `limit`. Backfilled Jupiter Lock rows have no on-chain creation time
    /// (`created_at_estimated`) and are excluded unless
    /// `include_estimated: Some(true)`. LP locks are not included. Keyed API only.
    pub async fn locks_feed(
        &self,
        params: &TokenLocksFeedParams,
    ) -> Result<TokenLocksFeedResponse> {
        self.core.get("/tokens/locks", params).await
    }

    /// v0.26 — Upcoming unlock EVENTS across all active lock / vesting
    /// contracts inside a window (`GET /tokens/unlocks`, PRO+) — which locked
    /// supply hits the market this week, how much, from whose lock.
    ///
    /// One [`TokenUnlock`] per active contract = its NEXT unlock event in the
    /// window ([`UnlockEventKind`]: cliff / period / final / tranche) with
    /// `unlock_at`, `in_seconds`, `amount_*`, plus `window_amount_*` = that
    /// contract's total release over the whole window, the token block and
    /// the [`UnlockLockRef`] it belongs to. Continuous per-second streams
    /// (Streamflow payroll) contribute only their cliff / final events.
    /// [`TokenUnlocksParams`]: [`UnlockWindow`] (`1h` … `90d`, default `7d`),
    /// `mint` / `program` / `kind` / `min_usd` / `min_pct_of_supply`,
    /// [`UnlocksSort`] (`soonest` default, `largest_usd`, `largest_pct`),
    /// `limit` 1–200. Base-unit amounts are `String`s; ui / usd / pct `None`
    /// when unknown. LP locks not included. Keyed API only.
    pub async fn unlocks(&self, params: &TokenUnlocksParams) -> Result<TokenUnlocksResponse> {
        self.core.get("/tokens/unlocks", params).await
    }

    /// v0.26 — pump.fun creator-fee sharing on a mint
    /// (`GET /tokens/{mint}/fee-shares`, PRO+): who receives what share of
    /// the coin's creator fees.
    ///
    /// [`TokenFeeSharesResponse::config`] is the on-chain `SharingConfig`
    /// ([`FeeSharingConfig`]): `admin`, `status`, `is_default` (`Some(true)`
    /// = 100% to the creator — a real answer, not a miss), `redirected_bps`
    /// (share going to non-admin addresses), `social_bps`, and each
    /// [`FeeShareholder`]'s `share_bps`, `is_admin`, `is_social_pda` (fees
    /// earmarked for a platform identity — [`FeeShareSocial::platform`] 2 = X,
    /// `user_id` is the platform-native numeric id, not the handle) and what
    /// it has `received`. `config.source` is [`FeeConfigSource::Stream`] (our
    /// table — only non-default splits are stored) or `Chain` (live PDA read;
    /// `config` is `None` and `config_error` set only when every RPC endpoint
    /// failed). [`FeeDistributions`] rolls up every `distribute_creator_fees`
    /// payout (per-recipient received, `past_recipients` no longer in the
    /// split); `history` is the config change log and `recent_distributions`
    /// the latest payouts. Amounts are quote base units as `String`s (SOL
    /// lamports unless a stable-quoted coin); ui / usd may be `None`.
    /// **Event history starts 2026-08-17.** Keyed API only.
    pub async fn fee_shares(&self, mint: &str) -> Result<TokenFeeSharesResponse> {
        self.core
            .get(&format!("/tokens/{}/fee-shares", mint), &())
            .await
    }

    /// v0.26 — pump.fun fee-event feed (`GET /tokens/fee-claims`, PRO+),
    /// newest first: [`FeeEventType::Distribution`] (creator fees paid
    /// pro-rata to the SharingConfig shareholders, with `payouts` per address),
    /// `SocialClaim` (fees earmarked for a platform identity — platform 2 = X
    /// — claimed to a `recipient` wallet), `SharesCreated` / `SharesUpdated` /
    /// `SharesReset`, `CreatorTransferred`, and `CreatorClaim` (the plain
    /// creator vault claim — per creator, no `mint`; EXCLUDED unless requested
    /// via [`TokenFeeClaimsParams::event_type`]).
    ///
    /// Default 100%-to-creator configs and zero-amount distributions are not
    /// stored. Poll forward with `pagination.next_since` → `since`, or
    /// subscribe to the **`token:fee_claims`** WebSocket channel (event
    /// `token:fee_claim`, payload [`TokenFeeClaimEvent`] — the writer's raw
    /// row, not this enriched shape). `amount_raw` is quote base units as a
    /// `String`; `amount` / `amount_usd` may be `None`. **History starts
    /// 2026-08-17.** Keyed API only.
    pub async fn fee_claims(
        &self,
        params: &TokenFeeClaimsParams,
    ) -> Result<TokenFeeClaimsResponse> {
        self.core.get("/tokens/fee-claims", params).await
    }

    /// v0.27 — Token momentum fires, newest first (`GET /tokens/surges`, PRO+).
    ///
    /// Two [`SurgeKind`]s. **`Surge`** — a token < 30 min old whose market cap
    /// runs hard vs its LAUNCH MC: [`SurgeTier::Early`] (≤ 10 min, ≥ $12k,
    /// ≥ 3× launch MC), `Strong` (≤ 30 min, ≥ $30k, ≥ 6× launch AND ≥ 2× the
    /// lowest sample of the last 3 min — it is climbing now), `Breakout`
    /// (≤ 2 min, ≥ $45k, ≥ 8×). Each tier fires at most once per mint and
    /// must be SUSTAINED (floor + multiple hold on the current tick and on a
    /// sample ≥ 10 s older; nothing fires before 20 s of age — a one-tick
    /// mark is a spike, not a surge). **`Revival`** — a token with no
    /// 1-minute trade candle for ≥ 24 h that started trading again,
    /// CONFIRMED by the tape (≥ 5 buys, ≥ $500 buy volume, MC ≥ 1.5× the
    /// pre-dormancy close — or ≥ 20 buys / ≥ $5k regardless), never by the
    /// price mark alone. Hard gates on both: liquidity ≥ $1.5k and ≥ 2% of
    /// MC when known, MC ≤ $100B, and the MC gained must be PAID FOR (buy
    /// volume ≥ 3% of the move — a spoof-pool mark moves MC on ~$0).
    ///
    /// Every [`TokenSurgeEvent`] carries [`SurgeTape`] (buys / sells / volume
    /// since birth or revival; `unique_buyers` only when the mint is in
    /// wallet-trade coverage — `wallet_data_available: false` otherwise,
    /// never an inferred zero), [`SurgeKol`], [`SurgeEarlyBuyers`] (first-20
    /// cohort: bundled / sold / sniper wallets), [`SurgeDeployer`] and
    /// `risk_flags` ([`SurgeRiskFlag`]) — the honest half. Rows ≥ 65 min old
    /// also carry [`SurgeOutcome`] (`priced_after_1h: false` = no candle in
    /// the hour, not zero) and `stats: Some(true)` returns per-(kind, tier)
    /// hit-rates ([`SurgeStats`]) over `days` — out-of-sample by
    /// construction. The live thresholds are echoed in `definitions`. Poll
    /// forward with `pagination.next_since` → [`TokenSurgesParams::since`],
    /// or subscribe to the **`token:surges`** WebSocket channel (events
    /// `token:surge` / `token:revival`, payload [`TokenSurgeStreamEvent`];
    /// subscribe filters `kinds[]`, `tiers[]`, `launchpads[]`,
    /// `exclude_flags[]`, `min_mc_usd` / `max_mc_usd`, `deployer_tier[]` —
    /// [`SurgeSubscribeFilters`]). Nearly every scalar is an `Option` —
    /// `None` means unknown, never zero. Keyed API only; BASIC gets 403.
    pub async fn surges(&self, params: &TokenSurgesParams) -> Result<TokenSurgesResponse> {
        self.core.get("/tokens/surges", params).await
    }

    /// v0.15 — 1-minute OHLC candles for a token, aggregated from the trade
    /// firehose. Returns open/high/low/close, USD volume, trade count, and
    /// market cap per bar. ULTRA unlocks buy/sell volume split, net flow,
    /// liquidity, MC high/low, and MEV volume per candle.
    ///
    /// Use [`CandlesParams`] to pick the timeframe (`tf`), `limit`, and an
    /// optional `from`/`to` time window — unset params are omitted from the
    /// query string.
    pub async fn candles(&self, mint: &str, params: &CandlesParams) -> Result<CandlesResponse> {
        self.core
            .get(&format!("/tokens/{}/candles", mint), params)
            .await
    }

    /// v0.22 — Mint-scoped trade tape: cursor-paginated raw trades for one
    /// token, newest first (PRO/ULTRA) — the backfill/history complement to
    /// the live DEX firehose stream.
    ///
    /// Each [`TokenTrade`] carries `tx_signature`, `wallet_address`, `action`,
    /// `sol_amount`, `token_amount`, `price_sol`/`price_usd`,
    /// `early_buyer_rank`, `slot`, `block_time`, and `traded_at`. Filter via
    /// [`TokenTradesParams`] (`action`, `wallet`, `since`/`until` — default is
    /// FULL history, not 90 days). Pass `next_cursor` from the previous
    /// response to page older trades; `has_more` tells you when to stop.
    ///
    /// Coverage honesty: capture starts 2026-04-12 and is pump.fun-pipeline
    /// scoped — the response's [`TokenTradesCoverage`] block
    /// (`history_start`, `scope`) makes both limits machine-readable.
    pub async fn trades(
        &self,
        mint: &str,
        params: &TokenTradesParams,
    ) -> Result<TokenTradesResponse> {
        self.core
            .get(&format!("/tokens/{}/trades", mint), params)
            .await
    }

    /// v0.16 — Aggregated buy/sell flow for a token over a rolling window (PRO+).
    /// Returns unique wallet/buyer/seller counts, buy/sell counts and SOL volumes,
    /// `net_sol` (`buy_sol` − `sell_sol`), and `trades_per_wallet`.
    ///
    /// Use [`TokenFlowParams`] to pick the `window` (`"1h"` default or `"24h"`) —
    /// an unset param is omitted from the query string.
    pub async fn token_flow(
        &self,
        mint: &str,
        params: &TokenFlowParams,
    ) -> Result<TokenFlowResponse> {
        self.core
            .get(&format!("/tokens/{}/flow", mint), params)
            .await
    }

    /// v0.8 — Filtered, sortable token directory (PRO+). Default `min_liq=2000`
    /// trims the long tail of phantom-MC tokens (low-liq pools producing absurd
    /// VWAP × supply products); set `Some(0.0)` to opt out. Computed filters
    /// (`min_volume_1h_usd`, `max_mev_share_pct`, `mc_change_1h_*`) over-fetch
    /// 3× from the DB and filter in app — pagination page size may be smaller
    /// than `limit` when those are set. Check `pagination.post_filtered` to
    /// detect.
    ///
    /// `sort` accepts (among others) the momentum values `"mc_change_5m_desc"`,
    /// `"mc_change_1h_desc"`, `"volume_1h_desc"`, and `"trending"`.
    pub async fn list(&self, params: &TokensListParams) -> Result<TokensListResponse> {
        self.core.get("/tokens", params).await
    }

    /// v0.18 — Pre-bond pump.fun tokens near graduation, ranked by velocity
    /// (PRO/ULTRA). Surfaces tokens climbing their bonding curve, with
    /// `progress_pct`, `velocity_pct_per_min`, `eta_minutes`, and a `stalled`
    /// flag for momentum that has stopped.
    ///
    /// Use [`AlmostBondedParams`] to filter by progress band, velocity floor,
    /// age, deployer tier, authority-revoked status, and liquidity, and to pick
    /// the [`AlmostBondedSort`] order (`velocity_desc` default) — unset params
    /// are omitted from the query string.
    pub async fn almost_bonded(
        &self,
        params: &AlmostBondedParams,
    ) -> Result<AlmostBondedResponse> {
        self.core.get("/tokens/almost-bonded", params).await
    }
}
