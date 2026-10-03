//! # MadeOnSol — official Rust SDK
//!
//! Solana KOL wallet tracking, Pump.fun deployer intelligence, alpha-wallet scoring,
//! and an all-DEX trade firehose.
//!
//! ## Get an API key
//!
//! Free tier: **200 requests/day, no credit card** at <https://madeonsol.com/pricing>.
//! Paid tiers (PRO $49/mo, ULTRA $149/mo) unlock higher rate limits, sub-hour windows,
//! WebSocket streaming, webhooks, and the all-DEX firehose.
//!
//! All keys start with `msk_`.
//!
//! ## Quick start
//!
//! ```no_run
//! use madeonsol::{MadeOnSol, types::KolFeedParams};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let api_key = std::env::var("MADEONSOL_API_KEY")?;
//! let client = MadeOnSol::new(api_key)?;
//!
//! let feed = client
//!     .kol
//!     .feed(&KolFeedParams { limit: Some(10), ..Default::default() })
//!     .await?;
//!
//! for trade in feed.trades {
//!     println!("{:?} bought {:?}", trade.kol_name, trade.token_symbol);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Namespaces
//!
//! - [`MadeOnSol::kol`] — KOL feed, leaderboard, coordination, PnL, trending tokens, alerts, scout leaderboard
//! - [`MadeOnSol::deployer`] — Pump.fun deployer leaderboard, alerts, trajectory
//! - [`MadeOnSol::alpha`] — alpha-wallet leaderboard, profiles, cap tables, buyer quality
//! - [`MadeOnSol::wallet_tracker`] — track arbitrary Solana wallets (watchlist)
//! - [`MadeOnSol::wallet`] — universal wallet stats, FIFO PnL, open positions, paginated trades (PRO+)
//! - [`MadeOnSol::coordination_alerts`] — push alerts on coordinated buying (PRO/ULTRA)
//! - [`MadeOnSol::price_alerts`] — price-drop / recovery alert rules CRUD (PRO/ULTRA)
//! - [`MadeOnSol::signals`] — Signal Scorecard: out-of-sample, machine-readable signal reliability
//! - [`MadeOnSol::tools`] — Solana tool directory search
//! - [`MadeOnSol::stream`] — WebSocket streaming token issuance + live session list/kill
//! - [`MadeOnSol::webhooks`] — webhook CRUD (PRO/ULTRA)
//!
//! Full API reference: <https://madeonsol.com/api-docs>

#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

mod client;
pub mod api;
pub mod error;
pub mod types;

use std::sync::Arc;

use crate::api::{
    alpha::Alpha, coordination_alerts::CoordinationAlerts, deployer::Deployer,
    first_touch_subscriptions::FirstTouchSubscriptions, kol::Kol, me::Me,
    price_alerts::PriceAlerts, signals::Signals, sniper::Sniper, stream::Stream, token::Token,
    tools::Tools, wallet::Wallet, wallet_tracker::WalletTracker, webhooks::Webhooks,
};
use crate::client::HttpCore;
use crate::error::{MadeOnSolError, Result};

pub use crate::error::MadeOnSolError as Error;

/// MadeOnSol API client.
///
/// Construct with [`MadeOnSol::new`] and a `msk_…` API key, then access the
/// namespaced sub-clients ([`kol`](Self::kol), [`deployer`](Self::deployer), etc.).
///
/// Cheap to clone — internal HTTP state is reference-counted.
///
/// # Example
///
/// ```no_run
/// use madeonsol::MadeOnSol;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = MadeOnSol::new(std::env::var("MADEONSOL_API_KEY")?)?;
/// let stats = client.deployer.stats().await?;
/// println!("{} deployers tracked", stats.tracked_count);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MadeOnSol {
    /// KOL wallet tracking endpoints.
    pub kol: Kol,
    /// Pump.fun deployer intelligence endpoints.
    pub deployer: Deployer,
    /// Alpha wallet intelligence: leaderboard, profiles, cap tables, buyer quality.
    pub alpha: Alpha,
    /// Token intelligence — comprehensive per-mint snapshot + batch lookups.
    pub token: Token,
    /// Account self-inspection — tier, quota, feature usage (v0.8).
    pub me: Me,
    /// Wallet tracker: watchlist CRUD, trades, summary.
    pub wallet_tracker: WalletTracker,
    /// Universal wallet endpoints — stats, FIFO PnL, open positions, paginated trades for any Solana wallet. PRO+.
    pub wallet: Wallet,
    /// Coordination alert rules CRUD (v1.1) — PRO/ULTRA.
    pub coordination_alerts: CoordinationAlerts,
    /// First-touch webhook subscriptions CRUD — ULTRA only. Use `kol.first_touches()` for read-only queries.
    pub first_touch_subscriptions: FirstTouchSubscriptions,
    /// Price-drop / recovery alert rules CRUD (v1.9) — PRO/ULTRA.
    pub price_alerts: PriceAlerts,
    /// Signal Scorecard (v0.16) — out-of-sample, machine-readable signal reliability + catalog.
    pub signals: Signals,
    /// Deshred pre-confirm pump.fun sniper feed + custom watchlist — PRO/ULTRA.
    pub sniper: Sniper,
    /// Solana tool directory search.
    pub tools: Tools,
    /// WebSocket streaming token issuance.
    pub stream: Stream,
    /// Webhook management (PRO/ULTRA).
    pub webhooks: Webhooks,
}

impl MadeOnSol {
    /// Construct a new client.
    ///
    /// `api_key` must start with `msk_`. Get a free key (200 req/day, no card)
    /// at <https://madeonsol.com/pricing>.
    ///
    /// # Errors
    ///
    /// Returns [`MadeOnSolError::MissingApiKey`] if the key is empty or missing the
    /// `msk_` prefix. The error message includes the signup URL so end users know
    /// where to go.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        if !api_key.starts_with("msk_") {
            // Print to stderr too — a bare Err can be swallowed and the user
            // never sees the link to /pricing.
            eprintln!(
                "\n[madeonsol] Missing or invalid API key.\n\
                 → Get a free key (200 req/day, no card) at https://madeonsol.com/pricing\n\
                 → Then: madeonsol::MadeOnSol::new(std::env::var(\"MADEONSOL_API_KEY\")?)?\n"
            );
            return Err(MadeOnSolError::MissingApiKey);
        }

        let core = Arc::new(HttpCore::new(api_key));
        Ok(Self {
            kol: Kol { core: Arc::clone(&core) },
            deployer: Deployer { core: Arc::clone(&core) },
            alpha: Alpha { core: Arc::clone(&core) },
            token: Token { core: Arc::clone(&core) },
            me: Me { core: Arc::clone(&core) },
            wallet_tracker: WalletTracker { core: Arc::clone(&core) },
            wallet: Wallet { core: Arc::clone(&core) },
            coordination_alerts: CoordinationAlerts { core: Arc::clone(&core) },
            first_touch_subscriptions: FirstTouchSubscriptions { core: Arc::clone(&core) },
            price_alerts: PriceAlerts { core: Arc::clone(&core) },
            signals: Signals { core: Arc::clone(&core) },
            sniper: Sniper { core: Arc::clone(&core) },
            tools: Tools { core: Arc::clone(&core) },
            stream: Stream { core: Arc::clone(&core) },
            webhooks: Webhooks { core },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_api_key() {
        let err = MadeOnSol::new("").unwrap_err();
        assert!(matches!(err, MadeOnSolError::MissingApiKey));
    }

    #[test]
    fn rejects_wrong_prefix() {
        let err = MadeOnSol::new("sk_live_abc").unwrap_err();
        assert!(matches!(err, MadeOnSolError::MissingApiKey));
    }

    #[test]
    fn accepts_valid_prefix() {
        let client = MadeOnSol::new("msk_test_abcdef").unwrap();
        // Smoke test — namespaces exist and the client clones cheaply.
        let _cloned = client.clone();
    }

    /// Regression: since 2026-08-27 `POST /stream/token` returns
    /// `expires_at: null` / `next_refresh_at: null` (stream tokens never
    /// expire) plus `rotated` / `lifetime`. 0.26.0's
    /// `expires_at: String` refused that body, so `get_token()` errored for
    /// every caller.
    #[test]
    fn stream_token_deserializes_null_expiry() {
        let t: crate::types::StreamToken = serde_json::from_str(
            r#"{"token":"abc","expires_at":null,"next_refresh_at":null,"rotated":false,
                "lifetime":"This token does not expire.",
                "ws_url":"wss://madeonsol.com/ws/v1/stream","usage":"connect"}"#,
        )
        .unwrap();
        assert_eq!(t.token, "abc");
        assert!(t.expires_at.is_none());
        assert!(t.next_refresh_at.is_none());
        assert_eq!(t.rotated, Some(false));
        assert!(t.lifetime.is_some());
        assert!(t.dex_ws_url.is_none());

        // Pre-2026-08-27 servers sent a timestamp and omitted the new fields.
        let old: crate::types::StreamToken = serde_json::from_str(
            r#"{"token":"abc","expires_at":"2026-08-28T00:00:00Z",
                "ws_url":"wss://madeonsol.com/ws/v1/stream","usage":"connect"}"#,
        )
        .unwrap();
        assert_eq!(old.expires_at.as_deref(), Some("2026-08-28T00:00:00Z"));
        assert!(old.rotated.is_none());
        assert!(old.lifetime.is_none());
    }

    /// Regression (2026-10-03 parity): the route nests the figures under
    /// `consensus`; the old flat type required a top-level `total_kol_buyers`
    /// and failed on every answer with KOL trades.
    #[test]
    fn kol_consensus_deserializes_nested_and_empty() {
        let r: crate::types::KolConsensusResponse = serde_json::from_str(
            r#"{"mint":"M","current_mc_usd":1000.5,"current_price_usd":null,
                "consensus":{"total_kol_buyers":3,"total_kol_sellers":1,"kol_exit_rate":0.33,
                "kol_any_sell_rate":0.33,"kol_exit_rate_definition":"x","complete":true,
                "truncated":false,"rows_scanned":12,"total_trades":12,"net_flow_sol":1.5,
                "total_buy_sol":2.0,"total_sell_sol":0.5,"first_kol_buy_at":null,
                "last_kol_buy_at":null,"first_touch_wallet":null,"first_touch_at":null,
                "median_entry_mc_usd":null}}"#,
        )
        .unwrap();
        assert_eq!(r.consensus.unwrap().total_kol_buyers, 3);
        let empty: crate::types::KolConsensusResponse = serde_json::from_str(
            r#"{"mint":"M","consensus":null,"total_kol_buyers":0,"total_kol_sellers":0,"complete":true}"#,
        )
        .unwrap();
        assert!(empty.consensus.is_none());
        assert_eq!(empty.total_kol_buyers, Some(0));
    }

    #[test]
    fn peak_history_deserializes_nested() {
        let r: crate::types::PeakHistoryResponse = serde_json::from_str(
            r#"{"mint":"M","found":true,"token":{"name":"n","symbol":"S","image_url":null},
                "peak_history":{"peak_mc_usd":5000.0,"bonded_at":null,"mc_tracking_complete":true}}"#,
        )
        .unwrap();
        assert_eq!(r.peak_history.unwrap().peak_mc_usd, Some(5000.0));
        let nf: crate::types::PeakHistoryResponse =
            serde_json::from_str(r#"{"mint":"M","found":false,"peak_history":null}"#).unwrap();
        assert!(!nf.found && nf.peak_history.is_none());
    }

    #[test]
    fn coverage_size_floor() {
        let c: crate::types::TradeCoverage = serde_json::from_str(
            r#"{"history_start":1775952000,"scope":"launchpad pipeline","in_scope":true,
                "size_floor":{"min_sol":0.05,"min_stable_usd":3.5,"applies_to":"buys"}}"#,
        )
        .unwrap();
        assert_eq!(c.size_floor.unwrap().min_sol, 0.05);
    }

    // ── API parity 2026-10-03, second pass: wire samples shaped like the
    //    route source on main (edaeadda).

    #[test]
    fn kol_feed_cursor_scan_stream_and_free_tier_delay() {
        let r: crate::types::KolFeedResponse = serde_json::from_str(
            r#"{"trades":[{"tx_signature":"sig","wallet_address":"W","action":"buy",
                "token_mint":"M","sol_amount":1.5,"token_amount":1000,"traded_at":"2026-10-03T08:00:00Z",
                "token":{"mint":"M","kol_activity":{"buying_kols":1,"selling_kols":0,"net_flow_sol":1.5,"signal":"neutral","top_buyers":[]}}}],
                "count":1,"data_age_seconds":3,"next_before":"2026-10-03T08:00:00Z",
                "next_cursor":"abc","has_more":true,
                "scan":{"post_filtered":true,"scanned":400,"scan_truncated":false,"scan_budget":2000},
                "next_since":"2026-10-03T08:00:00Z","since":null,
                "stream":{"channel":"kol:trades","url":"wss://madeonsol.com/ws/v1/stream",
                  "token_endpoint":"POST /api/v1/stream/token",
                  "subscribe":{"type":"subscribe","channels":["kol:trades"]},"note":"n"},
                "included":["token"],
                "include_truncated":{"token":["M2"],"note":"cap"},
                "delay":"5m","delay_seconds":300,"as_of":"2026-10-03T07:55:00Z",
                "delay_note":"Free tier is delayed 5 minutes.","upgrade":"https://madeonsol.com/pricing",
                "_rid":"r1"}"#,
        )
        .unwrap();
        assert_eq!(r.next_cursor.as_deref(), Some("abc"));
        assert_eq!(r.has_more, Some(true));
        assert_eq!(r.scan.as_ref().unwrap().scanned, 400);
        assert!(r.since.is_none());
        assert_eq!(r.stream.as_ref().unwrap().channel, "kol:trades");
        assert_eq!(r.included.as_deref(), Some(&["token".to_string()][..]));
        assert_eq!(r.include_truncated.unwrap().token, vec!["M2".to_string()]);
        assert_eq!(r.delay.delay_seconds, Some(300));
        assert_eq!(r.delay.delay.as_deref(), Some("5m"));
        assert!(r.trades[0].token.is_some());

        // Paid key, no include, older server: every new field absent.
        let paid: crate::types::KolFeedResponse =
            serde_json::from_str(r#"{"trades":[],"count":0}"#).unwrap();
        assert!(paid.next_cursor.is_none() && paid.has_more.is_none() && paid.scan.is_none());
        assert!(paid.delay.delay_seconds.is_none() && paid.stream.is_none());
    }

    #[test]
    fn feed_params_serialize_cursor_since_include() {
        use serde_json::json;
        let v = serde_json::to_value(crate::types::KolFeedParams {
            cursor: Some("abc".into()),
            since: Some("2026-10-03T00:00:00Z".into()),
            include: Some("token".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(v, json!({"cursor":"abc","since":"2026-10-03T00:00:00Z","include":"token"}));
        let v = serde_json::to_value(crate::types::DeployerAlertsParams {
            cursor: Some("c".into()),
            token_mint: Some("M".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(v, json!({"cursor":"c","token_mint":"M"}));
        let v = serde_json::to_value(crate::types::FirstTouchesParams {
            cursor: Some("c".into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(v, json!({"cursor":"c"}));
        let v = serde_json::to_value(crate::types::KolLeaderboardParams {
            offset: Some(50),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(v, json!({"offset":50}));
    }

    #[test]
    fn first_touches_cursor_fields() {
        let r: crate::types::FirstTouchesResponse = serde_json::from_str(
            r#"{"events":[],"count":0,"next_before":null,"next_cursor":null,"has_more":false,
                "next_since":null,"since":null,"data_age_seconds":null,
                "stream":{"channel":"kol:first_touches"}}"#,
        )
        .unwrap();
        assert_eq!(r.has_more, Some(false));
        assert!(r.next_cursor.is_none());
        assert_eq!(r.stream.unwrap().channel, "kol:first_touches");
    }

    #[test]
    fn kol_leaderboard_pagination_universe_entry_mc() {
        let r: crate::types::KolLeaderboardResponse = serde_json::from_str(
            r#"{"leaderboard":[{"name":"k","wallet":"W","pnl":12.5,"buy_count":3,"sell_count":2,
                  "volume":40.1,"entry_mc_samples":3,"avg_entry_mc_usd":15234},
                 {"name":null,"wallet":"W2","pnl":1,"buy_count":1,"sell_count":0,"volume":1,
                  "entry_mc_samples":null,"avg_entry_mc_usd":null}],
                "pagination":{"limit":2,"offset":0,"returned":2,"total":50,"has_more":true},
                "universe":{"kind":"top_kols_by_realized_pnl","period":"7d","max_size":50,"size":50,"note":"n"},
                "entry_mc_window_start":"2026-09-26T00:00:00Z","entry_mc_complete":true,
                "period":"7d","sort":"pnl","min_winrate":40}"#,
        )
        .unwrap();
        assert_eq!(r.pagination.unwrap().total, 50);
        assert_eq!(r.universe.unwrap().max_size, 50);
        assert_eq!(r.leaderboard[0].entry_mc_samples, Some(3));
        assert_eq!(r.leaderboard[0].avg_entry_mc_usd, Some(15234.0));
        assert!(r.leaderboard[1].entry_mc_samples.is_none());
        assert_eq!(r.min_winrate, Some(40.0));
        assert!(r.strategy.is_none());
    }

    #[test]
    fn deployer_alerts_cursor_launchpad_and_summary_counts() {
        let r: crate::types::DeployerAlertsResponse = serde_json::from_str(
            r#"{"alerts":[{"id":"a1","token_mint":"M","token_name":"n","token_symbol":"S",
                  "alert_type":"new_deploy","title":"t","message":"m","priority":"high",
                  "created_at":"2026-10-03T08:00:00Z","market_cap_at_alert":null,
                  "deployer_sol_balance":2.1,"launchpad":"pumpfun",
                  "deployers":{"wallet_address":"D","tier":"elite","total_tokens_deployed":40,
                    "total_bonded":12,"instant_bonds":2,"bonding_rate":0.3,"recent_outcomes":"BBD",
                    "recent_bond_rate":0.4,"best_token_peak_mc":900000,"runner_rate":0.5,
                    "runner_tokens":6,"labeled_tokens":12,"avg_time_to_bond_minutes":14},
                  "kol_buys":{"count":2,"total_sol":3.5,"kols":["a","b"]}}],
                "limit":20,"offset":0,"next_before":"2026-10-03T08:00:00Z","next_cursor":"c1",
                "has_more":true,"kol_buys_complete":true,
                "scan":{"post_filtered":true,"scanned":60,"scan_truncated":false,"scan_budget":1000},
                "data_age_seconds":1}"#,
        )
        .unwrap();
        let a = &r.alerts[0];
        assert_eq!(a.launchpad.as_deref(), Some("pumpfun"));
        assert_eq!(a.deployers.instant_bonds, Some(2));
        assert_eq!(a.deployers.runner_tokens, Some(6));
        assert_eq!(r.next_cursor.as_deref(), Some("c1"));
        assert_eq!(r.kol_buys_complete, Some(true));
        assert!(r.scan.is_some() && r.delay.as_of.is_none());

        // Empty-mint answer from the route.
        let e: crate::types::DeployerAlertsResponse = serde_json::from_str(
            r#"{"alerts":[],"limit":20,"offset":0,"next_before":null,"next_cursor":null,
                "has_more":false,"kol_buys_complete":true,"data_age_seconds":null}"#,
        )
        .unwrap();
        assert_eq!(e.has_more, Some(false));
    }

    #[test]
    fn deployer_profile_response_both_answers() {
        let r: crate::types::DeployerProfileResponse = serde_json::from_str(
            r#"{"is_deployer":true,
                "deployer":{"id":"7c1","wallet_address":"D","total_tokens_deployed":40,"total_bonded":12,
                  "instant_bonds":2,"bonding_rate":0.3,"recent_bond_rate":0.4,"tier":"elite","is_tracked":true,
                  "avg_time_to_bond_minutes":14.5,"best_token_peak_mc":900000,"avg_peak_mc":120000.5,
                  "recent_outcomes":"BBD","runner_rate":0.5,"runner_tokens":6,"labeled_tokens":12,
                  "post_bond_survival_rate":null,"post_bond_2x_rate":null,"post_bond_labeled_count":0,
                  "first_seen_at":"2026-01-01T00:00:00Z","last_deploy_at":null,"last_bond_at":null,"label":null},
                "pump_stats":{"total":40,"bonded":12,"bondingRate":0.3,"bestAthMc":900000,"avgAthMc":50000.25},
                "pump_tokens":[{"mint":"P","complete":true}],"pump_error":false,
                "launchpad_tokens":[{"mint":"L","name":null,"symbol":null,"launchpad":"launchlab",
                  "complete":false,"deployed_at":"2026-10-01T00:00:00Z","bonded_at":null,"peak_market_cap":null}],
                "funding":{"source":"x"}}"#,
        )
        .unwrap();
        assert!(r.is_deployer);
        let d = r.deployer.unwrap();
        assert_eq!(d.instant_bonds, Some(2));
        assert_eq!(d.avg_time_to_bond_minutes, Some(14.5));
        assert_eq!(r.pump_stats.unwrap().bonding_rate, 0.3);
        assert_eq!(r.pump_error, Some(false));
        assert_eq!(r.launchpad_tokens[0].launchpad, "launchlab");
        assert!(r.funding.is_some() && r.funding_features.is_none());

        let nd: crate::types::DeployerProfileResponse = serde_json::from_str(
            r#"{"is_deployer":false,"wallet":"W","deployer":null,"pump_stats":null,
                "pump_tokens":[],"pump_error":null,"launchpad_tokens":[]}"#,
        )
        .unwrap();
        assert!(!nd.is_deployer && nd.deployer.is_none() && nd.pump_error.is_none());
        assert_eq!(nd.wallet.as_deref(), Some("W"));
    }

    #[test]
    fn wallet_tracker_add_update_remove_shapes() {
        let add: crate::types::WatchlistAddResponse = serde_json::from_str(
            r#"{"wallet":{"wallet_address":"W","label":"me","added_at":"2026-10-03T08:00:00Z"}}"#,
        )
        .unwrap();
        assert_eq!(add.wallet.wallet_address, "W");
        let upd: crate::types::WatchlistUpdateResponse = serde_json::from_str(
            r#"{"wallet":{"wallet_address":"W","label":null,"added_at":"2026-10-03T08:00:00Z"}}"#,
        )
        .unwrap();
        assert!(upd.wallet.label.is_none());
        let rm: crate::types::WalletTrackerDeleteResponse =
            serde_json::from_str(r#"{"removed":"W"}"#).unwrap();
        assert_eq!(rm.removed, "W");
    }

    #[test]
    fn webhook_shapes_use_is_active() {
        let list: crate::types::WebhookListResponse = serde_json::from_str(
            r#"{"webhooks":[{"id":7,"url":"https://x.example/h","events":["kol:trade"],"filters":null,
                "is_active":true,"created_at":"2026-10-01T00:00:00Z","last_delivered_at":null,
                "consecutive_failures":0,
                "delivery_summary":{"total_24h":0,"success_24h":0,"failed_24h":0,"success_rate":100}}]}"#,
        )
        .unwrap();
        let w = &list.webhooks[0];
        assert!(w.is_active);
        assert_eq!(w.delivery_summary.as_ref().unwrap().success_rate, 100.0);

        let created: crate::types::WebhookCreateResponse = serde_json::from_str(
            r#"{"webhook":{"id":8,"url":"https://x.example/h","secret":"s3cr3t","events":["deployer:alert"],
                "filters":{"min_sol":1},"is_active":true,"created_at":"2026-10-03T00:00:00Z"},
                "note":"Save the secret — it will not be shown again."}"#,
        )
        .unwrap();
        assert_eq!(created.webhook.secret, "s3cr3t");

        let got: crate::types::WebhookGetResponse = serde_json::from_str(
            r#"{"webhook":{"id":8,"url":"https://x.example/h","events":["deployer:alert"],"filters":null,
                "is_active":false,"created_at":"2026-10-03T00:00:00Z","last_delivered_at":"2026-10-03T01:00:00Z",
                "consecutive_failures":3},
                "recent_deliveries":[{"event_type":"deployer:alert","status_code":null,"response_time_ms":null,
                  "delivered_at":"2026-10-03T01:00:00Z","error":"timeout"}]}"#,
        )
        .unwrap();
        assert!(!got.webhook.is_active);
        assert_eq!(got.recent_deliveries[0].error.as_deref(), Some("timeout"));

        let upd: crate::types::WebhookUpdateResponse = serde_json::from_str(
            r#"{"webhook":{"id":8,"url":"https://x.example/h","events":["deployer:alert"],"filters":null,
                "is_active":true,"updated_at":"2026-10-03T02:00:00Z"}}"#,
        )
        .unwrap();
        assert_eq!(upd.webhook.updated_at, "2026-10-03T02:00:00Z");

        let del: crate::types::WebhookDeleteResponse =
            serde_json::from_str(r#"{"deleted":true}"#).unwrap();
        assert!(del.deleted);

        let body = serde_json::to_value(crate::types::WebhookUpdateParams {
            is_active: Some(false),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(body, serde_json::json!({ "is_active": false }));
    }
}
