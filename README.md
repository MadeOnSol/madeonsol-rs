# madeonsol

[![Crates.io](https://img.shields.io/crates/v/madeonsol?style=flat-square)](https://crates.io/crates/madeonsol)
[![docs.rs](https://img.shields.io/docsrs/madeonsol?style=flat-square)](https://docs.rs/madeonsol)
[![Crates.io downloads](https://img.shields.io/crates/d/madeonsol?style=flat-square)](https://crates.io/crates/madeonsol)
[![GitHub stars](https://img.shields.io/github/stars/madeonsol/madeonsol-rs?style=flat-square&logo=github)](https://github.com/madeonsol/madeonsol-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

> ⭐ **[Star on GitHub](https://github.com/madeonsol/madeonsol-rs)** · 📂 **[Examples](./examples/)** · 📚 **[docs.rs](https://docs.rs/madeonsol)** · 🌐 **[API docs](https://madeonsol.com/api-docs)**

Official Rust SDK for the **[MadeOnSol](https://madeonsol.com) Solana API** — typed,
async, `tokio`-based, `rustls`-only.

> Real-time Solana trading intelligence: track 1,000+ KOL wallets with <3s latency,
> score 6,700+ Pump.fun deployers by reputation, detect multi-KOL coordination
> signals, push every pump.fun graduation the second it bonds, verify any wallet's
> current on-chain holdings, and stream every DEX trade across 9+ programs.
>
> **Free tier: 200 requests/day, every endpoint — no signup payment. Get a key at <https://madeonsol.com/pricing>.**
>
> **This is the keyed REST SDK** — authenticate with an API key (`msk_…`). It covers the full endpoint surface (KOL intelligence, deployer intel, token risk/buyer-quality/bundle, Signal Scorecard, wallet PnL, DEX firehose). Want **x402 pay-per-call** instead — no signup, your agent's wallet pays per request in USDC? Use the TypeScript [`madeonsol-x402`](https://www.npmjs.com/package/madeonsol-x402) or Python [`madeonsol-x402`](https://pypi.org/project/madeonsol-x402/) clients.

> **New in 0.26.1 — fix: stream tokens never expire, and `StreamToken` deserializes again.** Since 2026-08-27 `POST /stream/token` returns the SAME token on every call and it never expires — it stops working only if your subscription lapses or you replace it with `{"rotate": true}` (the old value keeps working for 60 s). The API now sends `expires_at: null` and `next_refresh_at: null`, which 0.26.0's `StreamToken { expires_at: String }` refused to deserialize, so `client.stream.get_token()` errored for every caller. **`StreamToken.expires_at` is now `Option<String>`** (always `None`; kept for wire compatibility — do not schedule refreshes on it), `next_refresh_at` stays `Option<String>` (always `None`), and two fields are new: `rotated: Option<bool>` (`Some(true)` when the call replaced an existing token) and `lifetime: Option<String>` (the server's plain-English statement of the above). New method **`client.stream.rotate_token()`** sends `{"rotate": true}` for the leaked-token case. A WebSocket close code `4001` means "call `get_token()` again and reconnect", never "the token timed out".

> **New in 0.26.0 — token locks & vesting, upcoming unlocks, and pump.fun creator-fee sharing.** Five keyed (PRO+) methods on `client.token` + two WebSocket channels. `token.locks(mint, &TokenLocksParams)` (`GET /tokens/{mint}/locks`, typed `TokenLocksResponse`) — every on-chain Streamflow / Jupiter Lock / Bonfida vesting contract on a mint with the schedule (start / cliff / period / end), the terms (`cancelable_by_sender` — a cancelable lock is a weaker promise — `cancelable_by_recipient`, `transferable`, `can_topup`) and a live-derived view (`locked_raw` now, `unlocked`, `withdrawn`, `claimable`, `LockStatus`, `next_unlock`), plus a `TokenLocksSummary` (exact `lock_count`, `distinct_lockers`, locked / deposited raw + ui + usd + % of supply, `unlocking_7d_*` / `unlocking_30d_*`, nearest `next_unlock`, `active_cancelable_by_sender`). `token.locks_feed(&TokenLocksFeedParams)` (`GET /tokens/locks`) — cross-token feed of NEW contracts, newest first, `since` / `before` cursors from `pagination.next_since` / `next_before`. `token.unlocks(&TokenUnlocksParams)` (`GET /tokens/unlocks`) — upcoming unlock EVENTS (`UnlockEventKind`: cliff / period / final / tranche) inside `UnlockWindow` `1h`…`90d` with `amount_*` and `window_amount_*`, sorted by `UnlocksSort`. **LP locks are NOT included** in any of the three. `token.fee_shares(mint)` (`GET /tokens/{mint}/fee-shares`, typed `TokenFeeSharesResponse`) — the pump.fun `SharingConfig`: who receives what share (bps) of a coin's creator fees, `is_admin` / `is_social_pda` (fees earmarked for an X account etc. — `FeeShareSocial::platform` 2 = X, `user_id` = the platform-native numeric id), `redirected_bps`, `social_bps`, `is_default: Some(true)` = 100% to the creator, plus the `FeeDistributions` rollup and config `history`. `token.fee_claims(&TokenFeeClaimsParams)` (`GET /tokens/fee-claims`) — the fee-event feed (`FeeEventType`: `Distribution` with `payouts`, `SocialClaim`, `SharesCreated` / `SharesUpdated` / `SharesReset`, `CreatorTransferred`; `CreatorClaim` only when asked via `event_type`). **Fee history starts 2026-08-17.** Every base-unit amount (`*_raw`) is a **`String`**; ui / usd / pct companions are `Option` and `None` when decimals or price are unknown. Streams: `token:locks` (event `token:lock`, payload `TokenLockEvent`, one frame per NEW contract) and `token:fee_claims` (event `token:fee_claim`, payload `TokenFeeClaimEvent`). **Keyed (`msk_`) API only — none of these are on the x402 rail; BASIC gets HTTP 403.**

> **New in 0.25.0 — live holder census: exact holder count, labelled holders, and pools that are named, not just excluded.** `client.token().holders(mint)` (typed `TokenHoldersResponse`) binds `GET /tokens/{mint}/holders` (PRO+): every token account of the mint read from the ledger at `confirmed` and merged per owner, so `concentration.holder_count` is EXACT (distinct non-zero owners minus pools / bonding curves / burns) — never a trade-derived estimate; it is `null` only when the provider refuses the census for a mega-cap, in which case you get the top-20 view and `source.census_fallback_reason` says so. Each disclosed owner carries our labels (`deployer` / `kol` / `early_buyer` / `bundle` / `bot` / `dump_cluster` — empty means unknown to us, not clean), and `excluded[]` NAMES what was taken out of the circulating denominator: `reason` = `pool` (with `dex` + `pool_address`), `bonding_curve` (pump.fun / LaunchLab), `burn`, or `program_account` only when we genuinely cannot attribute the PDA; `pool_pct` / `burned_pct` / `program_pct` split the exclusion. Amounts are raw u64 **strings**. Disclosure: PRO ranks 1–10, ULTRA 1–50, BUSINESS 1–100 — the maths is tier-independent. Big tokens take 5–30 s upstream: you get `503 holder_scan_in_progress` with `retry_after_seconds: 20` while the scan finishes into the cache, and the retry is instant.

> **New in 0.24.0 — two prices on the trade tape, and the right one is now the default.** The trade tape now tells you what a trade actually cost. `price_sol`/`price_usd` on each trade are THIS trade's executed price — `sol_amount / token_amount`, reconciling exactly with the amounts on the same row and with the PnL endpoints. Because `sol_amount` is the wallet's net SOL movement, that is the trader's all-in effective rate: swap fee and any account rent included, not the pool mid. The market-cap tracker's canonical pool price moved to the new **`market_price_sol`/`market_price_usd`** fields — sampled once per token per pool update, so every trade in the same slot shares it. Until now `price_sol` carried that canonical value and disagreed with the row's own amounts by a **7.9% median** (p90 ~74%): a stale market price reads low in a pump and high in a dump, so anything you averaged out of the tape inherited the bias instead of cancelling it. Use `price_sol` for cost basis, fills and PnL; `market_price_sol` for a per-token series independent of trade size and direction. `TokenTrade` and `WalletTrade` carry all four as `Option<f64>`, so an older server that omits them still deserializes.

> **New in 0.23.0** — **Pool depth / price impact + dev block on risk.** `client.token.depth(mint, &params)` (`GET /tokens/{mint}/depth`, PRO+) returns per-pool price-impact / slippage: for each supported pool a `DepthPool` with `spot_price_sol`, `fee_pct`, `source` (`"stream"` reserves or `"live_rpc"` curve virtual reserves), `reserves_age_ms`, per-size `DepthQuote`s (`size_sol`, `tokens_out`, `avg_price_sol`, `price_impact_pct`), and `to_move_price` (`DepthToMovePrice` — SOL to move the price 1/5/10%). Exact for constant-product AMMs and correct for pump.fun/bonk curves; concentrated pools (CLMM/Orca/DLMM), Meteora-DBC curves, and unclassified pools come back in `unsupported_pools` with a machine-readable `reason` instead of a wrong number. Pick buy sizes with `DepthParams::from_sizes(&[0.5, 1.0, 5.0, 10.0])` (`?sizes=` CSV, max 8, each ≤10000; default `0.5,1,5,10`). `client.token.risk(mint)` also gains a top-level `dev: Option<RiskDev>` block — deployer wallet activity: `buy_sol`/`buy_tokens`/`buy_supply_pct` at create, `bought_tokens_after` (catches the same-second-separate-tx dev buy), `sold_tokens`/`sold_sol`, `first_sell_at`/`last_sell_at`, live on-chain `holdings_tokens`/`holdings_supply_pct`/`wallet_empty`, and a coverage-gated `transferred_out` flag (every field `None` when unobservable — never a guess). New types: `DepthParams`, `TokenDepthResponse`, `DepthPool`, `DepthUnsupportedPool`, `DepthQuote`, `DepthToMovePrice`, `RiskDev`.
>
> **New in 0.22.0** — **Batch wallet classification + token trade tape + `bot_confidence` type fix.** `client.wallet.batch_classify(wallets)` (`POST /wallet/batch/classify`, PRO/ULTRA) returns reputation flags for 1–100 wallets in one call (counts as 1 request): each `WalletClassification` carries `is_sniper` / `is_bundler` / `is_dumper` / `is_kol` (+ `kol_name`), `bot_confidence`, and a `dump_cluster` cohort block. Flags are pump.fun-pipeline scoped — `false` = not observed, NOT verified clean; `is_bundler` is lifetime, `is_dumper` is a rolling 42-day window. `client.token.trades(mint, &params)` (`GET /tokens/{mint}/trades`, PRO/ULTRA) is the mint-scoped trade tape — cursor-paginated raw trades (default FULL history; capture starts 2026-04-12) with a machine-readable `coverage` honesty block. `WalletFlags` gains the same reputation flags + `dump_cluster`, and — **breaking type fix** — `WalletFlags.bot_confidence` is now `Option<String>` (text enum `"none"`/`"low"`/`"medium"`/`"high"`); it was mistyped `Option<f64>` and a server bug made it always `null` before, so no working code could have depended on the old type. `RiskInputs` gains `sniper_footprint` and `SniperDeploy` gains `footprint` — the slot-window launch-snipe rollup (`SniperFootprint`: `buys`, `buyers`, `sol`, `supply_pct`, `sniper_wallet_buys`, `data_available`, `as_of`; `None` = not observable, not zero). New types: `WalletBatchRequest`, `WalletClassification`, `WalletBatchClassifyResponse`, `WalletDumpCluster`, `TokenTradesParams`, `TokenTrade`, `TokenTradesFilters`, `TokenTradesCoverage`, `TokenTradesResponse`, `SniperFootprint`.
>
> **New in 0.21.0** — **Verified wallet holdings.** `client.wallet.holdings(address, &params)` (`GET /wallet/{address}/holdings`, ULTRA only) reads the wallet's actual SPL + Token-2022 token accounts and SOL balance directly from chain, enriches each with our price / MC / name / symbol data, and computes `transfer_delta` (on-chain amount minus trade-derived net position) to expose non-swap flows — airdrops, insider funding, wallet-hopping. Distinct from `client.wallet.positions()` (trade-derived FIFO): holdings is "what they actually hold right now". `WalletHoldingsResponse` carries `address`, `sol_balance`, a `Vec<Holding>` (each with `mint`, `symbol`, `name`, `amount`, `amount_raw`, `decimals`, `token_program` = `"spl"`/`"token2022"`, `price_usd`, `value_usd`, `market_cap_usd`, `is_bonded`, `trade_derived_amount`, `transfer_delta`), a `WalletHoldingsSummary` (`token_accounts`, `non_zero`, `returned`, `priced`, `total_value_usd`, `truncated`), `verified_at`, `trade_window_days`, `cache_hit`, and `ttl_seconds`. `WalletHoldingsParams` filters by `limit` (1–500, default 200) and `min_value_usd` (≥0, default 0). New types: `WalletHoldingsParams`, `WalletHoldingsResponse`, `WalletHoldingsSummary`, `Holding`.

> **New in 0.20.1** — **Token pools + deployer history.** `client.token.pools(mint)` (`GET /tokens/{mint}/pools`) returns every liquidity pool for a token across all tracked DEXes plus an aggregate `PoolsSummary` (`pool_count`, `active_pool_count`, `dex_count`, `dexes`, `total_liquidity_usd`, `primary_pool`, `primary_dex`, `top_pool_share_pct`). Each `Pool` carries `pool_address`, `dex`, `quote_mint`, `liquidity_usd`, `last_price_sol`, `last_swap_at`, `amm_id`, and `is_active`. `client.deployer.history(wallet, limit)` (`GET /deployer-hunter/{wallet}/history`, `limit` 1..=365) returns daily performance snapshots: each `DeployerSnapshot` has `date`, `tier`, `is_tracked`, `total_deployed`, `total_bonded`, `bonding_rate`, `recent_bond_rate`, `avg_peak_mc`, `best_token_peak_mc`; `is_deployer` is `false` when the wallet has never deployed. New types: `TokenPoolsResponse`, `Pool`, `PoolsSummary`, `DeployerHistoryParams`, `DeployerHistoryResponse`, `DeployerSnapshot`.

> **New in 0.20.0** — **Bundle intelligence.** `client.token.bundle(mint)` (`GET /tokens/{mint}/bundle`, PRO/ULTRA) detects wallets that bought a token in the same atomic transaction or the same slot — bundlers and coordinated snipers — and how much of supply they still hold. Returns a `BundleSummary` (`wallet_count`, `bundle_kind` = `atomic_tx`/`same_slot`/`none`, `held_ratio`, `held_pct_of_supply`, `fully_exited`, `buy_volume`, `tokens_held`) plus a per-wallet `Vec<BundleWallet>` breakdown (`rank`, `wallet`, `held_ratio`, `has_sold`, `atomic`, `tokens_held`). **ULTRA** populates wallet identity fields (`is_kol`, `kol_name`, `win_rate`, `bot_confidence`); lower tiers may return an empty `wallets` array. New types: `TokenBundle`, `BundleSummary`, `BundleWallet`, `BundleKind`.

> **New in 0.18.0** — **Almost-bonded tokens + trending sorts.** `client.token.almost_bonded(&params)` (`GET /tokens/almost-bonded`, PRO/ULTRA) returns pre-bond pump.fun tokens near graduation, ranked by velocity: each `AlmostBondedToken` carries `progress_pct`, `velocity_pct_per_min`, `eta_minutes`, a `stalled` flag, `real_sol_reserves`, `market_cap_usd`, `liquidity_usd`, `authorities_revoked`, `deployer_tier`, and `age_minutes`. `AlmostBondedParams` filters by `min_progress`/`max_progress`, `min_velocity_pct_per_min`, `max_age_minutes`, `deployer_tier`, `authority_revoked`, and `min_liq`, and picks the `AlmostBondedSort` order (`VelocityDesc` default, `ProgressDesc`, `EtaAsc`). New types: `AlmostBondedParams`, `AlmostBondedSort`, `AlmostBondedToken`, `AlmostBondedResponse`. `client.token.list(&params)` also accepts four new momentum `sort` values: `"mc_change_5m_desc"`, `"mc_change_1h_desc"`, `"volume_1h_desc"`, and `"trending"`.

> **New in 0.17.0** — **Token flow + deployer SOL balance.** `client.token.token_flow(mint, &params)` (`GET /tokens/{mint}/flow`, PRO+) returns aggregated buy/sell flow for a token over a rolling window: `unique_wallets`/`unique_buyers`/`unique_sellers`, `buy_count`/`sell_count`/`total_trades`, `buy_sol`/`sell_sol`/`net_sol` (buy − sell), and `trades_per_wallet`, plus the window `from` timestamp. `TokenFlowParams { window: Some("24h".into()) }` selects the window (`"1h"` default or `"24h"`). New types: `TokenFlowParams`, `TokenFlowResponse`. `DeployerAlert` also gains `deployer_sol_balance: Option<f64>` — the deployer wallet's SOL balance at alert time.

> **New in 0.16.0** — **Signal Scorecard.** New `client.signals` namespace. `client.signals.catalog()` returns the discovery index — every available signal with its `methodology` and a `performance_endpoint` (`SignalsCatalog`, `SignalCatalogEntry`). `client.signals.performance(name, &params)` returns a named signal's out-of-sample, machine-readable reliability — per-bucket `hit_rate` vs `base_rate`, `lift`, and `sample_n`, plus the test window and `methodology` (`SignalPerformance`, `SignalBucket`). Pass `SignalPerformanceParams { history: Some(true) }` to append the per-day drift series (`SignalHistoryEntry`). Valid signal names: `dump_cluster_count`, `runner_rate`, `recycled_early_buyer_count`, `coordination_count`. Open to any authenticated tier. New types: `SignalPerformanceParams`, `SignalPerformance`, `SignalBucket`, `SignalHistoryEntry`, `SignalsCatalog`, `SignalCatalogEntry`.

> **New in 0.15.0** — **OHLC candles.** `client.token.candles(mint, &params)` returns 1-minute OHLC candles aggregated from the trade firehose (PRO/ULTRA): per-bar `open`/`high`/`low`/`close`, `volume_usd`, `trades`, and `market_cap_usd`. ULTRA unlocks buy/sell volume split (`buy_volume_usd`, `sell_volume_usd`, `net_volume_usd`), open/close liquidity, MC high/low, buy/sell counts, and MEV volume per candle. `CandlesParams` selects `tf`, `limit`, and an optional `from`/`to` window. New types: `CandlesParams`, `Candle`, `CandlesResponse`.

> **New in 0.14.0** — **Token risk score.** `client.token.risk(mint)` returns a transparent 0–100 rug-risk / safety score (PRO/ULTRA, higher = riskier): an overall `risk_score` + `RiskBand` (`safe`/`caution`/`danger`), a per-factor `Vec<RiskFactor>` breakdown (each with `status`, `points`, and a human-readable `detail`), and the raw `RiskInputs` every factor was derived from (mint/freeze authority revocation, liquidity, transfer fee, launch cohort, deployer reputation, blacklist, …). New types: `TokenRisk`, `RiskFactor`, `RiskInputs`, `RiskBand`, `RiskFactorStatus`.

> **New in 0.13.0** — **Launch cohort, liquidity/MC ratio, deployer-tier filter, and KOL leaderboard timing.** `TokenResponseBody` gains `liquidity_to_mc_ratio`, `launch_cohort_sol`, and `launch_cohort_size`. `TokensListParams` gains `min_liq_mc_ratio`, `max_liq_mc_ratio`, and `deployer_tier` filters. `TokenSummary` (tokens list items) gains `liquidity_to_mc_ratio` and `deployer_tier`. `KolLeaderboardEntry` gains `median_hold_minutes_30d` and `percentile_early_entry_30d`.

> **New in 0.12.1** — **Deployer runner-rate fields.** `SniperDeploy`, `DeployerSummary`, `DeployerProfile`, and `DeployerLeaderboardEntry` now carry `runner_rate` (fraction of the deployer's labeled tokens that ran — peak ≥60min after deploy — vs dumped) and `labeled_tokens` (confidence denominator; gate on ≥3).

> **New in 0.12.0** *(2026-06-07)* — **Graduation events + dump-cluster detection.** `GraduationEvent` — typed payload for the `token:graduations` WebSocket channel: every pump.fun bond in real time (tracked deployer or not) with deployer tier, time-to-bond, and MC at bond. `AlphaBuyerQualityBreakdown` adds `dump_cluster_count` (out-of-sample: 3+ such wallets in the first-20 → 94% dump vs 61% base) and `recycled_early_buyer_count`. DEX firehose: replay buffer deepened to ~5 min; mint-scoped subs get in-band `dex:graduations` frames.

> **New in 0.10.0** *(2026-05-25)* — **Price alerts, scout leaderboard, KOL consensus, peak history, coordination history, wallet derived stats, trajectory snapshots.**
> `client.price_alerts` — full CRUD for MC-drop / recovery alert rules (PRO/ULTRA). `client.kol.scout_leaderboard()` — ranked scouts by swarm attraction rate. `client.token.kol_consensus(mint)` — per-token KOL buyer/seller breakdown. `client.token.peak_history(mint)` — ATH, decline from peak, MC snapshots post-bond. `client.kol.coordination_history()` — past coordination fires. `client.deployer.trajectory(wallet, params)` now accepts `include: Some("daily_snapshots")` for 90d snapshots. `WalletStatsResponse.derived` — win rate, ROI, best/worst trade, biggest miss, AI verdict.

## Get an API key

1. Visit **<https://madeonsol.com/pricing>**
2. Sign in with email or Solana wallet
3. Copy your `msk_…` key (free tier is unlocked instantly — 200 req/day, 10/min)

Paid tiers unlock higher rate limits, sub-hour windows, WebSocket streaming,
webhooks, and the all-DEX firehose:

| Tier | Price | Daily req | KOL trending sub-hour | Stream | Webhooks | DEX firehose |
|---|---|---|---|---|---|---|
| **Free** | $0 | 200 | — | — | — | — |
| **PRO** | €43/mo ≈ $49 | 10,000 | ✅ | ✅ | 3 | — |
| **ULTRA** | €131/mo ≈ $149 | 100,000 | ✅ | ✅ | 10 | ✅ |
| **BUSINESS** | €400/mo ≈ $449 | 500,000 | ✅ | ✅ | 30 | ✅ |

Annual: PRO €430/yr, ULTRA €1,310/yr, BUSINESS €4,000/yr (2 months free). EUR is the canonical price; USD shown for reference.

## Install

```toml
[dependencies]
madeonsol = "0.26"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Requires Rust 1.75+. Uses `reqwest` with `rustls-tls` (no OpenSSL dependency).

## Quick start

```rust
use madeonsol::{MadeOnSol, types::{KolFeedParams, KolAction}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Free key — get one at https://madeonsol.com/pricing
    let client = MadeOnSol::new(std::env::var("MADEONSOL_API_KEY")?)?;

    let feed = client.kol.feed(&KolFeedParams {
        limit: Some(10),
        action: Some(KolAction::Buy),
        ..Default::default()
    }).await?;

    for trade in feed.trades {
        let mc = trade.market_cap_usd_at_trade
            .map(|m| format!(" @ MC ${:.0}", m))
            .unwrap_or_default();
        println!("{:?} bought {:?} for {} SOL{}",
                 trade.kol_name, trade.token_symbol, trade.sol_amount, mc);
    }
    Ok(())
}
```

Run the bundled examples:

```sh
export MADEONSOL_API_KEY=msk_...
cargo run --example kol_feed
cargo run --example deployer_alerts
```

## Namespaces

The `MadeOnSol` client exposes namespaced sub-clients:

| Namespace | Purpose |
|---|---|
| `client.kol` | KOL feed, leaderboard, coordination, PnL, trending tokens, alerts, compare, **first_touches**, **scout_leaderboard**, **coordination_history** |
| `client.deployer` | Pump.fun deployer leaderboard, alerts, trajectory (+ daily snapshots), **history**, bonded tokens |
| `client.alpha` | Alpha-wallet leaderboard, profiles, cap tables, buyer quality |
| `client.token` | Per-mint snapshot, batch lookup, buyer quality, **kol_consensus**, **peak_history**, **risk** (+ `dev` block, 0.23), **batch_risk**, **bundle**, **pools**, **locks** / **locks_feed** / **unlocks** *(new 0.26 — token locks & vesting, upcoming unlocks; LP locks not included)*, **fee_shares** / **fee_claims** *(new 0.26 — pump.fun creator-fee sharing + fee-claim feed)*, **holders** *(new 0.25 — live holder census + concentration)*, **depth** *(new 0.23 — per-pool price impact)*, **candles**, **token_flow**, **trades** *(new 0.22 — mint-scoped trade tape)*, **almost_bonded**, directory list |
| `client.wallet_tracker` | Track arbitrary Solana wallets — watchlist CRUD, swap/transfer history |
| `client.wallet` | Universal wallet endpoints — stats + cross-product flags + derived analytics, FIFO PnL, open positions, paginated trades, **batch_classify** *(new 0.22 — bulk reputation flags, 1–100 wallets)* (PRO+), verified on-chain holdings (ULTRA) |
| `client.coordination_alerts` | Push alerts on coordinated buying (PRO/ULTRA) |
| `client.first_touch_subscriptions` | Push alerts on first-KOL-touch events (ULTRA) |
| `client.price_alerts` *(new 0.10)* | MC-drop / recovery price alert rules CRUD + event history (PRO/ULTRA) |
| `client.signals` *(new 0.16)* | **Signal Scorecard** — out-of-sample, machine-readable signal reliability (`performance`) + discovery catalog |
| `client.sniper` *(new 0.11)* | **Deshred** pre-confirm pump.fun deploy feed (~500ms head start) + custom deployer watchlist (PRO/ULTRA) |
| `client.tools` | Solana tool directory search |
| `client.stream` | Issue WebSocket streaming tokens (**non-expiring since 2026-08-27**), **rotate** them *(new 0.26.1)*, **list / kill live sessions** |
| `client.webhooks` | Webhook CRUD (PRO/ULTRA) |

Full reference: <https://docs.rs/madeonsol> · Interactive API docs: <https://madeonsol.com/api-docs>.

## Use cases

- **Copy-trading bot** — stream KOL buys via `client.kol.feed()` and mirror trades
- **DEX trade sniping** — subscribe to the all-DEX stream filtered by token / wallet
- **Deployer sniper** — monitor `client.deployer.alerts()` for elite-tier launches
- **Coordination detector** — flag tokens with `client.kol.coordination()` or push alerts
- **Scout signal** — track first-KOL-touch events filtered to S/A-tier scouts via `client.kol.first_touches()` (backtested: ~50% swarm rate vs 14% baseline)
- **Analytics dashboard** — combine leaderboard, PnL, and tool data
- **Telegram/Discord bot** — pipe alerts via webhooks into chat
- **Portfolio tracker** — use `client.kol.wallet()` to follow specific KOL positions

## Error handling

All methods return `Result<T, madeonsol::Error>`. The `Error::Api` variant exposes
HTTP status, server message, and the raw JSON body:

```rust
use madeonsol::{MadeOnSol, Error};

# async fn run(client: MadeOnSol) -> Result<(), Error> {
match client.kol.token("invalid-mint").await {
    Ok(activity) => println!("{:?}", activity),
    Err(Error::Api { status, message, .. }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(other) => return Err(other),
}
# Ok(())
# }
```

`Error::MissingApiKey` is returned by `MadeOnSol::new` if the key is empty or
doesn't start with `msk_` — the error message and a stderr hint both link to
<https://madeonsol.com/pricing>.

## First-touch signal *(new in 0.4)*

Every "first KOL buy on a token mint" event — when a tracked KOL is the first of the cohort to touch a token. Filterable by **scout tier** (S/A/B/C from `mv_kol_scout_score`), KOL winrate, token age, mint suffix.

**Backtest:** S-tier scouts attract ≥3 follow-on KOLs within 4h ~50% of the time vs ~14% baseline (38d / 491k buys / 72,549 events). Public leaderboard at <https://madeonsol.com/kol/scouts>.

```rust
use madeonsol::{FirstTouchPreset, FirstTouchesParams, ScoutTier};

let res = client
    .kol
    .first_touches(&FirstTouchesParams {
        preset: Some(FirstTouchPreset::Scout),
        min_scout_tier: Some(ScoutTier::S),
        limit: Some(20),
        ..Default::default()
    })
    .await?;

for e in res.events {
    println!(
        "{} scouted {} (scout_score={:?}%)",
        e.first_kol.name.unwrap_or_default(),
        e.token_symbol.unwrap_or_default(),
        e.first_kol.scout_score
    );
}
```

**Webhook subscriptions** (Ultra, up to 10 active per user) — push delivery, HMAC-SHA256 signed:

```rust
use madeonsol::{FirstTouchSubscriptionCreateParams, FirstTouchSubscriptionFilters, ScoutTier, CoordinationDeliveryMode};

let res = client
    .first_touch_subscriptions
    .create(&FirstTouchSubscriptionCreateParams {
        name: Some("S-tier scouts on pump tokens".into()),
        filters: Some(FirstTouchSubscriptionFilters {
            min_scout_tier: Some(ScoutTier::S),
            mint_suffix: Some("pump".into()),
            ..Default::default()
        }),
        delivery_mode: Some(CoordinationDeliveryMode::Webhook),
        webhook_url: Some("https://you.com/hooks/scout".into()),
    })
    .await?;
// store res.webhook_secret — shown ONCE
```

> **Don't poll — push.** Median lead time before the second KOL is 12 seconds. WebSocket channel: `kol:first_touches` (PRO+).

## Universal wallet endpoints *(new in 0.9)*

Per-wallet profile data for **any** Solana wallet — not just curated KOLs. FIFO cost-basis PnL over the last 90 days, cached server-side with dynamic TTL. Cache hits don't count against your daily quota. PRO+.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::WalletTradesParams;

// 1. Profile any wallet — works on KOLs, alpha traders, deployers, randoms.
let stats = client.wallet.stats("ASVzakePP6GNg9r95d4LPZHJDMXun6L6E4um4pu5ybJk").await?;
if let Some(s) = stats.stats {
    println!("{}: {} trades, {} unique tokens", stats.address, s.total_trades, s.unique_tokens);
}
println!("KOL: {} · alpha: {} · deployer: {}",
    stats.flags.is_kol, stats.flags.is_alpha_tracked, stats.flags.is_deployer);

// 2. Full FIFO PnL — realized + unrealized SOL, profit factor, drawdown,
//    daily curve, closed/open positions.
let pnl = client.wallet.pnl(&stats.address).await?;
println!("Realized: {:+.2} SOL · Unrealized: {:+.2} SOL", pnl.summary.realized_sol, pnl.summary.unrealized_sol);
if let Some(pf) = pnl.summary.profit_factor {
    println!("Win rate: {:.0}% · Profit factor: {:.2}", pnl.summary.win_rate.unwrap_or(0.0) * 100.0, pf);
}
for c in pnl.closed_positions.iter().take(5) {
    println!("  {}…  {:+.2} SOL ({:?}% ROI, {} min hold)",
        &c.token_mint[..8], c.pnl_sol, c.roi_pct, c.hold_minutes.unwrap_or(0));
}

// 3. Paginated raw trades — keep paging with next_cursor.
let mut params = WalletTradesParams { limit: Some(200), ..Default::default() };
loop {
    let page = client.wallet.trades(&stats.address, &params).await?;
    for t in &page.trades { /* … */ }
    if !page.has_more { break; }
    params.cursor = page.next_cursor;
}
# Ok(())
# }
```

**Cost-basis honesty.** Observable only inside the 90-day window. Overflow sells (no matching buy in window) are silently discarded rather than fabricated. `notes.cost_basis_observable_from` makes the cutoff visible.

## Deshred sniper alerts *(new in 0.11)*

The fastest path to a new pump.fun launch. Deploys are reconstructed from shred-level (**deshred**) data and surface **~500ms before the chain confirms them**. **PRO** sees elite + good deployers; **ULTRA** sees every tier and can keep a custom deployer watchlist. For live push use the `sniper:deploy` webhook, the `sniper:deploys` WebSocket channel, or `/alert sniper` in Telegram — these methods are for catch-up, backtesting, and watchlist management.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::{SniperRecentParams, SniperWatchlistAddParams};

// Deshred deploy feed — PRO: elite/good · ULTRA: all tiers
let feed = client.sniper.recent(&SniperRecentParams { limit: Some(50), ..Default::default() }).await?;
for d in &feed.deploys {
    println!("{} by {} (tier {:?})", d.symbol.as_deref().unwrap_or("?"), d.deployer_wallet, d.deployer_tier);
}

// Custom watchlist (ULTRA, max 50) — get deploys from only the deployers you track, any tier
client.sniper.add_to_watchlist(&SniperWatchlistAddParams {
    wallets: Some(vec!["7dEx...4pQ8".into(), "9aBc...2zZ1".into()]),
    label: Some("alpha devs".into()),
    ..Default::default()
}).await?;
let tracked = client.sniper.recent(&SniperRecentParams { watchlist: Some(true), ..Default::default() }).await?;
println!("{} deploys from watchlisted deployers", tracked.count);
# Ok(())
# }
```

## Price alerts *(new in 0.10)*

Get notified when a token's market cap drops below a threshold (and optionally on recovery). PRO: 5 rules, ULTRA: 25 rules. Delivered via WebSocket channel `price:alerts` and/or HMAC-signed webhook.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::{PriceAlertCreateParams, PriceAlertDeliveryMode, PriceAlertEventsParams};

// Create an alert: fire when MC drops 30%, then again on 50% recovery.
let res = client.price_alerts.create(&PriceAlertCreateParams {
    token_mint: "So11111111111111111111111111111111111111112".into(),
    drop_pct: 30.0,
    recovery_pct: Some(50.0),
    name: Some("SOL 30% dip".into()),
    delivery_mode: Some(PriceAlertDeliveryMode::Webhook),
    webhook_url: Some("https://you.com/hooks/price".into()),
}).await?;
// store res.webhook_secret — shown ONCE
println!("Alert {} created, status: {:?}", res.alert.id, res.alert.status);

// List active alerts
let alerts = client.price_alerts.list().await?;
for a in alerts.alerts {
    println!("{}: {} drop={}% status={:?}", a.id, a.token_mint, a.drop_pct, a.status);
}

// Check fired events
let events = client.price_alerts.events(&PriceAlertEventsParams {
    limit: Some(20),
    ..Default::default()
}).await?;
for e in events.events {
    println!("{} {} at MC ${:.0}", e.event_type, e.token_mint, e.current_mc_usd);
}
# Ok(())
# }
```

## New in 0.10: scout leaderboard, KOL consensus, peak history

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::{ScoutLeaderboardParams, ScoutTier, ScoutLeaderboardSort};

// Top scouts by swarm attraction rate
let scouts = client.kol.scout_leaderboard(&ScoutLeaderboardParams {
    limit: Some(10),
    scout_tier: Some(ScoutTier::S),
    sort: Some(ScoutLeaderboardSort::Swarm3PlusPct),
}).await?;
println!("{}", scouts);

// KOL consensus on a token
let consensus = client.token.kol_consensus("So11111111111111111111111111111111111111112").await?;
println!("{} buyers, {} sellers, exit rate {:?}%",
    consensus.total_kol_buyers, consensus.total_kol_sellers, consensus.kol_exit_rate);

// Peak MC history
let peak = client.token.peak_history("So11111111111111111111111111111111111111112").await?;
println!("ATH: {:?}, decline: {:?}%", peak.peak_mc_usd, peak.decline_from_peak_pct);

// 1-minute OHLC candles (PRO/ULTRA)
use madeonsol::types::CandlesParams;
let candles = client.token.candles(
    "So11111111111111111111111111111111111111112",
    &CandlesParams { tf: Some("1m".into()), limit: Some(60), ..Default::default() },
).await?;
for c in &candles.candles {
    println!("{} O:{} H:{} L:{} C:{} vol:${}", c.t, c.open, c.high, c.low, c.close, c.volume_usd);
}

// Aggregated buy/sell flow over a window (PRO+)
use madeonsol::types::TokenFlowParams;
let flow = client.token.token_flow(
    "So11111111111111111111111111111111111111112",
    &TokenFlowParams { window: Some("24h".into()) },
).await?;
println!("{} wallets · net {} SOL", flow.unique_wallets, flow.net_sol);
# Ok(())
# }
```

## New in 0.16: Signal Scorecard

Out-of-sample, machine-readable reliability for each enrichment signal, so bots
can weight them programmatically instead of asking. Open to any authenticated tier.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::SignalPerformanceParams;

// Discover the available signals + how to fetch each one's efficacy
let catalog = client.signals.catalog().await?;
for s in &catalog.signals {
    println!("{} → {}", s.name, s.performance_endpoint);
}

// Live, out-of-sample reliability for a named signal
let perf = client.signals.performance(
    "coordination_count",
    &SignalPerformanceParams { history: Some(false) },
).await?;
println!("methodology: {:?}", perf.methodology);
for b in &perf.buckets {
    println!("{}: hit {:?} vs base {:?} (lift {:?}, n={})",
        b.bucket, b.hit_rate, b.base_rate, b.lift, b.sample_n);
}
# Ok(())
# }
```

## Batch risk scoring *(new in 0.19)*

Score up to 50 mints for rug-risk in a single round-trip (PRO/ULTRA) — same
transparent per-factor breakdown as `client.token.risk(mint)`. Untracked mints
come back as error entries instead of failing the whole batch, so check
`is_error()` (or match on `error`) before reading the score.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
let res = client
    .token
    .batch_risk(vec![
        "So11111111111111111111111111111111111111112".into(),
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".into(),
    ])
    .await?;

println!("{} mints scored", res.count);
for t in res.tokens {
    if t.is_error() {
        println!("{}: {}", t.mint, t.error.unwrap_or_default()); // e.g. "not_tracked"
    } else {
        println!("{}: risk {:?} ({:?})", t.mint, t.risk_score, t.band);
    }
}
# Ok(())
# }
```

## Token locks, unlocks & pump.fun fee sharing *(new in 0.26)*

Five keyed (PRO+) methods on `client.token`. All base-unit amounts (`*_raw`)
are `String`s — parse them yourself; every ui / usd / pct companion is an
`Option` that is `None` when decimals or price are unknown. **None of these
are on the x402 rail**; BASIC gets HTTP 403.

- `client.token.locks(mint, &TokenLocksParams)` (`GET /tokens/{mint}/locks`)
  — every on-chain Streamflow / Jupiter Lock / Bonfida vesting contract on the
  mint (`TokenLock`: schedule, terms, live-derived `locked_raw` / `unlocked` /
  `withdrawn` / `claimable` / `LockStatus` / `next_unlock`) + a
  `TokenLocksSummary` (exact `lock_count`, `distinct_lockers`, locked /
  deposited totals, `unlocking_7d_*` / `unlocking_30d_*`,
  `active_cancelable_by_sender`). **LP locks are not included.**
- `client.token.locks_feed(&TokenLocksFeedParams)` (`GET /tokens/locks`) —
  NEW contracts across all mints, newest first; poll with
  `pagination.next_since` → `since`, or subscribe to WS `token:locks`
  (`TokenLockEvent`).
- `client.token.unlocks(&TokenUnlocksParams)` (`GET /tokens/unlocks`) —
  upcoming unlock events (`UnlockEventKind`) inside `UnlockWindow` (`1h` … `90d`,
  default `7d`), one per active contract with `amount_*` (next event) and
  `window_amount_*` (whole window), sorted by `UnlocksSort`.
- `client.token.fee_shares(mint)` (`GET /tokens/{mint}/fee-shares`) — the
  pump.fun `SharingConfig` (`FeeSharingConfig` + `FeeShareholder`, `is_default`
  = 100% to the creator, `is_social_pda` + `FeeShareSocial` — platform 2 = X),
  `FeeDistributions` rollup, `history`, `recent_distributions`. **Event
  history starts 2026-08-17.**
- `client.token.fee_claims(&TokenFeeClaimsParams)` (`GET /tokens/fee-claims`)
  — the fee-event feed (`FeeClaimEvent` / `FeeEventType`); `CreatorClaim` only
  when requested via `event_type`; live on WS `token:fee_claims`
  (`TokenFeeClaimEvent`).

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::{LockStatus, TokenLocksParams, TokenUnlocksParams, UnlockWindow, UnlocksSort};

let mint = "NUGye8S6CV82ZNrauf5YfXL2xJxvSvfiMAvy2U1sAVk";
let l = client.token.locks(mint, &TokenLocksParams { status: Some(LockStatus::Active), ..Default::default() }).await?;
let locked: u128 = l.summary.locked_raw.parse()?; // base units — a String, never a float
println!(
    "{} contracts, {} active, locked raw={} ({:?}% of supply), unlocking 7d {:?} usd, cancelable by sender: {}",
    l.summary.lock_count, l.summary.active_count, locked, l.summary.locked_pct_of_supply,
    l.summary.unlocking_7d_usd, l.summary.active_cancelable_by_sender,
);

let u = client.token.unlocks(&TokenUnlocksParams {
    within: Some(UnlockWindow::D7),
    sort: Some(UnlocksSort::LargestUsd),
    limit: Some(10),
    ..Default::default()
}).await?;
for e in &u.unlocks {
    println!("{} {:?} {} raw={} usd={:?}", e.unlock_at, e.event, e.mint, e.amount_raw, e.amount_usd);
}

let f = client.token.fee_shares("E2rQLGJxb1pq4u4AoXSAmqTbspupMXfgfbJsXU5npump").await?;
if let Some(cfg) = &f.config {
    println!("default={:?} redirected={}bps social={}bps shareholders={}", cfg.is_default, cfg.redirected_bps, cfg.social_bps, cfg.shareholders.len());
}
# Ok(())
# }
```

New types: `TokenLocksResponse`, `TokenLocksSummary`, `TokenLock`,
`LockNextUnlock`, `LockTokenInfo`, `TokenLocksParams`, `TokenLocksFeedParams`,
`TokenLocksFeedResponse`, `TimeCursorPagination`, `StreamPointer`,
`TokenUnlocksParams`, `TokenUnlocksResponse`, `TokenUnlock`, `UnlockLockRef`,
`UnlockWindowInfo`, `TokenUnlocksPagination`, `TokenLockEvent`, `LockProgram`,
`LockKind`, `LockStatus`, `UnlockEventKind`, `UnlockWindow`, `UnlocksSort`,
`TokenFeeSharesResponse`, `FeeSharingConfig`, `FeeShareholder`, `FeeShareSocial`,
`FeeDistributions`, `FeeRecentDistribution`, `FeeQuote`, `FeeConfigSource`,
`TokenFeeClaimsParams`, `TokenFeeClaimsResponse`, `FeeClaimEvent`,
`FeeClaimSocial`, `FeeClaimShareholder`, `FeeClaimPayout`, `FeeEventType`,
`TokenFeeClaimEvent`.

## Live holders + concentration *(new)*

`client.token.holders(mint)` (`GET /tokens/{mint}/holders`, PRO+) — a full
holder census read from the ledger at `confirmed`: every token account of the
mint (owner + balance), merged per owner. This is who holds **now**;
`client.alpha.cap_table` is who bought first.

- `concentration.holder_count` is **exact** (distinct non-zero owners minus
  excluded pools/curves/burns, at `slot`) and `None` only when the provider
  refused the census for a mega-cap mint — then `source.method` is
  `HoldersMethod::GetTokenLargestAccounts` (top-20 view) and
  `source.census_fallback_reason` is set. It is never estimated from trades.
- `amount_raw` on every `TokenHolder` / `TokenHoldersExcluded` is a raw u64
  **`String`** — never a float; parse it (`u64`/`u128`) yourself. `amount` is
  the UI-scaled convenience `f64`.
- Pools, bonding curves, burns and unattributed program accounts are
  **excluded** from the circulating denominator and listed in `excluded`,
  each named where possible: `HolderExcludedReason::Pool` (+ `dex`,
  `pool_address`), `BondingCurve` (pump.fun / LaunchLab), `Burn`, else
  `ProgramAccount`. The #1 raw account of a fresh memecoin is its own bonding
  curve. `concentration.pool_pct` / `burned_pct` / `program_pct` split them
  (over total supply).
- Disclosure is tier-gated: **PRO** ranks 1–10, **ULTRA** 1–50, **BUSINESS**
  1–100 (`disclosed` is your cap); `top1/top10/top20/top50/top100_share`, the
  cohort `*_pct` values and `holder_count` are computed over the full set and
  identical on every tier. All shares are 0–100.
- Each holder carries `labels: Vec<HolderLabel>` from MadeOnSol wallet
  intelligence (`Deployer` / `Kol` / `EarlyBuyer` / `Buyer` / `Bundle` /
  `Bot` / `DumpCluster`) plus `kol_name`, `early_buyer_rank`,
  `bot_confidence`, `historical_win_rate`. Empty labels = unknown to us, not
  verified clean.
- Latency: fresh pump.fun mints <1 s; 200k–550k-account tokens 6–11 s. While
  the upstream scan is still running the API answers **503**
  `error_kind: "holder_scan_in_progress"` with `retry_after_seconds: 20` — the
  scan keeps going and is cached, so the retry is instant.
  `holder_rpc_unavailable` (503, `retry_after_seconds: 15`) is a fail-closed
  RPC outage. Both arrive as `Error::Api { status: 503, body, .. }` — read
  `error_kind` / `retry_after_seconds` from `body`. Unknown mint: 404
  `error_kind: "not_a_mint"`.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::Error;

let mint = "So11111111111111111111111111111111111111112";
let h = loop {
    match client.token.holders(mint).await {
        Ok(h) => break h,
        Err(Error::Api { status: 503, body, .. })
            if body["error_kind"] == "holder_scan_in_progress" =>
        {
            // scan continues upstream and is cached — the retry is instant
            let secs = body["retry_after_seconds"].as_u64().unwrap_or(20);
            tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
        }
        Err(e) => return Err(e.into()),
    }
};

println!(
    "{:?} holders · top10 {:?}% of circulating · pools/curves {:?}% of supply ({} excluded)",
    h.concentration.holder_count, h.concentration.top10_share, h.concentration.pool_pct, h.excluded.len(),
);
for holder in &h.holders {
    let raw: u128 = holder.amount_raw.parse()?; // raw u64 string — never a float
    println!("#{} {} raw={} {:?}", holder.rank, holder.owner, raw, holder.labels);
}
# Ok(())
# }
```

New types: `TokenHoldersResponse`, `TokenHolder`, `TokenHoldersExcluded`,
`TokenHoldersConcentration`, `TokenHoldersDeployer`, `TokenHoldersSource`,
`HolderLabel`, `HolderExcludedReason`, `HoldersMethod`.

## Pool depth / price impact *(new in 0.23)*

How much SOL moves the price 1/5/10%, and what slippage each buy size eats —
per pool (PRO+). Exact for constant-product AMMs (streamed reserves, zero-RPC),
correct for pump.fun/bonk curves via live virtual reserves. Pools we can't
price honestly (CLMM/Orca/DLMM, Meteora-DBC) come back in `unsupported_pools`
with a `reason` instead of a wrong number.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::DepthParams;

let depth = client
    .token
    .depth(
        "So11111111111111111111111111111111111111112",
        &DepthParams::from_sizes(&[0.5, 1.0, 5.0, 10.0]), // or Default::default() for 0.5,1,5,10
    )
    .await?;

for p in &depth.pools {
    println!(
        "{} ({}, {}): spot {} SOL — {} SOL moves price 1%",
        p.pool_address, p.dex, p.source, p.spot_price_sol, p.to_move_price.pct_1,
    );
    for q in &p.quotes {
        println!("  buy {} SOL → {} tokens ({}% impact)", q.size_sol, q.tokens_out, q.price_impact_pct);
    }
}
for u in &depth.unsupported_pools {
    println!("{}: no depth — {}", u.pool_address, u.reason);
}
# Ok(())
# }
```

## Batch wallet classification + token trade tape *(new in 0.22)*

Screen up to 100 wallets for sniper / bundler / dumper / KOL reputation in one
request, and replay any token's raw trade history with cursor pagination
(PRO/ULTRA). Reputation flags are pump.fun-pipeline scoped — `false` means
"not observed", not "verified clean".

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::TokenTradesParams;

// 1. Bulk reputation screen — one request for up to 100 wallets.
let res = client
    .wallet
    .batch_classify(vec![
        "ASVzakePP6GNg9r95d4LPZHJDMXun6L6E4um4pu5ybJk".into(),
        "7dExa4pQ8XkbwsjXCADEHVXpXU9DDSzM8yBslkzX4pQ8".into(),
    ])
    .await?;
for w in res.wallets {
    println!(
        "{}: sniper={} bundler={} dumper={} kol={:?} bot={:?}",
        w.address, w.is_sniper, w.is_bundler, w.is_dumper, w.kol_name, w.bot_confidence,
    );
}

// 2. Token trade tape — full history (from 2026-04-12), newest first.
let mut params = TokenTradesParams { limit: Some(500), ..Default::default() };
loop {
    let page = client
        .token
        .trades("So11111111111111111111111111111111111111112", &params)
        .await?;
    for t in &page.trades { /* t.tx_signature, t.action, t.sol_amount, … */ }
    if !page.has_more { break; }
    params.cursor = page.next_cursor;
}
# Ok(())
# }
```

## Bundle intelligence *(new in 0.20)*

Detect wallets that bought a token in the same atomic transaction or same slot —
bundlers and coordinated snipers — how much of supply they still hold, and
whether the cohort has fully exited (PRO/ULTRA). **ULTRA** additionally labels
each wallet with KOL identity and bot-confidence.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
let b = client
    .token
    .bundle("So11111111111111111111111111111111111111112")
    .await?;

println!(
    "{} bundled wallets ({:?}), holding {:?} of supply, fully_exited={}",
    b.bundle.wallet_count, b.bundle.bundle_kind, b.bundle.held_ratio, b.bundle.fully_exited,
);

// ULTRA — per-wallet identity is populated
for w in b.wallets {
    println!(
        "#{} {} — held {:?}, sold={}, kol={:?}",
        w.rank, w.wallet, w.held_ratio, w.has_sold, w.kol_name,
    );
}
# Ok(())
# }
```

## WebSocket streams (PRO/ULTRA)

This crate does **not** ship a WebSocket client — `client.stream.get_token()`
returns the URL + token, and you connect with any WS library
(`tokio-tungstenite` recommended):

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
let token = client.stream.get_token().await?;
let ws_url = format!("{}?token={}", token.ws_url, token.token);
// then: tokio_tungstenite::connect_async(&ws_url).await
# Ok(())
# }
```

**Stream tokens do not expire** *(since 2026-08-27)*. `get_token()` returns the
same token on every call — call it on every reconnect and never schedule a
refresh: `expires_at` / `next_refresh_at` are always `None` (kept for wire
compatibility only). The token stops working only when your subscription
lapses, or when you replace it yourself with `client.stream.rotate_token()`
(`POST /stream/token` with `{"rotate": true}`) — the old value then keeps
working for 60 s so live sockets can reconnect. A WebSocket close code `4001`
means "call `get_token()` again and reconnect", never "the token timed out".

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
// Only if a token leaked — there is no reason to rotate on a schedule.
let fresh = client.stream.rotate_token().await?;
assert_eq!(fresh.rotated, Some(true));
# Ok(())
# }
```

Channels: `kol:trades`, `kol:coordination`, `kol:first_touches`, `deployer:alerts`, `wallet_tracker:events`, `copytrade:signals`, `price_alert:events`, `sniper:deploys`, `token:graduations` (`GraduationEvent`), `token:prices` (mint-scoped price / MC ticks), `token:locks` *(new 0.26 — event `token:lock`, `TokenLockEvent`: every NEW lock / vesting contract; LP locks not included)*, `token:fee_claims` *(new 0.26 — event `token:fee_claim`, `TokenFeeClaimEvent`: every pump.fun fee event; history starts 2026-08-17)*. All PRO+.

The DEX firehose URL (`token.dex_ws_url`) is only present for ULTRA subscribers.
See <https://madeonsol.com/api-docs> for the full subscribe/unsubscribe protocol.

### Session management *(new in 0.19)*

List every live socket on your account and force-disconnect a stale one to free
its connection slot (PRO/ULTRA):

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
let live = client.stream.sessions().await?;
for s in &live.sessions {
    println!("#{} {} {:?} ({} msgs)", s.id, s.service, s.channels, s.messages_sent);
}

// Kick a ghost socket that's still holding a slot.
if let Some(s) = live.sessions.first() {
    let res = client.stream.kill_session(&s.id).await?;
    println!("evicted {}: {}", res.id, res.evicted);
}
# Ok(())
# }
```

## Also available

| Platform | Package |
|---|---|
| TypeScript / Node | [`madeonsol`](https://www.npmjs.com/package/madeonsol) on npm |
| Python (LangChain, CrewAI) | [`madeonsol-x402`](https://pypi.org/project/madeonsol-x402/) on PyPI |
| MCP Server (Claude, Cursor) | [`mcp-server-madeonsol`](https://www.npmjs.com/package/mcp-server-madeonsol) · [Smithery](https://smithery.ai/servers/madeonsol/solana-kol-intelligence) · [Glama](https://glama.ai/mcp/servers/LamboPoewert/mcp-server-madeonsol) |
| ElizaOS | [`@madeonsol/plugin-madeonsol`](https://www.npmjs.com/package/@madeonsol/plugin-madeonsol) |
| Solana Agent Kit | [`solana-agent-kit-plugin-madeonsol`](https://www.npmjs.com/package/solana-agent-kit-plugin-madeonsol) |

## Links

- API docs: <https://madeonsol.com/api-docs> (interactive try-it-now)
- OpenAPI 3.1 spec: <https://madeonsol.com/api/v1/openapi.json>
- Postman collection: <https://madeonsol.com/api/v1/postman.json>
- Pricing & free key: <https://madeonsol.com/pricing>
- Issues: <https://github.com/madeonsol/madeonsol-rs/issues>

## License

MIT © [MadeOnSol](https://madeonsol.com)
